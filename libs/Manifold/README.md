# Manifold

The unit sphere, the Poincare ball, and the pictures of the circle,
the sphere, the torus and the Swiss roll: the distance measured along
the space, putting a point onto it, the tangent part of a vector at a
point, a step from a point along a tangent vector that stays on the
space and the arrow that undoes it, a mean that stays on the space,
how far off it a point is; and the points of each space at given
coordinates of its map, for drawing it.

```
"mf:" u_se< "Manifold"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Manifold.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo Manifold         # run its demos
just run-lib Manifold      # run its test programs
just test-lib Manifold     # check every baseline
```
