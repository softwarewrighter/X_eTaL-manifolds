Step representation-hyperbolic (docs/plan.md, Saga 1 row 7:
milestone M1, second half; hypothesis H1; docs/papers.md, the rows on
hyperbolic embeddings, which must say "read at source" before this
step builds on them).

Extend demos/representation-explorer with negative curvature and the
hierarchical data, and complete the H1 table.

Deliver:
1. The Poincare disk in the program: the distance between two points,
   a shortest path between them (as points), what happens to
   distances near the edge, one distance worked in full; why a tree
   wants such a space (room grows fast with radius), shown by counting
   and not only said.
2. An embedding of the tree fitted in each of the three spaces at the
   same dimension (2) and the same number of steps, from the same
   start: minimize the gap between tree distance and distance in the
   space; for the disk the step is taken in the space (a Riemannian
   step that stays inside the disk). The gradient checked against
   central differences. Report distortion by step and at the end, and
   the worst pair.
3. A classifier from each embedding (the node's branch), held-out
   accuracy.
4. Curvature as a control: the same fit at a few curvatures, the
   distortion at each, from presets; the flat space as the limit.
5. The H1 table complete in docs/experiments.md and on the page:
   three data sets (flat, directional, hierarchical) by three
   geometries: distortion, held-out accuracy, steps to a target, cost.
   Say plainly where the matching geometry helped, where it did not,
   and what the experiment cannot show (toy size, one seed family,
   dimension 2). Set the result against what
   docs/research-questions.md wrote down beforehand as counting for
   or against H1.
6. The page: the disk drawn from X_eTaL's coordinates, a path and a
   point's neighbors on it; the three embeddings of the same tree
   side by side on one scale of distortion; the curvature control
   explained from measured before and after values.

Numerical care: points near the disk's edge lose digits; state the
tolerance, clamp as the source paper does and say so, and measure how
far any point leaves the disk (the constraint violation of G8).

Libraries (CLAUDE.md rule 3): the Poincare ball joins libs/Manifold
here, under the naming step foundation chose: its distance, putting a
point into the ball, the tangent part, a step that stays inside the
ball (the exponential map or a retraction, which one stated) and its
inverse, a mean if the fit needs one; the hyperbolic functions from
Trig. Tests near the edge and at the origin, the gradient of the
distance checked against central differences.

Tests, the cold reading, baselines; README, the demo's README,
CHANGES.md, the plan; publish and check-live.
