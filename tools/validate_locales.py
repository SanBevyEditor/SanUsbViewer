"""Validate embedded translation catalogs without contacting a translation service."""
from pathlib import Path
import json
import re

ROOT = Path(__file__).resolve().parents[1]
SPEEDS = {"1,5 Mb/s": "1.5 Mbit/s", "12 Mb/s": "12 Mbit/s", "480 Mb/s": "480 Mbit/s", "5 Gb/s": "5 Gbit/s", "10 Gb/s ou +": "10 Gbit/s+"}


def load(path):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"Duplicate key: {path}: {key}")
            result[key] = value
        return result
    return json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=unique)


def main():
    languages = load(ROOT / "locales/languages.json")
    assert len(languages) == len({l["code"] for l in languages}) == 50
    source = load(ROOT / "locales/en.json")
    for language in languages:
        code = language["code"]
        catalog = load(ROOT / f"locales/{code}.json")
        assert catalog.keys() == source.keys(), code
        for key, value in catalog.items():
            assert value.strip(), (code, key)
            assert sorted(re.findall(r"\{[^{}]+\}", value)) == sorted(re.findall(r"\{[^{}]+\}", source[key])), (code, key)
        if code != "fr":
            assert all(catalog[key] == value for key, value in SPEEDS.items()), code
    print(f"50 catalogs validated: {len(source)} strings per language; parameters and bit-rate units preserved.")


if __name__ == "__main__":
    main()
