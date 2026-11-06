Step vector-fields (docs/plan.md, Saga 1 row 11: the research's
milestone M4, first half; docs/research.txt part 1 section 3,
"Vector fields: how geometry becomes motion").

Build demos/dynamics-explorer, live: track "dynamics", comparing a
gradient field with a general vector field as ML meets them.

The question: a model's state moves a little at every step; what
decides where it goes, and must it always be heading downhill?

The program:
1. A field as a direction and a size at every point, `dx/dt = v(x)`;
   five plane fields as presets (attractor, rotation, spiral, saddle,
   expanding spiral), each a 2 x 2 matrix; the arrows on a grid and
   the paths from several starts, as points computed by X_eTaL.
2. Integration as a choice with consequences: Euler and a fourth
   order Runge-Kutta step at the same step size on the rotation,
   where the exact path is a circle: the distance from the circle by
   step (Euler spirals out); step size and step count as controls.
3. Divergence and curl of each field by differences, beside the exact
   values; what each says (spreading or gathering; turning).
4. Gradient or not: a field that is minus the gradient of a loss has
   no curl; the rotation has no potential (walk once around and the
   "height" would not come back to itself: compute the sum around a
   loop). One case worked in full.
5. As ML meets it, the comparison of this demo: gradient descent on a
   loss (a gradient field: it settles) beside simultaneous descent
   and ascent on a two-player objective `f(x, y) = x y` (one player
   lowers f, the other raises it: the field is a rotation, and plain
   steps spiral outward). The same step size and starts; distance
   from the solution by step for each; one known repair (for example
   taking the players' steps in turn, or a look-ahead step) measured
   beside it and cited.
6. Stability at a rest point from the field's matrix: the signs of
   the real parts of its eigenvalues against what the paths did (for
   matrices that are not symmetric, compute the 2 x 2 case from trace
   and determinant, written out).

The page, in the arc of principle 6; arrows and paths drawn from the
run; the caption the research uses is kept true: these are plane
illustrations and one small game, not a learned field (that is step
manifold-flows). Link back to optimization-explorer: its paths were
flows of a gradient field under a metric.

Libraries (CLAUDE.md rule 3): the integrators (Euler, Runge-Kutta),
the divergence and curl by differences, and the arrows and contour
pictures are general, X_eTaL-libraries' by G3 (the asks step
research-foundation wrote): use them if landed at the pin, otherwise
written out in the program and named as the workaround. Nothing in
this step is particular to manifolds, so nothing joins Manifold.

Tests: paths against the exact solutions of the linear fields within
a stated tolerance, divergence and curl against exact values, the
loop sum zero for the gradient field and not for the rotation, every
symbol and control in the glossary, the browser check, baselines. A
cold reading.

README, the demo's README, CHANGES.md, the plan; publish and
check-live.
