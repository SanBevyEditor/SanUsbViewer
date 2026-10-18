# San USB Viewer

**日本語** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> 機械翻訳。修正は大歓迎です。

**現在のバージョン: `0.2.0`** — MIT ライセンス。

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

**Rust** で書かれたネイティブ USB エクスプローラー。ダーク インターフェイス、ビジュアル トポロジ、読みやすい診断機能を備えています。最初のバージョンは Windows 10/11 向けです。

## プロジェクトの起源

プロジェクトの発信元のビデオ: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus)。

このプロジェクトは、Apple での **WhatCable** と Windows での **USB Device Tree Viewer** の使用からインスピレーションを受けており、周辺機器、ハブ、およびそれらの USB 接続の速度を理解しています。これは独立した実装です。

## 特徴

- `nusb` を介した USB デバイスとコントローラーの実際の検出。
- バスとポートのパスによるツリー構造、ハブの深さ。
- 名前、シリアル番号、または VID:PID で検索します。 USB 3+ フィルターとハブ。
- ネゴシエートされたリンク速度: 1.5 / 12 / 480 Mb/s、5 Gb/s、
またはカテゴリ SuperSpeedPlus (10 Gb/s 以上)。
- ジャーニー図: コンピューター、中間ハブ、リンク、デバイス。
- Windows では、IOCTL によるハブへの追加の問い合わせにより、
デバイスによって発表された SuperSpeed の機能。 USB 2.0 経由で接続された SuperSpeed 互換デバイスは、パス制限診断を生成します。   USB 3 ハブの付属の USB 2 ハブは、480 Mb/s ブランチがケーブル障害に起因することを避けるために個別に扱われます。
- VID/PID 識別子、USB 記述子、リビジョン、ドライバー、シリアル、インターフェイス、
システムがこの情報を公開するとき。
- 3 秒ごとの自動更新、または F5 による手動更新。
- 最近観察された 200 件の変更のセッション ログ。
- 完全なステートメントを JSON (Ctrl+E) でエクスポートし、選択した詳細をコピーします。
- 統合されたガイドと明示的なデモンストレーション モード。バックアップとしては決して使用されません。
検出失敗の場合はサイレントになります。

## 投げる

Rust **1.95+**、MSVC ツールチェーン、および Windows 上の Visual Studio C++ ツール。この例では、RTK を使用してコマンドの出力をフィルタリングします。 RTK を使用しない場合は、`rtk` プレフィックスを削除するだけです。配布スクリプトは両方のケースを受け入れます。

```powershell
rtk cargo run --release
```

スタンドアロン バイナリは `target/release/san-usb-viewer.exe` で作成されます。 `build.ps1` スクリプトは、`dist/SanUsbViewer/` にディストリビューションも作成します。インベントリにはドライバの変更や WinUSB のインストールは必要ありません。アプリケーションはデバイス インターフェイスを必要としません。

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

キャプチャの宛先フォルダーが存在する必要があります。リリース バージョンはコンソールのない Windows アプリケーションです。コマンド ライン インベントリ用に開発中の `cargo run` を優先します。

## 結果を解釈する

**交渉速度**は接続速度であり、コピー速度のベンチマークではありません。有用な速度は、特にプロトコル、周辺機器、およびバスを共有する他のデバイスによって異なります。複数のデバイスの速度を合計して、利用可能な帯域幅を計算することはできません。

パッシブ USB ケーブルは通常、列挙可能なデバイスではありません。このバージョンでは、e-マーカー、USB Power Delivery の電源、コネクタのタイプを読み取ることができません。したがって、最大速度を保証するものではありません。診断は、観察されたリンクと、可能な場合は Windows インジケーターに基づいて行われます。低速リンクは、ケーブル、ポート、ハブ、またはデバイスによって制限される可能性があります。 USB 記述子のバージョン番号が最大速度として使用されることはありません。

ここでは、SuperSpeedPlus カテゴリは 10 Gb/s モードと 20 Gb/s モードを区別しません。 USB4/Thunderbolt とそのトンネルは検査されません。デバイスが接続されていないポートはインベントリされません。データのない充電ケーブルでは、目に見えるデバイスは作成されません。内部デバイス (Bluetooth、内蔵カメラなど) が USB ツリーに表示される場合があります。

アクセスできない情報は利用不可として表示されます。製造元とインターフェイスは Windows によって常に公開されるわけではありません。スキャンとスキャンの間で完全に発生するイベントは気付かれない可能性があります。ログはメモリ内にあり、最初にインベントリが成功した後に開始されます。

バス識別子とポート パスはシステム トポロジを記述します。物理コネクタには、USB 2 と SuperSpeed 用の 2 つの別個のパスを持つことができます。

## アーキテクチャと検証

`scanner.rs` はインベントリを実行します。 `windows_probe.rs` は、ハブからのネイティブ読み取りリクエストをカプセル化します。 `model.rs` にはデータと診断が含まれています。 `app.rs` はバックグラウンド タスクと選択を処理します。 `views.rs`、`inspector.rs`、`connection.rs`、`theme.rs` は egui/eframe とのインターフェースを描画します。インターフェイスはスキャン中は対話型のままであり、次のスキャンが失敗した場合でも、最後に成功した読み取りが保持されます。

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

ライブラリはクロスプラットフォームであり、Windows コードは条件付きです。この最初の配信では、macOS/Linux バージョンは検証されていません。

## 機密保持

インベントリは、テレメトリやネットワーク サービスなしで、ローカルで読み取り専用で機能します。ガイド リンクはクリックした場合にのみ開きます。 JSON レポートにはシリアル番号とシステム ID を含めることができます。それらを共有するかどうかはユーザー次第です。

## 技術参考資料

- [nusb: USB の列挙と情報](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

MITライセンス。別棟には引き続きそれぞれのライセンスが適用されます。

パブリック リポジトリには、コンパイルに必要なソース、翻訳、およびフリー フォントのみが含まれています。 USB 読み取り、診断キャプチャ、エクスポート、ローカル設定、キャッシュ、開発バイナリは、`.gitignore` によって除外されます。問題を報告するには、モード `--demo` を使用するか、レポートを共有する前にシリアル番号とシステム ID をレポートから削除してください。

## 言語 — バージョン 0.2.0

アプリケーションには **50 のインターフェイス言語**が含まれています。右上の言語コード (`FR`、`EN` など) をクリックしてスイッチャーを開きます。言語は、ネイティブ名、英語名、またはコードで検索できます。変更は即座に反映され、選択内容は Windows の `%APPDATA%/SanUsbViewer/preferences.json` に保存されます。優先することなく、システム言語が使用されます。サポートされていない言語は英語に戻ります。

この選択では、合計で最も話されている言語 (母国語と第 2 言語) が優先され、地域のバリエーションがグループ化されます。これは数値による分類ではありません。結果は方言のグループ分けによって異なります。選択されたバリアントには、簡体字中国語、標準アラビア語、フラニ語とウズベク語のラテン文字が含まれます。パンジャブ語ではグルムキーが使われます。

英語、中国語、ヒンディー語、スペイン語、フランス語、アラビア語、ベンガル語、ポルトガル語、ロシア語、ウルドゥー語、インドネシア語、ドイツ語、日本語、マラーティー語、ベトナム語、テルグ語、ハウサ語、トルコ語、パンジャブ語、スワヒリ語、フィリピン語、タミル語、ペルシア語、韓国語、アムハラ語、タイ語、ジャワ語、イタリア語、グジャラート語、カンナダ語、ボージプリ語、ビルマ語、ポーランド語、ヨルバ語、リンガラ語、オロモ語、フラニ語、マラヤーラム語、オーディア語、ウクライナ語、シンド語、ウズベク語、パシュトー語、スンダ語、マレー語、イボ語、ネパール語、アゼルバイジャン語、マイティリ語、ソマリ語。

コピーされたメニュー、診断、ヘルプ テキスト、アクティビティ メッセージ、および詳細はローカライズされます。ハードウェアとシステムによって提供される名前と識別子は保持されます。 JSON エクスポートでは、表示言語に関係なく、スキーマとインベントリ データが保持されます。バイトとの混同を避けるため、技術単位は **bit/s** (Mbit/s および Gbit/s) のままです。

フランス語と英語のカタログが参考になります。他の言語は**自動翻訳**であり、言語的な修正が必要になる場合があります。 `locales/*.json` ファイルはコンパイルに含まれており、修正して再コンパイルできます。アプリケーションによってネットワーク翻訳は実行されません。SIL オープン フォント ライセンスに基づいて、50 の言語と Noto フォントが埋め込まれています。アラビア語テキストでは、双方向の Unicode セグメントと書式設定前に計算された改行が使用されます。

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` は、テストまたはポータブル インストール用の別の設定ファイルを参照する場合があります。言語コードのみがそこに保存されます。
