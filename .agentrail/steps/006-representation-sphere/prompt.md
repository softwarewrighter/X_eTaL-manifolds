Step representation-sphere (docs/plan.md, Saga 1 row 6: the
research's milestone M1, first half; hypothesis H1; docs/research.txt,
"Example: Representation Explorer"; CLAUDE.md rules 4 to 6 and 9).

Build demos/representation-explorer, live: track "representation",
hypothesis H1, comparing flat (Euclidean) and spherical
representations on the flat and the directional data of step
experiment-kit, at equal dimension and equal trained numbers.

The question, in a newcomer's words: does it matter what shape of
space a model keeps its points in?

The program (short, with xetal doc comments; Manifold's sphere functions from
step foundation, X_eTaL-ML's and X_eTaL-libraries' functions, and the
asks' workarounds, by CLAUDE.md rule 3):
1. Distances: for the same pairs of points, straight-line, cosine and
   great-circle distance; one pair worked in full.
2. A shortest path between two points in each space (a segment; an
   arc of a great circle), as points computed by X_eTaL.
3. A mean in each space: the plain mean, and the mean on the sphere.
4. Nearest neighbors and a decision boundary under each distance.
5. An actual ML run: a small classifier trained on raw
   representations and on unit-length ones (the same network, seeds,
   split and number of steps): loss by step, held-out accuracy,
   steps to a target loss. Gradients checked against central
   differences in a test.
6. Distortion: how far each representation's distances are from the
   data's own.

The page, in the arc of principle 6: the question, background,
rationale, purpose; the intuition (a flat sheet and a globe, an
ambient and an intrinsic dimension); the mathematics with every
symbol explained; the experiment (choose the data set and the space;
the points, a path, the neighbors and the boundary redrawn from the
run); the training run; the comparison table (shell-adoption's);
what this shows and does not; the research extension (docs/papers.md
rows); a summary and what is next (the hyperbolic half). The sphere
is drawn turning (ask GM3's shared view, if landed at the pin; the
sphere's wire lines, the points and the path computed by X_eTaL) and
as a flat map with the numbers beside it (plan G13); if GM3 has not
landed, the flat map and the numbers, and the page says the turning
view is to come.

Expect and report honestly: on flat data the sphere may cost
accuracy; for unit vectors the three distances order neighbors the
same way (sphere-hello), so where a difference appears say what
caused it.

Libraries (CLAUDE.md rule 3): distortion (an embedding's distances
against given distances, as one number and by pair) joins
libs/Manifold here, with its docs and tests; the mean on the sphere
and the great-circle distance are Manifold's already. Nothing of
X_eTaL-fine-tuning's is copied.

Tests: the model natively (distances against hand arithmetic, the
mean on the sphere has length 1, the gradient check), every visible
symbol and control has a glossary term, the browser check, baselines
blessed after review. A cold reading of the page by a reviewer given
only its text, and its findings acted on.

README (Demos), the demo's README, docs/experiments.md (the H1 table
begun: two geometries by two data sets), CHANGES.md, the plan;
publish and check-live.
