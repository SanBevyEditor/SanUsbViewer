"""Check translated README coverage, links, version and preserved code blocks."""
from pathlib import Path
import json
import re
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def main():
    languages = json.loads((ROOT / "locales/languages.json").read_text(encoding="utf-8"))
    version = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["package"]["version"]
    editions = ROOT / "docs/readme"
    expected = {f"README.{language['code']}.md" for language in languages}
    actual = {path.name for path in editions.glob("README.*.md")}
    assert len(expected) == 50 and actual == expected, "README language coverage differs"
    source = (ROOT / "README.md").read_text(encoding="utf-8")
    fences = re.findall(r"```[^\n]*\n(.*?)```", source, re.S)
    index = (ROOT / "docs/README.md").read_text(encoding="utf-8")
    for name in sorted(expected):
        path = editions / name
        text = path.read_text(encoding="utf-8")
        assert version in text, f"Missing version: {name}"
        assert f"(readme/{name})" in index, f"Missing index entry: {name}"
        assert "(../../assets/screenshots/SanUsbViewer.jpg)" in text, f"Missing welcome image: {name}"
        assert not re.search(r"ZXQ|QXZ", text), f"Unrestored translation marker: {name}"
        assert re.findall(r"```[^\n]*\n(.*?)```", text, re.S) == fences, f"Changed code blocks: {name}"
    documents = [ROOT / "README.md", ROOT / "docs/README.md", *sorted(editions.glob("README.*.md"))]
    for path in documents:
        text = path.read_text(encoding="utf-8")
        for target in re.findall(r"\]\(([^)]+)\)", text):
            if not re.match(r"https?://|mailto:|#", target):
                assert (path.parent / target.split("#")[0]).exists(), f"Broken link in {path.name}: {target}"
    print(f"Validated {len(expected)} README languages: version {version}, index, images, links and unchanged code blocks.")


if __name__ == "__main__":
    main()
