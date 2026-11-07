Step research-foundation (docs/plan.md, Saga 1 row 3: the research's
milestone M0; principles 3 and 4; decisions G3, G7, G8, G12; docs/research.txt).

Documents only; no demo. Deliver:
1. docs/glossary.md: every term the plan's principle 4 lists and the
   research uses, each with a plain definition, a small example, what
   it is NOT confused with here, and where the first demo that uses
   it will be: manifold, ambient and intrinsic dimension, the manifold
   hypothesis (an assumption), tangent vector and tangent space,
   metric, geodesic, curvature (zero, positive, negative), topology,
   vector field, gradient field and potential, divergence and curl,
   flow and integrator, exponential map and retraction, Riemannian
   gradient, natural gradient, data manifold, parameter space,
   constraint set, polytope and the Birkhoff polytope, doubly
   stochastic matrix, Sinkhorn's iteration, orthogonal group, Stiefel
   and Grassmann manifolds, stratified set (matrices of rank at most
   r), residual stream, hyper-connections, mHC, distortion. Written
   so that its rows can become the pages' glossary rows unchanged
   (read ../X_eTaL-fine-tuning/shared/lesson/glossary.tsv for the
   columns; do not copy its rows).
2. docs/papers.md, the catalog (G12): a row for every paper
   docs/research.txt names or leans on (mHC and the hyper-connections
   paper before it, KromHC, the discrete-Ricci-flow paper, geometric
   neural ODEs on manifolds and Lie groups, ODE-based learning on
   manifolds, task-geometry decoupling in learnable-curvature
   hyperbolic networks, the Birkhoff geometry of mHC, xHC, mHC for
   parameter-efficient fine-tuning, adaptive curvature regularization,
   mixed-curvature learning) and the standard sources the first four
   demos need (hyperbolic embeddings, natural gradient, Riemannian
   optimization, Sinkhorn). For each: authors, date, venue, link; the
   claim the research makes of it; whether that claim holds when read;
   its assumptions; how it was read (at source in full, abstract only,
   not found); the notes made from reading it; what this repository
   plans to build from it. Find each on the web and read it; do not
   fill a row from memory or from the research text. No paper is
   tracked in this repository (the user, 2026-11-06): links and notes
   only, never a PDF. A paper that cannot be found is marked not found,
   and nothing in the plan may rest on it: list those for the user.
   Correct docs/plan.md where the research's account of a paper
   turns out wrong (dates and venues included).
3. docs/research-questions.md: the five research areas; H1, H2, H3
   each with the experiment that tests it, the data, the choices
   compared, the measures, the budget held equal, and, written before
   any run, what result would count for it, against it, or leave it
   open.
4. The experiment schema, in docs/research-questions.md or its own
   file: what a preset holds (the seed, the data, the choices, the
   budgets, the tolerances), what a run's record holds (the
   diagnostics of G8, each marked measured or analytical), how two
   runs are compared, what goes into a reg-rs baseline and what never
   does (runtime).
5. Every function the first four explorers need, sorted by G3
   (CLAUDE.md rule 3) and checked at the pin by reading the pinned
   libraries' docs and source. Start from the plan's section "The
   functions the demos need, and where each belongs" and correct it:
   - what exists already: named in the plan, with the function;
   - particular to manifolds and the geometry of ML: a row of the
     plan's table of this repository's libraries (Manifold, Mixing),
     with the function's name and type as planned and the step that
     builds it. Not an ask;
   - general: docs/xetal-libraries-asks.md, written here in the
     format of docs/xetal-ML-asks.md (GL1, GL2, ...);
   - machine learning, not about manifolds: rows added to
     docs/xetal-ML-asks.md after GM2 (GM3, ...); where
     X_eTaL-fine-tuning has asked for the same (its FM7, the learning
     task and harness), say so in the ask so one answer serves both.
   Each ask: a minimal example with the signature wanted in X_eTaL's
   naming style, the demos and steps that need it, the workaround
   (the expression written out), and blocker only where no readable
   workaround exists. The sorting of the three borderline needs the plan's settled
   questions name stands.
6. The sharing asks checked: read GM1, GM2, GF1 and GF2 against the
   siblings at their HEADs (has either repository answered, in a file
   of its own such as X_eTaL-ML's docs/fine-tuning-asks.md? has
   anything landed?) and bring their status up to date. Nothing
   outside this repository is changed.
7. docs/plan.md brought in line (the functions section, the open
   questions the reading settled), `agentrail plan --update
   docs/plan.md`; README (Documentation, Libraries, Status);
   CHANGES.md.

Report to the user: the papers not found or misdescribed, the asks
that are blockers, which steps wait on which asks, and where GM1 and
GF1 stand, since step shell-adoption is two steps away.
