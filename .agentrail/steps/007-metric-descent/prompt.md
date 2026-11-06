Step metric-descent (docs/plan.md, Saga 1 row 7: the research's
milestone M2, first half; docs/research.txt part 1 sections 3 and 6,
"Visualizing a change in metric").

Build demos/optimization-explorer, live: track "optimization",
comparing gradient descent under different metrics on one fixed
objective.

The question: is "downhill" one direction? (No: it depends on how
distance is measured.)

The program:
1. One loss, `L(x, y) = (x^2 + 3 y^2) / 2`, its contours as data the
   page draws, its gradient (checked against central differences).
2. The update `theta <- theta - eta * inverse(G) * gradient` for a
   positive-definite G: the identity (Euclidean), `diag(a, 1)` with a
   as the control, and the loss's own curvature as the metric
   (Newton's direction, where the bowl becomes round). The same start,
   the same step size policy stated; the path of each as points; steps
   to a target loss; the loss by step. The objective never changes:
   show it.
3. The steepest-descent direction at one point under two metrics,
   worked in full: the unit ball of each metric and the direction
   that lowers the loss most within it.
4. A step on a manifold against a flat step repaired afterwards: find
   the unit vector that maximizes `x' A x` for a small symmetric A
   (its leading eigenvector, checked against Matrix's) by (a) a
   Riemannian step on the sphere (the gradient's tangent part, a
   step, put back) and (b) a flat gradient step followed by
   renormalizing; the paths on the sphere, steps to a tolerance, how
   far each leaves the sphere between repairs.

The page, in the arc of principle 6. Keep apart, in words and
pictures: the loss (fixed), the metric (chosen), the path (a
consequence). A gradient field comes from a potential; say so, since
step vector-fields returns to it. The research's caution belongs on
the page: the formula with inverse(G) is the step in coordinates; on
a curved space a way back onto the space (an exponential map or a
retraction) is also needed, which part 4 shows.

Libraries (CLAUDE.md rule 3): the step on the sphere (the gradient's
tangent part, a step, put back) joins libs/Manifold here. The
preconditioned step (a positive-definite matrix applied to a
gradient) and the contour and field pictures are X_eTaL-ML's and
X_eTaL-libraries' by G3: use them if the asks have landed at the
pins (move a pin first, alone), otherwise write the expressions out
in the program as the asks' workarounds say. Nothing is copied.

Tests: paths against hand arithmetic for the first steps, the metric
control's presets replayed, the eigenvector agrees with the
library's, every symbol and control in the glossary, the browser
check, baselines. A cold reading.

README, the demo's README, CHANGES.md, the plan; publish and
check-live.
