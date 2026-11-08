# Manifold

The unit sphere, the Poincare ball, and the pictures of the circle,
the sphere, the torus and the Swiss roll: the distance measured along
the space, putting a point onto it, the tangent part of a vector at a
point, a step from a point along a tangent vector that stays on the
space and the arrow that undoes it, a mean that stays on the space,
how far off it a point is; the points of each space at given
coordinates of its map, for drawing it; and the distortion of an
embedding's distances against the distances they should keep.

```
"mf:" u_se< "Manifold"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `mf:` is the
recommended alias. Manifold uses Trig (X_eTaL-libraries) for
`tr:a_tan2`, `tr:a_sinh` and `tr:a_tanh`.

## How things are named

A point is a vector of Floats, in any number of dimensions; a set of
points is a matrix, one point per row. Each space has a name that
starts its functions (`s_phere`, `b_all` the Poincare ball, `c_ircle`,
`t_orus`, `r_oll` the Swiss roll), and the word after it is the same
verb for every space, so a program written for the sphere reads the
same on the ball (the plan's step representation-hyperbolic completes
`b_all...` with the sphere's verbs):

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
| Points | the points of the space at rows of coordinates of its map (for drawing it) |
| Arc, Geodesic | points along the shortest path between two points of the space (Arc on the sphere, Geodesic on the ball) |

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

## The sphere, drawn, and the circle

| Function | Type | What |
| -------- | ---- | ---- |
| `mf:s_pherePoints LL` | `Num a => a -> Float` | the point of the unit sphere at each (longitude, latitude) row of LL, radians: one row (x, y, z) per row; the sphere's map |
| `(p, q) mf:s_phereArc n` | `(Num a, Num b) => (a, b) -> Int -> Float` | n + 1 points along the great circle from p to q, p first, evenly spaced; all p when q is p or its opposite |
| `mf:c_irclePoints A` | `Num a => a -> Float` | the point of the unit circle at each angle of A, radians from the x axis: one row (x, y) per angle |

Longitude goes round the z axis from the x axis, latitude up from the
equator: a parallel is rows of one latitude, a meridian rows of one
longitude. The circle is the sphere of one dimension (the unit sphere
in two): `s_phereProject`, `s_phereDistance` and `s_phereArc` work on
its points as they stand.

## The torus

| Function | Type | What |
| -------- | ---- | ---- |
| `radii mf:t_orusPoints UV` | `(Num a, Num b) => a -> b -> Float` | the point of the torus of radii R r (`radii` is the vector `R r`: R from the middle of the hole to the middle of the tube, r the tube's) at each (u, v) row of UV, radians, u round the hole and v round the tube: ((R + r cos v) cos u, (R + r cos v) sin u, r sin v) |
| `radii mf:t_orusOff p` | `(Num a, Num b) => a -> b -> Float` | the distance from p to the circle through the middle of the tube, less r, as a size: 0 on the torus; one per row of a set |

The torus's map is the (u, v) rectangle, 0 to 2 pi each way, each
pair of opposite edges joined.

## The Swiss roll

| Function | Type | What |
| -------- | ---- | ---- |
| `mf:r_ollPoints TH` | `Num a => a -> Float` | the point of the roll at each (t, h) row of TH: (t cos t, h, t sin t); a flat sheet rolled round the y axis, t winding out, h up |
| `mf:r_ollAlong t` | `Num a => a -> Float` | the length of the spiral from its middle (t = 0) out to t, (t sqrt(1 + t^2) + asinh t) / 2: how far along the unrolled sheet the points at t lie |
| `a mf:r_ollDistance b` | `(Num a, Num b) => a -> b -> Float` | the distance along the sheet between the points at (t, h) a and b: the sheet is flat, so sqrt((along a - along b)^2 + (h_a - h_b)^2) |

The roll's map is the sheet itself, the rectangle of (`r_ollAlong t`,
h). Two points a turn apart are 2 pi apart in three dimensions and far
apart along the sheet: the classic data manifold, on which the nearest
point through the air is the wrong neighbor.

## The Poincare ball

| Function | Type | What |
| -------- | ---- | ---- |
| `p mf:b_allDistance q` | `(Num a, Num b) => a -> b -> Float` | the hyperbolic distance between two points inside the unit ball (the disk in two dimensions), 2 atanh of the length of q seen from p |
| `(p, q) mf:b_allGeodesic n` | `(Num a, Num b) => (a, b) -> Int -> Float` | n + 1 points along the geodesic from p to q, p first, evenly spaced by the ball's distance; all p when q is p |

q seen from p is Mobius addition, (-p) (+) q = ((1 + 2 u.v + |v|^2) u
+ (1 - |u|^2) v) / (1 + 2 u.v + |u|^2 |v|^2) with u = -p and v = q;
the geodesic is p (+) (the fraction s of that arrow, scaled the ball's
way: tanh(s atanh |w|) w / |w|). Near the middle the distance is twice
the plain one; toward the rim it grows without bound. Drawn flat, a
geodesic is an arc of a circle meeting the rim at right angles, or a
diameter. Project, Tangent, Step, Arrow, Mean and Off for the ball are
step representation-hyperbolic's.

## Distortion

| Function | Type | What |
| -------- | ---- | ---- |
| `D mf:d_istortion E` | `(Num a, Num b) => a -> b -> Float` | how badly the distances E of an embedding (every pair, a matrix) keep the distances D they should have: over the pairs of different points with D > 0, the mean and the worst of the fraction by which each pair is too long or too short, as a pair of Floats; 0 0 when every distance is kept |

The measure of H1 (`docs/research-questions.md`), the same whatever
space E was measured along: a tree's distances against the
straight-line distances of a layout, or the great-circle distances
of a set on the sphere, or the ball's. Multiplying every distance by
2 gives 1 1; halving gives 0.5 0.5.

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

From `../demos/tour.xtl`: two points of the sphere a tenth of a radian
apart along the equator, then two a quarter turn apart; for each, the
straight line through the sphere and the great circle along it:

```
      near := mf:s_pherePoints 2 2 r_eshape 0.0 0.0 0.1 0.0
      (s_qrt '+ r_/ ((1 s_elect near) - 2 s_elect near) ^ 2.0) c_at (1 s_elect near) mf:s_phereDistance 2 s_elect near
0.09995833854135666 0.1
      far := mf:s_pherePoints (Pi / 2.0) * 2 2 r_eshape 0.0 0.0 1.0 0.0
      (s_qrt '+ r_/ ((1 s_elect far) - 2 s_elect far) ^ 2.0) c_at (1 s_elect far) mf:s_phereDistance 2 s_elect far
1.414213562373095 1.5707963267948966
```

Near, the two agree to four digits: the sphere is locally flat. Far,
they do not. The roll and the disk in the same demo: two points of the
roll one turn apart are 6.28 apart through the air and 57.8 along the
sheet; two points of the disk 0.2 apart in the plane are 0.41 apart by
the disk's distance near the middle and 2.50 near the rim.

## Tests

`../tests/sphere.xtl`: the easy cases and the hard ones (the same
point, opposite points, a zero tangent vector, a point a millionth of
a radian from its neighbor), checked with Check: a step stays on the
sphere and goes the distance; an arrow is tangent, as long as the
distance, and undone by a step; the arrows from the mean cancel.

`../tests/tour.xtl`: a point of each drawn space lies where its
definition says (circle points have length 1, the map's poles are the
poles, torus points are at the two radii and `t_orusOff` says so, the
roll's points are t from the axis and h up, `r_ollAlong`'s rate is the
spiral's speed by a central difference); an arc or a geodesic runs
from its first point to its last, on the space, evenly spaced by the
space's distance; the ball's distance agrees with the acosh form, is
symmetric, about twice the plain distance near the middle and large
near the rim; a diameter is straight and another geodesic bows toward
the middle; the distortion of kept, doubled and halved distances, a
pair left out at D = 0.

## Demos

- [`demos/sphere.xtl`](../demos/sphere.xtl): three points, their
  distances, a step and its arrow, the two means.
- [`demos/tour.xtl`](../demos/tour.xtl): a point of each drawn space
  from its map, and one distance along each against the straight line.

## Provenance

Written for X_eTaL-manifolds, to X_eTaL's style guide and
X_eTaL-libraries' layout, so that a part that turns out general can
move there unchanged. The formulas are the standard ones for the
sphere as a Riemannian manifold (the exponential and logarithmic maps,
the Frechet mean by fixed-point iteration), as in Absil, Mahony and
Sepulchre, Optimization Algorithms on Matrix Manifolds (2008), chapter
3. The ball's Mobius addition and its geodesic are the gyrovector
forms (Ungar, Analytic Hyperbolic Geometry, 2005; the formula read at
source in Ungar, Hyperbolic Barycentric Coordinates, AJMAA 2009,
Definition 6.1), as the hyperbolic neural networks of Ganea,
Becigneul and Hofmann (NeurIPS 2018, their equations 6 to 15) write
them at c = 1; the Swiss roll is the data manifold of Tenenbaum, de
Silva and Langford's Isomap (Science 2000, their Figure 3), in the
coordinates everyone uses. Each is a row of `docs/papers.md`.
