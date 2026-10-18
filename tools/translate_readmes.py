"""Generate the 50 README editions from the public French README.

Only documentation prose is sent to the public translation service. Code blocks,
commands, inline code, URLs and paths are preserved, with validated placeholders.
The application never runs this development helper.
"""
from pathlib import Path
import argparse
import json
import re
import time
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def translate(texts, target):
    params = [("client", "dict-chrome-ex"), ("sl", "fr"), ("tl", target)]
    params.extend(("q", text) for text in texts)
    url = "https://clients5.google.com/translate_a/t?" + urllib.parse.urlencode(params)
    request = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
    for attempt in range(4):
        try:
            result = json.loads(urllib.request.urlopen(request, timeout=45).read())
            if not isinstance(result, list) or len(result) != len(texts) or not all(isinstance(t, str) and t.strip() for t in result):
                raise ValueError("Invalid translation response")
            return result
        except (OSError, ValueError):
            if attempt == 3:
                raise
            time.sleep(2 ** attempt)


def blocks(source):
    result = []
    paragraph = []
    fence = False

    def flush():
        if paragraph:
            result.append(("", " ".join(paragraph), True))
            paragraph.clear()
    for line in source.splitlines():
        if line.startswith("```"):
            flush()
            fence = not fence
            result.append(("", line, False))
        elif fence:
            result.append(("", line, False))
        elif not line.strip():
            flush()
            result.append(("", "", False))
        elif line.startswith("[English]("):
            # Every edition gets navigation relative to docs/readme instead.
            continue
        elif line.startswith("# San USB Viewer"):
            result.append(("", line, False))
        elif line.startswith("![San USB Viewer](assets/"):
            result.append(("", line.replace("](assets/", "](../../assets/"), False))
        elif match := re.match(r"^(#{1,6}\s+|[-*]\s+|\d+\.\s+)(.*)", line):
            flush()
            result.append((match[1], match[2], True))
        else:
            paragraph.append(line)
    flush()
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--only", nargs="*")
    args = parser.parse_args()
    languages = json.loads((ROOT / "locales/languages.json").read_text(encoding="utf-8"))
    source = (ROOT / "README.md").read_text(encoding="utf-8")
    parsed = blocks(source)
    destination = ROOT / "docs/readme"
    destination.mkdir(parents=True, exist_ok=True)
    for language in languages:
        code = language["code"]
        path = destination / f"README.{code}.md"
        if (args.only and code not in args.only) or path.exists():
            continue
        units = []
        protected = []

        def protect(match):
            text = match[0]
            if link := re.fullmatch(r"\[([^\]]+)\]\(([^)]+)\)", text):
                label, url = link.groups()
                label_index = None
                if not label.startswith("https://"):
                    label_index = len(units)
                    units.append(label)
                value = (label, url, label_index)
            else:
                value = text
            index = len(protected)
            protected.append(value)
            return f"ZXQ{index:04}QXZ"

        rows = []
        pattern = r"\[[^\]]+\]\([^)]+\)|`[^`]+`|https?://[^\s)]+|San USB Viewer|USB Device Tree Viewer|WhatCable|SuperSpeedPlus|SuperSpeed"
        for prefix, text, localize in parsed:
            if localize:
                text = re.sub(pattern, protect, text)
                index = len(units)
                units.append(text)
                rows.append((prefix, index, True))
            else:
                rows.append((prefix, text, False))
        translated = []
        if code == "fr":
            translated = units
        else:
            batch = []
            for unit in units:
                if sum(len(s) for s in batch) + len(unit) > 1600 and batch:
                    translated.extend(translate(batch, language.get("translator", code)))
                    batch.clear()
                    time.sleep(0.35)
                batch.append(unit)
            if batch:
                translated.extend(translate(batch, language.get("translator", code)))

        def restore(text):
            # Translators may insert spaces around placeholder digits.
            def replace(match):
                value = protected[int(match[1])]
                if isinstance(value, tuple):
                    label, url, index = value
                    if index is not None:
                        label = translated[index]
                    return f"[{label}]({url})"
                return value
            text = re.sub(r"ZXQ\s*(\d{4})\s*QXZ", replace, text, flags=re.I)
            if "ZXQ" in text or "QXZ" in text:
                raise ValueError(f"Unrestored placeholder in {code}")
            return text

        lines = []
        for prefix, value, localize in rows:
            if localize:
                original = units[value]
                translated_value = translated[value]
                before = sorted(re.findall(r"ZXQ(\d{4})QXZ", original))
                after = sorted(re.findall(r"ZXQ\s*(\d{4})\s*QXZ", translated_value, flags=re.I))
                if before != after:
                    # Some languages transliterate placeholder letters. Translate
                    # the surrounding prose separately so identifiers stay exact.
                    parts = re.split(r"(ZXQ\d{4}QXZ)", original)
                    indexes = [i for i, part in enumerate(parts)
                               if part.strip() and not re.fullmatch(r"ZXQ\d{4}QXZ", part)]
                    replacements = (translate([parts[i] for i in indexes], language.get("translator", code))
                                    if indexes else [])
                    for i, replacement in zip(indexes, replacements):
                        parts[i] = replacement
                    translated_value = " ".join(parts)
                lines.append(prefix + restore(translated_value))
            else:
                lines.append(prefix + value)
        catalog = json.loads((ROOT / f"locales/{code}.json").read_text(encoding="utf-8"))
        navigation = f"\n**{language['name']}** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)\n"
        if code != "fr":
            navigation += f"\n> {catalog['Traductions automatiques, à améliorer avec vos corrections.']}\n"
        lines.insert(1, navigation)
        content = "\n".join(lines).rstrip() + "\n"
        if re.findall(r"```[^\n]*\n(.*?)```", source, re.S) != re.findall(r"```[^\n]*\n(.*?)```", content, re.S):
            raise ValueError(f"Code blocks changed in {code}")
        path.write_text(content, encoding="utf-8")
        print(f"README.{code}.md: complete", flush=True)
        time.sleep(0.4)
    index = ["# San USB Viewer — 50 languages", "", "Version: **0.2.0** · [Repository](https://github.com/SanBevyEditor/SanUsbViewer)", "", "| Language | English | README |", "| --- | --- | --- |"]
    for language in languages:
        code = language["code"]
        index.append(f"| {language['name']} | {language['english']} | [{code}](readme/README.{code}.md) |")
    index.extend(["", "The French README is the source document. Other editions are machine translations; corrections are welcome.", "Code blocks, URLs and technical identifiers are preserved.", ""])
    (ROOT / "docs/README.md").write_text("\n".join(index), encoding="utf-8")


if __name__ == "__main__":
    main()
