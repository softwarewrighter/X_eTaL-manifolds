# Representation explorer: flat, the sphere and the disk

Three data sets of the experiment kit, each built with a known
structure, and three spaces to keep them in. Part 1: the same points
kept as they are in a flat space (measured by the straight line) and
as directions on the unit sphere (measured by the angle), on the flat
clusters and the directions: three distances on the same pairs with
one worked in full, a shortest path and a mean in each space, nearest
neighbors and the decision boundary under each distance, a small
classifier trained on each representation with everything else held
equal, and how badly each representation keeps the data's own
distances. Part 2: the Poincare disk, its distance worked in full and
its shortest path drawn, the room a tree needs counted; the tree
fitted in the plane, on the unit sphere and in the unit disk from the
same start by the same steps, three starts for the spread, a
classifier from each embedding, and the same fit at eight curvatures
from +1 to -4. The test of H1, with its null results. Live at
<https://softwarewrighter.github.io/X_eTaL-manifolds/representation-explorer/>.

## The question

Does it matter what shape of space a model keeps its points in? A
model keeps each thing it sees as a list of numbers; those numbers
can be read as a position in a flat room (the distance between two is
the straight line), as a direction (a point on the skin of a ball,
the distance the angle), or as a point inside a disk whose edge is
infinitely far away, with more room the farther out you go. The three
readings put the same numbers in different spaces, and a method that
works from distances gives different answers under each. The worry:
the choice is usually made by habit. This page makes it on purpose,
on data whose structure is known, and measures what each choice costs
and gains.

## Background

A representation is the way a point is kept: the numbers, and the
space they are read in. Flat: the point as it is, the straight line
as its distance. Spherical: its direction, the point scaled to length
1 (Manifold's `s_phereProject`), the angle between two (the
great-circle distance, `s_phereDistance`) as its distance. Hyperbolic:
a point inside the Poincare disk, the inside of the unit circle
measured so that a step near the rim counts for far more than the
same step near the middle, by 2 / (1 - r^2) at plain radius r; the
rim is infinitely far from every point, and the program keeps every
point under the radius 1 - 1e-5 as the papers it follows do. The
disk's curvature is -1, the sphere's +1, the plane's 0: on the sphere
circles are shorter than 2 pi times their radius, in the disk longer
(2 pi sinh r, doubling about every 0.7 of radius). The three sets are
the baseline's: flat clusters (seed 1, 3 clusters of 40 in the plane,
spread 1.6), directions (seed 5, 4 classes of 40 in three dimensions,
cone 0.6, lengths 0.5 to 2 that carry nothing), and the tree (depth 4,
branching 2: 31 nodes, 30 edges, distances along it 1 to 8, each
node's class the branch of the root it hangs from). Nearest neighbors
(Learn's `k_nn`, k = 5; k = 3 on the tree) and a held-out quarter by
seed 11, as the baseline. The classifier of part 1: one hidden layer
of 8 tanh units then softmax, trained by Adam on the cross-entropy;
Net writes its training step from the spec. The classifiers of part
2: nearest neighbors under the space's own distance from its distance
matrix, and Learn's logistic classifier on the embedded coordinates.
Distortion (Manifold's `d_istortion`): over every pair, the fraction
by which a representation's distance is too long or too short,
averaged and at worst, against each set's own distances; in part 1
after one scale is fitted (`f_itScale`), in part 2 at a fixed scale.
An embedding, in part 2, is a fit: the tree's nodes put into a space
and moved by gradient descent on the distortion, each node stepped
along the space (the plane's plain step; Manifold's `s_phereStep` and
`b_allStep`, the exponential maps, against the gradient along the
space, `s_phereDistancesGradient` and `b_allDistancesGradient`).

## The idea

Hold everything equal but the space: the points, the split, the
seeds, k, the network, its starting weights, step count and learning
rate; read each point flat and on the sphere; measure. For the tree,
the same start in every space (seeded numbers as a tangent vector at
the origin of each, carried in by its own step), the same steps and
rate, the scale fixed (an edge should come out 1 in every space): at
a fixed scale the curvature means something, the same tree at the same
size in a smaller or larger space, where with the scale free every
curvature of one sign is the same space. Whatever moves between the
columns is the geometry's doing. The three sets were built so that
each space has a set that matches it. H1 says the matching space does
at least as well and the crossed spaces cost; what would count against
it, written before the run: the sphere scoring as high as the plane on
the clusters, the plane as high as the sphere on the directions; for
the tree, by the rule of `docs/research-questions.md` written before
the saga, the plane's distortion within the spread of the disk's over
three starts (the difference between the two spaces on the first
start against the largest less the smallest of the disk's three
ends), or the plane above the disk on accuracy. That rule reads a tie
on accuracy as for H1 where part 1's rule read a tie against, and was
written for starts compared unpaired where the design pairs them (the
same seed in every space); both readings are given. A null result on
any set is reported as one.

## What this is, and is not

The comparison of H1: three geometries by three data sets, with the
baseline's rows as the row against (its nearest-neighbor rows on the
clusters and the directions; its plain layout of the tree, distortion
0.18 on average and 0.80 at worst), a training run on each
representation of the first two sets, and a fit of the tree in each
space with three starts and a curvature sweep. Not a proof that one
space is right: three small sets; in part 1 one run per cell, in part
2 three starts of a fit of 31 nodes in dimension 2 at 200 steps; and
the H1 table has seven cells of nine, the clusters and the directions
not fitted in the disk (120 or 160 points make 15 or 27 times the
tree's pairs, so the nine fits would take minutes in a page, not
seconds). The classifiers are
not trained models of anything outside this page. Live at the address
above; every number on the page is the programs', run in the browser.

## The intuition

A sheet of paper and a globe. On the sheet the distance between two
cities is the ruler's; on the globe every city is a direction from
the center and the distance is how far round you turn to look from
one to the other; a city twice as far from the center in the same
direction is the same city. Keep positions on the globe and it throws
away what tells them apart; keep directions on the sheet and the
ruler adds in the lengths, which say nothing. Then a family tree on a
sheet: each generation doubles, and by the fifth the sheet has run out
of room at the edge, so cousins eight steps apart along the tree sit
side by side. The disk is a sheet whose edge is infinitely far away
and whose room doubles with every step outward, as the generations
do; the sphere is a sheet that closes up on itself, with no two points
more than a half turn apart.

## The mathematics

Three distances for p and q: the straight line sqrt(sum (p_i -
q_i)^2); the cosine distance 1 - p . q / (|p| |q|); the angle acos of
that cosine (the program takes it as 2 atan2(|u - v|, |u + v|) on the
unit points, which loses no digits near 0 or pi, and checks the two
agree). The disk's distance for u and v inside the unit disk: 2
asinh(|u - v| / sqrt((1 - |u|^2) (1 - |v|^2))), the same as Nickel
and Kiela's acosh(1 + 2 |u - v|^2 / ((1 - |u|^2) (1 - |v|^2))), and 2
atanh r from the middle to a point at plain radius r; the program
takes the asinh form and checks the two agree. Shortest paths: the
segment p + t (q - p); the arc of the great circle (`s_phereArc`),
with the chord 2 sin(angle / 2) shorter than the arc; in the disk an
arc of a circle meeting the rim at right angles (`b_allGeodesic`, by
Mobius addition), bowed toward the middle. Means: the plain mean,
which for unit points falls inside the sphere; the mean on the sphere
(`s_phereMean`). Nearest neighbors under the angle: for unit points
the chord grows with the angle, so the straight line between unit
points orders neighbors as the angle does (checked on all 160). The
network: h = tanh(x W1), z = h W2, softmax(z); the loss the mean of
minus the log of the true class's probability; the gradient Net's
backpropagation, checked against central differences at five weights.
The fit: the loss is the mean over pairs of (E / D - 1)^2, E the
distance along the space and D the tree's; its gradient by the
distances is a matrix W, 2 (E / D - 1) / D over the pairs, and by the
points, for node i, the sum over j of W_ij times the gradient of
d(P_i, P_j) by P_i: in the plane (P_i - P_j) / |P_i - P_j|; on the
sphere -P_j / sin(angle), then the tangent part; in the disk the
gradient of 2 asinh z scaled by (1 - |P_i|^2)^2 / 4, the metric's
correction. The step: P - lr G in the plane; cos(t) P + sin(t) G / t
on the sphere; Mobius addition of P and tanh(lambda |v| / 2) v / |v|
in the disk, lambda = 2 / (1 - |P|^2), then the clamp to radius 1 -
1e-5 with how far any point went past it kept as the constraint's
violation. Curvature as a scale: a sphere of curvature K > 0 is the
unit sphere with every length times 1 / sqrt K, a disk of curvature K
< 0 the unit disk with every length divided by sqrt -K, and a step at
rate lr along the space of curvature K is the unit space's step at
rate lr sqrt |K| with the same W. Every gradient is checked against
central differences: Manifold's three against its distances
(`libs/Manifold/tests/fit.xtl`, the disk's at the middle and near the
rim), and the program's whole loss in the plane and in the disk at
h = 0.00001.

## What is compared

Part 1: two sets by two representations, four cells; on each, nearest
neighbors and the network, the same split (seed 11), k = 5, the same
network (51 numbers on the clusters, 68 on the directions), the same
seeded starting weights, Adam at rate 0.02 for 300 steps. Part 2: the
tree by three spaces at dimension 2, the unit sphere (+1), the plane
(0) and the unit disk (-1), from the same start (seeds 31, 32, 33;
normal numbers times 0.5), 200 steps at rate 50, the scale fixed; and
the control, the same fit from the first start at curvatures +1, +1/4,
+1/16, 0, -1/16, -1/4, -1, -4 for 100 steps each (the three main fits
read at their step 100 for +1, 0 and -1). The measures: held-out
accuracy; for the network the loss by step and the steps to a loss
under 0.5; for the fit the mean distortion and the worst pair by step
and at the end, the steps to a mean distortion under 0.2, the
distortion with the scale forgiven (part 1's measure: after one
scale is fitted so that the mean distance matches, not so that the
distortion is least, so it can come out above the fixed-scale
figure), how far past the rim any step went, the accuracy of nearest
neighbors (k = 3, a node's most neighbors along the tree) under the
space's own distance and of the logistic classifier (200 steps at rate
0.5) on the held-out 7 of 31 nodes; the numbers fitted (2 coordinates
per node, 3 on the sphere) and the operations (6 per entry of the 31
by 31 distance matrix per coordinate per step). The rate 50 and the target 0.2 were set by a first run
(rates 1 to 200 tried, 50 the largest the plane's fit stayed stable
at; the target chosen so that the plane's speed could be read, its
fits settling near 0.15), and the step count raised from 150 to 200
after that run showed the disk's fits still falling while the plane's
had stopped (of these three choices the rate favors the plane's
stability and the step count the disk); all of it is in
`presets/tree.txt`, data the program reads. Written before the run: the sphere should cost on the
clusters; the directions should gain a little on the sphere under
nearest neighbors and less under the network; the disk should keep the
tree's distances with the least distortion and the unit sphere with the most (its farthest points
are pi apart and the tree needs 8), the plane between; the classifiers
should tell the two branches apart in any space that keeps the tree
at all; the control should fall from +1 through 0 to some negative
curvature and rise again at -4.

## The run

Part 1: nearest neighbors on each held-out quarter under each
distance, and four trainings of the network, 300 steps each from the
same weights, the training loss kept at every step. Part 2: fourteen
fits of the tree (three spaces by three starts, and the control's
five other curvatures), each a loop of the same step with the mean
distortion kept at every step; then the two classifiers on each of the
first start's three embeddings. Nothing is tuned to the result beyond
what the paragraph above says.

## The program

`representation-explorer.xtl` (part 1): the kit written out
(`u:s_plit`, `u:r_un`, `u:c_hords`, `u:o_rders`); the two sets two
ways; the three distances on three pairs and the worked pair; the
segment, the arc and the chord; the means per class; nearest neighbors
on the four cells and the boundary grids; the four training runs
(`net:t_rain<` writes each step), their losses, the network's guesses
and record, the convergence rows; the distortions and the worst pair;
the record of every run; the wire sphere and the map for the page.
`representation-hyperbolic.xtl` (part 2): the preset read from
`presets/tree.txt` (its words dropped, its numbers kept in order); the
tree, its distances, classes, edges and plain layout (Kit's); the
disk's distance from the middle at five radii, the worked pair, the
geodesic and the straight segment with their lengths along the disk,
the room by level; the fit (`u:m_easure`, one step per kind of space,
the same start carried into each, `u:f_it` at a curvature from a
seed); the nine fits, the edge distortions, the worst pairs, the
spread; the path and the neighbors in the disk; the two classifiers;
the record and the baseline's row; the control; the wire sphere and
the sphere embedding's map. After each core: the printed sections and
the checks, the gradient checks among them.

## How it works

The split and run as the baseline. The decision boundary: Learn's
`k_nn` from the training rows alone, applied to a grid. The training
loop: Net's step applied `steps` times by `p_ower` to a state tuple.
The fit loop: `p_ower` over the state (the points, the log, the
curvature), each step measuring (W and the row of loss, mean
distortion and worst pair) before it moves, with Manifold's
`f_latDistances`, `s_phereDistances` or `b_allDistances` for E and
`f_latDistancesGradient`, `s_phereDistancesGradient` or
`b_allDistancesGradient` for the gradient along the space, then P -
lr G, `s_phereStep` or `b_allStep` and `b_allProject` (the violation
read by `b_allOff` first). The distortion: `d_istortionPairs` for the
edges' colors and the worst pair; `f_itScale` and `d_istortion` for
the scale-forgiven column. The curvature K enters as the scale of the
distances (times 1 / sqrt K on the sphere, divided by sqrt -K in the
disk) and of the step (lr sqrt |K|) and of the start (sqrt |K|).

## Results

From `reg/cli-representation-explorer.out` (part 1), the record
(accuracy, held out, numbers trained, operations), all measured:

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

From `reg/cli-representation-hyperbolic.out` (part 2), the tree's
record, all measured (the first start; distortion at a fixed scale,
an edge 1):

| Space | Mean distortion | Worst pair | Steps to under 0.2 | Scale forgiven | k-NN | Logistic | Fitted | Operations |
| ----- | --------------- | ---------- | ------------------ | -------------- | ---- | -------- | ------ | ---------- |
| the unit sphere (+1) | 0.591 | 0.952 | not reached | 0.602 | 0.429 | 0.429 | 93 | 3459600 |
| the plane (0) | 0.151 | 0.620 | 24 | 0.170 | 1.000 | 0.857 | 62 | 2306400 |
| the unit disk (-1) | 0.102 | 0.287 | 29 | 0.102 | 1.000 | 0.857 | 62 | 2306400 |

No step in the disk went past the rim (violation 0). The three
starts' end distortions: sphere 0.591, 0.591, 0.591 (spread 0.0003);
plane 0.151, 0.296, 0.151 (spread 0.145); disk 0.102, 0.237, 0.142
(spread 0.135). The baseline's plain layout: 0.179 mean, 0.805 worst,
nearest neighbors 1.000, below the plane's and the disk's second
starts. The control (mean distortion at step 100, every row and
picture at that step): +1 0.591, +1/4 0.310, +1/16 0.175, 0 0.151,
-1/16 0.132, -1/4 0.097, -1 0.122, -4 0.337. The disk: a point at plain radius 0.5 is 1.099 from
the middle, at 0.9 2.944, at 0.99 5.293, at 0.999 7.600, at the rim's
radius 12.206; u = (0.6, 0) and v = (0, 0.6) are 2.188 apart along the
disk (0.849 plain), the geodesic between them 2.188 long and the
straight segment 2.244 along the disk; the room by level (neighbors
apart in the plane, in the disk): 2.00, 2.00; 2.83, 3.34; 2.30, 4.11;
1.56, 4.75; 0.98, 5.36.

Against H1, by the rule written before the run. Part 1 as before: the
clusters count for it (the sphere costs 0.13 under nearest neighbors
and 0.10 under the network); the directions count for it under nearest
neighbors by one held-out point of 40 and against it under the
network, where the two spaces tied at 0.750: a null result. Part 2,
the tree: on accuracy the disk is at least the plane's (both 1.000
and 0.857) and the sphere far below (0.429), as the rule asks; on
distortion the disk is under the plane by 0.049 on the first start,
but the spread over three starts is 0.145 for the plane and 0.135 for
the disk (one start of each stopped in a local minimum far above the
others), and the rule counts the plane within the spread of the
matching space against H1: by the letter of the rule the tree's
distortion does not count for H1, a null result reported as one. The
tie on accuracy is for H1 by that rule and a null result by part 1's
stricter reading. Read start by start the disk is under the plane
every time (by 0.049, 0.059 and 0.009), which is the direction H1 says
and not what the rule asked. The other starts' curves say what kind
of end each is: the plane's second start is flat from step 50 (a
local minimum), the disk's second flattens by step 150 and its third
is still falling at step 200 (slow, not stuck). With the scale
forgiven the sphere is still 0.602: fitting a scale does not help, so
its loss is in the shape of its distances, not their unit; a larger
sphere helps only by letting the fit find another shape, as the
control's rows at +1/4 and +1/16 show. The control's lowest row is at
-1/4 on this start at step 100, with the rows from -1/16 to -1 within
0.035 of each other, less than the spread over starts, so which
negative curvature is lowest is not settled; the two sides meet at
the plane and -4, where the tree is pushed toward the rim, is too
much. A first run with the scale free saw the sphere's fit shrink its
embedding without end toward the plane's; the fixed scale measures
that instead.

## Beyond this page

The cosine comparison of word vectors and the unit-sphere embeddings
of face and sentence models drop the length on purpose; Nickel and
Kiela's (2017) Poincare embeddings of WordNet recovered a hierarchy's
links far more fully from a disk of a few dimensions than from a flat
space of the same size (they report ranks, not distortion); the exponential map and
Mobius addition the step uses are Ganea, Becigneul and Hofmann's
(2018); the distortion measure and the fixed-scale comparison across
curvatures follow Gu, Sala, Gunel and Re (2019), who learn each
factor's curvature; all read at source (`docs/papers.md`). Open: the
clusters and the directions fitted in the disk (two cells of the
table); a fit that escapes its local minima (restarts, a schedule for
the rate), so the spread is the geometry's and not the optimizer's; a
larger tree and trees of branching 3, where the counting says the
plane runs out of room by level 3; the spread over seeds in part 1; a
network given the length as an extra input; the deception H3 names (a
win on distortion with a loss on the task), which this page did not
see but was built to be able to see. Not established: that any real
data set is flat, directional or hierarchical; a real one's structure
is measured, by this page's distortion against a candidate space.

## Run it

```bash
just run representation-explorer                                  # part 1, with its checks
just run representation-explorer representation-hyperbolic.xtl    # part 2, with its checks
just test-demo representation-explorer                            # the baselines
```

Part 2's numbers are `presets/tree.txt`: change one and both the
command line and the page run the fit that way.

## Summary

On the clusters the shape of the space matters, in the direction the
data's structure says: kept on the sphere they lose 0.13 of held-out
accuracy under nearest neighbors and 0.10 under the network. On the
directions it matters by at most one held-out point of 40. On the
tree the disk keeps the distances with the least distortion (0.10
against the plane's 0.15 and the unit sphere's 0.59 on the first
start), the classifiers tell the branches apart in the plane and the
disk alike and fail on the sphere, and the control's distortion falls
from positive curvature through the plane to the negative side and
rises at -4. H1 by the rule written before the run: the clusters count
for it; the directions for it under nearest neighbors by one point and
against it under the network; the tree for it on accuracy (a tie,
which part 1's rule would read as a null result) and, on distortion,
not: the plane is within the spread over three starts of the disk, a
spread that is the fit's doing, though start by start the disk is
under the plane every time. Seven cells of nine are run.

## Next

metric-descent: a classifier trained on the sphere by steps that stay
on it (the sphere's step, as the fit's here), against steps in the
room put back onto it and against a plain step: the loss by step, the
size of each update, how far off the sphere each strays.

## Workarounds

- The split and the run's record are X_eTaL-ML's by the plan's G3
  (ask GM5, `docs/xetal-ML-asks.md`) and are written out in both
  programs, as in baseline-knn, until it lands.
- Pairwise straight-line distances (`u:c_hords` in part 1;
  Manifold's `f_latDistances` in part 2, so a fit reads the same in
  every space) and each point's neighbor order (`u:o_rders`) are
  X_eTaL-libraries' (GL1), written out as in sphere-hello.
- The preset is read by dropping every character that is not part of
  a number (`presets/tree.txt` is words and numbers; no word may hold
  a digit): a reader of labeled presets is not yet anyone's (the
  schema of `docs/research-questions.md` allows `.txt`).
