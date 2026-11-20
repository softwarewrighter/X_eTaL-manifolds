# Optimization explorer: is downhill one direction, and does a ruler pay?

Part 1: one bowl-shaped loss that
does not change, L(x, y) = (x^2 + 3 y^2) / 2, and gradient descent on
it under three rulers (metrics) from the same start with the same
step size policy; the steepest direction at one point under two
rulers, worked in full; and the same question on a curved space, a
step that stays on the sphere against a flat step repaired afterward.
Live at
[softwarewrighter.github.io/X_eTaL-manifolds/optimization-explorer](https://softwarewrighter.github.io/X_eTaL-manifolds/optimization-explorer/).
Part 2 trains a small softmax model three ways,
plain, a diagonal preconditioner and the natural-gradient step, with
one input badly scaled and as it is, and counts what each step costs.

## The question

Is downhill one direction? When training lowers a loss by stepping
against its gradient, is the direction of the step a fact about the
loss, or a choice? It is a choice: the gradient is the direction of
fastest rise per unit of distance, and distance is a metric, a ruler
laid on the parameter space. Change the ruler and the steepest
direction changes, though the loss and its contours do not move.

## Background

A loss is a number that says how badly a model is doing; here the
model is two numbers, x and y, and the loss a bowl three times as
steep along y as along x. Its contours are ellipses; its gradient
(x, 3 y) drawn at every point is a gradient field, a field of arrows
that comes from the one function L. A metric G gives a step v the
length sqrt(v' G v); diag(a, 1) counts x-steps a times as heavily. A
step under a metric is p <- p - eta P g with P the inverse of G (the
preconditioner). The step size policy here is the exact line search
along the direction, so the ruler chooses the direction and nothing
else (a ruler and twice that ruler give the same path).

Part 2's model is a softmax classifier (softmax regression) with six
weights: two classes' scores, each a bias plus a weight per input, the
third class's score fixed at 0 so the Fisher matrix is not singular.
Its loss is the cross-entropy. The Fisher matrix F (6 by 6) measures a
change of the weights by how much it changes the predictions; the
natural-gradient step solves (F + lambda I) d = g with damping
lambda; the diagonal preconditioner divides each weight's gradient by
F's diagonal entry plus lambda. Training runs at a fixed rate per
method, not a line search.

## The idea

Keep the loss fixed, change only the ruler, and measure what changes:
the first step's direction, the shape of the path, the steps to a
target loss, and what the ruler costs (numbers, multiplies per step).
The bowl's own curvature H = diag(1, 3) as the ruler makes the bowl
round and the floor one step away (Newton's direction); diag(a, 1)
dialed from 1/9 to 9 passes through that choice at a = 1/3.

On a model the bowl is the loss over the data, and its shape depends on
how the inputs are written: an input recorded as numbers twenty times
larger changes nothing about the classes and stretches the bowl along
its weight. A ruler that measures by predictions should not notice; a
plain step should. So part 2 trains on the data as they are and with
one input badly scaled, expecting the metric to matter in the second
case and to buy little in the first.

## What this is, and is not

An experiment on a toy loss, not on a model: everything can be checked
by hand, and the gradient is checked against central differences in
the program. It compares three rulers on one loss and, on the sphere,
two ways of keeping a step on the constraint. It is not a test of H1,
H2 or H3: it answers the optimization area's question of
`docs/research-questions.md` (does the metric of a step change the
path and the cost?) on a loss whose answer can be seen whole; whether a
metric pays on a model is part 2's.

Part 2 is a measurement on a real, if tiny, model: it forms the Fisher
exactly, which a model of a million weights could not (the matrix
grows with the square of the weights, its solve with the cube).

## The intuition

A long narrow valley: the steepest step from its side goes across the
valley, not along it, so the plain walker zigzags to the lake. A
ruler that counts sideways steps as long turns the valley round, and
the steepest direction by that ruler points down the valley. On the
sphere there is a further catch: the gradient's arrow is straight and
the surface is not, so a step either follows the surface (the
exponential map) or goes straight and is pushed back (a retraction).

On a model the ruler is about units: a plain step treats one unit of
every weight as equal, though one may move the predictions far more
than another. The Fisher ruler measures by the predictions, which do
not care how the inputs are written.

## The mathematics

The gradient g = (x, 3 y), the Hessian H = diag(1, 3). Under a metric
G the steepest direction is -P g, with P the inverse of G; scaled to
length 1 under G it is v = -P g / sqrt(g' P g), the point of G's unit
ball where the loss falls fastest. The exact line search along
d = P g on this bowl is eta = (g . d) / (d' H d). Under the plain
ruler each step is at right angles to the last (the zigzag), and the
rate depends on the condition number, 3; under diag(a, 1) the
condition number becomes the larger of 3 a and 1 / (3 a), 1 at
a = 1/3 where diag(a, 1) is H / 3.

On the sphere: the Rayleigh quotient f(p) = p' A p among unit vectors
is largest at A's leading eigenvector. Its gradient in the room is
2 A p; the Riemannian gradient on the unit sphere is that arrow's part
along the sphere. The exact step goes along the sphere by the
exponential map (Manifold's `s_phereDescend`: tangent part, then
`s_phereStep`); the retraction adds the tangent vector and scales back
(`s_phereRetract`); the flat step adds 2 eta A p, leaves the sphere by
about the arrow's length, and is scaled back (`s_phereProject`). The
flat step is the power iteration on I + 2 eta A.

Part 2: the gradient is g = (Q - Y)' X / n (Q the predicted
probabilities of the two free classes, Y the answers, X the inputs
with a 1 for the bias); the Fisher is the mean over rows of
kron(M_i, x_i x_i'), M_i = diag(q_i) - q_i q_i', checked against
that definition on one row written out and against central
differences of the gradient (for softmax regression it equals the
loss's curvature, so the natural step at rate 1 is Newton's). The cost
counted for what each step forms: 12 multiplies per training row for
the probabilities and gradient, 12 more for the diagonal alone, 36 more
for the whole Fisher and about 108 for the 6 by 6 solve.

## What is compared

Three rulers on the bowl, the same start (2.3, 1), the same 30 steps,
the same line search: the identity, diag(a, 1) at eight values of a
(1/9, 1/6, 1/3, 1/2, 1, 2, 3, 9: the preset gives them as ninths so
that a third is exact), and H. Measured: the path, the loss by step,
the steps to a loss under 1e-6, the loss after one and five steps,
the first step worked, the numbers in the ruler and the multiplies per
step on P g. On the sphere, three ways from the same start at rate
0.1 for 60 steps: the exact step, the retraction, the flat step
repaired; measured per step: the quotient, the distance along the
sphere to Matrix's eigenvector, how far off the sphere the step went
before repair; the steps to a distance under 1e-4.

Written before the run: the three paths coinciding, or the step counts
agreeing within one, would count against "downhill depends on the
ruler"; the flat step arriving at the same place in the same number of
steps would count against the step that stays on the sphere (the
repair then costs nothing visible here).

Part 2: plain gradient descent (rate 0.5), the diagonal preconditioner
(0.5) and the natural-gradient step (1, damping 0.001), from all
weights 0, for 60 steps, on the kit's flat clusters (120 points, 90
trained on, 30 held out), with x1 times 20 and as they are; the plain
rate swept (0.5, 0.05, 0.005, 0.0005) on the badly scaled version.
Written before the run: the metric pays where it stays under a loss of
0.35 in fewer multiplies in all than the plain step; with the bad
scaling it should, on clean data it should buy little (a null result
for it), and undamped its predictions should not depend on how the
inputs are written.

## The run

No model is trained: each descent is 30 steps of p <- p - eta P g with
eta from the line search, from the preset's start; each climb 60 steps
at the preset's rate. The preset is `presets/metrics.txt` (data, read
by keeping its digits and dropping its words).

Part 2 trains for real: ten runs of 60 steps (three methods on two
versions of the data, and four plain rates) and nine undamped runs of
ten steps for the invariance check, from `presets/training.txt`.

## The program

[`optimization-explorer.xtl`](optimization-explorer.xtl), in five
parts: the preset; the loss, its gradient (checked against central
differences at three points), its contours (the ellipses' own formula,
49 points each) and its gradient at a 7 by 5 grid; descent under a
metric (`u:e_ta` the line search, `u:s_tep` the update, `u:d_escend`
a path, the three rulers, the first steps worked, the record, the
control's eight descents); the steepest direction under two rulers
worked, each ruler's unit ball, and the check by sampling 360
directions; and the sphere (A, Matrix's eigenvalues and eigenvectors,
the three climbs, the gradient of the quotient checked, the record).
Then the printed sections and 24 checks: the first steps against hand
arithmetic (plain: eta = (2.3^2 + 9) / (2.3^2 + 27); Newton: eta = 1
and the floor), the zigzag's right angles, the control's rows at
a = 1 and a = 1/3 equal to the plain and Newton records, the step
counts falling toward a = 1/3 from both sides, the unit balls, the
sampled directions, the eigenvector reached by every climb, the flat
step off the sphere and the exact one not.

Part 2 is [`optimization-training.xtl`](optimization-training.xtl):
the preset; the data and split; the model (`u:p_robs`, `u:l_oss`,
`u:g_rad`); the Fisher (`u:f_isher` by blocks, `u:f_isherDiag` the
diagonal alone); the three steps and `u:t_rain`; the record with
"first under" and "stays under from"; the sweep; the Fisher's
eigenvalues; four changes of the weights measured by both rulers; the
invariance check. 17 checks: the gradient against central
differences, the Fisher against its definition and against the
gradient's central differences, its diagonal formed alone, the
metric's effect and the invariance.

## How it works

Every descent is one `p_ower` loop over the state (point, path,
losses); the metric enters only as P in `u:s_tep`, so the plain step,
the control and Newton's direction are the same line with a different
P. The line search is closed-form on a quadratic. The control runs
`u:d_escend` at each a and keeps every path and every loss by step for
the page. On the sphere, the three climbs are three `p_ower` loops
differing only in the step: `s_phereDescend`, `s_phereRetract` of the
tangent part, or `p + rate 2 A p` then `s_phereProject`, with
`s_phereOff` measured before the repair.

## Results

From the baseline (`reg/cli-optimization-explorer.out`). On the bowl,
from (2.3, 1) with loss 4.145: the plain ruler takes 11 steps to a loss
under 1e-6 (loss 0.983 after one step, 0.0031 after five); Newton's
direction takes 1 (eta = 1, the floor exactly); the control at
a = 1/9, 1/6, 1/3, 1/2, 1, 2, 3, 9 takes 9, 7, 1, 5, 11, 18, 22, 29
steps, with condition numbers 3, 2, 1, 1.5, 3, 6, 9, 27. Two values of
a with the same condition number (1/9 and 1; 1/6 and 1/2) take about
as many steps, not the same (9 and 11; 7 and 5): the condition number
bounds the rate, the start sets the rest. The first plain step goes to
(1.282, -0.328) with eta 0.4426; every plain step is at right angles
to the last (checked to 1e-12). The steepest direction at the start
is (-0.608, -0.794) under the plain ruler and (-0.799, -0.347) under
H; the sampled best of 360 directions agrees with each within the
sampling.

On the sphere, A = (4 1 0 / 1 3 1 / 0 1 2), eigenvalues 4.732, 3,
1.268, leading eigenvector (0.789, 0.577, 0.211), from
(1, -1, 1) / sqrt 3 at rate 0.1: the exact step reaches a distance
under 1e-4 in 24 steps and leaves the sphere by 1.1e-16 at most; the
retraction takes 24 too, its unrepaired step off the sphere by 0.045
at most; the flat step takes 49 and goes off the sphere by 0.946 at
most (0.906 on average) before each repair. All three end at the
quotient 4.7321 within 1e-9 of the largest eigenvalue. The contraction
factors from the eigenvalues (analytical) are 0.822 per step for the
flat step (the ratio of the top two eigenvalues of I + 0.2 A) and
0.654 for the step along the sphere (1 - 0.2 (4.732 - 3)); the
program checks the measured counts against the steps those predict,
within three. Nothing counts against "downhill depends on the ruler"
here (the paths differ, the counts differ by more than one), nor
against the step that stays on the sphere beside the flat step (twice
the steps); but by the same rule the retraction, arriving in the same
count and at the same end distance as the exact step, says the exact
step's sine and cosine bought nothing over the cheap step on this
problem: a result, recorded as one (the two part in the third decimal
at step 1 and agree to 1e-11 at the end). Results for the
record, not a hypothesis table: `docs/experiments.md`, "Optimization
geometry".


Part 2, from `reg/cli-optimization-training.out` (17 checks):

| method | x1 times 20: stays under 0.35 from | end loss | as they are: from | end loss | held out | multiplies per step | multiplies to stay under (as they are) |
| ------ | ---------------------------------- | -------- | ----------------- | -------- | -------- | ------------------- | -------------------------------------- |
| plain, rate 0.5 | not under at the end (dips under at 54) | 59.6 | 5 | 0.211 | 0.767 | 1,080 | 5,400 |
| diagonal, 0.5 | 4 | 0.164 | 4 | 0.164 | 0.867 | 2,160 | 8,640 |
| natural, 1 | 2 | 0.163 | 2 | 0.163 | 0.900 | 4,428 | 8,856 |

The plain rate sweep on x1 times 20 (0.5, 0.05, 0.005, 0.0005): end
losses 59.6, 0.731, 0.587, 0.760, none under the target at the end. The
Fisher at the start has condition number 13,906 with the bad scaling
and 124 without (112 times). Undamped, ten steps, largest gap in a
probability against the data as they are: plain 0.96 (rescaled) and
0.61 (mixed); diagonal 2e-16 and 0.29; natural 2e-16 and 5e-16.

## Beyond this page

The research's own picture of a change of metric (same loss, metric
diag(a, 1)) and its caution that p - eta inverse(G) g is the step in
coordinates and a curved space also needs a retraction or an
exponential map (`docs/papers.md`); Absil, Mahony and Sepulchre's
chapter 4 for the retraction and the step on the sphere; Amari for the
natural gradient, which part 2 tries. Open: whether a metric pays for
itself on a model, where the curvature changes from point to point and
a full H is out of reach (part 2); a fixed step size instead of a line
search, under which the ruler's size and the step size are one choice;
a ruler that changes from point to point (the Fisher metric).

Part 2 forms Amari's natural gradient (1998) exactly because the model
is tiny; practical methods approximate the Fisher. Adam's per-weight
scaling is a cousin of the diagonal preconditioner, not the natural
gradient. For a metric learned with the model (H3): the cheap fix of
writing the inputs to one scale is measured here (plain on the data as
they are stays under the target from step 5), and a learned metric
would have to beat it.

## Run it

```bash
just run optimization-explorer         # part 1, its sections and 24 checks
just run optimization-explorer optimization-training.xtl   # part 2, 17 checks
just show optimization-explorer        # as a notebook
just test-demo optimization-explorer   # CLI baseline, web tests, wasm build, browser check
just serve optimization-explorer       # the page on port 3285
```

The page (`web/`): the bowl with the three paths and the control's a
picked by buttons, the loss by step, the first steps worked; the
control's chart and table with the change narrated from the measured
values; the two unit balls with the worked direction; the sphere
turning with the three climbs and the eigenvector, the distance by
step, and the first steps' numbers; the comparison tables; the
glossary; the program and the preset. Every number is one the program
printed; picking a value of a runs nothing.

## Summary

Downhill is not one direction: on one bowl that did not move, three
rulers gave three first steps, three paths and three step counts; the
ruler of the bowl's own curvature made the floor one step away, and
the dial a showed the counts falling toward that ruler from both
sides. On the sphere, a step along the surface and a straight step
pushed back both reached the leading eigenvector; the straight one
left the sphere by a measured amount before each repair and took
twice the steps; the cheap retraction matched the exact step in count
and end distance, so the exact step bought nothing over it here. A preconditioner is a choice of geometry on the
parameter space: it changes the path and the count, not the loss, and
it costs numbers and multiplies per step; whether it pays on a model
is part 2's question.

On the model, with one input twenty times too large, the plain step
thrashed and none of four rates settled in 60 steps, while the
diagonal and natural steps took the same 4 and 2 steps as on clean
data: the metric paid. On the data as they are the natural step took 2
steps against plain's 5 and spent 8,856 multiplies against 5,400: it
did not pay, a null result. Undamped, the natural step's predictions
ignored a rescaling and a mixing of the inputs; the diagonal's only the
rescaling; the plain step's neither.

## Next

mhc-explorer: a constraint on a transformation instead of a ruler on a
step, residual streams mixed by doubly stochastic and orthogonal
matrices, and whether holding the mix to a set keeps a deep stack
stable (H2).

## Workarounds

- A step under a metric (`(lr, P) op:p_reconditioned (w, g)`) is
  X_eTaL-ML's by ask GM4 (`docs/xetal-ML-asks.md`): written out as
  `p - eta * P mx:m_ul g` in `u:s_tep`.
- The contours of a function of two variables are X_eTaL-libraries'
  by ask GL6 (`docs/xetal-libraries-asks.md`): this loss's contours
  are ellipses, drawn from their own formula (`u:c_ontour`), marked
  analytical on the page; the gradient field is the gradient at a
  grid of points, drawn as arrows by the page.
- A reader of labeled presets is X_eTaL-libraries' by GL9: the
  preset's digits are kept and its words dropped, so no word in
  `presets/metrics.txt` may hold a digit (the anisotropies are given
  as ninths, 1 1.5 3 4.5 9 18 27 81, divided by 9 in the program).
- The eigenvector check uses Matrix's `e_igenvectors` as it stands at
  the pin (no ask).
- The Fisher matrix of the model and the natural-gradient step are
  X_eTaL-ML's by GM4: written out in `optimization-training.xtl`
  (`u:f_isher`, `u:f_isherDiag`, `u:s_tepNatural`), with Matrix's
  `s_olve` for the system (a Cholesky solve is GL7). The split is
  GM5's, written out as in representation-explorer.
