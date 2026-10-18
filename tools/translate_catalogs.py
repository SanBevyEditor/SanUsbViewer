"""Build-time translation helper. Sends only public UI text, never USB data.

Not shipped with or called by the application. Existing catalogs are preserved.
"""
from pathlib import Path
import argparse
import json
import re
import time
import urllib.parse
import urllib.request
from validate_locales import SPEEDS


def translate(texts, target):
    params = [("client", "dict-chrome-ex"), ("sl", "en"), ("tl", target)]
    params.extend(("q", text) for text in texts)
    url = "https://clients5.google.com/translate_a/t?" + urllib.parse.urlencode(params)
    request = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
    result = json.loads(urllib.request.urlopen(request, timeout=45).read())
    if not isinstance(result, list) or len(result) != len(texts) or not all(isinstance(v, str) and v.strip() for v in result):
        raise RuntimeError(f"Unexpected translation response for {target}")
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--only", nargs="*")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    source = json.loads((root / "locales/en.json").read_text(encoding="utf-8"))
    languages = json.loads((root / "locales/languages.json").read_text(encoding="utf-8"))
    for language in languages:
        code = language["code"]
        path = root / f"locales/{code}.json"
        if code in ("en", "fr") or (args.only and code not in args.only) or path.exists():
            continue
        translated = {}
        batch = []
        def flush():
            if not batch:
                return
            texts = [source[key] for _, key in batch]
            output = translate(texts, language.get("translator", code))
            for (_, key), value in zip(batch, output):
                # Preserve the sole runtime placeholder even if its name was translated.
                expected = re.findall(r"\{\w+\}", source[key])
                actual = re.findall(r"\{[^{}]+\}", value)
                if len(expected) != len(actual):
                    raise RuntimeError(f"Invalid placeholders: {code}/{key}")
                for old, new in zip(actual, expected):
                    value = value.replace(old, new)
                translated[key] = value.strip()
            batch.clear()
            time.sleep(0.5)
        for index, (key, value) in enumerate(source.items()):
            if sum(len(source[k]) + 8 for _, k in batch) + len(value) > 1800:
                flush()
            batch.append((index, key))
        flush()
        translated.update(SPEEDS)
        if translated.keys() != source.keys():
            raise RuntimeError(f"Incomplete catalog: {code}")
        path.write_text(json.dumps(translated, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        print(f"{code}: {len(translated)} strings", flush=True)
        time.sleep(0.7)


if __name__ == "__main__":
    main()
