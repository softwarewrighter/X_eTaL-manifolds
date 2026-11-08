# Kit

The experiment kit's data sets, each with a known structure and made
from its own seed: flat (Gaussian clusters), directional (classes that
differ by direction and not by length) and hierarchical (a tree, its
distances along the tree, a class per branch, and a plain layout of it
in the plane); and the seeded numbers they are made of.

```
"kt:" u_se< "Kit"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `kt:` is the
recommended alias. Kit uses Manifold (this repository) for the sphere
and Graphs (X_eTaL-libraries) for the tree's distances.

## Why a library, and why here

The plan (G3, "The functions the demos need") lets a data set be a
file with the program that wrote it, or a function of a library here
when it needs one. These are functions: the explorers vary the sizes,
the spread and the seed of a set to see when a geometry's advantage
appears, and a file could hold one setting only. They are this
repository's because what they make is a geometric structure
(directions on the sphere, by Manifold; a tree and its distances) and
no repository with no interest in manifolds would want them; the
seeded generator inside, which is general, is written out here until
X_eTaL-libraries' Random takes a seed of its own
(`docs/xetal-libraries-asks.md`, GL8).

Every set is a function of its seed and its sizes alone, not of the
run's seed (`xetal run --seed`) nor of what was drawn before it, so
two demos asking for the same set get the same numbers, and the
comparisons of this repository, between geometries on the same data,
hold their data equal by construction. A data set is `(X, y)`: `X` a
matrix, one point per row; `y` its labels, 1 to k.

## Seeded numbers

| Function | Type | What |
| -------- | ---- | ---- |
| `kt:u_niform (seed, n)` | `(Int, Int) -> Float` | n Floats in [0, 1): Park and Miller's (1988) sequence x -> 16807 x mod (2^31 - 1) started from 1 + \|seed\| mod (2^31 - 2), divided by 2^31 - 1 |
| `kt:n_ormal (seed, n)` | `(Int, Int) -> Float` | n standard normal Floats, by Box and Muller from 2n uniform numbers of the seed |

A prefix of a longer draw is the shorter draw of the same seed.

## The data sets

| Function | Type | What |
| -------- | ---- | ---- |
| `kt:f_lat (seed, k, n, d, spread)` | `(Int, Int, Int, Int, a) -> (Float, Int)` | k Gaussian clusters of n points each in d dimensions: centers are standard normal numbers of the seed scaled by 4, each point its center plus spread times a standard normal vector; (X, y) with y the cluster, 1 to k, in order |
| `kt:d_irections (seed, k, n, cone)` | `(Int, Int, Int, a) -> (Float, Int)` | k classes of n points in three dimensions that differ by direction, not length: a class direction on the unit sphere, each point that direction plus cone times a normal vector put back onto the sphere, scaled by a length drawn between 0.5 and 2; (X, y) |
| `kt:t_ree (depth, branching)` | `(Int, Int) -> Int` | the edges of a rooted tree, every inner node with `branching` children, every leaf `depth` from the root; node 1 the root, nodes numbered level by level; two rows, parent over child, as Graphs takes them |
| `kt:t_reeDistances (depth, branching)` | `(Int, Int) -> Float` | the distance along the tree between every two nodes (edges on the one path), by Graphs' shortest paths; n by n, 2 depth at most |
| `kt:t_reeClasses (depth, branching)` | `(Int, Int) -> Int` | each node's branch, the child of the root it descends from, 1 to branching; the root is given 1 |
| `kt:t_reeLayout (depth, branching)` | `(Int, Int) -> Float` | a plain picture of the tree in the plane, one (x, y) per node: the root at the origin, each level a circle of radius its depth, a level's nodes spread evenly round it in order; a layout, not an embedding that keeps the tree's distances |

Depth 4 and branching 2 give the hierarchical set of the plan: 31
nodes, 30 edges, distances up to 8.

## Examples

From `../demos/example.xtl`:

```
      kt:t_ree (2, 2)
1 1 2 2 3 3
2 3 4 5 6 7
      kt:t_reeClasses (2, 2)
1 1 2 1 1 2 2
      (X, y) := kt:f_lat (1, 3, 2, 2, 0.2)
      y
1 1 2 2 3 3
```

## Tests

`../tests/kit.xtl`: the same seed gives the same numbers and another
seed others, a prefix of a draw is the shorter draw, the uniform and
normal numbers have the mean and spread they should; each set has the
shape and labels it says and reproduces from its seed; within a flat
cluster points are nearer than any two points of different clusters;
the directional set's lengths vary and its angles do not within a
class; the tree has 31 nodes and 30 edges at depth 4, every node but
the root one parent that comes before it, distances symmetric with
the farthest pair 2 depth apart, classes by branch; the layout puts
each node on the circle of its depth and each branch in its half of
the plane, and distorts the tree's distances (Manifold's measure),
which is the baseline's point.

## Demos

- [`demos/example.xtl`](../demos/example.xtl): the three sets at small
  sizes, from their seeds.

## Provenance

Written for X_eTaL-manifolds (step experiment-kit), to X_eTaL's style
guide and X_eTaL-libraries' layout. The generator is Park and Miller,
"Random number generators: good ones are hard to find", Communications
of the ACM 31(10), 1988 (the minimal standard, multiplier 16807); the
normal numbers by Box and Muller (1958). The data sets follow
`docs/research-questions.md` (H1's three structures).
