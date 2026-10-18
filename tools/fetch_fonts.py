"""Download the pinned, openly licensed fonts embedded by the application."""
from pathlib import Path
import hashlib
import json
import urllib.request

ROOT = Path(__file__).resolve().parents[1] / "assets/fonts"
SCRIPTS = ["Sans", "SansArabic", "SansDevanagari", "SansBengali", "SansGurmukhi",
           "SansGujarati", "SansTamil", "SansTelugu", "SansKannada", "SansMalayalam",
           "SansOriya", "SansThai", "SansMyanmar", "SansEthiopic"]


def fetch(name, url):
    path = ROOT / name
    if not path.exists():
        data = urllib.request.urlopen(url, timeout=90).read()
        if not data or data[:15].lower().startswith(b"<!doctype html"):
            raise RuntimeError(f"Unexpected font response: {name}")
        path.write_bytes(data)
    return {"file": name, "url": url, "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def main():
    ROOT.mkdir(parents=True, exist_ok=True)
    manifest = []
    for script in SCRIPTS:
        name = f"Noto{script}-Regular.ttf"
        url = f"https://raw.githubusercontent.com/notofonts/noto-fonts/main/hinted/ttf/Noto{script}/{name}"
        manifest.append(fetch(name, url))
        print(name, flush=True)
    manifest.append(fetch("NotoSansCJKsc-Regular.otf", "https://raw.githubusercontent.com/notofonts/noto-cjk/main/Sans/OTF/SimplifiedChinese/NotoSansCJKsc-Regular.otf"))
    manifest.append(fetch("OFL.txt", "https://raw.githubusercontent.com/notofonts/noto-fonts/main/LICENSE"))
    manifest.append(fetch("OFL-CJK.txt", "https://raw.githubusercontent.com/notofonts/noto-cjk/main/Sans/LICENSE"))
    (ROOT / "sources.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
