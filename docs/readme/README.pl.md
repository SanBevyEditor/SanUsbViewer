# San USB Viewer

**Polski** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Tłumaczenia maszynowe; poprawki mile widziane.

**Aktualna wersja: `0.2.0`** — licencja MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Natywny Eksplorator USB napisany w **Rust**, z ciemnym interfejsem, topologią wizualną i czytelną diagnostyką. Pierwsza wersja przeznaczona dla systemu Windows 10/11.

## Geneza projektu

Film u początków projektu: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Projekt inspirowany jest zastosowaniami **WhatCable** na Apple i **USB Device Tree Viewer** na Windows: zrozumienie urządzeń peryferyjnych, koncentratorów i szybkości ich połączeń USB. Jest to niezależna implementacja.

## Cechy

- Rzeczywiste wykrywanie urządzeń USB i kontrolerów za pomocą `nusb`.
- Struktura drzewa według ścieżki autobusowej i portu, z głębokością piasty.
- Szukaj według nazwy, numeru seryjnego lub VID:PID; Filtry i koncentratory USB 3+.
- Negocjowana prędkość łącza: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
lub kategoria SuperSpeedPlus (10 Gb/s lub więcej).
- Schemat podróży: komputer, koncentratory pośrednie, łącze i urządzenie.
- W systemie Windows dodatkowe przesłuchanie koncentratora przez IOCTL w celu znalezienia
Możliwości SuperSpeed ogłoszone przez urządzenie. Urządzenie kompatybilne z SuperSpeed podłączone przez USB 2.0 generuje diagnostykę ograniczeń ścieżki.   Koncentratory USB 2 współpracujące z koncentratorami USB 3 są traktowane oddzielnie, aby uniknąć przypisania ich rozgałęzienia 480 Mb/s usterce kabla.
- Identyfikatory VID/PID, deskryptor USB, wersja, sterownik, numer seryjny i interfejsy,
kiedy system ujawni te informacje.
- Automatyczna aktualizacja co trzy sekundy lub ręczna za pomocą klawisza F5.
- Dziennik sesji ostatnich 200 zaobserwowanych zmian.
- Wyeksportuj całą instrukcję w formacie JSON (Ctrl+E) i skopiuj wybrane szczegóły.
- Zintegrowany przewodnik i wyraźny tryb demonstracyjny, nigdy nie używany jako kopia zapasowa
cichy w przypadku niepowodzenia wykrywania.

## Rzucić

Z Rust **1.95+**, łańcuchem narzędzi MSVC i narzędziami Visual Studio C++ w systemie Windows. W przykładach zastosowano RTK do filtrowania danych wyjściowych poleceń; bez RTK, po prostu usuń prefiks `rtk`. Skrypt dystrybucyjny akceptuje oba przypadki.

```powershell
rtk cargo run --release
```

Samodzielny plik binarny jest produkowany w `target/release/san-usb-viewer.exe`. Skrypt `build.ps1` tworzy również dystrybucję w `dist/SanUsbViewer/`. Do inwentaryzacji nie jest konieczna modyfikacja sterownika ani instalacja WinUSB: aplikacja nie wymaga interfejsów urządzeń.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Musi istnieć folder docelowy przechwytywania. Wersja wydana to aplikacja Windows bez konsoli; preferuj `cargo run` w fazie rozwoju dla inwentarza wiersza poleceń.

## Zinterpretuj wyniki

**Wynegocjowana prędkość** jest szybkością połączenia, a nie wskaźnikiem szybkości kopiowania. Użyteczne prędkości zależą w szczególności od protokołu, urządzenia peryferyjnego i innych urządzeń współdzielących magistralę. Prędkości z wielu urządzeń nie sumują się przy obliczaniu dostępnej przepustowości.

Pasywny kabel USB z reguły nie jest urządzeniem policzalnym. Ta wersja nie czyta swojego e-markera, mocy USB Power Delivery ani typu złącza. Dlatego nie rości sobie prawa do poświadczenia swojej maksymalnej prędkości. Diagnoza opiera się na zaobserwowanym powiązaniu i, jeśli są dostępne, na wskaźnikach Windows. Wolne łącze może być ograniczone przez kabel, port, koncentrator lub urządzenie. Numer wersji deskryptora USB nigdy nie jest używany jako maksymalna prędkość.

Kategoria SuperSpeedPlus nie rozróżnia tutaj trybów 10 i 20 Gb/s. USB4/Thunderbolt i ich tunele nie są sprawdzane. Porty bez podłączonego urządzenia nie są inwentaryzowane. Kabel ładujący bez danych nie tworzy widocznego urządzenia. Urządzenia wewnętrzne (Bluetooth, wbudowana kamera itp.) mogą pojawić się w drzewie USB.

Niedostępne informacje są wyświetlane jako niedostępne. Producent i interfejsy nie zawsze są ujawniane przez system Windows. Zdarzenia, które mają miejsce całkowicie pomiędzy skanami, mogą pozostać niezauważone. Dziennik znajduje się w pamięci i rozpoczyna się po pierwszej pomyślnej inwentaryzacji.

Identyfikatory magistrali i ścieżki portów opisują topologię systemu. Złącze fizyczne może mieć dwie oddzielne ścieżki dla USB 2 i SuperSpeed.

## Architektura i walidacja

`scanner.rs` wykonuje inwentaryzację; `windows_probe.rs` hermetyzuje natywne żądania odczytu z koncentratorów; `model.rs` zawiera dane i diagnostykę. `app.rs` obsługuje zadania w tle i selekcję; `views.rs`, `inspector.rs`, `connection.rs` i `theme.rs` rysują interfejs za pomocą egui/eframe. Interfejs pozostaje interaktywny podczas skanowania i zachowuje ostatni pomyślny odczyt, jeśli kolejne skanowanie się nie powiedzie.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Biblioteki są wieloplatformowe, a kod systemu Windows jest warunkowy. Wersje systemu macOS/Linux nie są sprawdzane w ramach tej pierwszej dostawy.

## Poufność

Inwentarz działa lokalnie, tylko do odczytu, bez telemetrii i usług sieciowych. Linki do przewodnika otwierają się tylko po kliknięciu. Raporty JSON mogą zawierać numery seryjne i identyfikatory systemu: udostępnienie ich zależy od Ciebie.

## Referencje techniczne

- [nusb: wyliczenie i informacje o USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

Licencja MIT. Budynki gospodarcze nadal podlegają odpowiednim pozwoleniom.

Repozytorium publiczne zawiera wyłącznie źródła, tłumaczenia i darmowe czcionki niezbędne do kompilacji. Odczyty USB, przechwytywania diagnostyczne, eksporty, preferencje lokalne, pamięci podręczne i pliki binarne programistyczne są wykluczone przez `.gitignore`. Aby zgłosić problem, użyj trybu `--demo` lub usuń numery seryjne i identyfikatory systemu ze swoich raportów przed ich udostępnieniem.

## Języki — wersja 0.2.0

Aplikacja zawiera **50 języków interfejsu**. Kliknij kod języka (`FR`, `EN` itp.) w prawym górnym rogu, aby otworzyć przełącznik. Możesz wyszukiwać język według jego nazwy rodzimej, nazwy angielskiej lub kodu. Zmiana jest natychmiastowa, a wybór jest przechowywany w `%APPDATA%/SanUsbViewer/preferences.json` pod Windows. Bez preferencji używany jest język systemowy; Nieobsługiwany język zostanie przywrócony do języka angielskiego.

Wybór faworyzuje łącznie najczęściej używane języki (język ojczysty i drugi język), grupując warianty regionalne. To nie jest klasyfikacja numeryczna: wyniki różnią się w zależności od grupy dialektów. Wybrane warianty obejmują chiński uproszczony, standardowy język arabski oraz alfabety łacińskie w językach Fulani i Uzbecki; Pendżabski używa Gurmukhi.

Angielski, chiński, hindi, hiszpański, francuski, arabski, bengalski, portugalski, rosyjski, urdu, indonezyjski, niemiecki, japoński, marathi, wietnamski, telugu, hausa, turecki, pendżabski, suahili, filipiński, tamilski, perski, koreański, amharski, tajski, jawajski, włoski, gudżarati, kannada, Bhojpuri, birmański, polski, joruba, lingala, oromo, fulani, Malajalam, orija, ukraiński, sindhi, uzbecki, paszto, sundajski, malajski, igbo, nepalski, azerski, maithili i somalijski.

Skopiowane menu, diagnostyka, teksty pomocy, komunikaty o aktywności i szczegóły są zlokalizowane. Nazwy i identyfikatory dostarczone przez sprzęt i system zostają zachowane. Eksport JSON zachowuje swój schemat i dane o zasobach, niezależnie od języka wyświetlania. Jednostki techniczne pozostają w formacie **bit/s** (Mbit/s i Gbit/s), aby uniknąć pomylenia z bajtami.

Katalogi francuski i angielski służą jako punkt odniesienia. Pozostałe języki to **tłumaczenia automatyczne**, które mogą wymagać poprawek językowych. Pliki `locales/*.json` są uwzględniane w kompilacji i można je poprawić, a następnie ponownie skompilować. Aplikacja nie wykonuje tłumaczenia sieciowego: 50 języków i czcionek Noto jest osadzonych w ramach licencji SIL Open Font License. Tekst arabski wykorzystuje dwukierunkowe segmenty Unicode i znaki nowej linii obliczane przed formatowaniem.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` może odnosić się do innego pliku preferencji do testowania lub instalacji przenośnej. Zapisywany jest tam jedynie kod języka.
