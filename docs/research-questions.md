# Research questions

The one question every demo serves (docs/plan.md):

> How does the geometry of a representation, a parameter space or a
> computational transformation influence how a machine-learning model
> learns and performs inference?

Five research areas carve it up; three hypotheses, H1 to H3, are what
the first release tests. For each hypothesis, before any run: the
experiment, the data, the choices compared, the measures, the budget
held equal, and what result counts for it, against it, or leaves it
open (plan, principle 3: a null result is a result). The experiment
schema at the end says what a preset, a run's record, a comparison and
a baseline hold.

## The five research areas

| Area | Question | Track (demo.toml) | Explorer |
| ---- | -------- | ----------------- | -------- |
| the data manifold | does the data lie near a curved thing of low intrinsic dimension, and what does a method that knows it gain? | start, representation | manifold-tour, baseline-knn |
| representation geometry | does the space a model puts its points in (flat, spherical, hyperbolic; its curvature and dimension) change what it learns? | representation | representation-explorer |
| optimization geometry | does the metric of a training step (plain, preconditioned, natural, on a manifold) change the path and the cost of training? | optimization | optimization-explorer |
| constrained transformations | does holding a transformation to a set (doubly stochastic, orthogonal, low rank) keep a deep network stable, and at what cost? | constraints | mhc-explorer |
| dynamics on manifolds | when a network is a flow, what does the field's geometry (gradient or not; flat or held to a manifold) do to where it carries points? | dynamics | dynamics-explorer |

A sixth, adaptive geometry (learning the geometry with the model), is
H3 and Saga 2's.

## H1: a geometry that matches the data's structure helps a representation

**Claim.** Points with a directional structure are better represented
on a sphere, points with a hierarchical (tree) structure in a
hyperbolic space, and flat clusters in a flat space, than in the other
two spaces at the same dimension: distances are kept with less
distortion and a simple classifier on them is at least as accurate.

**Experiment** (representation-explorer, steps representation-sphere
and representation-hyperbolic). Three seeded data sets of the
experiment kit, each built with a known structure: flat clusters (k
Gaussian blobs in the plane, with labels), directions (unit vectors in
three dimensions in cones about a few axes, with labels; their lengths
carry nothing), and a tree (a rooted tree of depth 4 and branching 3,
whose distances are the path lengths). Each set is embedded in three
spaces at the same dimension: the flat plane (dimension 2), the sphere
(the unit sphere in three dimensions, intrinsic dimension 2) and the
Poincare disk (dimension 2). The embedding is fitted by the same
method in each space (gradient descent on the distortion with the
space's own gradient step: Manifold's step for the sphere and the
ball), from the same seeded start, for the same number of steps and
the same learning rate schedule.

**Measures** (G8): distortion (the average over pairs of
|d_space / d_true - 1|, and the worst pair); nearest-neighbor accuracy
under the space's distance (k = 5, a held-out quarter); the accuracy of
a logistic classifier on the embedded coordinates (Learn's, same
steps); the mean of each class on the space and how far the plain mean
is off it; steps to reach a distortion target; cost counted (numbers
trained, operations per step).

**Held equal.** Dimension, data, split, seed, steps, learning rate,
the fitting method, the classifier.

**Counts for H1.** For each data set, the matching space has the
lowest distortion by a margin larger than the spread over three seeds,
and its accuracy is at least the others'.

**Counts against H1.** The flat space is within the seed spread of the
matching space on distortion for the directional or the tree data, or
beats it on accuracy; or the matching space wins on distortion and
loses on accuracy (the deception of H3, seen early).

**Leaves it open.** Mixed results across the three seeds; a win on
distortion with no difference in accuracy (then H1 holds for the
representation and says nothing about the task, and the page says
so).

**The baseline row.** baseline-knn: k nearest neighbors under the
straight-line distance on all three data sets, the row every H1 entry
is set against.

## H2: a constraint on a transformation keeps a deep network stable

**Claim.** Mixing residual streams by doubly stochastic matrices
(mHC) or orthogonal matrices keeps the size of the signal forward and
of the gradient backward within a fixed range through any depth, where
free mixing (hyper-connections) lets them grow or shrink by a factor
that increases with depth; and a small model so constrained trains as
well or better at depth, at a cost that is measured.

**Experiment** (mhc-explorer, steps mhc-matrices and mhc-depth).
Mixing's stack: n = 4 streams of width 8, a block per layer (a small
dense layer with a nonlinearity, as a stand-in for a transformer
block), and a mixing matrix per layer that is one of: plain residual
(the identity), free (hyper-connections: a learned n by n matrix),
doubly stochastic (Sinkhorn's iteration on exp of learned logits, k
sweeps, k a control), orthogonal (the nearest orthogonal matrix by the
SVD of a learned matrix). Depths 4, 8, 16, 32. First untrained (the
matrices seeded), measuring the gain through the stack; then trained
on one small task (the flat-clusters classification of the kit) at
each depth.

**Measures.** The size of the signal after each layer and of the
gradient before it (the largest and the mean over a batch); the
composite gain (the largest absolute row sum forward, column sum
backward, as the mHC paper measures it) by depth; the row and column
sum errors of each doubly stochastic matrix after k sweeps; the
singular values of each mixing matrix and of the composite; the mean
of the streams kept and their differences shrunk (the two channels of
the Birkhoff-geometry paper); training loss by step and held-out
accuracy at each depth; numbers trained and operations per step.

**Held equal.** Data, split, seed, width, depth, block, steps,
learning rate, initialization (the free matrices start at the identity
too).

**Counts for H2.** The composite gain of free mixing grows with depth
(by more than 10 at depth 32) and the constrained ones stay within [0.5,
2]; and at depth 32 the constrained models reach a lower loss or an
accuracy higher by more than the seed spread, at a cost per step
within 2x of the free one.

**Counts against H2.** Free mixing's gain stays bounded at these
depths (then the instability needs a depth or a width this repository
does not build, and the page says so); or the constrained models train
worse at every depth; or doubly stochastic mixing collapses the
streams to their mean (the difference channel fading, as the
Birkhoff-geometry paper predicts) and loses accuracy to the plain
residual.

**Leaves it open.** Stability shown (the gain bounded) with no
difference in training at the depths tried: H2's first half holds and
its second half is untested at this scale, which the page says in
those words.

**What the papers say, for the record.** mHC (DeepSeek, 2025) reports
the gain reduced from about 3000 to about 1.6 at 27B parameters and a
loss lower by 0.021; KromHC (2026) and the Birkhoff-geometry paper
(2026) both note the twenty Sinkhorn sweeps leave the constraint
approximate; the PEFT paper (2026) found that in fine-tuning the
identity mixing beat a learned one. None of these is reproduced at
toy scale by this repository until it is; docs/papers.md says which
claims were read and how.

## H3: geometry learned with the model can beat geometry chosen for it, and can also deceive

**Claim.** On a data set that mixes structures (part flat, part
hierarchical), a curvature learned with the model reaches a lower
loss than any one fixed curvature; and the learned curvature can
improve the task loss while making the geometry less faithful (the
distortion worse), so task loss alone does not validate a geometry.

**Experiment** (Saga 2, learned-curvature; the measures are fixed
now so Saga 1's experiment kit records them). The tree data and the
flat clusters mixed; a representation in a space of curvature c, c
fixed at -1, 0, +1 and c learned by gradient descent with the model
(the mixed-curvature and adaptive-curvature literature, docs/papers.md);
the same classifier, steps and seeds.

**Measures.** Loss and accuracy by step; distortion against the data's
true distances by step; the learned c by step; cost.

**Held equal.** As H1, plus the parameterization of c.

**Counts for H3.** Learned c reaches a loss lower than every fixed c by
more than the seed spread, on the mixed data; and in at least one run
the distortion worsens while the loss improves.

**Counts against H3.** A fixed c matches or beats the learned one;
or the learned c settles at one of the fixed values (then learning
bought nothing a choice would not have).

**Leaves it open.** The learned c wins on loss and the distortion also
improves: H3's first half holds and its second (the deception) is not
seen on this data.

## The experiment schema

What every comparison shares (step experiment-kit builds it; the
kinds here are fixed now).

**A preset** (`demos/<slug>/presets/<name>.xtl` or `.txt`, data not
code): the seed; the data set (its name and the parameters it was
generated with: sizes, noise, depth and branching for the tree); the
split (the held-out fraction and the seed of the split); the choices
compared, as a list of names the program understands (`flat`,
`sphere`, `ball`; `identity`, `free`, `stochastic`, `orthogonal`);
the budgets (steps, learning rate and its schedule, the number of
Sinkhorn sweeps, the dimension); the tolerances (what counts as on the
manifold, what counts as converged); the measures to record.

**A run's record** (what the program prints, and what a page replays):
for each choice compared, the diagnostics of G8 that apply, each
marked measured or analytical: training loss by step; held-out
accuracy; the size of the update by step; numerical stability (the
largest value seen, whether any step was rejected); constraint
violation (off the manifold, row and column sum errors, distance from
orthogonal, by step); geometric distortion; cost counted (numbers
trained, operations, iterations). Every number from the X_eTaL run;
the record is deterministic from the preset.

**How two runs are compared.** Same preset but for the choice; the
difference of each measure, and the spread of the measure over the
three seeds of the preset family, so a difference smaller than the
spread is reported as none. A comparison table on a page names the
measure, the two values, the difference and the spread; it says
"better" only for a measure with a direction, and never for the
geometry.

**What goes into a reg-rs baseline.** The record as printed: every
measured number at the printed precision, so a later X_eTaL or a
later library is tested against today's results (G7). What never
does: runtime (measured apart, on the page's "cost" line as a count
of operations, never as seconds), the build host, dates, anything
from the browser's own state (an angle, a mode).
