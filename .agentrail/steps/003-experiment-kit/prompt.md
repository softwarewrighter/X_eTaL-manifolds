Step experiment-kit (docs/plan.md, Saga 1 row 3; principle 1;
decisions G3, G5, G7, G8, G9; docs/research-questions.md, the
experiment schema; CLAUDE.md rules 3, 4 and 9).

Build what every comparison in this saga shares, at the command line,
so that later steps compare geometries and not data handling. No web
page in this step (step shell-adoption).

Deliver:
1. Three small seeded data sets with a known structure, each with a
   note of how it was made and its seed:
   - flat: a few Gaussian clusters in the plane and in a few more
     dimensions;
   - directional: classes that differ by direction and not by length
     (points around a few directions, lengths varied);
   - hierarchical: a small tree (about 31 nodes), its edges, the
     distance between every two nodes along the tree (Graphs'
     shortest paths), and a class for each node from its branch.
   By G3: a generator that is about geometric structure (directions
   on a sphere through Manifold, a tree and its distances) belongs to
   this repository, as functions of a library here or, if no function
   is needed, as data files with the short program that wrote them;
   say which you chose and why.
2. The split, the run's record and the comparison of runs, as the
   schema says. These are machine learning and not about manifolds:
   they are X_eTaL-ML's by G3 (the ask step research-foundation
   wrote, beside X_eTaL-fine-tuning's FM7). Check at X_eTaL-ML's HEAD
   whether it has landed; if so move the pin first (alone, `just gate
   --full`, its own commit) and use it; if not, write the expressions
   out in the program as the ask's workaround says, and keep them few.
3. The diagnostics of G8 in the record: training loss, held-out
   accuracy, size of an update, constraint violation, distortion
   (Manifold's, added here if step foundation did not), cost counted;
   each figure marked measured or analytical. Runtime never in a
   baseline.
4. demos/baseline-knn (track "start", intro, CLI only for now): k
   nearest neighbors (Learn's) under straight-line distance on all
   three data sets: held-out accuracy, and for the tree the distortion
   of its node coordinates against tree distance. This is the baseline
   row of the H1 table in docs/experiments.md, begun here. Its page
   comes with step shell-adoption.
5. Tests: the data reproduces from its seeds; the record's fields
   against the schema; the gate's `compares` check passes for the
   template and refuses a one-choice demo; baselines blessed after
   review.

README (Demos, Libraries, Status), the demo's README in the page arc,
CHANGES.md, the plan. Report where GM1 and GF1 stand: the next step
waits on them.
