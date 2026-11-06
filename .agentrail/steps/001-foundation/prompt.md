Step foundation (docs/plan.md, Saga 1 row 1; decisions G1, G2, G3,
G4, G5, G11; CLAUDE.md rules 2, 3, 3b, 3c).

Build this repository's machinery, adapted from X_eTaL-fine-tuning's
(read ../X_eTaL-fine-tuning/justfile, scripts/, .cargo/config.toml,
.gitignore, demos/_template and templates/Library as the model;
adapt, with a note in each script of where it came from; change
nothing outside this repository), its first library and its first
demo. No web page is built in this step: pages wait for the shared
teaching shell (asks GM1 and GF1; step shell-adoption).

Deliver:
1. The pins (G1): XETAL_ML_COMMIT and XETAL_LIBRARIES_COMMIT (committed
   refs of ../X_eTaL-ML and ../X_eTaL-libraries, their HEADs when you
   start, unless X_eTaL-fine-tuning's pins are older and known good:
   say which you took and why); scripts cloning each into the
   gitignored work/, refusing a clone with local changes, building the
   X_eTaL that X_eTaL-ML pins and linking bin/xetal. `just xetal`,
   `just ml-pin [REF]`, `just libs-pin [REF]`, `just xetal-version`.
2. XETAL_PATH (G2): `just path`, `just eval`: this repository's
   libs/*/src, then X_eTaL-ML's, then X_eTaL-libraries'.
3. scripts/py and .python-version (uv only; never the system python3).
4. The gate, `just gate` and `just gate --full`, as the sibling's: the
   pinned toolchain check, American spelling, ASCII markdown
   (sw-markdown-checker), the X_eTaL style checks the sibling runs,
   doc-test, the libraries' and the demos' reg-rs baselines. `just
   new-demo`, `just run`, `just show`, `just test`, `just test-demo`,
   `just bless`; `just new-lib`, `just run-lib`, `just test-lib`,
   `just bless-lib`. The page recipes (`just pages`, `just serve` and
   `just serve-pages` on port 3285, `just publish`, `just check-live`,
   the browser checks, the catalog page) are adapted too, and the gate
   builds and checks pages once a demo has a `web/`; none has yet.
5. Ask GM2 made precise (docs/xetal-ML-asks.md, "GM2: the scripts,
   shared"): as you adapt each script, record there which could have
   been called unchanged from the clone at the pin and what each of
   the others needed changed. That list is this step's contribution
   to not copying next time. Do not copy shared/lesson or any part of
   it (rule 3c).
6. demos/_template with the demo.toml fields of G5: `problem`,
   `compares` (two or more geometric choices), `track` (start,
   representation, optimization, constraints, dynamics, adaptive),
   `hypothesis` (H1, H2, H3 or none), `order`; `scripts/demos.py
   check` refuses fewer than two choices unless `intro = true`, with a
   self-test. templates/Library for `just new-lib`.
7. .gitignore (as the sibling's, with agentrail's session transcripts
   ignored), CHANGES.md (the sibling's header and categories, a line
   for every commit so far), .cargo/config.toml, images/ (the X_eTaL
   logo and favicon as the siblings have them, a title image "X_eTaL
   manifolds" in the siblings' style) and the README's header using
   them.
8. The first library of this repository (rule 3a, 3b): libs/Manifold
   (the alias `mf:` unless a sibling's library uses it: check the
   pinned libraries' aliases), with the unit sphere only: the
   great-circle distance between two points, putting a vector onto
   the sphere, the tangent part of a vector at a point, a step from a
   point along a tangent vector that stays on the sphere and its
   inverse, a mean on the sphere. Names in X_eTaL's naming style, how
   a space is named in a call (so that the Poincare ball can join in
   step representation-hyperbolic without renaming) decided here and
   written in its docs. Typed, `xetal doc` comments with examples the
   gate runs, tests near the hard cases (the same point, opposite
   points, a zero tangent vector), docs/README.md, a small demo.
   Read X_eTaL-libraries' Matrix and Trig for style and reuse them;
   read ../X_eTaL/docs/lang-choices.md and reference.md first.
9. A first demo on it, demos/sphere-hello (CLI only for now, track
   "start", intro): a short program on a handful of unit vectors
   showing (a) that straight-line distance and great-circle distance
   order every point's neighbors the same way, with the relation
   between the two distances checked numerically, and (b) that the
   plain mean of points on the sphere is not on it, with its length
   printed, beside Manifold's mean. Its README in the page arc (the
   question, background, what it shows, what it does not, summary),
   its baselines. It proves the chain from the pins to a tested
   library and program.
10. README: the Build section true (remove "Not buildable yet"),
    Demos, Libraries and Status updated. CLAUDE.md: the overview's "no
    machinery" paragraph removed, the recipes in Useful Commands true.

`just publish` waits for the first live page (step shell-adoption);
say so in the summary.
