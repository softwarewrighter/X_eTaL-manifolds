# Mixing

Residual streams mixed under a constraint. A residual block carries
one stream x and adds what a layer computes from it, x + F(x);
hyper-connections (Zhu et al., ICLR 2025) carry n streams side by side
and mix them at every layer by an n by n matrix; mHC (Xie et al.,
2025) holds that matrix to the doubly stochastic matrices (no entry
negative, every row and every column summing to 1). This library has
the layer over n streams as mHC writes it, the streams' mean and
differences, the path from the identity to uniform mixing, and how far
a matrix is from each kind of mixing, with the gain mHC measures.

```
"hc:" u_se< "Mixing"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `hc:` is the
recommended alias. The tests use Matrix (X_eTaL-libraries) for
products and singular values; the library itself needs nothing.

Streams are a matrix with one row per stream (n rows) and one column
per coordinate (d columns); a vector is n streams of one number each.
A mixing matrix is n by n. Every result is a Float.

## Streams

| Function | Type | What |
| -------- | ---- | ---- |
| `hc:e_xpand (n, x)` | `Num a => (Int, a) -> Float` | one stream x copied into n streams, n rows |
| `hc:m_ean X` | `Num a => a -> Float` | the streams' mean, coordinate by coordinate (one number for a vector of streams) |
| `hc:d_ifferences X` | `Num a => a -> Float` | each stream less the mean, X's shape |
| `hc:s_pread X` | `Num a => a -> Float` | the size of the differences, sqrt of the sum of their squares; 0 when every stream is the same |

## The layer

| Function | Type | What |
| -------- | ---- | ---- |
| `(H, a, b) 'f hc:l_ayer X` | `(Num a, Num b, Num c, Num d) => (Float -> Float) -> (a, b, c) -> d -> Float` | one layer over the streams as mHC's equation 3 writes it: the read r = a' X, the layer's output f r, the streams mixed by H and the output written back with weights b: H X + b f(a' X); a and b are vectors of n; with one stream and H = a = b = 1, x + f(x) |

## The path from the identity to uniform mixing

| Function | Type | What |
| -------- | ---- | ---- |
| `hc:b_lend (t, n)` | `(Float, Int) -> Float` | (1 - t) I + t J / n: the identity at t = 0, uniform mixing at t = 1, doubly stochastic for every t in between; its singular values are 1 and n - 1 times 1 - t |

## How far a matrix is from each kind

| Function | Type | What |
| -------- | ---- | ---- |
| `hc:s_umErrors H` | `Num a => a -> Float` | the largest row sum error and column sum error against 1, two numbers |
| `hc:n_egativity H` | `Num a => a -> Float` | how far below 0 the most negative entry is, 0 if none |
| `hc:s_tochasticOff H` | `Num a => a -> Float` | the largest of the two sum errors and the negativity: 0 on the doubly stochastic matrices |
| `hc:o_rthogonalOff H` | `Num a => a -> Float` | the largest entry, in size, of H' H less the identity: 0 for an orthogonal matrix |
| `hc:g_ain H` | `Num a => a -> Float` | mHC's gain: the largest sum of a row's entries' sizes (forward) and of a column's (backward), two numbers; 1 1 for a doubly stochastic matrix, at most the product of the factors' for a product |

A doubly stochastic matrix keeps the streams' mean (its columns sum to
1) and does not enlarge their differences (its largest singular value
is 1); repeated, it shrinks them toward the mean (the Birkhoff-geometry
paper's two channels, `docs/papers.md`). `tests/mixing.xtl` checks the
layer against x + F(x), the path's matrices on the set, the mean kept
and the differences shrunk by 1 - t, the singular values, and the
violation measures on a permutation, uniform mixing, a rotation and a
free positive matrix.

The set of doubly stochastic matrices is the Birkhoff polytope, a
polytope with flat faces and the permutation matrices as its corners,
not a smooth manifold everywhere (`docs/glossary.md`, polytope); mHC's
name, "manifold-constrained", is the paper's.

## Not here

Sinkhorn's iteration (each row divided by its sum, then each column,
repeated), which makes a positive matrix doubly stochastic, is matrix
scaling, general mathematics: it is asked of X_eTaL-libraries (GL3,
`docs/xetal-libraries-asks.md`) with the polar factor (the nearest
orthogonal matrix). Meanwhile it is written out where it is used:
[`demos/sinkhorn.xtl`](../demos/sinkhorn.xtl) and
`demos/mhc-explorer/mhc-explorer.xtl` (`u:s_weep`), and the polar factor
by Matrix's `s_vd`.
