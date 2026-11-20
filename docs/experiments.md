# Experiments

The results tables of the hypotheses (`docs/research-questions.md`),
one row per run, every number from a reg-rs baseline of the demo named
(`demos/<slug>/reg/`), with its null results where a geometry cost
more and helped nothing (plan, principle 3). Begun by step
experiment-kit with the baseline row; the explorers add theirs.

## H1: a geometry that matches the data's structure helps a representation

The three data sets of the kit (`libs/Kit`), split the same seeded way
(the held-out quarter, split seed 11), k = 5 nearest neighbors (k = 3
on the tree). Accuracy is held-out accuracy; distortion is Manifold's
(the mean over pairs, the worst pair) against the data's own
distances. All measured.

| Data | Space | Method | Accuracy | Distortion | Cost (operations) | Demo | Note |
| ---- | ----- | ------ | -------- | ---------- | ----------------- | ---- | ---- |
| flat, plane (seed 1) | flat | k-NN, straight line | 0.900 | | 5400 | baseline-knn | the baseline row |
| flat, five dimensions (seed 2) | flat | k-NN, straight line | 0.967 | | 13500 | baseline-knn | |
| directions (seed 5) | flat | k-NN, straight line | 0.750 | | 14400 | baseline-knn | lengths kept, which carry nothing |
| directions (seed 5) | flat, lengths dropped | k-NN, straight line between unit points | 0.775 | | 14400 | baseline-knn | 2.5 points from dropping the lengths; not yet the sphere's own distance |
| tree, depth 4 branching 2 | flat (a plain layout) | k-NN on the layout | 1.000 | 0.179 mean, 0.805 worst | 336 | baseline-knn | the layout's halves give the branch away; its distances are the point |
| flat, plane (seed 1) | flat | k-NN, straight line | 0.900 | 0, 0 (its own distances) | 5400 | representation-explorer | the baseline row again |
| flat, plane (seed 1) | sphere (directions from the origin) | k-NN, the angle | 0.767 | 0.486 mean, 7.491 worst (scale 3.884 fitted) | 5400 | representation-explorer | the sphere costs 0.133: two clusters lie the same way from the origin; counts for H1 |
| flat, plane (seed 1) | flat | network 2-8-3, Adam 0.02, 300 steps | 0.867 | | 2754000 | representation-explorer | loss 1.558 to 0.160; under 0.5 at step 13 |
| flat, plane (seed 1) | sphere | network 2-8-3, the same | 0.767 | | 2754000 | representation-explorer | loss 1.319 to 0.234; under 0.5 at step 28; the sphere costs 0.100; counts for H1 |
| directions (seed 5) | flat | k-NN, straight line | 0.750 | 0.370 mean, 77.975 worst (scale 0.946 fitted) | 14400 | representation-explorer | the baseline row again; the worst pair (94, 152) is two points in nearly the same direction at far-apart lengths: scaled straight-line distance 79 times the angle |
| directions (seed 5) | sphere | k-NN, the angle | 0.775 | 0, 0 (its own distances) | 14400 | representation-explorer | one held-out point of 40 gained; the baseline's lengths-dropped row, since the chord orders unit points as the angle does |
| directions (seed 5) | flat | network 3-8-4, Adam 0.02, 300 steps | 0.750 | | 4896000 | representation-explorer | loss 1.461 to 0.407; under 0.5 at step 203 |
| directions (seed 5) | sphere | network 3-8-4, the same | 0.750 | | 4896000 | representation-explorer | loss 1.452 to 0.507, not under 0.5 in 300 steps; **null result**: a tie, which the rule written before the run counts against H1; why the network tied (a hidden layer can represent a rule that ignores the lengths) and why the flat run's loss fell faster are not measured |

| tree, depth 4 branching 2 | the unit sphere (curvature +1), fitted | k-NN under the angle, k = 3; logistic on the coordinates | 0.429; 0.429 | 0.591 mean, 0.952 worst (fixed scale; 0.602 with the scale forgiven) | 3459600 | representation-explorer, part 2 | 200 steps at rate 50 from seed 31; not under 0.2; the sphere is too small (diameter pi for a tree of diameter 8) and its loss is not fixed by forgiving the scale; three starts 0.591, 0.591, 0.591 |
| tree, depth 4 branching 2 | the plane (0), fitted | the same | 1.000; 0.857 | 0.151 mean, 0.620 worst | 2306400 | representation-explorer, part 2 | under 0.2 at step 24; three starts 0.151, 0.296, 0.151 (spread 0.145: a local minimum) |
| tree, depth 4 branching 2 | the Poincare disk (-1), fitted | the same | 1.000; 0.857 | 0.102 mean, 0.287 worst | 2306400 | representation-explorer, part 2 | under 0.2 at step 29; no step past the rim; three starts 0.102, 0.237, 0.142 (spread 0.135); under the plane on every start, by 0.049, 0.059, 0.009 |
| tree, depth 4 branching 2 | curvature +1/4, +1/16, -1/16, -1/4, -4, fitted | the control | | at step 100: 0.310, 0.175, 0.132, 0.097, 0.337 (the plane's 0.151, the sphere's 0.591, the disk's 0.122 at the same step) | | representation-explorer, part 2 | the lowest row at -1/4 on one start, the rows from -1/16 to -1 within 0.035 (less than the spread over starts); both sides meet at the plane; -4 too much |

What the table says (three geometries by three data sets; part 1 one
run per cell, part 2 three starts): on the clusters the matching space
scores higher and the crossed one costs, under both methods, which
counts for H1; on the directions the sphere gains one point of 40
under nearest neighbors (for H1, within one point) and ties under the
network (against H1, by the rule written before the run); on the tree
the disk keeps the distances with the least distortion and the classifiers tie between the
plane and the disk, which on accuracy is what the rule asks, but on
distortion the plane is within the spread over three starts of the
disk (0.049 against 0.135), which the rule counts against H1: a
**null result** by the letter, with the direction H1 says on every
start. The spread is the fit's local minima (one start of each stopped
far above the others), not the geometry's. The disk on the clusters
and the directions is not run (two cells of nine): a fit of 120 or
160 points would not run in a page.

## Optimization geometry (no hypothesis: the area's question)

Does the metric of a step change the path and the cost? Measured on a
loss whose answer can be seen whole (optimization-explorer, part 1,
step metric-descent; `demos/optimization-explorer/reg/cli-optimization-explorer.out`):
L = (x^2 + 3 y^2) / 2 from (2.3, 1) (loss 4.145), 30 steps, the exact
line search as the step size policy for every ruler.

| ruler G | condition number left | steps to loss under 1e-6 | loss after 1, 5 steps | numbers in G; multiplies per step on P g | note |
| ------- | --------------------- | ------------------------ | --------------------- | ---------------------------------------- | ---- |
| the identity (plain) | 3 | 11 | 0.983; 0.0031 | 0; 0 | first step to (1.282, -0.328), eta 0.4426; every step at right angles to the last |
| diag(a, 1) at a = 1/9, 1/6, 1/3, 1/2, 1, 2, 3, 9 | 3, 2, 1, 1.5, 3, 6, 9, 27 | 9, 7, 1, 5, 11, 18, 22, 29 | after 5: 3.3e-4, 1.3e-5, 0, 4.1e-7, 0.0031, 0.056, 0.121, 0.297 | 1; 2 | the counts fall toward a = 1/3 from both sides; the same condition number (1/9 and 1) does not give the same count (9 and 11) |
| H = diag(1, 3), the bowl's curvature (Newton's direction) | 1 | 1 | 0; 0 | 3; 4 | eta = 1, the floor exactly |

And on the sphere, the leading eigenvector of A = (4 1 0 / 1 3 1 /
0 1 2) (eigenvalue 4.732) climbed to from (1, -1, 1) / sqrt 3 at rate
0.1, 60 steps:

| step | steps to a distance under 1e-4 | distance at the end | off the sphere before repair, farthest; mean | note |
| ---- | ------------------------------ | ------------------- | -------------------------------------------- | ---- |
| the gradient's tangent part, the exponential map (`s_phereDescend`) | 24 | 2.1e-11 | 1.1e-16; 1.9e-18 | lands on the sphere |
| the tangent part, the cheap retraction (`s_phereRetract`) | 24 | 2.2e-11 | 0.045; 0.0027 | the same count and end distance as the exact step: by the rule written before the run, the exact step bought nothing over the cheap one here (a result) |
| a flat step p + 2 eta A p, scaled back | 49 | 1.1e-5 | 0.946; 0.906 | the power iteration on I + 2 eta A; twice the steps. Analytical contraction per step near the eigenvector: 0.822 (flat) against 0.654 (along the sphere), which predict 48 and 22 steps; one rate measured only |

What the tables say: the ruler changed the path and the count and not
the loss; nothing here counts against "downhill depends on the
ruler" (the rule written before the run: the paths coinciding or the
counts within one). The cost is in the ruler's numbers and multiplies,
which on a model with many weights is what decides; part 2 measures
that on a model.

On a model (optimization-explorer, part 2, step metric-training;
`demos/optimization-explorer/reg/cli-optimization-training.out`, 17
checks): a softmax classifier of six weights on the kit's flat
clusters (seed 1, three clusters of 40, spread 1.6; split seed 11, 90
trained on, 30 held out), from all weights 0, 60 steps, each method at
its own rate set before the run (plain 0.5, diagonal 0.5, natural 1,
damping 0.001), target a training loss of 0.35; once with x1 times 20,
once as it is. The cost is counted for what each step forms
(exponentials, divisions and additions not counted, the same per row
for every method).

| data | method | stays under 0.35 from | first under | loss at the end | held-out accuracy | multiplies per step | multiplies to stay under | note |
| ---- | ------ | --------------------- | ----------- | --------------- | ----------------- | ------------------- | ------------------------ | ---- |
| x1 times 20 | plain | not at the end | 54 | 59.6 | 0.533 | 1,080 | | thrashes: updates up to 21.8; at rates 0.05, 0.005, 0.0005 the end losses are 0.731, 0.587, 0.760, none under the target |
| x1 times 20 | diagonal (the Fisher's diagonal) | 4 | 4 | 0.164 | 0.867 | 2,160 | 8,640 | the same as on the data as they are |
| x1 times 20 | natural (the Fisher, solved) | 2 | 2 | 0.163 | 0.900 | 4,428 | 8,856 | the same as on the data as they are |
| as they are | plain | 5 | 5 | 0.211 | 0.767 | 1,080 | 5,400 | |
| as they are | diagonal | 4 | 4 | 0.164 | 0.867 | 2,160 | 8,640 | |
| as they are | natural | 2 | 2 | 0.163 | 0.900 | 4,428 | 8,856 | **null result for the metric**: fewer steps, more multiplies in all |

The Fisher's condition number at the start: 13,906 with x1 times 20,
124 as it is (112 times). Undamped, ten steps from all weights 0, the
largest gap in a predicted probability against the data as they are:
plain 0.96 with x1 rescaled and 0.61 with x1 replaced by x1 + x2;
diagonal 2e-16 and 0.29; natural 2e-16 and 5e-16 (the natural step
ignores any invertible linear rewriting of the inputs, the diagonal
only a rescaling, as the theory says).

What these say, against the rule written before the run (the metric
pays where it stays under the target in fewer multiplies in all than
the plain step): with one input badly scaled the metric paid, since the
plain step did not settle at any of four rates in 60 steps (a rate
between those tried, or more steps, was not run); with the inputs as
they are it did not (8,856 against 5,400 multiplies, under a factor of
two), a null result for it, recorded as one. The accuracies differ by
one or two of 30 held-out points. For softmax regression the Fisher
equals the loss's curvature, so the natural step here is Newton's; in
a network the two differ. A model this small hides the real cost: the
Fisher grows with the square of the weights and its solve with the
cube.

For a metric learned with the model (H3, Saga 2; the research's
G_phi, not built here): the metric that paid here paid by undoing how
the inputs were written, and the cheap fix of writing them to one
scale is the "as they are" rows, where plain gradient descent is the
cheapest in multiplies. A learned metric would have to beat that fix
on a problem where no rescaling of the inputs suffices (a network's
hidden layers, a curvature that changes from point to point), and pay
for its own numbers; that it would start behind is a conjecture, not a
measurement.

## H2: a constraint on a transformation keeps a deep network stable

No rows yet (steps mhc-matrices and mhc-depth).

## H3: geometry learned with the model

No rows yet (Saga 2).
