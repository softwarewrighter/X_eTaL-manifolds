# Run with scripts/py (the uv venv .venv), never a system python.
"""shared/glossary.tsv from docs/glossary.md (just glossary).

docs/glossary.md is the one source, edited by hand: its tables' rows
(id | term | explanation | example | first demo). This writes them as
the tab-separated rows the pages' teaching shell reads (X_eTaL-ML's
shared/lesson, LESSON_GLOSSARY in .cargo/config.toml): id, term,
explanation, example, the demo that teaches it (its slug, only when
that demo has a page, so the link is never dead). The gate runs
`--check`, which fails when the file on disk is not what this would
write, so the two never drift.

  scripts/py scripts/glossary.py          # write shared/glossary.tsv
  scripts/py scripts/glossary.py --check  # fail if it is not current
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "docs" / "glossary.md"
TARGET = ROOT / "shared" / "glossary.tsv"
HEADER = ("# This repository's ideas, explained simply (lesson::eli5): id, the term, the explanation, "
          "a worked example, the demo that teaches it. MADE by scripts/glossary.py from docs/glossary.md, "
          "the one source: edit that file, then `just glossary`; the gate checks they agree.")


def rows(text):
    """(id, term, explanation, example, demo) for each table row of the glossary."""
    out = []
    for line in text.splitlines():
        if not line.startswith("| ") or line.startswith("| id ") or line.startswith("| -- "):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) != 5:
            sys.exit(f"glossary: a row with {len(cells)} cells, not 5: {line[:60]}")
        out.append(tuple(cells))
    return out


def tsv(text):
    lines = [HEADER]
    for id_, term, simply, example, demo in rows(text):
        if not re.fullmatch(r"[a-z0-9-]+", id_):
            sys.exit(f"glossary: an id must be lower-case letters, digits and dashes: {id_}")
        if "\t" in simply or "\t" in example:
            sys.exit(f"glossary: no tabs inside a cell: {id_}")
        lesson = demo if (ROOT / "demos" / demo / "web" / "Cargo.toml").is_file() else ""
        lines.append("\t".join([id_, term, simply, example, lesson]))
    return "\n".join(lines) + "\n"


def main(argv):
    made = tsv(SOURCE.read_text())
    ids = [r[0] for r in rows(SOURCE.read_text())]
    dup = {i for i in ids if ids.count(i) > 1}
    if dup:
        sys.exit(f"glossary: ids given twice: {', '.join(sorted(dup))}")
    if argv[1:] == ["--check"]:
        if not TARGET.is_file() or TARGET.read_text() != made:
            sys.exit("glossary: shared/glossary.tsv is not what docs/glossary.md gives (run just glossary)")
        print(f"glossary: ok ({len(ids)} ideas)")
        return 0
    TARGET.parent.mkdir(exist_ok=True)
    TARGET.write_text(made)
    print(f"glossary: {len(ids)} ideas written to shared/glossary.tsv")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
