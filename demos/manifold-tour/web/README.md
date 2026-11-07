# Manifold tour, the page

A Yew app on X_eTaL-ML's `shared/microscope` (by path from the pinned
clone, `work/xetal-ml`) and this repository's `shared/site`. It runs
the demo's program once (`src/micro.rs`: the core of
`../manifold-tour.xtl` followed by one `r_avel` line per array), reads
every coordinate, wire line and distance back, and draws them
(`src/app.rs`); the words are `src/text.rs`. Tested natively in
`tests/micro.rs` against the command-line baseline; in the browser by
`browser.txt` (`just test-demo manifold-tour`).

## The turning view is a workaround

`src/turn.rs` turns a picture in three dimensions (yaw round the up
axis and pitch toward the viewer, from pointer drag, the arrow keys
and touch; an orthographic projection; the angle shown in the
caption) and is this demo's own. It is the demo-private stopgap named
in ask GM3 of `docs/xetal-ML-asks.md` (the shared turning view of
X_eTaL-ML) and is removed when GM3 lands at the pin. It is
presentation only (CLAUDE.md rule 3a, plan G13): it moves points,
segments and wire lines that X_eTaL computed and computes no
coordinate, distance or number shown on the page.

## Build

`just serve manifold-tour` (trunk, port 3285); `just pages` builds it
into `pages/manifold-tour/`; `just publish` puts it live.
