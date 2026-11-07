# Changes

Every commit, newest first, grouped by day. Times are Pacific
(UTC-08:00), as committed.

Categories: `feat` new capability, `fix` a bug or wrong behavior,
`refactor` structure without behavior change, `test` tests only,
`build` build and tooling, `demo` a demo (an experiment or an
introduction) or a change to one, `lib` a library or a change to one,
`docs` documentation, `plan` saga planning and reordering, `release`
milestone release, `chore` agentrail bookkeeping (step complete, saga
archive), `pin` a move of the X_eTaL-ML or X_eTaL-libraries pin.

## 2026-11-06

- `build` the machinery (step foundation): the pins `XETAL_ML_COMMIT` (X_eTaL-ML abc1745, which pins X_eTaL abdd371) and `XETAL_LIBRARIES_COMMIT` (3465af3), both the siblings' HEADs, cloned into the gitignored `work/` and built by X_eTaL-ML's own script (`just xetal`, `just ml-pin`, `just libs-pin`, `just xetal-version`); `XETAL_PATH` from this repository's `libs/`, then X_eTaL-ML's, then X_eTaL-libraries' (`just path`, `just eval`); `scripts/py` on a uv-managed `.venv`; the gate (`just gate`, `--full`) with the pinned-toolchain check, the tooling self-tests, the libraries' and demos' reg-rs baselines, doc-test, rebind, spelling and ASCII markdown; the demo and library recipes; the page recipes (`just pages`, `just serve` and `just serve-pages` on port 3285, `just publish`, `just check-live`, the browser checks, screenshots, the catalog) adapted and waiting for the first page; `demos/_template` with the fields of plan G5 (`problem`, `compares`, `track`, `hypothesis`, `intro`) and `scripts/demos.py check` refusing fewer than two choices, with a self-test; `templates/Library`; `.gitignore`, `.cargo/config.toml` (`MICROSCOPE_LIBS` naming `libs/` and the X_eTaL-libraries clone), `images/` and the README's header. Every script notes where it came from and what changed: the record is ask GM2 in `docs/xetal-ML-asks.md`
- `lib` Manifold (`mf:`), the unit sphere: `s_phereProject`, `s_phereDistance` and `s_phereDistances` (2 atan2(|p - q|, |p + q|), accurate near 0 and pi), `s_phereTangent`, `s_phereStep` (the exponential map), `s_phereArrow` (the logarithmic map), `s_phereMean` (the Frechet mean by 30 fixed-point steps), `s_phereOff`; the naming rule a second space (the Poincare ball) will follow, in its docs; 18 doc examples, 33 checks near the hard cases (the same point, opposite points, a zero tangent vector, a millionth of a radian), a demo, baselines
- `demo` sphere-hello (track start, an introduction, CLI only): six points on the sphere; the chord and the arc order every point's neighbors the same way, checked on the numbers with chord = 2 sin(arc / 2); the plain mean is 0.43 inside the sphere and the mean on the sphere is 0.098 radians from it pushed out. README in the page arc; baseline
- `plan` step manifold-tour inserted after foundation (the user: a live page with visualizations of manifolds, an introduction to the concept, the kinds and what each is for here): the first page, on X_eTaL-ML's microscope by path, before the shared teaching shell lands; G2 amended, G13's view by a demo-private workaround until GM3 lands
- `plan` plan, rules, asks and the geometry-lab saga: `docs/plan.md` (principles, decisions G1 to G13, the tracks, hypotheses H1 to H3, the demo portfolio, Saga 1 in thirteen steps, the sagas after it), `docs/xetal-ML-asks.md` (GM1 the shared teaching shell, GM2 the shared scripts, GM3 the shared turning view), `docs/xetal-fine-tuning-asks.md` (GF1, GF2), `docs/research.txt`, CLAUDE.md with the rules (AGENTS.md its link), README, COPYRIGHT, LICENSE, `.agentrail/`
- `chore` the repository: an empty README
