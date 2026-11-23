# Mixing

Residual streams mixed under a constraint: the layer over n streams as
mHC writes it, the streams' mean and differences, the path from the
identity to uniform mixing, and how far a matrix is from doubly
stochastic or orthogonal, with mHC's gain. The reference is
[`docs/README.md`](docs/README.md).

```
"hc:" u_se< "Mixing"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Mixing.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo Mixing         # run its demos
just run Mixing          # run its test programs
just test-lib Mixing     # check every baseline
```
