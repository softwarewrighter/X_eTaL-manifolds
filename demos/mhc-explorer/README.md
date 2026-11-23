# mHC explorer: what may the mixing do?

Part 1, the matrices. A deep network can carry several streams of
numbers side by side and mix them at every layer by a matrix; mHC
holds that matrix to the doubly stochastic matrices, which can only
average. This page takes the matrices one at a time: the layer over
several streams, the doubly stochastic matrices of size 3 and a path
through them, Sinkhorn's iteration worked, what a doubly stochastic, a
free and an orthogonal matrix each do to a signal mixed again and
again, and the set itself, a polytope. Live at
[softwarewrighter.github.io/X_eTaL-manifolds/mhc-explorer](https://softwarewrighter.github.io/X_eTaL-manifolds/mhc-explorer/).
Part 2 stacks them in a network.

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

## The idea

Before stacking matrices in a network, look at one at a time: count
its freedom, walk a path through the set, make one from a free matrix
and watch the errors fall, and mix one signal with each kind again and
again (a hundred mixes is what a stack a hundred layers deep does to
the part of the signal the layers leave alone).

## What this is, and is not

The matrices alone: one matrix of each kind, chosen or made by the
program, not learned and not depending on the input (actual mHC learns
matrices that depend on the input). Nothing is trained. It compares
three kinds of mixing on one signal with four matrices: doubly
stochastic (Sinkhorn's result), free (the positive matrix it came
from, and that matrix scaled down) and orthogonal (the nearest to the
free one).

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

## The run

Nothing is trained. The program builds the matrices from seeds and the
preset, mixes the signal and measures.

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

## How it works

The Mixing library (`libs/Mixing`, `hc:`) holds the layer, the
streams' mean and differences, the path and the measures of how far a
matrix is from each set; the program composes them with Matrix's
singular values and SVD. Sinkhorn's iteration and the polar factor are
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

## Beyond this page

Zhu et al. (Hyper-Connections), Xie et al. (mHC), Li, Sha and Yang
(The Birkhoff Geometry of mHC), all read at source (`docs/papers.md`).
Open: the constraint through a stack of layers and in training (H2,
part 2); how many Sinkhorn sweeps a learned matrix needs; orthogonal
mixing as the alternative that keeps the differences.

## Run it

```bash
just run mhc-explorer            # the program, its sections and 20 checks
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

## Next

Part 2: the same kinds of mixing in a stack of layers, the size of the
signal forward and of the gradient backward through depth, and a small
network trained under each (H2).

## Workarounds

- Sinkhorn's iteration (matrix scaling) and the polar factor (the
  nearest orthogonal matrix) are X_eTaL-libraries' by GL3: written out
  in the program (`u:s_weep`; `(o_\ U) mx:m_ul V` from `mx:s_vd`) and in
  Mixing's demo.
- A reader of labeled presets is X_eTaL-libraries' by GL9: the preset's
  digits kept and its words dropped.
- The shared code linker links a function parameter as a built-in
  (GM10): the page shows `hc:l_ayer` without its source aside.
