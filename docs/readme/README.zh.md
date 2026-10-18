# San USB Viewer

**中文** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> 机器翻译；欢迎指正。

**当前版本：`0.2.0`** — MIT 许可证。

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

用 **Rust** 编写的本机 USB 资源管理器，具有深色界面、可视拓扑和可读诊断。第一个版本适用于 Windows 10/11。

## 项目起源

项目来源视频：[https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus)。

该项目的灵感来自于 Apple 上 **WhatCable** 和 Windows 上 **USB Device Tree Viewer** 的使用：了解外围设备、集线器及其 USB 连接的速度。这是一个独立的实现。

## 特征

- 通过`nusb`实际检测USB设备和控制器。
- 按总线和端口路径划分的树结构，具有枢纽深度。
- 按名称、序列号或 VID:PID 搜索； USB 3+ 过滤器和集线器。
- 协商链接速度：1.5 / 12 / 480 Mb/s、5 Gb/s、
或类别 SuperSpeedPlus（10 Gb/s 或更高）。
- 旅程图：计算机、中间集线器、链路和设备。
- 在 Windows 下，通过 IOCTL 对集线器进行额外询问以找出
该设备宣布的 SuperSpeed 功能。通过 USB 2.0 连接的 SuperSpeed 兼容设备会产生路径限制诊断。   USB 3 集线器的 USB 2 集线器同伴被单独处理，以避免将其 480 Mb/s 分支归因于电缆故障。
- VID/PID 标识符、USB 描述符、版本、驱动程序、串行和接口、
当系统公开此信息时。
- 每三秒自动更新一次，或使用 F5 手动更新。
- 最近 200 个观察到的更改的会话日志。
- 以 JSON (Ctrl+E) 格式导出完整的语句并复制选定的详细信息。
- 集成指南和显式演示模式，绝不用作备份
检测失败时保持沉默。

## 扔

使用 Rust **1.95+**、Windows 上的 MSVC 工具链和 Visual Studio C++ 工具。示例使用 RTK 来过滤命令的输出；没有RTK，只需删除`rtk`前缀。分发脚本接受这两种情况。

```powershell
rtk cargo run --release
```

独立的二进制文件在 `target/release/san-usb-viewer.exe` 中生成。 `build.ps1` 脚本还在 `dist/SanUsbViewer/` 中创建一个发行版。清单无需修改驱动程序或安装 WinUSB：应用程序不需要设备接口。

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

捕获的目标文件夹必须存在。发布版本是一个无控制台的 Windows 应用程序；在开发命令行清单时更喜欢 `cargo run`。

## 解释结果

**协商速度**是连接速度，而不是复制速度的基准。有用的速度尤其取决于协议、外设和共享总线的其他设备。多个设备的速度不会相加来计算可用带宽。

无源 USB 电缆通常不是可枚举设备。此版本不读取其电子标记、USB 供电功率及其连接器类型。因此，它并未声称证明其最大速度。诊断基于观察到的链接以及 Windows 指示器（如果有）。慢速链路可能受到电缆、端口、集线器或设备的限制。 USB 描述符版本号从不用作最大速度。

SuperSpeedPlus 类别此处不区分 10 和 20 Gb/s 模式。 USB4/Thunderbolt 及其隧道未经过检查。没有连接设备的端口不会被清点。没有数据的充电线不会创建可见的设备。内部设备（蓝牙、内置摄像头等）可能会出现在 USB 树中。

无法访问的信息显示为不可用。 Windows 并不总是公开制造商和接口。完全在扫描之间发生的事件可能会被忽视。该日志位于内存中，并在第一次成功盘点后开始。

总线标识符和端口路径描述了系统拓扑。物理连接器可以有两个单独的路径用于 USB 2 和 SuperSpeed。

## 架构和验证

`scanner.rs`进行盘点； `windows_probe.rs`封装来自集线器的本机读请求； `model.rs` 包含数据和诊断。 `app.rs` 处理后台任务和选择； `views.rs`、`inspector.rs`、`connection.rs`和`theme.rs`用egui/eframe绘制界面。该界面在扫描期间保持交互状态，并在下一次扫描失败时保留上次成功的读取。

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

这些库是跨平台的，并且 Windows 代码是有条件的。 macOS/Linux 版本在首次交付中未经过验证。

## 保密性

库存在本地运行，只读，无需遥测或网络服务。指南链接仅在单击时打开。 JSON 报告可以包含序列号和系统标识符：共享它们取决于您。

## 技术参考

- [nusb：USB 枚举和信息](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [微软：IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

麻省理工学院许可证。附属建筑仍须遵守各自的许可证。

公共存储库仅包含编译所需的源代码、翻译和免费字体。 `.gitignore` 排除 USB 读数、诊断捕获、导出、本地首选项、缓存和开发二进制文件。要报告问题，请使用模式 `--demo` 或在共享报告之前从报告中删除序列号和系统 ID。

## 语言——版本0.2.0

该应用程序包括 **50 种界面语言**。单击右上角的语言代码（`FR`、`EN` 等）可打开切换器。您可以通过语言的本机名称、英文名称或代码来搜索语言。更改是立即的，并且选择存储在 Windows 下的 `%APPDATA%/SanUsbViewer/preferences.json` 中。无偏好则使用系统语言；不受支持的语言将恢复为英语。

该选择偏向于总体上使用最多的语言（母语和第二语言），并将区域变体分组在一起。这不是数字分类：结果因方言分组而异。所选的变体包括简体中文、标准阿拉伯语以及富拉尼语和乌兹别克语的拉丁文字；旁遮普语使用古尔穆基语。

英语、中文、印地语、西班牙语、法语、阿拉伯语、孟加拉语、葡萄牙语、俄语、乌尔都语、印度尼西亚语、德语、日语、马拉地语、越南语、泰卢固语、豪萨语、土耳其语、旁遮普语、斯瓦希里语、菲律宾语、泰米尔语、波斯语、韩语、阿姆哈拉语、泰语、爪哇语、意大利语、古吉拉特语、卡纳达语、博杰普尔语、缅甸语、波兰语、约鲁巴语、林加拉语、奥罗莫语、富拉尼语、马拉雅拉姆语、奥迪亚语、乌克兰语、信德语、乌兹别克语、普什图语、巽他语、马来语、伊博语、尼泊尔语、阿塞拜疆语、迈蒂利语和索马里语。

复制的菜单、诊断、帮助文本、活动消息和详细信息均已本地化。保留硬件和系统提供的名称和标识符。无论显示语言如何，JSON 导出都会保留其架构和库存数据。技术单位仍为 **bit/s**（Mbit/s 和 Gbit/s），以避免与字节混淆。

法文和英文目录可供参考。其他语言是**自动翻译**，可能需要语言更正。 `locales/*.json`文件包含在编译中，可以修正后重新编译。应用程序不进行网络翻译：在 SIL 开放字体许可证下嵌入了 50 种语言和 Noto 字体。阿拉伯文本使用格式化之前计算的双向 Unicode 段和换行符。

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` 可以引用另一个首选项文件进行测试或便携式安装。那里只保存语言代码。
