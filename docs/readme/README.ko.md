# San USB Viewer

**한국어** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> 기계 번역; 수정을 환영합니다.

**현재 버전: `0.2.0`** — MIT 라이센스.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

어두운 인터페이스, 시각적 토폴로지 및 판독 가능한 진단 기능을 갖춘 **Rust**로 작성된 기본 USB 탐색기. Windows 10/11용 첫 번째 버전입니다.

## 프로젝트의 유래

프로젝트 원본 영상: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

이 프로젝트는 Apple의 **WhatCable** 및 Windows의 **USB Device Tree Viewer** 사용에서 영감을 받아 주변 장치, 허브 및 USB 연결 속도를 이해합니다. 이는 독립적인 구현입니다.

## 특징

- `nusb`를 통해 USB 장치 및 컨트롤러를 실제로 감지합니다.
- 허브 깊이가 포함된 버스 및 포트 경로별 트리 구조입니다.
- 이름, 일련번호 또는 VID:PID로 검색하세요. USB 3+ 필터 및 허브.
- 협상된 링크 속도: 1.5 / 12 / 480Mb/s, 5Gb/s,
또는 카테고리 SuperSpeedPlus(10Gb/s 이상).
- 여정 다이어그램: 컴퓨터, 중간 허브, 링크 및 장치.
- Windows에서는 IOCTL이 허브를 추가로 조사하여
장치에서 발표한 SuperSpeed 기능입니다. USB 2.0을 통해 연결된 SuperSpeed 호환 장치는 경로 제한 진단을 생성합니다.   USB 3 허브의 USB 2 허브 컴패니언은 480Mb/s 분기가 케이블 결함으로 인한 것을 방지하기 위해 별도로 취급됩니다.
- VID/PID 식별자, USB 설명자, 개정판, 드라이버, 직렬 및 인터페이스,
시스템이 이 정보를 노출할 때.
- 3초마다 자동 업데이트하거나 F5를 눌러 수동으로 업데이트합니다.
- 최근 관찰된 200개의 변경 사항에 대한 세션 로그입니다.
- 전체 문을 JSON(Ctrl+E)으로 내보내고 선택한 세부정보를 복사합니다.
- 통합 가이드 및 명시적 데모 모드, 백업으로 사용되지 않음
감지 실패 시 침묵합니다.

## 던지다

Rust **1.95+**를 사용하면 Windows에서 MSVC 도구 체인 및 Visual Studio C++ 도구를 사용할 수 있습니다. 예제에서는 RTK를 사용하여 명령 출력을 필터링합니다. RTK가 없으면 `rtk` 접두사를 제거하면 됩니다. 배포 스크립트는 두 경우를 모두 허용합니다.

```powershell
rtk cargo run --release
```

독립 실행형 바이너리는 `target/release/san-usb-viewer.exe`에서 생성됩니다. `build.ps1` 스크립트는 또한 `dist/SanUsbViewer/`에 배포판을 생성합니다. 인벤토리를 위해 드라이버 수정이나 WinUSB 설치가 필요하지 않습니다. 애플리케이션에는 장치 인터페이스가 필요하지 않습니다.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

캡처 대상 폴더가 존재해야 합니다. 릴리스 버전은 콘솔이 없는 Windows 애플리케이션입니다. 명령줄 인벤토리 개발에서는 `cargo run`를 선호합니다.

## 결과 해석

**협상 속도**는 연결 속도이며 복사 속도의 기준이 아닙니다. 유용한 속도는 특히 프로토콜, 주변 장치 및 버스를 공유하는 기타 장치에 따라 달라집니다. 여러 장치의 속도는 사용 가능한 대역폭을 계산하는 데 합산되지 않습니다.

패시브 USB 케이블은 일반적으로 열거 가능한 장치가 아닙니다. 이 버전은 e-마커, USB Power Delivery 전원, 커넥터 유형을 읽지 않습니다. 따라서 최대 속도를 인증한다고 주장하지 않습니다. 진단은 관찰된 링크를 기반으로 하며, 가능한 경우 Windows 표시기를 기반으로 합니다. 느린 링크는 케이블, 포트, 허브 또는 장치에 의해 제한될 수 있습니다. USB 설명자 버전 번호는 최대 속도로 사용되지 않습니다.

SuperSpeedPlus 범주는 여기서 10Gb/s 모드와 20Gb/s 모드를 구분하지 않습니다. USB4/Thunderbolt 및 해당 터널은 검사되지 않습니다. 장치가 연결되지 않은 포트는 인벤토리에 포함되지 않습니다. 데이터가 없는 충전 케이블은 눈에 보이는 장치를 생성하지 않습니다. 내부 장치(블루투스, 내장 카메라 등)가 USB 트리에 나타날 수 있습니다.

접근할 수 없는 정보는 접근할 수 없는 것으로 표시됩니다. 제조업체와 인터페이스가 Windows에 항상 노출되는 것은 아닙니다. 스캔 사이에 완전히 발생하는 이벤트는 눈에 띄지 않을 수 있습니다. 로그는 메모리에 있으며 첫 번째 성공적인 인벤토리 이후에 시작됩니다.

버스 식별자와 포트 경로는 시스템 토폴로지를 설명합니다. 물리적 커넥터에는 USB 2 및 SuperSpeed에 대한 두 개의 별도 경로가 있을 수 있습니다.

## 아키텍처 및 검증

`scanner.rs`는 인벤토리를 수행합니다. `windows_probe.rs`는 허브의 기본 읽기 요청을 캡슐화합니다. `model.rs`에는 데이터 및 진단이 포함되어 있습니다. `app.rs`는 백그라운드 작업과 선택을 처리합니다. `views.rs`, `inspector.rs`, `connection.rs` 및 `theme.rs`는 egui/eframe을 사용하여 인터페이스를 그립니다. 인터페이스는 스캔 중에 대화형 상태를 유지하며 다음 스캔이 실패할 경우 마지막으로 성공한 읽기를 유지합니다.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

라이브러리는 크로스 플랫폼이며 Windows 코드는 조건부입니다. 이번 첫 배송에서는 macOS/Linux 버전이 검증되지 않았습니다.

## 기밀성

인벤토리는 원격 측정이나 네트워크 서비스 없이 로컬에서 읽기 전용으로 작동합니다. 가이드 링크는 클릭 시에만 열립니다. JSON 보고서에는 일련번호와 시스템 식별자가 포함될 수 있습니다. 공유 여부는 귀하에게 달려 있습니다.

## 기술 참고자료

- [nusb: USB 열거 및 정보](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [마이크로소프트: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

MIT 라이센스. 별채에는 해당 라이센스가 적용됩니다.

공개 저장소에는 컴파일에 필요한 소스, 번역 및 무료 글꼴만 포함되어 있습니다. USB 판독, 진단 캡처, 내보내기, 로컬 기본 설정, 캐시 및 개발 바이너리는 `.gitignore`에서 제외됩니다. 문제를 보고하려면 `--demo` 모드를 사용하거나 보고서를 공유하기 전에 보고서에서 일련 번호와 시스템 ID를 제거하십시오.

## 언어 — 버전 0.2.0

이 애플리케이션에는 **50개의 인터페이스 언어**가 포함되어 있습니다. 오른쪽 상단의 언어 코드(`FR`, `EN` 등)를 클릭하여 스위처를 엽니다. 모국어 이름, 영어 이름 또는 코드로 언어를 검색할 수 있습니다. 변경 사항은 즉시 적용되며 선택 사항은 Windows의 `%APPDATA%/SanUsbViewer/preferences.json`에 저장됩니다. 기본 설정이 없으면 시스템 언어가 사용됩니다. 지원되지 않는 언어는 영어로 되돌아갑니다.

선택에서는 지역적 변형을 함께 그룹화하여 전체적으로 가장 많이 사용되는 언어(모국어 및 제2 언어)를 선호합니다. 이것은 숫자로 분류한 것이 아닙니다. 방언을 그룹화하는 방법에 따라 결과가 달라집니다. 선택된 변형에는 중국어 간체, 표준 아랍어 및 풀라니어와 우즈베크어의 라틴 문자가 포함됩니다. 펀자브어는 구르무키어를 사용합니다.

영어, 중국어, 힌디어, 스페인어, 프랑스어, 아랍어, 벵골어, 포르투갈어, 러시아어, 우르두어, 인도네시아어, 독일어, 일본어, 마라티어, 베트남어, 텔루구어, 하우사어, 터키어, 펀잡어, 스와힐리어, 필리핀어, 타밀어, 페르시아어, 한국어, 암하라어, 태국어, 자바어, 이탈리아어, 구자라트어, 칸나다어, 보즈푸리어, 버마어, 폴란드어, 요루바어, 링갈라어, 오로모어, 풀라니어, 말라얄람어, 오디아, 우크라이나어, 신디어, 우즈베크어, 파슈토어, 순다어, 말레이어, 이그보어, 네팔어, 아제르바이잔어, 마이틸리어, 소말리아어.

복사된 메뉴, 진단, 도움말 텍스트, 활동 메시지 및 세부 정보가 현지화되었습니다. 하드웨어와 시스템에서 제공하는 이름과 식별자는 유지됩니다. JSON 내보내기는 표시 언어에 관계없이 스키마 및 인벤토리 데이터를 유지합니다. 기술 단위는 바이트와의 혼동을 피하기 위해 **비트/초**(Mbit/s 및 Gbit/s)로 유지됩니다.

프랑스어 및 영어 카탈로그는 참고 자료로 사용됩니다. 다른 언어는 **자동번역**이므로 언어적 수정이 필요할 수 있습니다. `locales/*.json` 파일은 컴파일에 포함되어 수정된 후 다시 컴파일될 수 있습니다. 애플리케이션에서는 네트워크 번역을 수행하지 않습니다. SIL 오픈 글꼴 라이센스에 따라 50개 언어와 Noto 글꼴이 내장되어 있습니다. 아랍어 텍스트는 서식 지정 전에 계산된 양방향 유니코드 세그먼트와 개행을 사용합니다.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG`는 테스트 또는 휴대용 설치를 위해 다른 기본 설정 파일을 참조할 수 있습니다. 여기에는 언어 코드만 저장됩니다.
