//! Small local preferences file. No USB inventory is persisted here.
use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf};

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Preferences {
    pub(crate) language: Option<String>,
}

fn path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("SAN_USB_VIEWER_CONFIG") {
        return Some(path.into());
    }
    let base = if cfg!(windows) {
        std::env::var_os("APPDATA")
    } else {
        std::env::var_os("XDG_CONFIG_HOME").or_else(|| {
            std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config").into_os_string())
        })
    };
    base.map(|p| PathBuf::from(p).join("SanUsbViewer/preferences.json"))
}

pub(crate) fn load() -> Preferences {
    path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

pub(crate) fn save(code: &str) -> io::Result<()> {
    let path = path()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No configuration directory"))?;
    save_to(&path, code)
}

fn save_to(path: &std::path::Path, code: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(&Preferences {
        language: Some(code.into()),
    })?;
    std::fs::write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preference_round_trip_contains_only_the_language() -> Result<(), Box<dyn std::error::Error>>
    {
        let path =
            std::env::temp_dir().join(format!("san-usb-language-{}.json", std::process::id()));
        save_to(&path, "ja")?;
        let text = std::fs::read_to_string(&path)?;
        std::fs::remove_file(path)?;
        let pref: Preferences = serde_json::from_str(&text)?;
        assert_eq!(pref.language.as_deref(), Some("ja"));
        assert!(!text.contains("devices"));
        Ok(())
    }
}
