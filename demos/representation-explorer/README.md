# Representation explorer: flat against the sphere

The same points kept two ways, as they are in a flat space (measured
by the straight line) and as directions on the unit sphere (measured
by the angle), on two data sets of the experiment kit built with a
known structure: flat clusters, whose structure is the straight
line's, and directions, whose structure is the angle's. For each:
three distances on the same pairs with one worked in full, a shortest
path and a mean in each space, nearest neighbors and the decision
boundary under each distance, a small classifier trained on each
representation with everything else held equal, and how badly each
representation keeps the data's own distances. The first test of H1.
Live at
<https://softwarewrighter.github.io/X_eTaL-manifolds/representation-explorer/>.

## The question

Does it matter what shape of space a model keeps its points in? A
model keeps each thing it sees as a list of numbers; those numbers
can be read as a position in a flat room (the distance between two is
the straight line) or as a direction (a point on the skin of a ball,
the distance the angle). The two readings put the same numbers in
different spaces, and a method that works from distances gives
different answers under each. The worry: the choice is usually made
by habit. This page makes it on purpose, on data whose structure is
known, and measures what each choice costs and gains.

## Background

A representation is the way a point is kept: the numbers, and the
space they are read in. Flat: the point as it is, the straight line
as its distance. Spherical: its direction, the point scaled to length
1 (Manifold's `s_phereProject`), the angle between two (the
great-circle distance, `s_phereDistance`) as its distance. Same
ambient dimension; the sphere has one less intrinsic dimension. The
two sets are the baseline's: flat clusters (seed 1, 3 clusters of 40
in the plane, spread 1.6) and directions (seed 5, 4 classes of 40 in
three dimensions, cone 0.6, lengths 0.5 to 2 that carry nothing).
Nearest neighbors (Learn's `k_nn`, k = 5) and a held-out quarter by
seed 11, as the baseline. The classifier: one hidden layer of 8 tanh
units then softmax, trained by Adam on the cross-entropy; Net writes
its training step from the spec. Distortion (Manifold's
`d_istortion`): over every pair, the fraction by which a
representation's distance is too long or too short, averaged and at
worst, against each set's own distances (the straight line for the
clusters, the angle for the directions), after one scale is fitted
(`f_itScale`).

## The idea

Hold everything equal but the space: the points, the split, the
seeds, k, the network, its starting weights, step count and learning
rate; read each point flat and on the sphere; measure. Whatever moves
between the two columns is the geometry's doing. The two sets were
built so that each space has a set that matches it and one that does
not. H1 says the matching space does at least as well and the crossed
space costs; what would count against it, written before the run: the
sphere scoring as high as the plane on the clusters, or the plane as
high as the sphere on the directions. A null result on either set is
reported as one.

## What this is, and is not

The first comparison of H1: two geometries by two data sets, with the
baseline's nearest-neighbor rows as the row against, and an actual
training run on each representation. Not a proof that one space is
right (two small sets, one run per cell: one split, one set of
starting weights, no spread over seeds), not the hyperbolic half (the
next page), and the classifier is not a trained model of
anything outside this page. Live at the address above; every number
on the page is the program's, run in the browser.

## The intuition

A sheet of paper and a globe. On the sheet the distance between two
cities is the ruler's; on the globe every city is a direction from
the center and the distance is how far round you turn to look from
one to the other; a city twice as far from the center in the same
direction is the same city. Keep positions on the globe and it throws
away what tells them apart; keep directions on the sheet and the
ruler adds in the lengths, which say nothing. Putting a point on the
sphere spends one dimension, the length, on purpose: noise on one set,
signal on the other.

## The mathematics

Three distances for p and q: the straight line sqrt(sum (p_i -
q_i)^2); the cosine distance 1 - p . q / (|p| |q|); the angle acos of
that cosine (the program takes it as 2 atan2(|u - v|, |u + v|) on the
unit points, which loses no digits near 0 or pi, and checks the two
agree). Shortest paths: the segment p + t (q - p); the arc of the
great circle (`s_phereArc`), with the chord 2 sin(angle / 2) shorter
than the arc. Means: the plain mean, which for unit points falls
inside the sphere; the mean on the sphere (`s_phereMean`), the point
whose summed squared distances along the sphere is least. Nearest
neighbors under the angle: for unit points the chord grows with the
angle, so the straight line between unit points orders neighbors as
the angle does (checked on all 160), and that is how it is run. The
classifier: h = tanh(x W1), z = h W2, softmax(z); the loss is the
mean of minus the log of the true class's probability; the gradient
is Net's backpropagation, checked against central differences
((loss(w + h) - loss(w - h)) / 2h, h = 0.00001) at five weights;
Adam steps each weight by the rate times a running mean of its
gradient over the root of a running mean of its square.

## What is compared

Two sets by two representations, four cells; on each, nearest
neighbors and the network. The same split (seed 11), k = 5, the same
network (51 numbers on the clusters, 68 on the directions), the same
seeded starting weights (0.3 times normal numbers, seeds 21 and 22),
Adam at rate 0.02 for 300 steps. The measures: held-out accuracy for
both methods; the loss by step, the loss at the end and the steps to a
loss under 0.5 for the network; the distortion of each representation
against its set's own distances. Written before the run: the sphere
should cost accuracy on the clusters (it keeps only which way from
the origin each point lies); on the directions it should gain a
little under nearest neighbors, as the baseline's lengths-dropped row
did, and less under the network, which can learn to ignore the
lengths; the crossed cells' worst pairs should be far off (two points
in nearly the same direction at different lengths).

## The run

Nearest neighbors on each held-out quarter under each distance, and
four trainings of the network, one per cell, 300 steps each from the
same weights, the training loss kept at every step. Nothing is tuned
to the result.

## The program

`representation-explorer.xtl`: the kit written out (`u:s_plit`,
`u:r_un`, `u:c_hords`, `u:o_rders`); the two sets two ways; the three
distances on three pairs and the worked pair; the segment, the arc
and the chord; the means per class; nearest neighbors on the four
cells and the boundary grids (the plane, 21 by 21; the sphere's map,
24 by 12, under the angle and under the straight line at three
lengths); the four training runs (`net:t_rain<` writes each step),
their losses, the network's guesses and record, the convergence rows;
the distortions and the worst pair; the record of every run; the wire
sphere and the map for the page. After the core: the printed sections
and the checks, the gradient check among them.

## How it works

The split and run as the baseline. The decision boundary: Learn's
`k_nn` from the training rows alone, applied to a grid of points (the
grid's directions for the angle; the grid's unit points scaled to a
length for the straight line on the map). The training loop: Net's
step applied `steps` times by `p_ower` to a state tuple, with a
lambda that adds each step's training loss to a vector. The
distortion: each set's own distances (the chords of the clusters, the
angles of the directions) against each representation's, scaled by
`f_itScale`, through `d_istortion`, `f_latDistortion` and
`s_phereDistortion`; `d_istortionPairs` finds the worst pair.

## Results

From `reg/cli-representation-explorer.out`, the record (accuracy,
held out, numbers trained, operations), all measured:

| Method | Set | Representation | Accuracy | Held out | Trained | Operations |
| ------ | --- | -------------- | -------- | -------- | ------- | ---------- |
| nearest neighbors | flat clusters | flat | 0.900 | 30 | 0 | 5400 |
| nearest neighbors | flat clusters | sphere | 0.767 | 30 | 0 | 5400 |
| nearest neighbors | directions | flat | 0.750 | 40 | 0 | 14400 |
| nearest neighbors | directions | sphere | 0.775 | 40 | 0 | 14400 |
| the network | flat clusters | flat | 0.867 | 30 | 51 | 2754000 |
| the network | flat clusters | sphere | 0.767 | 30 | 51 | 2754000 |
| the network | directions | flat | 0.750 | 40 | 68 | 4896000 |
| the network | directions | sphere | 0.750 | 40 | 68 | 4896000 |

Convergence (loss at step 0, at step 300, steps to a loss under 0.5):
clusters flat 1.558, 0.160, 13; clusters sphere 1.319, 0.234, 28;
directions flat 1.461, 0.407, 203; directions sphere 1.452, 0.507,
not reached. Distortion (scale fitted, mean, worst): clusters on the
sphere 3.884, 0.486, 7.491; directions kept flat 0.946, 0.370,
77.975 (the worst pair: points 94 and 152); the two matching rows are
0 by construction.

Against H1, by the rule written before the run: the clusters count
for it (the sphere costs 0.13 under nearest neighbors and 0.10 under
the network: the lost dimension was the signal). The directions count
for it under nearest neighbors by one held-out point of 40 (0.75 to
0.775, the baseline's lengths-dropped row again, within what one
point can move), and against it under the network, where the two
spaces tied at 0.750 and the rule said a tie counts against: a null
result, reported as one. Why they tied is not measured: a hidden
layer can represent a rule that ignores the lengths, which would be
consistent with the tie, but whether this network did, and why the
flat run's training loss fell faster (0.407 against 0.507 at step
300), are open. The worst pair's 77.975 is the factor less 1: its
scaled straight-line distance is 79 times its angle. The three
distances order the three pairs differently under the straight line
and under the angle (the straight line calls the cross-class pair
1 and 121 nearer than 1 and 41; the angle does not), the cosine
distance and the angle alike.

## Beyond this page

The cosine comparison of word vectors and the unit-sphere embeddings
of face and sentence models drop the length on purpose; the measure
of distortion is Nickel and Kiela's (2017) and Gu, Sala, Gunel and
Re's (2019), read at source (`docs/papers.md`). Open: the spread over
seeds (one split, one starting weight set here); a network that is
told the length and one that is not; directions whose lengths are
made to carry a class, where the sphere would lose. Not established:
that any real data set is directional or flat; a real one's structure
is measured, by this page's distortion against a candidate space.

## Run it

```bash
just run representation-explorer       # the runs, with their checks
just test-demo representation-explorer # the baseline
```

## Summary

On the clusters the shape of the space matters, in the direction the
data's structure says: kept on the sphere they lose 0.13 of held-out
accuracy under nearest neighbors and 0.10 under the network. On the
directions it matters by at most one held-out point of 40: kept flat
they lose 0.025 under nearest neighbors and nothing under the
network. The crossed representations keep their set's distances to
0.49 and 0.37 on average. H1 so far, by the rule written before the
run: the clusters count for it; the directions count for it under
nearest neighbors by one point and against it under the network,
where the two spaces tied. Not shown: the spread over seeds (one run
per cell), why the network tied, or anything about real data.

## Next

representation-explorer, part 2 (step representation-hyperbolic):
the plane, the sphere and the Poincare disk fitted to the tree at the
same dimension; the H1 table complete. Then metric-descent: the same
classifier trained on the sphere by steps that stay on it.

## Workarounds

- The split and the run's record are X_eTaL-ML's by the plan's G3
  (ask GM5, `docs/xetal-ML-asks.md`) and are written out in the
  program, as in baseline-knn, until it lands.
- Pairwise straight-line distances (`u:c_hords`) and each point's
  neighbor order (`u:o_rders`) are X_eTaL-libraries' (GL1), written
  out as in sphere-hello.
