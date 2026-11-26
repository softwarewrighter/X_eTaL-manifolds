# mHC explorer: what may the mixing do?

Part 1, the matrices. A deep network can carry several streams of
numbers side by side and mix them at every layer by a matrix; mHC
holds that matrix to the doubly stochastic matrices, which can only
average. This page takes the matrices one at a time: the layer over
several streams, the doubly stochastic matrices of size 3 and a path
through them, Sinkhorn's iteration worked, what a doubly stochastic, a
free and an orthogonal matrix each do to a signal mixed again and
again, and the set itself, a polytope. Part 2 stacks them: a
small network of 4 to 32 layers over four streams under four kinds of
mixing, measured layer by layer at the start of training and then
trained. Live at
[softwarewrighter.github.io/X_eTaL-manifolds/mhc-explorer](https://softwarewrighter.github.io/X_eTaL-manifolds/mhc-explorer/).

## The question

When a network carries several streams of numbers side by side and
mixes them at every layer, what may the mixing be allowed to do?

## Background

A residual block keeps a stream x and adds what a layer F computes
from it: x + F(x). Hyper-connections (Zhu et al., ICLR 2025) keep n
streams and mix them at every layer by an n by n matrix; mHC (Xie et
al., 2025; ICML 2026) writes the layer as X <- H X + b F(a' X) and
holds H to the doubly stochastic matrices: no entry negative, every
row and every column summing to 1. Such a matrix keeps the streams'
mean (its columns sum to 1) and cannot enlarge their differences (its
largest singular value is 1). An orthogonal matrix keeps lengths and
may have negative entries. The set of doubly stochastic matrices is the
Birkhoff polytope, whose corners are the permutation matrices.

Part 2 asks it of a network: does holding the mixing to a constraint
set help a deep stack carry a signal and train?

## The idea

Before stacking matrices in a network, look at one at a time: count
its freedom, walk a path through the set, make one from a free matrix
and watch the errors fall, and mix one signal with each kind again and
again (a hundred mixes is what a stack a hundred layers deep does to
the part of the signal the layers leave alone). Then stack them, each
layer adding its own output, and measure the signal going up, the
gradient coming down and the streams' differences, layer by layer,
before training and while training.

## What this is, and is not

The matrices alone: one matrix of each kind, chosen or made by the
program, not learned and not depending on the input (actual mHC learns
matrices that depend on the input). Nothing is trained. It compares
three kinds of mixing on one signal with four matrices: doubly
stochastic (Sinkhorn's result), free (the positive matrix it came
from, and that matrix scaled down) and orthogonal (the nearest to the
free one). Part 2 is a network, small on purpose: 4 streams of 4
numbers, a 4 by 4 dense layer and tanh per layer, one small task, 3
seeds; its mixing is learned but does not depend on the input (mHC's
does). The full training is done offline by the same program and read
back; the page trains the first steps again and checks they agree.

## The intuition

Three cups of water and a rule for pouring between them. A doubly
stochastic rule keeps the total and evens the levels out; a free rule
may fill the cups from nothing or let the water vanish; an orthogonal
rule is a reshuffling with signs, which keeps the total size and need
not even the levels out.

## The mathematics

The layer: r = a' X, the layer's output F(r), the streams H X + b F(r).
With every stream equal to x, the weights a summing to 1, b = 1 and H's
rows summing to 1, it gives n copies of x + F(x). The count: n^2
entries, 2 n sums of which 2 n - 1 are independent (checked as
Matrix's rank of the conditions), (n - 1)^2 free: four for n = 3. The
path: H(t) = (1 - t) I + t J / n, singular values 1 and 1 - t.
Sinkhorn: rows then columns divided by their sums, repeated; after
each sweep the columns sum to 1 exactly. A signal mixed m times is
H^m v: its mean is kept when H's columns sum to 1, its spread is
multiplied per mix by at most H's second singular value; a free
matrix's powers grow or die at the size of its largest eigenvalue,
which its largest singular value bounds; an orthogonal matrix keeps
the length. The orthogonal matrix used is A's polar factor from
Matrix's SVD.

Part 2's kinds, each made from a layer's n by n trained numbers T: the
plain residual, H = I; free, H = I + T; doubly stochastic, 20 sweeps
of Sinkhorn's iteration on exp(z I + T), z = 4 a pull toward the
identity; orthogonal, the Cayley transform (I - K)^-1 (I + K) of K =
(T - T') / 2. A layer: r = a' X, F = tanh(r W + beta), X becomes H X +
b F, every number trained. Every gradient (through the layer, the
Sinkhorn sweeps and the Cayley transform) is written out in Mixing and
checked against central differences.

## What is compared

The problem: what one mixing matrix, applied again and again, does to
the streams' mean, their differences and their size. The choices:
doubly stochastic, free (two), orthogonal, on one signal (3, -1, 0.5)
mixed 1, 10 and 100 times (`presets/matrices.txt`). Hypothesis H2's
first half, at the level of one matrix. Written before the run: a
doubly stochastic matrix that changes the mean, enlarges the spread or
has a singular value above 1, or Sinkhorn's errors not falling, would
count against the constraint doing what its definition says; a free
matrix whose powers stay bounded and away from 0, against "free mixing
can blow up or die away"; a change of length, against "orthogonal keeps
length"; a spread after 100 mixes well below the start, against
"orthogonal keeps the differences". The free matrices are chosen so
that their outcomes follow from their singular values: they illustrate
the definitions rather than test them.

Part 2 (`presets/depth.txt`): the four kinds (plain residual, free,
doubly stochastic, orthogonal) on the kit's flat clusters (72 points,
54 trained on, 18 held out), depths 4, 8, 16, 32, 3 seeds, the same
seeded start for every kind, plain gradient descent at rate 0.05 for
200 steps. Written before the run (`docs/research-questions.md`, H2):
it counts for H2 if free mixing's composite gain grows past 10 at 32
layers while the constrained stay within 0.5 to 2, and at 32 layers the
constrained stacks reach a lower loss or an accuracy higher by more
than the seeds' range, at a cost within twice the free one's; against,
if free mixing's gain stays bounded, if the constrained stacks end at a
higher loss at every depth, or if doubly stochastic mixing collapses
the streams and loses accuracy to the plain residual. Changed, said as
such: the start is the identity plus seeded numbers, not the identity;
runs that stop being finite count as not trained.

## The run

Part 1: nothing is trained. The program builds the matrices from seeds
and the preset, mixes the signal and measures. Part 2: 48 trainings
(4 kinds, 4 depths, 3 seeds), run offline by `just train mhc-explorer`
(`train/train.sh`: `mhc-depth.xtl` up to its model, then one
`u:r_un` per seed; the records in `data/`, with `data/runs.txt` saying
what made them, about three minutes, four at a time); the start of
training measured live; the first 20 steps at 4 layers trained again
on the page and compared with the records.

## The program

[`mhc-explorer.xtl`](mhc-explorer.xtl): the preset; one stream through
x + F(x) and three copies through `hc:l_ayer`; the count (with the
rank check) and the path (`hc:b_lend`, with sums, smallest entry,
singular values, `hc:s_tochasticOff`); Sinkhorn's iteration (`u:s_weep`,
written out: GL3) for 20 sweeps, one sweep worked; the four matrices,
the signal's mean, spread (`hc:s_pread`) and length after each count,
the measured rates, and each matrix's singular values, gain
(`hc:g_ain`) and errors against each set; the size-2 segment, the six
permutation matrices, the corner and middle steps and the zero
entries. 20 checks.

[`mhc-depth.xtl`](mhc-depth.xtl): the preset; the data, split and
standardized; the model (`u:s_tart`, `u:e_mbed`, `u:p_ass`: Mixing's
`hc:s_tack` and `hc:s_tackBack` between a read-in and NN's read-out);
`u:r_un`, one training with Optim's `s_gd`; the gradient checked on ten
numbers per kind; every stack measured at the start (`hc:s_izes`,
`hc:s_preads`, `hc:g_ain` of the product); the cost counted; the page's
own run; the records read back; the H2 table. 5 checks.

## How it works

The Mixing library (`libs/Mixing`, `hc:`) holds the layer, the
streams' mean and differences, the path and the measures of how far a
matrix is from each set, and for part 2 the four kinds of mixing made
from trained numbers with their gradients, a layer and a stack carried
forward and back, and the sizes and spreads by state; the programs
compose them with Matrix's singular values and SVD and X_eTaL-ML's NN
and Optim. Sinkhorn's iteration and the polar factor are
general (X_eTaL-libraries' by GL3) and are written out in the program.

## Results

From the baseline (`reg/cli-mhc-explorer.out`, 20 checks):

- Three copies of x through the layer, mixed by the matrix at t = 0.5:
  three copies of x + F(x), to rounding.
- Size 3: 9 entries, 5 independent sums (the rank), 4 free.
- The path: every matrix's row and column sums 1, no negative entry,
  singular values 1, 1 - t, 1 - t.
- Sinkhorn from the free A: the row error falls at every sweep, from
  5.19 to 0.18 after one sweep and under 1e-6 at sweep 9; the columns
  sum to 1 after every sweep. mHC's 20 sweeps leave 1.3e-15.
- The signal, length 3.20, after 100 mixes: under Sinkhorn's matrix the
  mean stays 0.8333 and the spread falls from 2.86 to 2.4e-15; under
  the free A the length reaches 6.6e71 (measured rate 5.23 per mix,
  largest singular value 5.34); under A scaled down it falls to 9.1e-32
  (rate 0.49); under the orthogonal matrix the length stays 3.20, the
  mean goes from 0.83 to 0.28 and the spread from 2.86 to 3.16; its
  gain is 1.24, from negative entries.
- The set: the six permutation matrices on it and orthogonal; from the
  identity (six entries at 0) a step of -0.1 along D leaves it (smallest
  entry -0.033), +0.1 stays; from the middle (no zero entry) both stay.

Nothing counts against the constraint doing what its definition says,
nor against the free and orthogonal statements; the free outcomes
were chosen to follow from the definitions. The doubly stochastic
matrix forgets the streams' differences, the cost the Birkhoff-geometry
paper names; whether that matters in a trained network is part 2's
measurement.

Part 2, from the baseline (`reg/cli-mhc-depth.out`) and the records
(the full table is in `docs/experiments.md`, H2):

- At the start, 32 layers, means over 3 seeds: the free mixing's
  composite gain 272 (3.8, 11.8, 82 at 4, 8, 16 layers), the doubly
  stochastic 1.00, the orthogonal 1.79. The signal grew up the stack
  5.8 times for the plain residual, 5.7 doubly stochastic, 4.9
  orthogonal, 180 free; the gradient down it 26, 26, 11 and 128.
- The doubly stochastic stack and the plain residual are the same
  network at the start (columns summing to 1 keep the mean, and the
  loss sees only the mean); the doubly stochastic stack's spread fell
  to 0.088 of the start at 32 layers, the orthogonal one's grew to 5.3.
- Trained: free mixing stopped being finite for 2 of 3 seeds at 16
  layers and all 3 at 32; every other seed trained. Against the plain
  residual, no constraint differed beyond the seeds' range except the
  orthogonal stack's held-out accuracy at 8 layers (0.944 against
  0.833 to 0.889, one point of 18). The task is solved at 4 layers.
- Every gradient matched central differences to under 1e-9; the
  page's 20 steps matched the records to rounding; 20 Sinkhorn sweeps
  from the random start leave the rows off by up to 0.0064.

So H2's first half holds at these depths and its second half holds
against free mixing only; against the plain residual it is open at
this scale. A mixing held to its set did not keep the whole network's
size: the plain residual's grew too.

## Beyond this page

Zhu et al. (Hyper-Connections), Xie et al. (mHC), Li, Sha and Yang
(The Birkhoff Geometry of mHC), all read at source (`docs/papers.md`).
Related, in a sibling repository: X_eTaL-fine-tuning's training-at-depth
lesson trains a plain skip, hyper-connections and mHC at 12 to 48
layers on a token task. Open: mixing that depends on the input; a task
that needs its depth; the rate each kind needs; how many Sinkhorn
sweeps a learned matrix needs near the identity; the other constraint
sets of the constraint-lab saga.

## Run it

```bash
just run mhc-explorer            # the program, its sections and 20 checks
just run mhc-explorer mhc-depth.xtl   # part 2: the stacks, the records read back, 5 checks
just train mhc-explorer          # part 2's 48 trainings, offline (about three minutes)
just show mhc-explorer           # as a notebook
just test-demo mhc-explorer      # CLI baseline, web tests, wasm build, browser check
just serve mhc-explorer          # the page on port 3285
just test-lib Mixing             # the Mixing library's baselines
```

## Summary

Several streams mixed by one matrix per layer contain the one-stream
residual block. A doubly stochastic matrix of size 3 has four free
numbers; the path from the identity to uniform mixing stays in the set.
Sinkhorn's iteration made a free positive matrix doubly stochastic to
1e-6 in nine sweeps. Mixed a hundred times, a signal kept its mean and
lost its differences under the doubly stochastic matrix, blew up or
died away under the free ones, and kept its length under the orthogonal
one. The set is a polytope, its corners the permutation matrices.

Several such layers stacked: free mixing's gain and the free stack's
signal grew with depth and its training stopped being finite at 16 and
32 layers; doubly stochastic and orthogonal mixing kept the gain bounded
and trained at every depth, the first forgetting its streams'
differences; against the plain residual neither differed beyond the
seeds' range but for one point at 8 layers.

## Next

Other constraint sets for the mixing, and mixing that depends on the
input (the constraint-lab saga).

## Workarounds

- Sinkhorn's iteration (matrix scaling) and the polar factor (the
  nearest orthogonal matrix) are X_eTaL-libraries' by GL3: written out
  in the program (`u:s_weep`; `(o_\ U) mx:m_ul V` from `mx:s_vd`) and in
  Mixing's demo.
- A reader of labeled presets is X_eTaL-libraries' by GL9: the preset's
  digits kept and its words dropped.
- The shared code linker links a function parameter as a built-in
  (GM10): the page shows `hc:l_ayer` without its source aside.
- Sinkhorn's iteration with its gradient, which part 2 trains through,
  is private to Mixing (`m_ix`, `m_ixBack`) while GL3 is open; a
  Hyper-Connections library with the same pieces is asked of X_eTaL-ML
  by X_eTaL-fine-tuning (FM13), noted here as GM11.
- The split (GM5) is written out.
