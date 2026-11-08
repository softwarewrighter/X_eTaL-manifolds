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

Rows to come: the sphere's distance on the directions and the flat
sets (representation-sphere); the Poincare disk, the sphere and the
plane fitted to the tree at the same dimension, with their distortion
(representation-hyperbolic); what counts for and against H1 is in
`docs/research-questions.md`.

## H2: a constraint on a transformation keeps a deep network stable

No rows yet (steps mhc-matrices and mhc-depth).

## H3: geometry learned with the model

No rows yet (Saga 2).
