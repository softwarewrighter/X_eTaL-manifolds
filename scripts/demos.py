# Run with scripts/py (the uv venv .venv), never a system python.
"""The demos' descriptions (demos/<slug>/demo.toml).

  scripts/demos.py list         # the slugs, in catalog order
  scripts/demos.py check        # validate every demo.toml (exit 1 on a problem)
  scripts/demos.py json         # every description as JSON, in catalog order
  scripts/demos.py --self-test  # the checks on known descriptions

Every demo compares (docs/plan.md, G5): its demo.toml names the ML
`problem`, the geometric choices it `compares` (two or more), its
`track` and the `hypothesis` it bears on (H1, H2, H3 or "none"). An
introduction, `intro = true`, may compare fewer than two.

Directories starting with "_" (the template) are not demos.
XETAL_DEMOS_DIR overrides demos/ (the runner's self-test uses it).
Adapted from X_eTaL-fine-tuning's scripts/demos.py: the fields of G5
replace its track list and curriculum number (docs/xetal-ML-asks.md,
GM2).
"""
import json
import os
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STATUSES = {"draft", "live", "deferred"}
# The tracks of docs/plan.md ("Research areas and tracks").
TRACKS = {"start", "representation", "optimization", "constraints", "dynamics", "adaptive"}
HYPOTHESES = {"H1", "H2", "H3", "none"}
# The sections that frame a demo's README, as its page is framed.
# The arc of a page (plan, principle 6; CLAUDE.md rule 9), as a demo's
# README has it too: the shell's framing parts and this repository's.
FRAMING = ("## The question", "## Background", "## The idea", "## What this is, and is not", "## The intuition", "## The mathematics",
           "## What is compared", "## The run", "## Results", "## Beyond this page", "## Summary", "## Next")
FIELDS = {"slug": str, "title": str, "summary": str, "concepts": list,
          "status": str, "order": int, "needs": list,
          "track": str, "problem": str, "compares": list, "hypothesis": str}
# Optional: an introduction (fewer than two choices allowed); the
# demo's key line of X_eTaL and why it is one expression (the catalog
# card shows both, the line rendered by xetal).
OPTIONAL = {"intro": bool, "line": str, "why": str}


def demo_dirs():
    base = Path(os.environ.get("XETAL_DEMOS_DIR", ROOT / "demos"))
    return sorted(d for d in base.iterdir()
                  if d.is_dir() and not d.name.startswith(("_", ".")))


def problems(name, meta, readme):
    """What is wrong with a demo's description; readme is its README's text, or None."""
    out = [f"{name}: missing or wrong type: {k}" for k, t in FIELDS.items()
           if not isinstance(meta.get(k), t)]
    out += [f"{name}: wrong type: {k}" for k, t in OPTIONAL.items()
            if k in meta and not isinstance(meta[k], t)]
    if meta.get("slug") != name:
        out.append(f"{name}: slug {meta.get('slug')!r} is not the directory name")
    if meta.get("status") not in STATUSES:
        out.append(f"{name}: status must be one of {sorted(STATUSES)}")
    if meta.get("track") not in TRACKS:
        out.append(f"{name}: track must be one of {sorted(TRACKS)}")
    if meta.get("hypothesis") not in HYPOTHESES:
        out.append(f"{name}: hypothesis must be one of {sorted(HYPOTHESES)}")
    compares = meta.get("compares")
    if isinstance(compares, list):
        if not all(isinstance(c, str) and c.strip() for c in compares):
            out.append(f"{name}: compares lists the geometric choices, each a non-empty string")
        elif len(compares) < 2 and not meta.get("intro"):
            out.append(f"{name}: compares names {len(compares)} choice(s); every demo compares at least two (docs/plan.md, G5), or says intro = true")
    if isinstance(meta.get("problem"), str) and not meta["problem"].strip():
        out.append(f"{name}: problem must name the ML problem")
    if readme is None:
        out.append(f"{name}: no README.md")
    else:
        # No demo jumps into its subject (CLAUDE.md, the teaching rules).
        out += [f"{name}: README.md has no section '{h}'" for h in FRAMING if f"\n{h}\n" not in readme]
    return out


def load():
    demos, errs = [], []
    for d in demo_dirs():
        f = d / "demo.toml"
        if not f.is_file():
            errs.append(f"{d.name}: no demo.toml")
            continue
        try:
            meta = tomllib.loads(f.read_text())
        except tomllib.TOMLDecodeError as e:
            errs.append(f"{d.name}: demo.toml: {e}")
            continue
        readme = (d / "README.md").read_text() if (d / "README.md").is_file() else None
        errs += problems(d.name, meta, readme)
        meta["web"] = (d / "web" / "Cargo.toml").is_file()
        meta["picture"] = (d / "screenshot.png").is_file()
        meta["recording"] = (d / "recording.webp").is_file()
        demos.append(meta)
    demos.sort(key=lambda m: (m.get("order", 0), m.get("slug", "")))
    return demos, errs


def self_test():
    readme = "".join(f"\n{h}\n" for h in FRAMING)
    good = dict(slug="probe", title="Probe", summary="s", concepts=["a"], status="draft", order=1, needs=[],
                track="representation", problem="classification", compares=["flat", "spherical"], hypothesis="H1")
    assert problems("probe", good, readme) == [], problems("probe", good, readme)
    one = dict(good, compares=["flat"])
    assert any("at least two" in p for p in problems("probe", one, readme)), "one choice accepted"
    assert problems("probe", dict(one, intro=True), readme) == [], "an introduction refused"
    assert any("hypothesis" in p for p in problems("probe", dict(good, hypothesis="H9"), readme))
    assert any("track" in p for p in problems("probe", dict(good, track="lora"), readme))
    assert any("problem" in p for p in problems("probe", dict(good, problem=" "), readme))
    assert any("compares" in p for p in problems("probe", dict(good, compares=["flat", ""]), readme))
    assert any("Summary" in p for p in problems("probe", good, readme.replace("## Summary", "## Summed up")))
    assert any("no README" in p for p in problems("probe", good, None))
    print("demos: self-test ok")


def main(cmd):
    if cmd == "--self-test":
        self_test()
        return 0
    demos, errs = load()
    if errs:
        print("\n".join(errs), file=sys.stderr)
        return 1
    if cmd == "list":
        print("\n".join(m["slug"] for m in demos))
    elif cmd == "json":
        print(json.dumps(demos, indent=2))
    elif cmd != "check":
        print(__doc__, file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "check"))
