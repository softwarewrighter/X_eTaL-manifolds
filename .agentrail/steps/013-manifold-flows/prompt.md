Step manifold-flows (docs/plan.md, Saga 1 row 13: milestone M4,
second half; docs/research.txt part 1 sections 3, 5 and 10;
docs/papers.md, the rows on neural ODEs and on flows on manifolds,
read at source first).

Extend demos/dynamics-explorer with a learned field, and compare a
flow in flat space with one held to a sphere.

Deliver:
1. A small network as a vector field `v(x)` (a few dozen weights),
   and inference as integration: a point is carried along the field
   for a fixed number of steps. Trained so that points of the
   directional data (step experiment-kit), put on the unit sphere,
   are carried from a start arrangement to their class's region; the
   loss and its gradient through the unrolled steps, checked against
   central differences on a tiny case. By G3 the network as a field
   and its unrolled gradient are X_eTaL-ML's (the ask step research-foundation
   wrote: use it if landed at the pin, otherwise written out); what
   holds the flow to the sphere (the tangent part of a field at a
   point, a step that stays on the sphere) is Manifold's, added here.
   If a workaround would be unreadable the step waits and the user is
   told.
2. The comparison, same data, seeds, network size, steps and step
   size:
   a. flat: the field unconstrained in the ambient space, the point
      free to leave the sphere;
   b. flat, repaired: the same, put back on the sphere at the end;
   c. on the sphere: the field's tangent part only, each step
      followed by a step back onto the sphere (a retraction).
   Measures: where the points end (loss, held-out accuracy), how far
   they leave the sphere along the way (constraint violation by
   step), the cost counted.
3. What a flow can and cannot do: a flow that can be run backward
   moves points without tearing: two classes that start interleaved
   on a circle cannot be separated by such a flow on the circle, and
   can once a dimension is added. Show the small case, by the run;
   state it as the limit it is, and that changing topology is Saga
   4's subject, not shown here.
4. The page: the learned field as arrows on the sphere (turning through
   GM3's shared view where landed, and as a flat map with the numbers;
   plan G13), the points moving along it step by step from the
   recorded states (nothing recomputed in the browser), the three
   variants side by side, the violation chart. The learned field is
   marked learned; the earlier fields are marked illustrations.
5. docs/experiments.md: the dynamics results, with what the sphere
   constraint bought and cost; README Status: the four explorers
   live.

Tests: the tangent part is orthogonal to the point within tolerance,
the on-sphere variant's violation stays under the stated tolerance,
replay of each preset, every symbol and control in the glossary, the
browser check, baselines. A cold reading.

README, the demo's README, CHANGES.md, the plan; publish and
check-live.
