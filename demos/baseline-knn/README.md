# Baseline: nearest neighbors

The row every comparison of the first hypothesis is set against: k
nearest neighbors under the straight-line distance, the distance a
model uses when no geometry has been chosen, on the three data sets of
the experiment kit, each split the same seeded way, each run's
accuracy and cost recorded as the experiment schema says. Command line
only; its page comes with step shell-adoption.

## The question

Before any geometry is chosen: how well does the plainest method, the
nearest neighbors by the straight line through the room, classify
three data sets built with a known structure (flat clusters,
directions, a tree)? And for the tree, how badly does a plain picture
of it keep its distances? Every later comparison must beat these
numbers, on the same data, split and seeds, or say it did not.

## Background

A data set here is a matrix of points, one per row, and a label per
row. The experiment kit (the Kit library) makes three from explicit
seeds: flat Gaussian clusters, in the plane and in five dimensions;
classes that differ by direction and not by length; and a tree of 31
nodes, with the distance between any two nodes along the tree and a
plain layout of it in the plane. k nearest neighbors (Learn's) guesses
a held-out point's label from the labels of its k nearest training
points. A run's record is the held-out accuracy, how many points were
held out, how many numbers were trained (none, here) and the
operations counted (one distance per held-out point per training
point, each of d coordinates); runtime is never recorded.

## What is compared

An introduction (`intro = true`): one method, the straight line, on
three sets. The only comparison made here is on the directional set:
the same points with their lengths dropped (put onto the unit sphere),
still under the straight line, against the points as they are. The
record rows (accuracy, held out, numbers trained, operations), all
measured:

| Set | Points | Accuracy | Held out | Trained | Operations |
| --- | ------ | -------- | -------- | ------- | ---------- |
| flat, plane (3 clusters of 40, spread 1.6, seed 1) | 120 | 0.900 | 30 | 0 | 5400 |
| flat, five dimensions (4 of 30, spread 1.6, seed 2) | 120 | 0.967 | 30 | 0 | 13500 |
| directions (4 of 40, cone 0.6, lengths 0.5 to 2, seed 5) | 160 | 0.750 | 40 | 0 | 14400 |
| the same, lengths dropped | 160 | 0.775 | 40 | 0 | 14400 |
| the tree's layout (depth 4, branching 2; k = 3) | 31 | 1.000 | 7 | 0 | 336 |

The tree's layout keeps the tree's distances badly: distortion 0.179
on average over the pairs and 0.805 for the worst pair (Manifold's
`d_istortion`): the number the hyperbolic disk is set against. Its
nearest-neighbor accuracy is 1.0 because the layout puts each branch
in its own half of the plane, which says nothing about distances.

## The program

`baseline-knn.xtl`: the kit written out (`u:s_plit`, a seeded
held-out quarter; `u:r_un`, one run and its record; `u:c_ompare`, two
records and their difference; `u:c_hords`, pairwise distances), then
the runs, each checked with Check against what the data's structure
promises, and the record of every run as one table at the end.

## How it works

The split: the grade of n seeded uniform numbers is a seeded order of
the rows; the first quarter is held out. The run: Learn's `k_nn` on
the training rows, applied to the held-out rows; accuracy is the
share guessed right. The record is a vector of four Floats; the
comparison is three rows (a, b, b less a). The distortion: the
straight-line distances between the layout's points, against the
tree's distances, by Manifold.

## Run it

```bash
just run baseline-knn       # the runs, with their checks
just test-demo baseline-knn # the baseline
```

## Summary

Nearest neighbors by the straight line gets 90 and 97 percent of the
flat clusters, 75 percent of the directions (77.5 with the lengths
dropped: the lengths, which carry nothing, cost 2.5 points), and every
node of the tree's layout; the layout keeps the tree's distances to
within 18 percent on average and 80 percent at worst. These are the
H1 table's first row (`docs/experiments.md`). What it does not show:
anything about a chosen geometry; representation-explorer (steps
representation-sphere and representation-hyperbolic) runs the same
sets, split and seeds on the sphere and in the Poincare disk.

## Workarounds

- The split, the run's record and the comparison are X_eTaL-ML's by
  the plan's G3 (ask GM5, `docs/xetal-ML-asks.md`, beside
  X_eTaL-fine-tuning's FM7) and are written out in the program, three
  short functions, until it lands.
- Pairwise straight-line distances (`u:c_hords`) are X_eTaL-libraries'
  (GL1), written out as in sphere-hello.
