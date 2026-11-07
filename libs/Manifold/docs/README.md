# Manifold

The unit sphere, and in time the Poincare ball: the distance measured
along the space, putting a point onto it, the tangent part of a vector
at a point, a step from a point along a tangent vector that stays on
the space and the arrow that undoes it, a mean that stays on the
space, and how far off it a point is.

```
"mf:" u_se< "Manifold"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `mf:` is the
recommended alias. Manifold uses Trig (X_eTaL-libraries) for
`tr:a_tan2`.

## How things are named

A point is a vector of Floats, in any number of dimensions; a set of
points is a matrix, one point per row. Each space has a name that
starts its functions, and the word after it is the same verb for every
space, so a program written for the sphere reads the same on the ball
when the ball joins (the plan's step representation-hyperbolic adds
`b_all...` with the same verbs):

| Verb | What it does |
| ---- | ------------ |
| Project | a vector put onto the space |
| Distance | the distance between two points, measured along the space |
| Distances | the same for every pair of rows of a set |
| Tangent | the part of a vector that lies along the space at a point |
| Step | from a point, along a tangent vector, staying on the space (the exponential map) |
| Arrow | the tangent vector a Step would take from one point to another (the logarithmic map) |
| Mean | the mean of a set, on the space (the Frechet or Karcher mean) |
| Off | how far a point is from the space, as a size: 0 when it is on it |

Every result is a Float.

## The unit sphere

| Function | Type | What |
| -------- | ---- | ---- |
| `mf:s_phereProject v` | `Num a => a -> Float` | v scaled to length 1; the zero vector is refused |
| `p mf:s_phereDistance q` | `(Num a, Num b) => a -> b -> Float` | the great-circle distance, the angle between p and q in radians (0 to pi) |
| `mf:s_phereDistances P` | `Num a => a -> Float` | the great-circle distance between every pair of rows, a matrix |
| `p mf:s_phereTangent v` | `(Num a, Num b) => a -> b -> Float` | v less its part along p |
| `p mf:s_phereStep v` | `(Num a, Num b) => a -> b -> Float` | the point reached from p along the tangent vector v, for v's length |
| `p mf:s_phereArrow q` | `(Num a, Num b) => a -> b -> Float` | the tangent vector at p that steps to q; zero for q = p and for the opposite point |
| `mf:s_phereMean P` | `Num a => a -> Float` | the mean of the rows on the sphere; rows whose plain mean is zero are refused |
| `mf:s_phereOff p` | `Num a => a -> Float` | the difference between the length of p and 1, as a size |

The distance is computed as 2 atan2(|p - q|, |p + q|), which keeps its
digits near 0 and near pi, where acos of the product p.q does not; the
set's distances sum each coordinate's differences for the same reason.
The step is cos t p + sin t v / t with t the length of v, put back
onto the sphere so rounding never leaves it. The mean starts from the
plain mean put onto the sphere and takes 30 steps along the average
of the arrows to the rows; points spread over less than a hemisphere
have one mean, which the steps reach.

## Examples

From `../demos/sphere.xtl` (`P` is the three axes, one per row;
`p` and `q` its first two rows):

```
      p mf:s_phereDistance q
1.5707963267948966
      v := p mf:s_phereArrow q
      v
0.0 1.5707963267948966 0.0
      p mf:s_phereStep v
0.00000000000000006123233995736766 1.0 0.0
      m := ('+ r_/ P) / 3.0
      mf:s_phereOff m
0.42264973081037427
      mf:s_phereMean P
0.5773502691896257 0.5773502691896257 0.5773502691896257
```

The plain mean of three points on the sphere lies well inside it (0.42
short of the surface); the mean on the sphere is their common
direction.

## Tests

`../tests/sphere.xtl`: the easy cases and the hard ones (the same
point, opposite points, a zero tangent vector, a point a millionth of
a radian from its neighbor), checked with Check: a step stays on the
sphere and goes the distance; an arrow is tangent, as long as the
distance, and undone by a step; the arrows from the mean cancel.

## Demos

- [`demos/sphere.xtl`](../demos/sphere.xtl): three points, their
  distances, a step and its arrow, the two means.

## Provenance

Written for X_eTaL-manifolds, to X_eTaL's style guide and
X_eTaL-libraries' layout, so that a part that turns out general can
move there unchanged. The formulas are the standard ones for the
sphere as a Riemannian manifold (the exponential and logarithmic maps,
the Frechet mean by fixed-point iteration), as in Absil, Mahony and
Sepulchre, Optimization Algorithms on Matrix Manifolds (2008), chapter
3.
