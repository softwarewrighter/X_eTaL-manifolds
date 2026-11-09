# Sphere hello

The first program of this repository: six points on the unit sphere,
two ways to measure how far apart they are, and two ways to average
them. It proves the chain from the pinned X_eTaL to a tested library
(Manifold) and a program, and it shows, in numbers, the two facts the
later experiments build on.

## The question

If points live on a sphere, does it matter whether distance is
measured straight through the sphere or along its surface? And what
is "the average" of points on a sphere, when the ordinary average
falls inside it?

## Background

A unit vector is a point on the unit sphere: an arrow of length 1
from the origin. Two points on it are apart by the straight line
between them (the chord, which cuts through the sphere) or by the
shortest path along the surface (an arc of a great circle: the angle
between the two arrows, in radians). The surface is the manifold; the
arc is the distance measured on it. The sphere is the simplest curved
space, and the one representation experiments use first (the plan's
step representation-sphere), so its facts are worth seeing plainly
before anything is trained on it.

## The idea

Measure the same six points two ways and see whether the ways
disagree where it matters: on which points are each other's
neighbors, and on where the middle of the points is. If the straight
line and the great circle rank every point's neighbors the same way,
a method that only ranks neighbors loses nothing by using the
straight line; if the plain average falls off the sphere, a method
that averages unit vectors must either accept a point that is not one
or use the mean that stays on the sphere.

## What this is, and is not

An introduction (`intro = true`): one set of six points, measured
two ways, and averaged two ways. It trains nothing. Live at
<https://softwarewrighter.github.io/X_eTaL-manifolds/sphere-hello/>
on the shared teaching shell: pick a pair and read its two distances,
see each point's neighbors under each, and the two means.

## The intuition

Two cities on the globe: the string through the earth between them is
shorter than the flight along the surface, by little for near cities
and by much for far ones, but the nearest city to any city is the
same by both. And the average of the cities' positions, as points in
space, is somewhere under the ground: the "middle" of points on a
sphere has to be found on the sphere.

## The mathematics

Points p and q on the unit sphere (length 1). The chord is the
straight-line distance, sqrt(sum (p_i - q_i)^2). The arc is the angle
between them, 2 atan2(|p - q|, |p + q|), in radians, 0 to pi. The two
are tied by chord = 2 sin(arc / 2), which is why they rank neighbors
alike: sin is increasing on [0, pi / 2], so a larger arc is a larger
chord. The plain mean is the coordinate-wise average, whose length is
under 1 unless the points coincide; the mean on the sphere is the
point m on it that makes the sum over the points of arc(m, p_i)^2
least, found by stepping from the projected plain mean along the
average of the arrows (logarithmic maps) to the points until they
cancel.

## What is compared

On one set of six points, chosen by hand: the chord against the arc,
for every pair; and the plain mean (coordinate by coordinate) against
the mean on the sphere (the point along the surface closest, in
summed squared arcs, to the points). What would count against the
claim "the two distances agree on neighbors" is any point whose
neighbors come in a different order by the two.

## The run

No model is trained: the run is the program, which measures the six
points. Its record is the two distance matrices, the two neighbor
orders and the two means, which the page reads back.

## The program

[`sphere-hello.xtl`](sphere-hello.xtl), in four parts:

1. the points: six directions put onto the sphere with
   `mf:s_phereProject`, and a check that each is on it
   (`mf:s_phereOff`);
2. two distances: the chord between every pair (written out in the
   program: a straight-line distance is general, and X_eTaL-libraries
   has none for rows yet; see Workarounds) and the arc
   (`mf:s_phereDistances`); the check that a chord is 2 sin of half
   its arc, on every pair, and that the arc is never the shorter;
3. the same neighbors: each point's neighbors in order of chord and
   in order of arc, and the check that the two orders are the same;
4. two means: the plain mean, its length (how far inside the sphere
   it is), and `mf:s_phereMean`, which is on the sphere; and the
   distance between the mean on the sphere and the plain mean put
   onto it, which is not zero.

## How it works

The chord between two points is the length of their difference. The
arc is the angle between them, which Manifold computes as
2 atan2(|p - q|, |p + q|), a form that keeps its digits for close
points. The chord is 2 sin(arc / 2), and sin is increasing on half a
turn, so a longer arc always means a longer chord: that is why the
two order neighbors the same way, and the program checks it on the
numbers rather than taking the argument's word.

The plain mean adds the coordinates and divides by six; its length
comes out as 0.57, so it sits well inside the sphere. The mean on the
sphere starts from that point pushed out onto the surface and takes
steps along the surface (each along the average of the arrows to the
points) until the arrows cancel; the result is on the sphere, 0.098
radians (about 5.6 degrees) from where the plain mean was pushed
out, so the two are different points.

## Results

From the baseline (`reg/cli-sphere-hello.out`): on the six points the
chord and the arc give the same neighbor order for every point; the
widest gap between arc and chord over all pairs is 0.456 (at the
widest arc, 2.464); the plain mean has length 0.571, so it is 0.429
inside the sphere; the mean on the sphere is 0.098 radians from the
plain mean put onto the sphere. Nothing counts against "the two
distances agree on neighbors" here; it would on a set with ties or
with points near opposite each other, which the page notes.

## Beyond this page

The mean on the sphere is the Frechet (Karcher) mean of Absil, Mahony
and Sepulchre's chapter 3 (`docs/papers.md`); that the chord and the
arc rank alike follows from chord = 2 sin(arc / 2) and holds for any
points, not only these six. Open: whether using the arc instead of the
chord changes what a classifier learns on data with a directional
structure (H1, representation-explorer); the baseline page measures
the straight line's accuracy there first.

## Run it

```bash
just run sphere-hello          # run the program
just show sphere-hello         # as a notebook: each statement, then its output
just test-demo sphere-hello    # its CLI baseline (reg-rs)
```

What the run prints (measured, from the baseline
[`reg/cli-sphere-hello.out`](reg/cli-sphere-hello.out)): the six
points, the chord and the arc matrices, the widest gap between the
two (0.578 radians, on the widest arc, 2.464), the neighbor orders,
the plain mean and its length (0.571), the mean on the sphere, and
the six checks, all `ok`.

## Summary

On these points the chord and the arc order every point's neighbors
the same way, so for "who is nearest" the choice of distance does
not matter on a sphere; it matters for how far, and for anything that
adds distances up: a mean. The plain mean of points on a sphere is
not on it, and the mean along the surface is a different direction,
not just the plain mean pushed out. What this does not show: anything
about learning; the representation experiments (the plan's Saga 1,
steps representation-sphere and after) ask whether these facts change
what a model learns. Next: a page for this program, when the shared
shell has landed, and the baseline classifier (step experiment-kit).

## Next

baseline-knn: the three data sets of the kit and nearest neighbors
under the straight line, the first row of the H1 table; then
representation-explorer, where the sphere's distance is compared with
the straight line on the directional data.

## Workarounds

The straight-line distance between every pair of rows is written out
in the program (`u:c_hords`): a general function, for
X_eTaL-libraries (to be asked for in `docs/xetal-libraries-asks.md`
by step research-foundation; Geometry's `ge:d_istances` takes points
in the plane only).
