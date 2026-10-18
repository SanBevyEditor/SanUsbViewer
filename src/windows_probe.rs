//! Read-only Windows hub queries. No device interface is claimed or reconfigured.
use crate::model::UsbCapabilities;
use std::{
    ffi::{OsStr, c_void},
    mem::{size_of, zeroed},
    os::windows::{
        ffi::OsStrExt,
        io::{AsRawHandle, FromRawHandle, OwnedHandle},
    },
    ptr,
};
use windows_sys::Win32::{
    Devices::{
        DeviceAndDriverInstallation::{
            CM_GET_DEVICE_INTERFACE_LIST_PRESENT, CM_Get_Device_Interface_List_SizeW,
            CM_Get_Device_Interface_ListW, CR_BUFFER_SMALL, CR_SUCCESS,
        },
        Usb::{
            GUID_DEVINTERFACE_USB_HUB, IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2,
            USB_NODE_CONNECTION_INFORMATION_EX_V2,
        },
    },
    Foundation::{GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING},
    System::IO::DeviceIoControl,
};

pub(crate) fn capabilities(parent: &OsStr, port: u32) -> Option<UsbCapabilities> {
    if port == 0 {
        return None;
    }
    let path = hub_interface(parent)?;
    // SAFETY: path is a NUL-terminated UTF-16 string. No security attributes or
    // template handle are passed; the returned valid handle is owned below.
    let raw = unsafe {
        CreateFileW(
            path.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            ptr::null(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };
    if raw == INVALID_HANDLE_VALUE {
        return None;
    }
    // SAFETY: CreateFileW returned a new, valid handle; OwnedHandle closes it once.
    let handle = unsafe { OwnedHandle::from_raw_handle(raw) };
    // SAFETY: this Windows POD structure consists solely of integers and unions
    // of integers. Zero is a valid initial representation of every field.
    let mut info: USB_NODE_CONNECTION_INFORMATION_EX_V2 = unsafe { zeroed() };
    info.ConnectionIndex = port;
    info.Length = size_of::<USB_NODE_CONNECTION_INFORMATION_EX_V2>() as u32;
    info.SupportedUsbProtocols.ul = 7;
    let mut returned = 0;
    // SAFETY: the input and output share the same initialized, correctly sized
    // IOCTL buffer, as required by METHOD_BUFFERED. The synchronous call finishes
    // before the buffer or owned hub handle can be dropped.
    let success = unsafe {
        DeviceIoControl(
            handle.as_raw_handle(),
            IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2,
            (&raw const info).cast::<c_void>(),
            size_of_val(&info) as u32,
            (&raw mut info).cast::<c_void>(),
            size_of_val(&info) as u32,
            &mut returned,
            ptr::null_mut(),
        )
    };
    if success == 0 || returned < size_of_val(&info) as u32 {
        return None;
    }
    // SAFETY: ul is the documented integer representation of these flag unions,
    // and the successful IOCTL initialized the entire structure.
    let (flags, protocols) = unsafe { (info.Flags.ul, info.SupportedUsbProtocols.ul) };
    Some(decode_flags(flags, protocols))
}

fn decode_flags(flags: u32, protocols: u32) -> UsbCapabilities {
    UsbCapabilities {
        device_super_speed: flags & 0x02 != 0,
        device_super_speed_plus: flags & 0x08 != 0,
        port_usb3_protocol: protocols & 0x04 != 0,
    }
}

fn hub_interface(parent: &OsStr) -> Option<Vec<u16>> {
    let id: Vec<u16> = parent.encode_wide().chain(Some(0)).collect();
    // A hot-unplug can change the buffer size. Retry a bounded number of times.
    for _ in 0..3 {
        let mut length = 0;
        // SAFETY: the GUID and NUL-terminated instance ID remain valid for the
        // duration of the call, and length is an initialized writable u32.
        let result = unsafe {
            CM_Get_Device_Interface_List_SizeW(
                &mut length,
                &GUID_DEVINTERFACE_USB_HUB,
                id.as_ptr(),
                CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
            )
        };
        if result != CR_SUCCESS || !(2..=65536).contains(&length) {
            return None;
        }
        let mut buffer = vec![0u16; length as usize];
        // SAFETY: buffer holds length writable UTF-16 units. All pointers refer
        // to live storage; the API never retains them.
        let result = unsafe {
            CM_Get_Device_Interface_ListW(
                &GUID_DEVINTERFACE_USB_HUB,
                id.as_ptr(),
                buffer.as_mut_ptr(),
                length,
                CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
            )
        };
        if result == CR_BUFFER_SMALL {
            continue;
        }
        if result != CR_SUCCESS {
            return None;
        }
        let end = buffer.iter().position(|c| *c == 0)?;
        if end == 0 {
            return None;
        }
        buffer.truncate(end + 1);
        return Some(buffer);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_capability_is_independent_of_operating_flags() {
        let capabilities = decode_flags(0x02 | 0x08, 0x02);
        assert!(
            capabilities.device_super_speed
                && capabilities.device_super_speed_plus
                && !capabilities.port_usb3_protocol
        );
    }

    #[test]
    fn operating_flag_alone_does_not_imply_a_capability_flag() {
        assert!(!decode_flags(0x01, 0x04).device_super_speed);
    }
}
