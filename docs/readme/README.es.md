# San USB Viewer

**Español** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Traducciones automáticas; las correcciones son bienvenidas.

**Versión actual: `0.2.0`** — Licencia MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Explorador USB nativo escrito en **Rust**, con una interfaz oscura, topología visual y diagnósticos legibles. Primera versión destinada a Windows 10/11.

## Origen del proyecto

Vídeo del origen del proyecto: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

El proyecto está inspirado en los usos de **WhatCable** en Apple y **USB Device Tree Viewer** en Windows: comprensión de los periféricos, concentradores y las velocidades de sus conexiones USB. Esta es una implementación independiente.

## Características

- Detección real de dispositivos USB y controladores a través de `nusb`.
- Estructura arbórea por camino de autobuses y puerto, con profundidad de eje.
- Busque por nombre, número de serie o VID:PID; Filtros y concentradores USB 3+.
- Velocidad de enlace negociada: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
o categoría SuperSpeedPlus (10 Gb/s o más).
- Diagrama de viaje: computadora, hubs intermedios, enlace y dispositivo.
- En Windows, interrogación adicional del hub por parte de IOCTL para averiguar el
Capacidades SuperSpeed anunciadas por el dispositivo. Un dispositivo compatible con SuperSpeed conectado mediante USB 2.0 produce un diagnóstico de limitación de ruta.   Los concentradores USB 2 que acompañan a los concentradores USB 3 se tratan por separado para evitar atribuir su rama de 480 Mb/s a un fallo del cable.
- Identificadores VID/PID, descriptor USB, revisión, controlador, serie e interfaces,
cuando el sistema expone esta información.
- Actualización automática cada tres segundos, o manual con F5.
- Registro de sesión de los últimos 200 cambios observados.
- Exporte el extracto completo en JSON (Ctrl+E) y copie los detalles seleccionados.
- Guía integrada y modo de demostración explícito, nunca utilizado como respaldo.
silencioso en caso de fallo de detección.

## Tirar

Con Rust **1.95+**, la cadena de herramientas MSVC y las herramientas Visual Studio C++ en Windows. Los ejemplos utilizan RTK para filtrar la salida de los comandos; sin RTK, simplemente elimine el prefijo `rtk`. El script de distribución acepta ambos casos.

```powershell
rtk cargo run --release
```

El binario independiente se produce en `target/release/san-usb-viewer.exe`. El script `build.ps1` también crea una distribución en `dist/SanUsbViewer/`. No es necesaria ninguna modificación del controlador ni instalación de WinUSB para el inventario: la aplicación no requiere interfaces de dispositivo.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

La carpeta de destino para una captura debe existir. La versión de lanzamiento es una aplicación de Windows sin consola; Prefiero `cargo run` en desarrollo para inventario de línea de comando.

## Interpretar los resultados

La **velocidad negociada** es la de la conexión y no un punto de referencia de velocidad de copia. Las velocidades útiles dependen en particular del protocolo, del periférico y de los demás dispositivos que comparten el bus. Las velocidades de varios dispositivos no se suman para calcular el ancho de banda disponible.

Un cable USB pasivo generalmente no es un dispositivo enumerable. Esta versión no lee su marcador electrónico, ni la alimentación USB Power Delivery, ni su tipo de conector. Por tanto, no pretende certificar su velocidad máxima. El diagnóstico se basa en el enlace observado y, si está disponible, en los indicadores de Windows. Un enlace lento puede estar limitado por el cable, puerto, concentrador o dispositivo. El número de versión del descriptor USB nunca se utiliza como velocidad máxima.

La categoría SuperSpeedPlus no distingue aquí entre modos de 10 y 20 Gb/s. USB4/Thunderbolt y sus túneles no se inspeccionan. Los puertos sin un dispositivo conectado no están inventariados. Un cable de carga sin datos no crea un dispositivo visible. Los dispositivos internos (Bluetooth, cámara integrada, etc.) pueden aparecer en el árbol USB.

La información inaccesible se muestra como no disponible. Windows no siempre expone el fabricante y las interfaces. Los eventos que ocurren completamente entre exploraciones pueden pasar desapercibidos. El registro está en la memoria y comienza después del primer inventario exitoso.

Los identificadores de bus y las rutas de puerto describen la topología del sistema. Un conector físico puede tener dos rutas independientes para USB 2 y SuperSpeed.

## Arquitectura y validación

`scanner.rs` realiza inventario; `windows_probe.rs` encapsula solicitudes de lectura nativas de concentradores; `model.rs` contiene datos y diagnósticos. `app.rs` maneja la selección y las tareas en segundo plano; `views.rs`, `inspector.rs`, `connection.rs` y `theme.rs` dibujan la interfaz con egui/eframe. La interfaz permanece interactiva durante un escaneo y conserva la última lectura exitosa si falla el siguiente escaneo.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Las bibliotecas son multiplataforma y el código de Windows es condicional. Las versiones de macOS/Linux no están validadas en esta primera entrega.

## Confidencialidad

El inventario funciona localmente, de solo lectura, sin telemetría ni servicios de red. Los enlaces de la guía solo se abren al hacer clic. Los informes JSON pueden contener números de serie e identificadores del sistema: compartirlos depende de usted.

## Referencias técnicas

- [nusb: enumeración e información de USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

Licencia MIT. Las dependencias siguen sujetas a sus respectivas licencias.

El repositorio público contiene sólo las fuentes, traducciones y fuentes gratuitas necesarias para la compilación. `.gitignore` excluye lecturas de USB, capturas de diagnóstico, exportaciones, preferencias locales, cachés y binarios de desarrollo. Para informar un problema, utilice el modo `--demo` o elimine los números de serie y los ID del sistema de sus informes antes de compartirlos.

## Idiomas: versión 0.2.0

La aplicación incluye **50 idiomas de interfaz**. Haga clic en el código de idioma (`FR`, `EN`, etc.) en la parte superior derecha para abrir el conmutador. Puede buscar un idioma por su nombre nativo, su nombre en inglés o su código. El cambio es inmediato y la elección se almacena en `%APPDATA%/SanUsbViewer/preferences.json` en Windows. Sin preferencia se utiliza el idioma del sistema; Un idioma no admitido vuelve al inglés.

La selección favorece las lenguas más habladas en total (lengua materna y segunda lengua), agrupando variantes regionales. No se trata de una clasificación numérica: los resultados varían según la agrupación de dialectos. Las variantes seleccionadas incluyen el chino simplificado, el árabe estándar y las escrituras latinas de fulani y uzbeko; Punjabi usa Gurmukhi.

Inglés, chino, hindi, español, francés, árabe, bengalí, portugués, ruso, urdu, indonesio, alemán, japonés, marathi, vietnamita, telugu, hausa, turco, punjabi, swahili, filipino, tamil, persa, coreano, amárico, tailandés, javanés, italiano, gujarati, kannada, bhojpuri, birmano, polaco, yoruba, lingala, oromo, fulani, malayalam, odia, ucraniano, sindhi, Uzbeko, pastún, sundanés, malayo, igbo, nepalí, azerí, maithili y somalí.

Se localizan los menús, diagnósticos, textos de ayuda, mensajes de actividad y detalles copiados. Se conservan los nombres e identificadores proporcionados por el hardware y el sistema. La exportación JSON mantiene su esquema y datos de inventario, independientemente del idioma de visualización. Las unidades técnicas permanecen en **bit/s** (Mbit/s y Gbit/s) para evitar confusiones con bytes.

Los catálogos en francés e inglés sirven como referencia. Otros idiomas son **traducciones automáticas**, que pueden requerir correcciones lingüísticas. Los archivos `locales/*.json` se incluyen en la compilación y se pueden corregir y luego volver a compilar. La aplicación no realiza ninguna traducción en red: los 50 idiomas y las fuentes Noto están integradas, bajo la licencia SIL Open Font. El texto árabe utiliza segmentos Unicode bidireccionales y nuevas líneas calculadas antes del formato.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` puede hacer referencia a otro archivo de preferencias para realizar pruebas o una instalación portátil. Allí sólo se guarda el código de idioma.
