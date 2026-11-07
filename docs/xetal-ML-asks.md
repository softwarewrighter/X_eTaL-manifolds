# Asks for X_eTaL-ML

What this repository needs from
[X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML), reached
at the pinned commit (`XETAL_ML_COMMIT`, cloned into `work/xetal-ml/`
once step foundation has made the pin; `../X_eTaL-ML` is read only
from here: its own plan takes these). General needs are in
`docs/xetal-libraries-asks.md` (written by step research-foundation);
what is asked of X_eTaL-fine-tuning is in
[`xetal-fine-tuning-asks.md`](xetal-fine-tuning-asks.md).

Two kinds of ask live here (`docs/plan.md`, G2 and G3):

- **sharing**: code that exists in a sibling and that this repository
  would otherwise have to copy. The user's rule (2026-11-06): share,
  do not replicate. These are GM1 and GM2, below.
- **functions**: machine learning that is not about manifolds (a
  learning task and its record, a preconditioned optimizer step, a
  network unrolled as a flow). Added by step research-foundation after
  each is checked at the pin. What is particular to manifolds is not
  asked for: it is developed here, in `libs/`.

Each entry: status, kind, the ask, which demos or steps here need it,
and the workaround in use. Status: **not yet taken** (filed here, to
be taken to that repository's plan), **planned** or **in progress**,
**landed** (at the pin, the workaround removed), **declined** (with
its reason), **blocker** (no workaround: the step waits, and the user
is told).

Written 2026-11-06 from reading X_eTaL-ML at b1dce11 and
X_eTaL-fine-tuning at d78aca4.

| # | Status | Kind | Ask | Needed by | Workaround |
| - | ------ | ---- | --- | --------- | ---------- |
| GM1 | planned there (X_eTaL-ML abc1745 inserted its step manifolds-asks for GM1 and GM3, 2026-11-06); **blocker** for step shell-adoption | refactoring | The teaching shell, shared: take X_eTaL-fine-tuning's `shared/lesson` in beside `shared/microscope`, configured by the dependent. Details below | every live page here (first: sphere-hello and baseline-knn, step shell-adoption) | none that does not copy the crate. Steps foundation, research-foundation and experiment-kit need no page and do not wait |
| GM2 | not yet taken | refactoring | The scripts, shared: the repository-independent ones runnable from the clone at the pin against a dependent's root; 23 of 33 are unchanged here. Details and the record below | step foundation | adapted copies, each with a note of where it came from, as X_eTaL-fine-tuning holds them |
| GM3 | planned there (X_eTaL-ML abc1745, step manifolds-asks); **workaround in use** since step manifold-tour (2026-11-07): `demos/manifold-tour/web/src/turn.rs`, the demo's own orthographic projection by yaw and pitch from pointer, keyboard and touch, removed when GM3 lands at the pin | sharing | The turning view, shared: points, segments and a wire surface in three dimensions, turned by yaw and pitch from pointer, keyboard and touch, in `microscope`. Details below | sphere-hello and representation-explorer pages (steps shell-adoption and representation-sphere) | the flat views and the numbers, which every page has anyway; not a blocker |

## GM1: the teaching shell, shared

**What exists.** X_eTaL-ML's `shared/microscope` is the page shell
every dependent already builds on by path, through its pin: it runs a
program, reads back arrays, draws source, and since X_eTaL-fine-tuning's
asks FM1 and FM2 it embeds a dependent's libraries (`MICROSCOPE_LIBS`)
and takes the dependent's site (`chrome::Site`, `header_of`,
`footer_of`). On top of it X_eTaL-fine-tuning wrote the teaching
shell, `shared/lesson` in that repository: about 1,400 lines of Rust
in ten modules (`chrome` frames and asides, `eli5` the glossary and
its marked prose, `explain` a library function from its own source,
`code` linked source, `mode` three depths, `heat`, `stepper`,
`arithmetic`, `narrate`, `predict`), `lesson.css`, `glossary.tsv`,
`builtins.tsv`, and a `build.rs`.

**Why a second repository cannot use it as it is.** Read at d78aca4:

1. Its dependencies are paths through its own repository's clone:
   `microscope = { path = "../../work/xetal-ml/shared/microscope" }`
   and `xetal-play` at
   `../../work/xetal-ml/work/xetal/components/web/crates/xetal-play`.
   Reached from here it would build against X_eTaL-fine-tuning's
   X_eTaL-ML, not the one this repository pins, so a page and the
   command line could run different languages.
2. The site is constant: `chrome::REPO` and `chrome::SITE` name
   X_eTaL-fine-tuning, and `explain` falls back to
   `("X_eTaL-fine-tuning", "main")` for a library of the repository's
   own. A page here would link its footer and its sources there.
3. The glossary is compiled in: `eli5` does
   `include_str!("../glossary.tsv")`, the fine-tuning lessons' terms
   (a weight, an adapter, a rank). A page here needs a manifold, a
   metric, a geodesic.
4. `build.rs` finds the repository as `CARGO_MANIFEST_DIR/../..` and
   reads there `XETAL_ML_COMMIT`, `XETAL_LIBRARIES_COMMIT`,
   `work/xetal-ml/XETAL_COMMIT`, `libs/`, `work/*/libs` and
   `demos/*/demo.toml` (for `lessons.rs` and `origins.rs`), and takes
   the build's commit with `git rev-parse` where the crate lies.
   Inside another repository's clone every one of those is that
   repository's.
5. The frames are a closed set of six (`Frame`: the question, the
   background, the rationale, the purpose, the summary, what is
   next). This repository's page arc keeps those and adds parts of
   its own (`docs/plan.md`, principle 6).

**Asked.** The mechanism moved into X_eTaL-ML as a crate beside
`shared/microscope` (`shared/lesson`, or the name X_eTaL-ML prefers),
so that each dependent reaches it through the X_eTaL-ML pin it
already has, and what is now fixed becomes the dependent's to supply.
The design is X_eTaL-ML's; what a dependent needs to be able to say:

| What | Now | Wanted |
| ---- | --- | ------ |
| dependencies | paths through a dependent's `work/` | `microscope` and `xetal-play` as paths inside X_eTaL-ML (`../microscope`), as microscope itself has them |
| the site | constants naming X_eTaL-fine-tuning | the page passes a `microscope::chrome::Site`, as FM2 gave microscope; the repository of a dependent's own libraries taken from it |
| the glossary | one file compiled in | the mechanism shared; the rows from the dependent: one or more glossary files named at build time in its `.cargo/config.toml` `[env]`, in the manner of `MICROSCOPE_LIBS` (say `LESSON_GLOSSARY`), after an optional shared base of terms every X_eTaL page needs (`builtins.tsv` is shared: it describes the language) |
| the root and the pins | `CARGO_MANIFEST_DIR/../..`, two pin files by name | the dependent's root named at build time (say `LESSON_ROOT`), and the pins shown in the footer taken from what the dependent names, since a dependent may pin other repositories |
| the lessons index | the containing repository's `demos/*/demo.toml` | the dependent's, found from its root |
| the frames | six kinds, closed | the six kept; a dependent may add kinds of its own with their titles, and its browser check names the kinds a page must have |
| the stylesheet | `lesson.css` in X_eTaL-fine-tuning | beside `microscope.css`, linked the same way |

A minimal example of what a page here would then write (names are
illustrations):

```
# demos/<slug>/web/Cargo.toml
lesson     = { path = "../../../work/xetal-ml/shared/lesson" }
microscope = { path = "../../../work/xetal-ml/shared/microscope" }

# .cargo/config.toml
[env]
MICROSCOPE_LIBS = { value = "libs:work/xetal-libraries/libs", relative = true }
LESSON_ROOT     = { value = ".", relative = true }
LESSON_GLOSSARY = { value = "shared/glossary.tsv", relative = true }
```

and in the page, `lesson::chrome::header_of(&SITE, title, lede)`,
`frame(kind, title, body)`, `lesson::eli5::prose(...)` as the
fine-tuning pages call them now.

**How it lands here.** One re-pin (`just ml-pin`, the full gate, the
pin file in its own commit), then step shell-adoption wires it. It
pairs with GF1: the code is X_eTaL-fine-tuning's to offer, and that
repository then depends on the shared crate too. If the two
repositories prefer that the shell stay in X_eTaL-fine-tuning and be
made configurable there, that also answers this ask, at the price of
a third pin here (X_eTaL-fine-tuning's) and of its clone carrying an
X_eTaL-ML of its own; this repository's preference is the move,
because it keeps one X_eTaL-ML under a page.

**The user allows a third pin** (2026-11-06: any sibling may be pinned
and cloned under `work/`), so the alternative above is open to the
two repositories; the preference stays the move.

## GM2: the scripts, shared

**What exists.** Each dependent repository holds its own `scripts/`
adapted from X_eTaL-ML's: the gate, the catalog (`demos.py`), the
browser checks, the pages build, publish and check-live, `new-demo`,
`py`. A rough diff on 2026-11-06 of X_eTaL-fine-tuning's copies against
X_eTaL-ML's found the larger part of `demos.py`, `build-pages.sh`
and `gate.sh` unchanged and about half of `browser-check.sh`: the
same text in two repositories, and step foundation here would make
a third.

**Asked.** The repository-independent scripts runnable from the
clone at the pin against a dependent's root (the root, the site's
name and the port as arguments or environment), so that a dependent's
`justfile` calls them and keeps only what differs: its `demo.toml`
fields and their check, the frames its pages must have, its pins.

**Not a blocker.** Step foundation adapted copies as the siblings did,
noted in each where it came from, and recorded here which scripts it
could have called unchanged and what each of the others needed
changed. That list is the specification of this ask.

**The record (step foundation, 2026-11-06, against X_eTaL-fine-tuning
at d78aca4).** Of 33 scripts, 23 are unchanged but for a comment
naming where they came from; the 10 others changed only in the ways
listed, none of which needs the script's logic.

Unchanged (callable from a clone against this root as they are, if
the root came in as an argument or environment): `py`, `xt`,
`xetal-ml.sh`, `xetal-libraries.sh`, `ml-pin.sh`, `libpath.sh`,
`doc-test.sh`, `run-demo.sh`, `run-lib.sh`, `new-demo.sh`,
`new-lib.sh`, `test-demos.sh`, `test-libs.sh`, `selftest-demos.sh`,
`selftest-libs.sh`, `libs.py`, `check-spelling.py`, `check-rebind.py`,
`check-examples.py`, `doc-site.sh`, `browser-check.sh`,
`browser-check.py` (a comment: a page here marks its own sections
`data-frame` until the shell does), `affected.py` (only its self-test's
names).

Changed, and what:

| Script | What differs | Why |
| ------ | ------------ | --- |
| `check-xetal.sh` | the libraries probed (Embed, Check, Matrix, Trig instead of LoRA, Check, Matrix) | each repository probes what its libraries build on: a list to pass in |
| `demos.py` | the `demo.toml` fields: `problem`, `compares`, `hypothesis`, `intro` and their checks replace `track`'s list and `lesson`; a self-test | each repository's description schema: the shared script would take a schema, or each repository its own `demos.py` and nothing else |
| `gate.sh` | the shared step tests `shared/` only if it holds a Cargo workspace; the pages are built only once a demo has a `web/`; the markdown list is computed (every tracked or new `.md` but `CLAUDE.md` and the saga's) instead of written out | the two guards suit any dependent; the computed list removes a per-repository edit |
| `build-catalog.py` | the repository, the header's texts, the track names, the card's second line (the problem, the choices, the hypothesis) | the texts belong to each repository: a shared builder would take them from a file |
| `build-pages.sh`, `serve-pages.sh`, `publish-pages.sh`, `check-live.sh`, `browser-dom.sh`, `screenshots.sh` | the site's name (`X_eTaL-manifolds`) and, in `serve-pages.sh`, the port (3285) | one name and one port per repository: two parameters |

So the ask, made precise: the 23 take the repository root; the six
page scripts take the site's name and port; `check-xetal.sh` takes
the probes; `build-catalog.py` takes its texts from a file; `gate.sh`
needs nothing further once it computes its markdown list; `demos.py`
stays each repository's own, or takes a schema. The `justfile` would
then be the only copy, and short.

## GM3: the turning view, shared

**What exists.** Read at b1dce11: the embedding explorer turns its
cloud of words with `dots(xyz, yaw, pitch)` in
`demos/embedding-explorer/web/src/view.rs`, about thirty lines
(scale by a typical radius, yaw then pitch, a mild perspective, the
farthest drawn first), private to that demo; `microscope::stageview`
turns a stage's values in three dimensions with its own `Turn`
(pointer drag), for stage values and not for an arbitrary picture.
Neither draws a surface or a path.

**Asked.** One turning view in `microscope`, for any page: points
(with a color, a label and a depth order), segments (a path, a
tangent vector), a wire surface given as lines (a sphere's meridians
and parallels, computed by the page's X_eTaL program, so the Rust
draws and never computes the geometry), yaw and pitch from pointer
drag, arrow keys and touch, a stated projection, and the angle as
page state so that a preset can name a view. The embedding explorer
and stageview could then use it too.

**Why asked, not written** (`docs/plan.md`, G2, G13): the user
prefers three-dimensional views in general, so the sibling demos
want the same part; a second projection here would be a copy of a
thing X_eTaL-ML already has twice. Not a blocker: a page has its flat
reading view and its numbers in any case, and shows the turning view
once this lands.

**The workaround in use** (step manifold-tour, 2026-11-07): the
manifold-tour page turns its circle, sphere, torus and Swiss roll with
`demos/manifold-tour/web/src/turn.rs` (about sixty lines: yaw round
the up axis, pitch toward the viewer, orthographic, segments sorted
back to front and lightened by depth, the angle in the caption), fed
only coordinates the X_eTaL program printed. It is what this ask
wants, written small and private to one demo, as the plan allows
(G13 as amended) and no more: when GM3 lands the module goes and the
page calls the shared view.
