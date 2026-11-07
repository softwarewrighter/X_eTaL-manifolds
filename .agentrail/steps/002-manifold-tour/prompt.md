Step manifold-tour (docs/plan.md, Saga 1 row 2; the user, 2026-11-06:
"when can I see some visualizations of manifolds ... in a live demo
using gh pages", "plan to show manifolds, a kind of intro to the
manifold concept, and different kinds and how this will help with
future manifold ML experiments/demos"; decisions G2 as amended, G5
(intro), G13; CLAUDE.md rules 3, 3a, 3c, 9, 13).

The first live page of this repository, before the shared teaching
shell has landed: an introduction to what a manifold is, the kinds
this repository will use, and what each is for in the experiments to
come. It is the page a newcomer meets first, and the page that proves
the chain from X_eTaL to a turning picture in the browser.

What is allowed here and what is not (G2 as amended 2026-11-06):
- The page is built on X_eTaL-ML's shared/microscope by path from the
  pinned clone (work/xetal-ml/shared/microscope), as X_eTaL-ML's own
  demos are: its run, source, canvas and chrome (header_of with this
  repository's Site). That is sharing, not copying.
- Nothing of X_eTaL-fine-tuning's shared/lesson is copied: no frames,
  glossary, stepper or explain machinery. The page has its question,
  background and summary as plain sections of its own; when GM1 lands
  (step shell-adoption) it is re-framed by the shell and loses
  nothing.
- The turning view (G13): if X_eTaL-ML's shared turning view (ask
  GM3) has landed at the pin, use it. If not, the demo's web/ holds a
  small view of its own, as X_eTaL-ML's embedding-explorer does:
  yaw and pitch from pointer drag, arrow keys and touch, an
  orthographic projection of the coordinates X_eTaL printed; marked
  in web/README and in docs/xetal-ML-asks.md GM3 as the workaround to
  remove when GM3 lands. The projection is presentation (rule 3a): it
  draws points, segments and wire lines; every coordinate, distance
  and number shown comes from the X_eTaL run.
- A flat reading view beside every turning one (G13): a map or chart
  drawn from the same printed numbers, and the numbers themselves.

Deliver:
1. libs/Manifold grows the coordinates the tour needs, each a function
   that returns points X_eTaL computed (rule 3a): the circle (one
   dimension in two), the sphere (its wire lines: parallels and
   meridians, and a great-circle arc between two points, by the
   step function of step foundation), the torus (two radii), a Swiss
   roll (a sheet rolled up: the classic data manifold, with the
   coordinate along the sheet that unrolls it), and the Poincare disk
   (its boundary circle and a few geodesic arcs, computed by the
   disk's formulas, drawn flat). Each typed, documented, with
   doc-tested examples and tests (a point of each lies where its
   definition says: on the sphere, at the torus's two radii, on the
   sheet). The Poincare disk's distance and geodesic go in the same
   library part that step representation-hyperbolic will extend;
   name it now so nothing is renamed then.
2. demos/manifold-tour (track start, intro = true, order before
   sphere-hello): the program prints, for each manifold, its name,
   its intrinsic and ambient dimension, its points and wire lines,
   and one measurement that shows what a manifold is: two nearby
   points on the sphere, their straight-line and great-circle
   distance (nearly equal: locally flat, the heart of the idea),
   then two far-apart points (not equal). For the Swiss roll: two
   points close in three dimensions but far along the sheet. For the
   disk: two points near the rim and their distance, large. CLI
   baselines.
3. The page, web/: the arc of rule 9 as plain sections (the question
   in a newcomer's words: "what is a manifold, and why would a
   machine-learning experiment care?"; background; then one section
   per manifold, each with its turning view, its flat view, its
   numbers, and "what it is for here": which later demo (the plan's
   portfolio) uses it and what question it serves; then a summary
   against the question and what is next). The words of the glossary
   to come (manifold, intrinsic and ambient dimension, locally flat,
   geodesic, data manifold, a polytope is not a manifold) each
   explained in place where they first appear, in plain words, so
   that step research-foundation's glossary can take its rows from
   this page. Every number on the page from the run; figures marked
   measured or analytical (the wire lines are analytical, the
   distances measured). The page shows all the code it runs.
   Keyboard and touch for the turning views; the angle is page
   state.
4. The site: build-pages, the catalog (this page first, linked from
   the catalog's introduction as the place to start), the
   cross-reference, screenshots; web/browser.txt and the browser
   baseline; `just publish`, then `just check-live`: the page live at
   https://softwarewrighter.github.io/X_eTaL-manifolds/manifold-tour/.
5. Docs: the demo's README in the arc; README (Demos, Status, the
   live link); docs/plan.md (the portfolio row, Saga 1 table); GM3's
   row in docs/xetal-ML-asks.md says whether the workaround is in
   use; CHANGES.md.

Not in this step: any comparison of geometries on an ML problem (the
explorers), a trained model, or the teaching shell's machinery. The
page says plainly that it measures distances and trains nothing.
