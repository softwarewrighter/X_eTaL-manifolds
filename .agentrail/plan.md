# X_eTaL-manifolds -- Implementation Plan

Exploring the geometry of machine learning, in X_eTaL (the
eXperimental Extensible Typed Array Language, developed in
`../X_eTaL`). The repository is a small, reproducible laboratory: each
demo takes one machine-learning problem, changes the geometry it is
solved in (the space a representation lives in, the metric a gradient
is measured by, the set a transformation is held to, the vector field
a state follows), and measures what improves, what degrades and what
stays the same, with every number on a page computed by X_eTaL.

The one question every demo serves:

> How does the geometry of a representation, a parameter space or a
> computational transformation influence how a machine-learning model
> learns and performs inference?

Source (archival, not normative): `docs/research.txt`, two parts. The
first is a survey: manifolds, vector fields, metrics, topology and
dynamics as separately controllable things; where mHC
(manifold-constrained hyper-connections) fits; research that changes a
metric, a curvature, a dimension or a topology; learned metrics; the
link to LoRA and shared subspaces. The second is the decision to do
this work in a new repository, X_eTaL-manifolds, with its mission,
its boundary, ten candidate demos, three hypotheses and a first
release of five milestones (M0 to M4). This plan turns it into sagas
and steps. The research's claims about papers are the research's: each
paper is read at its source before anything is built on it (decision
G12).

Development is driven by agentrail sagas (one active saga in
`.agentrail/`, finished sagas archived to `.agentrail-archive/`), as
in the sibling repositories. Every step ends with the gate (`just
gate`), docs updated (`README.md`, `CHANGES.md`, this plan, the asks
files, the demo's pages), a sane `.gitignore`, a detailed commit to
`main` (with the `.agentrail/` changes), `agentrail complete`, and a
push.

`.agentrail/plan.md` is a copy of this file, refreshed with
`agentrail plan --update docs/plan.md` whenever it changes.

## Where this repository sits

| Repo | Answers |
| ---- | ------- |
| X_eTaL | what the language is |
| X_eTaL-libraries | language-level extensibility (`.xtl`, `.xtlm`): Matrix, Geometry, Plot, Equations, Graphs, Random, Statistics, Trig, Check; where reusable geometry and plotting primitives belong |
| X_eTaL-ML | why array languages fit machine learning: the ML libraries (NN, Net, Optim, Learn, Embed, Flow, Norm, LoRA) and focused demos; where general ML algorithms belong |
| X_eTaL-fine-tuning | how a model is adapted, taught step by step and measured; where geometric techniques are applied to parameter adaptation (LoRA subspaces, shared bases) |
| **X_eTaL-manifolds** | **how geometry changes ML behavior**: experiments that compare geometries, metrics, constraints and dynamics on the same small problem, and pages that explain them |
| X_eTaL-extensions | optional native (Rust) numerical code, should a demo ever need it |

What is in scope: small reproducible ML experiments that compare
geometries, metrics, constraints or geometric dynamics, and the
interactive pages that explain the equations and show what changes
during training or inference.

What is out of scope: a comprehensive differential-geometry package,
a geometry textbook, production LLM training, general plotting
infrastructure, another LoRA implementation, a numerical backend of
its own, and a gallery of 3-D surfaces that measures nothing.

## Guiding principles

1. **Compare, explain, measure.** The research's guiding rule: every
   major demo compares at least two geometric choices on the same ML
   problem (same data, split, seeds, comparable parameter budgets),
   explains why the choice could matter, and measures the difference.
   A demo that only draws a surface is not a demo of this repository.
2. **ML questions, not mathematical objects.** Demos are organized by
   the ML behavior investigated (a representation, an optimization, a
   constraint, a flow), not by sphere, torus and hyperbolic space. The
   objects are the primitives; the questions are the demos.
3. **A null result is a result.** Each hypothesis is written with what
   would count against it before the run. Where geometric machinery
   adds cost without improving ML behavior, the page and the results
   table say so. No claim that a geometry is always better; learning
   the geometry is not automatically better than choosing it, and a
   task loss going down does not show the geometry is faithful.
4. **One name for one thing.** The words are used loosely in the
   literature; here the glossary keeps them apart: the data manifold,
   a parameter space, a constraint set; a smooth manifold, a polytope
   (the Birkhoff polytope is one, with faces and corners, not a smooth
   manifold), a stratified set (matrices of rank at most r); a metric,
   a curvature, a dimension (ambient and intrinsic), a topology; a
   gradient field (from a potential) and a general vector field (a
   rotation has no potential). The manifold hypothesis is called an
   assumption, not a fact.
5. **Every number is real.** Values, shapes, trajectories, loss
   figures and the sentences that quote them come from the X_eTaL run;
   the browser code lays them out, draws and animates them, and never
   recomputes the mathematics. Drawing a sphere or a disk is
   presentation; the coordinates drawn are X_eTaL's.
6. **One arc for every page** (the research, "a standard educational
   format"): intuition (the ML problem in plain language, with a
   simple geometric picture); the mathematics (every variable
   explained and tied to the picture); the interactive experiment
   (change the geometry, curvature, dimension, metric or constraint);
   an actual ML run (a small reproducible model, not only an
   animation); analysis and comparison (accuracy, convergence,
   distortion, stability, cost); the research extension (what this
   establishes, what remains unknown, which papers motivate more).
   Framed as the sibling lessons are: the question in a newcomer's
   words, background, rationale and purpose first; a summary against
   the question and what is next last.
7. **New eyes.** The reader may know neither differential geometry
   nor machine learning nor X_eTaL. Every term opens a plain
   explanation; one calculation is worked in full; code sits in
   asides; nothing is over-claimed.
8. **Teach X_eTaL too.** The equation, the picture and the X_eTaL
   expression that computed it are linked. The research's synchronized
   equation, tensor and geometry view (select an operation in a
   pipeline and its vectors, tangent space or matrix light up) is the
   aim the pages work toward.

## Architecture decisions

Decisions G1 to G12. Those marked "carried over" are the sibling
X_eTaL-fine-tuning's (its F numbers), taken here as they stand; G2,
G3 and the port in G11 are the user's for this repository
(2026-11-06).

| # | Decision | Why |
| - | -------- | --- |
| G1 | **A pin for each repository asked of, moved one at a time** (carried over, F1). `XETAL_ML_COMMIT` holds the known-good X_eTaL-ML commit: `just xetal` clones it into the gitignored `work/xetal-ml/` and runs that clone's own script, which builds the X_eTaL it pins; `bin/xetal` links to that binary. `XETAL_LIBRARIES_COMMIT` holds this repository's own X_eTaL-libraries commit, cloned into `work/xetal-libraries/`. X_eTaL has no pin here: it is X_eTaL-ML's. One pin moves at a time, `just gate --full` runs, the pin file is committed alone. Nothing under `work/` is edited. | Asks land in several repositories separately; each is taken and tested alone, so a failure names the change that caused it. |
| G2 | **Share, do not replicate** (the user, 2026-11-06). `XETAL_PATH` is this repository's `libs/*/src`, then X_eTaL-ML's, then X_eTaL-libraries'. The pages are built on X_eTaL-ML's page shell (`shared/microscope`) by path. The teaching shell the sibling lessons use (X_eTaL-fine-tuning's `shared/lesson`: frames, glossary, depths, stepper, heat maps, the narrator of changes) is not copied here: the refactoring that lets a second repository use it is asked for, of X_eTaL-ML (ask GM1, `docs/xetal-ML-asks.md`) and of X_eTaL-fine-tuning (ask GF1, `docs/xetal-fine-tuning-asks.md`). The same for the scripts every dependent now holds as adapted copies (GM2, not a blocker). No page built on the teaching shell is built here until GM1 has landed; command-line programs, libraries and documents do not wait. Amended 2026-11-06 (the user: a live page with visualizations of manifolds, soon): one page before the shell, manifold-tour, built on `shared/microscope` alone by path from the pinned clone, as X_eTaL-ML's own demos are, with its own plain sections; the shell re-frames it when GM1 lands. | Code has one home; a second copy of the shell would drift from the first within a week. |
| G3 | **Each function in the library where it belongs; language last** (the user, 2026-11-06). A demo's program stays a short composition of library functions and built-in words. A function it needs goes: (a) into a library **of this repository**, `libs/<Name>/`, when it is particular to manifolds and the geometry of ML (a manifold's distances and maps, a step that stays on a manifold, distortion, constrained mixing of residual streams, a flow held to a manifold); (b) to **X_eTaL-libraries**, as an ask, when it is general (matrix scaling, integrators, the divergence and curl of a field, field and contour pictures); (c) to **X_eTaL-ML**, as an ask, when it is machine learning and not about manifolds (a learning task and its record, a preconditioned optimizer step, a network unrolled as a flow); (d) native code to X_eTaL-extensions; (e) the language last. While an ask is open the expression is written out where it is used, named in the ask and on the page. Rust is for the pages' presentation only, never for an algorithm or a number shown. | The research: "reusable primitives in existing libraries", "X_eTaL-first", "Rust acceleration belongs in X_eTaL-extensions"; and an experimental repository must be able to build what only it needs without waiting on another. |
| G4 | **Layout as in the sibling demo repositories.** Each demo is `demos/<slug>/` with `demo.toml`, `README.md`, `*.xtl`, `data/`, `presets/`, `reg/` reg-rs baselines, `web/`. Each library is `libs/<Name>/` with `src/`, `tests/`, `docs/README.md`, `demos/`, written to X_eTaL's style guide as X_eTaL-libraries' are, so that one that turns out general can move there unchanged. The research's tree (`experiments/`, `tests/`, `site/`) is conceptual, as it says itself: datasets are a demo's `data/`, configurations its `presets/`, results and regression tests its `reg/` baselines, the site is `pages/`. | One convention across the X_eTaL repositories; the tooling already reads it. |
| G5 | **A demo declares what it compares.** `demo.toml` names the `problem`, the geometric choices compared (`compares`, two or more), its `track`, the hypothesis it bears on (`hypothesis`: H1, H2, H3 or none) and its `order`; `scripts/demos.py check` refuses a demo with fewer than two choices unless it is marked an introduction. | Principle 1, checked by the gate and not by memory. |
| G6 | **A trace, not a re-implementation** (carried over, F6). A page's experiment is an X_eTaL program whose pipelines are Flow pipelines; the page replays the recorded stages and states. A training loop keeps what it shows in its state tuple. | Principle 5. |
| G7 | **Deterministic and regression-tested.** Fixed seeds, the same data and split for every choice compared, presets stored as data, tolerances written down. reg-rs baselines pin each program's output and each page's trace, so a later X_eTaL is tested against today's results. Every gradient is checked against central differences; every constraint has its violation measured (row and column sums, orthogonality, distance from the manifold). | The research: "fixed random seeds, identical datasets and data splits, comparable parameter budgets, recorded numerical tolerances", "machine-readable files". |
| G8 | **A common set of diagnostics.** Every experiment reports, where it applies: training loss, held-out accuracy, the size of an update, numerical stability, constraint violation, geometric distortion, and cost counted (numbers trained, operations, iterations). Runtime is measured apart and is never in a baseline. For mHC: singular values and row and column sum errors. | Comparisons across demos need the same account. |
| G9 | **Measured or analytical, always said** (carried over, F8). A figure from a run and a figure from a formula are marked apart; an illustration that is not a measurement says so. | The research's own captions: "a 2D Euclidean illustration, not a learned neural field". |
| G10 | **Small first.** Small vectors and matrices, tiny networks trained in X_eTaL, in the browser, no GPU. Large models, topology-changing networks and production-scale geometric networks are deferred until the first experiments have produced measurements. | The research: "for the first few phases, all the mathematics can be implemented with small vectors and matrices". |
| G11 | **Same process as the siblings** (carried over, F9): `just` is the entry point (recipes call `scripts/*.sh`), adapted from X_eTaL-fine-tuning's scripts; the gate; `CHANGES.md` a line per commit; American spellings; ASCII-only markdown (`sw-markdown-checker`); Python only through uv (`scripts/py`); `pages/` built locally and published as the `gh-pages` branch; port 3285 for the locally served demos (the user, 2026-11-06). The scripts are adapted copies until ask GM2 lands. | One process to learn. |
| G12 | **Papers are read at source.** `docs/papers.md` is the catalog: for each paper its authors, date, venue and link, (no paper is tracked here: links and notes only, the user, 2026-11-06), the claim used, its mathematical assumptions, how it was read (source, abstract, or only through `docs/research.txt`), what is implemented here and whether the result was reproduced at toy scale. Nothing is built on a paper known only through the research text. | Several papers the research names are from 2026 and are known here only through it; the area is fast-moving and its terms are used loosely. |
| G13 | **Three-dimensional views first** (the user, 2026-11-06). Where the object is a surface or a cloud in three dimensions (a sphere, its tangent planes, a path on it, a point cloud, later a Swiss roll) the page shows it turning, by pointer, keyboard and touch, from coordinates X_eTaL computed; beside it, always, a flat reading view (a map of the sphere, a chart of the quantities drawn) and the numbers, since a turned picture cannot be read off exactly and a browser baseline cannot pin an angle. The Poincare disk is flat by nature and stays so. The turning view is shared presentation: X_eTaL-ML's (ask GM3), not written here. | Intuition for curvature, tangency and a mean leaving the sphere comes from turning the thing; reading and testing come from the flat view and the numbers. |

## Layout

```
demos/<slug>/            an experiment and its page (G4)
  demo.toml README.md *.xtl data/ presets/ reg/ web/
demos/_template/         what just new-demo copies
libs/<Name>/             a library of this repository (G3, G4)
shared/                  this repository's own page parts on the shared shells
                         (shared/site: the six frames it adds to the arc, the
                         marks measured and analytical, the comparison table, the
                         aside for its own libraries' functions) and its glossary
                         rows (shared/glossary.tsv, made from docs/glossary.md);
                         the shells themselves are X_eTaL-ML's lesson and
                         microscope, by path (GM1, GM3)
scripts/                 the logic behind the just recipes
docs/plan.md            this plan
docs/research.txt        the research it came from (archival)
docs/glossary.md         the words, kept apart (principle 4)
docs/papers.md           the paper catalog (G12)
docs/research-questions.md  the hypotheses, their tests and what counts against them
docs/experiments.md      the results, hypothesis by hypothesis, null results included
docs/xetal-*-asks.md     what is asked of the sibling repositories (G3)
XETAL_ML_COMMIT          the X_eTaL-ML commit this repo is known to work with
XETAL_LIBRARIES_COMMIT   the X_eTaL-libraries commit
work/                    the clones (gitignored)
pages/                   the built site (not tracked; just publish)
```

As of step shell-adoption (2026-11-08) the machinery, `libs/Manifold`
(the sphere, the drawn spaces, the ball's distance and geodesic,
distortion), `libs/Kit` (the three data sets), `shared/site` and
`shared/glossary.tsv`, and three live pages on the shared teaching
shell (`demos/manifold-tour`, `demos/sphere-hello`,
`demos/baseline-knn`) exist, with `docs/glossary.md`,
`docs/papers.md`, `docs/research-questions.md` and
`docs/experiments.md`; the other demos are made by the steps of Saga 1
that name them.

## Research areas and tracks

A demo's `track` in its `demo.toml` is one of these.

| Track | Central question | Example |
| ----- | ---------------- | ------- |
| start | what is this repository, and how do I read a page? | the first small demo |
| representation | where do learned representations live? | Euclidean, spherical and hyperbolic embeddings |
| optimization | how does geometry affect learning? | Euclidean and metric-aware gradient steps |
| constraints | which transformations are permitted? | mHC; doubly stochastic, orthogonal and unconstrained mixing |
| dynamics | how do representations move? | gradient flow, general vector fields, flows held to a manifold |
| adaptive | can a model learn or change its geometry? | learned curvature, metric or subspace (after the others have baselines) |

## Hypotheses

Stated in full, with measures and what would count against each, in
`docs/research-questions.md` (Saga 1, step research-foundation).

| # | Hypothesis | Test | Saga |
| - | ---------- | ---- | ---- |
| H1 | Geometry matching: a representation space whose geometry matches the structure of the task can improve learning efficiency or generalization | Euclidean, spherical and hyperbolic embeddings on flat, directional and hierarchical data, at equal dimension and budget | 1 |
| H2 | Geometric constraints: constraining a network's transformations can improve numerical stability and the propagation of information | plain residual, unconstrained, doubly stochastic and orthogonal mixing in small residual networks of growing depth | 1 |
| H3 | Adaptive geometry: learning geometric properties jointly with the model's parameters can beat a fixed geometry on heterogeneous tasks | fixed, trainable and mixed curvature on the same data; task loss set against geometric distortion | 2 |

## The demo portfolio

From the research's ten candidates. Only the first four (P0) are
built in Saga 1; the rest are a roadmap.

| # | Demo (slug) | ML problem | Geometric comparison | Priority | Saga |
| - | ----------- | ---------- | -------------------- | -------- | ---- |
| | manifold-tour | none: an introduction to what a manifold is, the kinds used here and what each is for (live since 2026-11-07) | the circle, the sphere, the torus, a Swiss roll, the Poincare disk: one measurement each of local flatness | first | 1 |
| | sphere-hello | nearest neighbors and an average of unit vectors | straight-line and great-circle distance; a plain mean and one put back on the sphere | first | 1 |
| | baseline-knn | classification by nearest neighbors | none yet: straight-line distance on the three data sets, the row every later comparison is set against (built, step experiment-kit: `docs/experiments.md`) | first | 1 |
| 01 | representation-explorer | classification and clustering | flat, spherical, hyperbolic | P0 | 1 |
| 02 | optimization-explorer | training a small model | Euclidean and metric-aware updates | P0 | 1 |
| 04 | mhc-explorer | stability of residual streams | plain residual, unconstrained, doubly stochastic, orthogonal mixing | P0 | 1 |
| 03 | dynamics-explorer | evolving hidden states | gradient flow and general vector fields; a flow in flat space and one held to a sphere | P0 | 1 |
| 05 | learned-curvature | hierarchical classification | fixed and learned curvature | P1 | 2 |
| 06 | attention-geometry | similarity and retrieval | dot product, angular, geodesic | P1 | 2 |
| 07 | generative-flows | transport of a distribution | Euclidean and manifold-constrained flows | P1 | 2 |
| | constraint-lab | residual mixing beyond mHC | Kronecker-factored, sparse, expanded-stream and other constraint sets | P1 | 3 |
| 08 | adaptive-manifold | representation learning | fixed and mixed or adaptive geometry | P2 | 4 |
| 09 | topology-and-dimension | separability of representations | fixed and augmented latent dimensions | P2 | 4 |
| 10 | geometric-weight-updates | fine-tuning | fixed and evolving low-rank subspaces | P2 | 4 (with X_eTaL-fine-tuning) |

The research numbers dynamics 03 and mHC 04 but orders its milestones
representation, optimization, mHC, dynamics; the saga follows the
milestones. Representation comes first though mHC prompted the work:
it is the easiest to teach by eye and the others stand on it.

## The functions the demos need, and where each belongs

By decision G3 (the user, 2026-11-06): particular to manifolds and
the geometry of ML, a library here; general, X_eTaL-libraries;
machine learning that is not about manifolds, X_eTaL-ML. Sorted on
2026-11-06 from the siblings' export lists and checked on 2026-11-07
(step research-foundation) at the pins, reading each library's source
and doc comments (X_eTaL-libraries 3465af3, X_eTaL-ML abc1745); the
asks are written: `docs/xetal-libraries-asks.md` (GL1 to GL7) and the
function asks GM4 to GM6 in `docs/xetal-ML-asks.md`, each with its
status, kind, the demos that need it, a minimal example with the
signature wanted, and the workaround in use. None is a blocker.

What is there at the pins, read in the source: Matrix (`m_ul`,
`s_olve`, `i_nverse`, `d_et`, `o_rthonormal`, `r_ank`, `p_roject`,
`r_emainder`, `c_oordinates`, `e_igenvalues`, `e_igenvectors`,
`s_vd`, `s_ingular`, `t_runcate`: so the polar factor, the singular
values of a mixing matrix and a spectral measure are one call each;
no Sinkhorn), Graphs (`w_eighted`, `s_hortest`: the tree's distances;
`l_evels`, `c_omponents`), Random (seeded `u_niform!`, `n_ormal!`,
`s_huffle!`, `d_eal!`), Statistics, Trig (`a_tan2`, `a_sin`, `a_cos`,
`s_inh`, `c_osh`, `a_sinh`, `a_cosh`, `a_tanh`, `h_ypot`), Plot
(`s_catter`, `l_ine!`, `l_ines!`, `c_hart!`, `f_unction!`,
`l_inspace`; no field or contour picture), Geometry (plane geometry
only: `d_istances` takes points of the plane), Lists (`r_aze`,
`w_indows`, `d_eltas`), Check; Learn (`k_means`, `a_ssign`,
`f_arthest`, `k_nn`, `p_ca`, `p_roject`, `l_ogistic`, `p_redict`),
Embed (`u_nit`, `c_osines`, `n_earest`), Optim (`s_gd`, `a_dam`,
`a_damWith`: no preconditioned step), NN (`d_ense`, `s_oftmax`,
`c_rossEntropy`, `m_se`, `a_ccuracy`, `o_neHot`, `a_rgmax`), Net (a
macro library: `m:n_etwork`, `m:m_odel`, `m:g_radient`, `m:t_rain`,
`m:p_arams`, a network and its gradient from one line), Flow (a macro
library: `m:p_ipe`, `m:s_tages`, `m:t_ext`, `m:m_ath`: pipelines and
their trace), Integrals (a macro library for definite integrals of one
variable, not an integrator of a field), Norm, Activations.

Libraries of this repository (names and aliases proposed; settled by
the step that first builds each, against the aliases the siblings
use):

| Library | What | Built by |
| ------- | ---- | -------- |
| Manifold (`mf:`) | the sphere (`s_phereProject`, `s_phereDistance`, `s_phereDistances`, `s_phereTangent`, `s_phereStep`, `s_phereArrow`, `s_phereMean`, `s_phereOff`: built, step foundation) and the pictures of the circle, the sphere, the torus and the Swiss roll from their maps with the ball's `b_allDistance` and `b_allGeodesic` (built, step manifold-tour); then the ball's Project, Tangent, Step, Arrow, Mean, Off by the same verbs (representation-hyperbolic); `d_istortionPairs`, `f_itScale`, `s_phereDistortion` and `f_latDistortion` of a set of points against given distances, and `s_phereWires` for the pictures (built, step representation-sphere); `b_allDistortion` with the ball (representation-hyperbolic); the cheap retraction `s_phereRetract` beside the step, and a gradient step that stays on the space, `(lr) s_phereDescend (p, g)` (metric-descent); the tangent part of a field at a set of points and a flow held to the sphere, `s_phereFlow` (manifold-flows). The flat torus when a demo needs it | foundation, manifold-tour, representation-sphere, representation-hyperbolic, metric-descent, manifold-flows |
| Mixing (`hc:`) | residual streams and their mixing: `(n, w) hc:s_treams x` (n copies), `hc:b_lock` (a layer over n streams with a mixing matrix), the mixing `hc:f_ree M`, `k hc:s_tochastic L` (Sinkhorn's k sweeps on exp of the logits L, private until GL3 lands), `hc:p_ermutations w` (an exact convex combination of the permutations, after KromHC), `hc:o_rthogonal M` (the polar factor by `mx:s_vd`); `hc:s_tack` of blocks; `hc:g_ain` (the composite's largest absolute row and column sums, as mHC measures them), `hc:c_hannels` (the mean of the streams and their differences, after the Birkhoff-geometry paper), `hc:v_iolation` (row and column sum errors, distance from orthogonal); the sizes of the signal forward and of the gradient backward by depth | mhc-matrices, mhc-depth |
| Kit (`kt:`) | the three data sets, each a function of its own seed and sizes: `f_lat` (Gaussian clusters), `d_irections` (classes by direction, lengths varied, on Manifold's sphere), `t_ree`, `t_reeDistances` (Graphs' shortest paths), `t_reeClasses`, `t_reeLayout`; and the seeded numbers they are made of (`u_niform`, `n_ormal`, Park-Miller, until GL8). Functions, not files, because the explorers vary the sizes, the spread and the seed; here, because what they make is a geometric structure (built, step experiment-kit) | experiment-kit |

Asked of the siblings:

| Need | Asked of | What | First needed |
| ---- | -------- | ---- | ------------ |
| the teaching shell, shared | X_eTaL-ML (GM1), X_eTaL-fine-tuning (GF1) | **landed and taken** (X_eTaL-ML 501ab15; the pin at 2d23a96 since step shell-adoption, 2026-11-08): every page is on `shared/lesson` | done |
| the turning view, shared | X_eTaL-ML (GM3) | **landed and taken**: `microscope::turning` turns every picture; the demo-private projection is gone | done |
| the shell's gaps found in wiring | X_eTaL-ML (GM7, GM8, GM9) | a dependent's own libraries credited to X_eTaL-libraries (GM7); a comparison table of choices against measures (GM8); the linker's anchors for subscripted built-ins (GM9) | thin workarounds in `shared/site` and `scripts/browser-check.py`; not blockers |
| the scripts, shared | X_eTaL-ML (GM2) | waiting on this repository's list, which is GM2's record (step foundation) | not a blocker: adapted copies meanwhile |
| the new-eyes checklist, one copy | X_eTaL-fine-tuning (GF2) | not yet taken | not a blocker: cited at a commit meanwhile |
| pairwise distances in any dimension; small array words | X_eTaL-libraries (GL1, GL2) | the straight-line distance between every pair of rows; pairs, repeats, a polyline's length | every demo; written out meanwhile |
| matrix scaling | X_eTaL-libraries (GL3) | Sinkhorn's alternating row and column normalization, its sweep count and the row and column sum errors; the polar factor (one line on `mx:s_vd`, which is at the pin) | mhc-explorer |
| integrators and field measures | X_eTaL-libraries (GL4, GL5) | Euler and Runge-Kutta steps of dx/dt = v(x, t); the divergence and curl of a plane field by differences | dynamics-explorer |
| field and contour pictures | X_eTaL-libraries (GL6) | a vector field drawn as arrows, trajectories over it, the contours of a function of two variables; the pages draw from printed numbers meanwhile | optimization-explorer; dynamics-explorer |
| a Cholesky solve | X_eTaL-libraries (GL7) | for the Fisher matrix; `mx:s_olve` meanwhile | optimization-explorer |
| a step under a metric | X_eTaL-ML (GM4) | a gradient step preconditioned by a diagonal or a matrix (Optim); the empirical Fisher matrix of a small softmax model | optimization-explorer |
| a learning task and its record | X_eTaL-ML (GM5) | the split, the run's record and the comparison; X_eTaL-fine-tuning asked the same (its FM7, which X_eTaL-ML marked not planned), so one answer serves both | step experiment-kit; written here meanwhile |
| a network as a flow | X_eTaL-ML (GM6) | a small network as a vector field, unrolled for a fixed number of steps, its gradient through the steps (Net's) | dynamics-explorer |

## Saga 1 -- geometry-lab  [ACTIVE]

The research's first release, M0 to M4: the research foundation, then
four explorers, each a live page comparing geometric choices on one
small ML problem. An explorer is built in two steps so that each step
ends with something measured and live. Step manifold-tour makes the
first live page on the page shell alone (G2 as amended), an
introduction to manifolds; the steps after it until shell-adoption
need no page, which gives ask GM1 (the shared teaching shell) time to
land; step shell-adoption waits for it, and no step copies the shell
to get around it.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | foundation | the machinery, adapted from X_eTaL-fine-tuning's (what must differ noted for ask GM2): the two pins (G1), `just xetal`, the search path, `scripts/py`, the gate, the catalog and the site build (port 3285), `demos/_template` with the `demo.toml` fields of G5, the library tooling (`just new-lib`, `just test-lib`), `.gitignore`, `CHANGES.md`, the logo and title; the first library of this repository, Manifold, with the sphere; and a first demo on it, sphere-hello (command line only): straight-line and great-circle distance between unit vectors rank neighbors the same, a plain mean leaves the sphere, proving the chain from the pins to a tested library and program |
| 2 | manifold-tour | the first live page (G2 as amended, G13; the user, 2026-11-06): an introduction to what a manifold is, the kinds these pages use (the circle, the sphere, the torus, a Swiss roll, the Poincare disk, each with its intrinsic and ambient dimension and one measurement that shows it is locally flat) and what each is for in the experiments to come; every coordinate from Manifold; each surface turning, with a flat view and the numbers beside it, by the shared view of GM3 if landed, else a demo-private projection noted as the workaround; built on `shared/microscope` by path, published, `just check-live` |
| 3 | research-foundation | M0: `docs/glossary.md` (principle 4); `docs/papers.md`, every paper the research names read at its source or marked as not found (G12); `docs/research-questions.md`, H1 to H3 each with its test, its measures and what would count against it; the experiment schema (a preset, a run's record, the diagnostics of G8); the function needs sorted by G3 and checked at the pin, `docs/xetal-libraries-asks.md` written and the function asks added to `docs/xetal-ML-asks.md` |
| 4 | experiment-kit | what every comparison shares, at the command line: three seeded data sets with a known structure (flat clusters, directions on a sphere, a tree), the split, the run's record and the diagnostics as the schema says, the `compares` check of G5 in the gate; proven by baseline-knn: k nearest neighbors under straight-line distance on all three, the baseline row of the H1 table |
| 5 | shell-adoption | once asks GM1 and GF1 have landed: the X_eTaL-ML pin moved (alone, the full gate), the shared teaching shell wired to this repository (its site, its glossary rows from `docs/glossary.md`, the parts of its page arc, the browser check that requires them), the comparison table of a page (each figure measured or analytical), and the first live pages, sphere-hello and baseline-knn, published. If the asks have not landed the step is blocked and the user is told; the shell is not copied |
| 6 | representation-sphere | representation-explorer, live, part 1 (M1): flat against spherical on the flat and the directional data at equal dimension: distances and shortest paths, nearest neighbors and decision boundaries, a mean on each space, a small classifier on raw and on unit-length representations; accuracy, convergence and distortion measured (distortion joins Manifold) |
| 7 | representation-hyperbolic | representation-explorer, part 2: the Poincare ball joins Manifold; the tree; an embedding fitted in each of the three spaces at equal dimension, its distortion and the accuracy from it; curvature as a control; the H1 table, three data sets by three geometries, with where the matching geometry did not help |
| 8 | metric-descent | optimization-explorer, live, part 1 (M2): one fixed loss, `L = (x^2 + 3 y^2) / 2`, its contours, and the paths of gradient descent under different metrics: the objective does not change, the path and the number of steps do; the steepest direction depends on the metric; a step on a sphere (Manifold's: project, step, put back) against a flat step followed by renormalizing |
| 9 | metric-training | optimization-explorer, part 2: a small model trained with plain gradient descent, a diagonal preconditioner and a natural-gradient approximation on the same data and seeds: loss by step, steps to a target, the cost of each step counted, and whether the metric paid for itself |
| 10 | mhc-matrices | mhc-explorer, live, part 1 (M3): the library Mixing begun; residual streams and their mixing matrix; the doubly stochastic matrices of size 3 (nine entries, four free) and the path from the identity to uniform mixing; Sinkhorn's iteration from a free matrix, its row and column sum errors by iteration; singular values, the mean of the streams kept and their differences shrunk; the Birkhoff polytope taught as a polytope |
| 11 | mhc-depth | mhc-explorer, part 2: Mixing's stack; the size of the signal forward and of the gradient backward through a growing stack, with plain residual, free, doubly stochastic and orthogonal mixing; then the same four trained on one small task at several depths; the H2 table, with what each constraint cost and where it did not help |
| 12 | vector-fields | dynamics-explorer, live, part 1 (M4): a field as a direction at every point; attractor, rotation, spiral, saddle, expanding spiral; Euler and Runge-Kutta paths and their error; divergence and curl; a gradient field against one with no potential, as ML meets it: gradient descent on a loss beside simultaneous descent and ascent on a two-player objective, which circles |
| 13 | manifold-flows | dynamics-explorer, part 2: a small learned field carrying one set of points to another, in flat space and held to a sphere (Manifold's tangent field and a step that stays on it), on the directional data: where the points end, how far they leave the sphere, the cost; what a flow that can be run backward cannot do (it cannot tear one piece into two) |
| 14 | lab-review | the release's acceptance pass: `docs/experiments.md` with the H1 and H2 results and their null results; every page read cold against the new-eyes checklist; every visible symbol and control explained (a test lists them against the glossary); a replay test for each preset; the libraries' docs and tests complete, and what in them has turned out general offered to X_eTaL-libraries; the catalog, the README and the paper catalog true; Saga 2 planned with the user |

Acceptance for the saga: four explorers live, each comparing at
least two geometric choices on the same problem with measured
results; H1 and H2 each have a results table that includes where
geometry did not help; every number shown comes from the X_eTaL run;
the glossary keeps a manifold, a polytope, a metric, a curvature and
a topology apart and the pages keep to it; the paper catalog says how
each paper was read.

## Saga 2 -- adaptive-geometry  [PLANNED, to be planned]

The research's P1 demos and its third hypothesis. Opens with a
planning step that reads the papers at source, settles the demos with
the user and writes `docs/adaptive-plan.md`.

- learned-curvature (H3): curvature trained with the representation
  on hierarchical data; task loss set against geometric distortion,
  since the one can improve while the other gets worse (the research's
  reading of a November 2026 paper on learnable-curvature hyperbolic
  networks, to be read at source); fixed, trainable and mixed
  curvature.
- attention-geometry: similarity and retrieval by dot product, by
  angle and by distance along the space, on X_eTaL-ML's Attention
  library.
- generative-flows: a distribution carried to another by a learned
  flow, in flat space and held to a manifold.

## Saga 3 -- constraint-lab  [PLANNED, to be planned]

mHC as a research track of its own, after mhc-explorer has a
baseline: a laboratory of constraint sets for residual mixing. Opens
with a planning step (`docs/constraints-plan.md`). Candidates, each
known so far only through the research text: Kronecker products of
small doubly stochastic matrices (KromHC), expanded streams with
sparse updates (xHC), residual routing as a fine-tuning method beside
LoRA (reported there as not consistently better alone), the Birkhoff
geometry of what mHC keeps (mean and difference channels,
contraction), and other sets with other meanings: Stiefel, symmetric
positive definite, symplectic, sparse stochastic. They are not
interchangeable: an orthogonal matrix may have negative entries, a
doubly stochastic one may not.

## Saga 4 -- topology-and-subspaces  [PLANNED, to be planned]

The research's P2 demos, the most exploratory. Opens with a planning
step (`docs/topology-plan.md`).

- topology-and-dimension: two interlocking circles and a Swiss roll;
  the representation and its neighborhood graph at each layer; a
  fixed flat embedding, a learned-curvature one and a flow with added
  dimensions: bending, separating and changing topology told apart.
  Feature geometry over training as a graph and its curvature (the
  research's reading of a paper on discrete Ricci flow).
- adaptive-manifold: a fixed geometry against a mixed or adaptive one.
- geometric-weight-updates: fixed, rotating and rank-changing
  low-rank subspaces (Stiefel and Grassmann geometry; a change of
  rank changes the dimension, it is not a smooth deformation). The
  geometry is shown here at toy scale; its use for adapters is
  X_eTaL-fine-tuning's (its Saga 3, shared subspaces): the boundary
  is settled with the user when this saga is planned.

Deferred beyond these: networks that change topology while training,
fine-tuning of large models, production-scale geometric networks,
geometry-aware search without gradients (X_eTaL-fine-tuning's Saga 4
is its home).

## Open questions (for the user)

None at the moment. Each saga's planning step and step lab-review put
new ones here.

## Cross-cutting

- Move a pin (`just ml-pin`, `just libs-pin`) when an ask has landed
  upstream, or at the start of a saga; never in the middle of a step,
  and one pin at a time, in its own commit, after `just gate --full`.
- When an ask lands, the step that moves the pin removes its
  workaround and marks it landed.
- Every comparison: the same data, split and seeds for every choice;
  budgets stated; each figure marked measured or analytical; what the
  run does not show, said.
- Every page: the six-part arc, every symbol and control explained,
  reachable by keyboard and touch; the flat illustration and the
  learned or trained thing never confused.
- `docs/papers.md` gains a row before a step builds on a paper, not
  after.

## Settled questions

- The repository's name and scope (the research, its second part):
  X_eTaL-manifolds, an experimental and educational repository on the
  intersection of manifolds and ML, not a general mathematical
  library.
- Three hypotheses (H1 to H3), four first demos (representation,
  optimization, mHC, dynamics), one page format, X_eTaL first with
  reusable primitives in the existing libraries (the research's final
  recommendation).
- Representation is the first explorer, though mHC prompted the work.
- mHC is a major track, not the whole project.
- Topology-changing networks, large-model fine-tuning and
  production-scale geometric networks are deferred.
- Share, do not replicate (the user, 2026-11-06): the teaching shell
  is not copied from X_eTaL-fine-tuning; the refactoring that lets
  both repositories use one copy is asked of X_eTaL-ML and
  X_eTaL-fine-tuning (GM1, GF1), and live pages wait for it (G2).
- Where a library lives (the user, 2026-11-06): particular to
  manifolds and the geometry of ML, developed here; general,
  X_eTaL-libraries; machine learning not about manifolds, X_eTaL-ML
  (G3).
- Port 3285 for this repository's locally served demos (the user,
  2026-11-06).
- The plan is `docs/plan.md`, as in every sibling (the user,
  2026-11-06).
- How asks reach the siblings (the user, 2026-11-06): the user points
  the other repositories' agents at this repository's
  `docs/xetal-<repo>-asks.md` files; this repository reads their
  answers at their HEADs and takes what lands by moving a pin. A pin
  and a gitignored clone of any sibling are allowed when needed (G1).
- Papers (the user, 2026-11-06): none is tracked in this repository;
  `docs/papers.md` keeps the link to each and the notes made from
  reading it (G12).
- Views (the user, 2026-11-06): three-dimensional views first, a flat
  reading view and the numbers beside them (G13); the turning view is
  asked of X_eTaL-ML (GM3).
- The sorting of three borderline needs stands as the plan has it
  (2026-11-06): a preconditioned step and the Fisher matrix are
  X_eTaL-ML's, Sinkhorn's iteration is X_eTaL-libraries', the data
  sets with a geometric structure are this repository's. Confirmed at
  the pins 2026-11-07 (GM4, GL3; the data sets in the experiment kit).
- The papers (2026-11-07, step research-foundation, `docs/papers.md`):
  every paper the research names was found and read, all but one at
  source (the task-geometry paper is paywalled: abstract and front
  matter); "adaptive manifold curvature regularization" is no distinct
  paper and nothing rests on it; the mixed-curvature paper is Gu,
  Sala, Gunel and Re; the Birkhoff-geometry paper's notion is a
  fading-memory horizon, not "information retention". H1 to H3 are
  written with what counts for and against each before any run
  (`docs/research-questions.md`).
- GM1 and GM3 landed in X_eTaL-ML 501ab15 (2026-11-07) and were
  taken by step shell-adoption (2026-11-08, the pin at 2d23a96): every
  page is on the shared teaching shell and turns with the shared view;
  GF1 is answered through GM1; GM2 waits on nothing from here. Wiring
  the shell filed GM7, GM8 and GM9, each with a thin workaround here.
  The page arc is twelve frames (the shell's six and this
  repository's six), required of every page by the browser check and
  of every README by the demos check. No ask is a blocker.
