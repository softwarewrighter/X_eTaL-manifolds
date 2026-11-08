# Kit

The experiment kit's data sets, each with a known structure and made
from its own seed: flat (Gaussian clusters), directional (classes that
differ by direction and not by length) and hierarchical (a tree, its
distances along the tree, a class per branch, and a plain layout of it
in the plane); and the seeded numbers they are made of.

```
"kt:" u_se< "Kit"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Kit.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo Kit         # run its demos
just run Kit          # run its test programs
just test-lib Kit     # check every baseline
```
