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

What the table says so far (two geometries by two data sets, one run
per cell: one split, one starting weight set): on the clusters the
matching space scores higher and the crossed one costs, under both
methods, which counts for H1; on the directions the sphere gains one
point of 40 under nearest neighbors (for H1, within one point) and
ties under the network (against H1, by the rule written before the
run). The spread over seeds is not yet measured.

Rows to come: the Poincare disk, the sphere and the plane fitted to
the tree at the same dimension, with their distortion
(representation-hyperbolic); what counts for and against H1 is in
`docs/research-questions.md`.

## H2: a constraint on a transformation keeps a deep network stable

No rows yet (steps mhc-matrices and mhc-depth).

## H3: geometry learned with the model

No rows yet (Saga 2).
