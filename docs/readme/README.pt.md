# San USB Viewer

**Português** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Traduções automáticas; correções são bem-vindas.

**Versão atual: `0.2.0`** — licença MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

USB Explorer nativo escrito em **Rust**, com interface escura, topologia visual e diagnóstico legível. Primeira versão destinada ao Windows 10/11.

## Origem do projeto

Vídeo na origem do projeto: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

O projeto é inspirado nos usos de **WhatCable** na Apple e **USB Device Tree Viewer** no Windows: entendendo periféricos, hubs e as velocidades de suas conexões USB. Esta é uma implementação independente.

## Características

- Detecção real de dispositivos e controladores USB via `nusb`.
- Estrutura em árvore por barramento e trajeto portuário, com profundidade de hub.
- Pesquise por nome, número de série ou VID:PID; Filtros e hubs USB 3+.
- Velocidade de link negociada: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
ou categoria SuperSpeedPlus (10 Gb/s ou mais).
- Diagrama de jornada: computador, hubs intermediários, link e dispositivo.
- No Windows, interrogatório adicional do hub pelo IOCTL para descobrir o
Capacidades SuperSpeed anunciadas pelo dispositivo. Um dispositivo compatível com SuperSpeed conectado via USB 2.0 produz um diagnóstico de limitação de caminho.   Os hubs USB 2 companheiros dos hubs USB 3 são tratados separadamente para evitar atribuir sua ramificação de 480 Mb/s a uma falha de cabo.
- Identificadores VID/PID, descritor USB, revisão, driver, serial e interfaces,
quando o sistema expõe essas informações.
- Atualização automática a cada três segundos ou manual com F5.
- Log de sessão das últimas 200 alterações observadas.
- Exporte a instrução completa em JSON (Ctrl+E) e copie os detalhes selecionados.
- Guia integrado e modo de demonstração explícito, nunca usado como backup
silencioso em caso de falha de detecção.

## Lançar

Com Rust **1.95+**, o conjunto de ferramentas MSVC e ferramentas do Visual Studio C++ no Windows. Os exemplos usam RTK para filtrar a saída dos comandos; sem RTK, basta remover o prefixo `rtk`. O script de distribuição aceita ambos os casos.

```powershell
rtk cargo run --release
```

O binário independente é produzido em `target/release/san-usb-viewer.exe`. O script `build.ps1` também cria uma distribuição em `dist/SanUsbViewer/`. Nenhuma modificação de driver ou instalação de WinUSB é necessária para o inventário: o aplicativo não requer interfaces de dispositivos.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

A pasta de destino de uma captura deve existir. A versão de lançamento é um aplicativo Windows sem console; prefira `cargo run` em desenvolvimento para inventário de linha de comando.

## Interprete os resultados

A **velocidade negociada** é a da conexão e não uma referência de velocidade de cópia. As velocidades úteis dependem em particular do protocolo, do periférico e dos restantes dispositivos que partilham o barramento. As velocidades de vários dispositivos não são somadas para calcular a largura de banda disponível.

Um cabo USB passivo geralmente não é um dispositivo enumerável. Esta versão não lê seu marcador eletrônico, nem a potência USB Power Delivery, nem seu tipo de conector. Portanto, não pretende certificar a sua velocidade máxima. O diagnóstico é baseado no link observado e, se disponível, nos indicadores do Windows. Um link lento pode ser limitado pelo cabo, porta, hub ou dispositivo. O número da versão do descritor USB nunca é usado como velocidade máxima.

A categoria SuperSpeedPlus não faz distinção entre os modos de 10 e 20 Gb/s aqui. USB4/Thunderbolt e seus túneis não são inspecionados. As portas sem um dispositivo conectado não são inventariadas. Um cabo de carregamento sem dados não cria um dispositivo visível. Dispositivos internos (Bluetooth, câmera integrada, etc.) podem aparecer na árvore USB.

Informações inacessíveis são exibidas como indisponíveis. O fabricante e as interfaces nem sempre são expostos pelo Windows. Os eventos que ocorrem inteiramente entre as verificações podem passar despercebidos. O log fica na memória e começa após o primeiro inventário bem-sucedido.

Identificadores de barramento e caminhos de porta descrevem a topologia do sistema. Um conector físico pode ter dois caminhos separados para USB 2 e SuperSpeed.

## Arquitetura e validação

`scanner.rs` realiza inventário; `windows_probe.rs` encapsula solicitações de leitura nativas de hubs; `model.rs` contém dados e diagnósticos. `app.rs` lida com tarefas e seleção em segundo plano; `views.rs`, `inspector.rs`, `connection.rs` e `theme.rs` desenham a interface com egui/eframe. A interface permanece interativa durante uma varredura e retém a última leitura bem-sucedida se a próxima varredura falhar.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

As bibliotecas são multiplataforma e o código do Windows é condicional. As versões macOS/Linux não são validadas nesta primeira entrega.

## Confidencialidade

O inventário funciona localmente, somente leitura, sem telemetria ou serviços de rede. Os links do guia só abrem com um clique. Os relatórios JSON podem conter números de série e identificadores de sistema: você decide compartilhá-los.

## Referências técnicas

- [nusb: enumeração e informações USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

Licença do MIT. Os anexos permanecem sujeitos às respetivas licenças.

O repositório público contém apenas as fontes, traduções e fontes gratuitas necessárias para a compilação. Leituras USB, capturas de diagnóstico, exportações, preferências locais, caches e binários de desenvolvimento são excluídos pelo `.gitignore`. Para relatar um problema, use o modo `--demo` ou remova os números de série e IDs do sistema dos seus relatórios antes de compartilhá-los.

## Idiomas — versão 0.2.0

O aplicativo inclui **50 idiomas de interface**. Clique no código do idioma (`FR`, `EN`, etc.) no canto superior direito para abrir o switcher. Você pode pesquisar um idioma por seu nome nativo, nome em inglês ou código. A alteração é imediata e a escolha fica armazenada em `%APPDATA%/SanUsbViewer/preferences.json` no Windows. Sem preferência, é utilizado o idioma do sistema; Um idioma não suportado é revertido para inglês.

A seleção privilegia as línguas mais faladas no total (língua materna e segunda língua), agrupando variantes regionais. Esta não é uma classificação numérica: os resultados variam dependendo do agrupamento de dialetos. As variantes selecionadas incluem chinês simplificado, árabe padrão e as escritas latinas de Fulani e Uzbeque; Punjabi usa Gurmukhi.

Inglês, chinês, hindi, espanhol, francês, árabe, bengali, português, russo, urdu, indonésio, alemão, japonês, marathi, vietnamita, telugu, hausa, turco, punjabi, suaíli, filipino, tâmil, persa, coreano, amárico, tailandês, javanês, italiano, gujarati, canarês, bhojpuri, birmanês, polonês, iorubá, lingala, oromo, fulani, malaiala, odia, ucraniano, sindi, Uzbeque, pashto, sudanês, malaio, igbo, nepalês, azeri, maithili e somali.

Menus copiados, diagnósticos, textos de ajuda, mensagens de atividades e detalhes são localizados. Os nomes e identificadores fornecidos pelo hardware e pelo sistema são mantidos. A exportação JSON mantém seu esquema e dados de inventário, independentemente do idioma de exibição. As unidades técnicas permanecem em **bit/s** (Mbit/s e Gbit/s) para evitar confusão com bytes.

Os catálogos francês e inglês servem de referência. Outros idiomas são **traduções automáticas**, que podem exigir correções linguísticas. Os arquivos `locales/*.json` estão incluídos na compilação e podem ser corrigidos e recompilados. Nenhuma tradução em rede é realizada pelo aplicativo: os 50 idiomas e as fontes Noto estão incorporados, sob a licença SIL Open Font. O texto em árabe usa segmentos Unicode bidirecionais e novas linhas calculadas antes da formatação.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` pode se referir a outro arquivo de preferências para teste ou instalação portátil. Apenas o código do idioma é salvo lá.
