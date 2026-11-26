//! The page, on the shared teaching shell: the arc of frames; the
//! streams as parallel channels before and after one layer; the count
//! of free entries and the path from the identity to uniform mixing, a
//! point picked by the reader, its matrix as a heat map with its sums
//! and singular values; Sinkhorn's iteration worked and its errors by
//! sweep; four kinds of matrix and what each does to a signal; the
//! polytope's corners; the comparison; every idea explained; the
//! program. Presentation only: every number shown is one X_eTaL printed,
//! and picking a point runs nothing.

use yew::prelude::*;

use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::definition;
use lesson::heat::{Heat, Matrix};
use lesson::narrate;
use site::{comparison, functions, mark, own_function, Cell, ANALYSIS, EXPERIMENT, EXTENSION, INTUITION, MATHEMATICS, RUN};

use gloo_timers::callback::Timeout;

use crate::depth::{program as depth_program, run as depth_run, Depth, PRESET as DEPTH_PRESET, RUNS};
use crate::micro::{program, run, Mhc, M3, KINDS, PRESET};
use crate::stack;
use crate::text;

fn show(x: f64) -> String {
    format!("{x:.4}")
}

fn tiny(x: f64) -> String {
    if x == 0.0 {
        "0".into()
    } else if x.abs() < 0.001 || x.abs() >= 100000.0 {
        format!("{x:.2e}")
    } else {
        format!("{x:.4}")
    }
}

fn m3(m: &M3) -> Matrix {
    Matrix { rows: 3, cols: 3, items: m.to_vec() }
}

fn heat(name: &str, caption: &str, m: &M3, scale: f64, hint: bool) -> Html {
    html! { <Heat name={name.to_string()} caption={caption.to_string()} matrix={m3(m)} scale={scale} numbers=true shared=true legend={hint} hint={hint} /> }
}

const LANE: [&str; 3] = ["#e8590c", "#2b8a3e", "#1c7ed6"];

/// Streams as parallel channels: one lane per stream, a bar per coordinate (up for positive, down for negative), on one scale.
fn lanes(rows: &[Vec<f64>], label: &str, scale: f64) -> Html {
    let (w, lane) = (300.0, 46.0);
    let h = lane * rows.len() as f64 + 10.0;
    let d = rows.first().map(|r| r.len()).unwrap_or(1);
    let step = (w - 40.0) / d as f64;
    html! {
        <svg viewBox={format!("0 0 {w} {h}")} role="img" aria-label={label.to_string()}>
            { for rows.iter().enumerate().map(|(s, r)| {
                let mid = 5.0 + lane * (s as f64 + 0.5);
                html! { <g>
                    <line class="axis" x1="34" y1={format!("{mid:.1}")} x2={format!("{:.1}", w - 4.0)} y2={format!("{mid:.1}")} />
                    <text class="axis-lbl" x="2" y={format!("{:.1}", mid + 4.0)}>{ format!("stream {}", s + 1) }</text>
                    { for r.iter().enumerate().map(|(j, v)| {
                        let x = 40.0 + step * (j as f64 + 0.3);
                        let hgt = (v / scale).clamp(-1.0, 1.0) * (lane / 2.0 - 4.0);
                        let (y, hh) = if hgt >= 0.0 { (mid - hgt, hgt) } else { (mid, -hgt) };
                        html! { <rect x={format!("{x:.1}")} y={format!("{y:.1}")} width={format!("{:.1}", step * 0.5)} height={format!("{:.1}", hh.max(0.5))} fill={LANE[s % 3]}><title>{ format!("stream {}, coordinate {}: {}", s + 1, j + 1, show(*v)) }</title></rect> }
                    }) }
                </g> }
            }) }
        </svg>
    }
}

fn streams(m: &Mhc) -> Html {
    let scale = m.streams.iter().chain(&m.mixed).flatten().fold(0.0_f64, |a, v| a.max(v.abs()));
    let row = |v: &[f64]| v.iter().map(|x| show(*x)).collect::<Vec<_>>().join(", ");
    html! {
        <section class="space" id="streams">
            <h2>{ "From one stream to three" }</h2>
            { p(&format!("One stream x of {} numbers through the residual block x + F(x), F(x) = tanh(W x); then three copies of x through the layer over three streams, mixed by the matrix halfway along the path below (a doubly stochastic matrix), read by their mean and written back to each: three copies of x + F(x) again. Then three different streams through the same layer.", m.d)) }
            <table class="numbers">
                <thead><tr><th>{ "" }</th><th>{ "numbers" }</th></tr></thead>
                <tbody>
                    <tr><td>{ "x" }</td><td>{ row(&m.x) }</td></tr>
                    <tr><td>{ "x + F(x), one stream" }</td><td>{ row(&m.one) }</td></tr>
                    { for m.copies.iter().enumerate().map(|(s, c)| html! { <tr><td>{ format!("stream {} after the layer over three copies", s + 1) }</td><td>{ row(c) }</td></tr> }) }
                </tbody>
            </table>
            <div class="figure">
                <figure class="flat">
                    { lanes(&m.streams, "three streams of four numbers each, before the layer: one lane per stream, a bar per number, up for positive and down for negative", scale) }
                    <figcaption>{ "Three different streams before the layer: one lane per stream, one bar per number (up positive, down negative), on one scale with the picture beside it. Hover or tap a bar for its value." }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    { lanes(&m.mixed, "the same three streams after one layer: closer to each other, each with the layer's output added", scale) }
                    <figcaption>{ "The same streams after one layer: mixed by the doubly stochastic matrix (they move toward each other) and each given the layer's output F(mean)." }{ mark(true) }</figcaption>
                </figure>
            </div>
        </section>
    }
}

fn path_buttons(chosen: usize, m: &Mhc, pick: Callback<usize>) -> Html {
    html! {
        <div class="controls" role="group" aria-label="the point t on the path from the identity to uniform mixing">
            { for m.path.iter().enumerate().map(|(k, pt)| {
                let pick = pick.clone();
                html! { <button class={classes!(chosen.eq(&k).then_some("on"))} onclick={Callback::from(move |_| pick.emit(k))} aria-pressed={if chosen == k { "true" } else { "false" }}>{ format!("t = {:.1}", pt.t) }</button> }
            }) }
        </div>
    }
}

/// The singular values along the path, each point a dot; the chosen one ringed.
fn singular_chart(m: &Mhc, chosen: usize) -> Html {
    let (w, h) = (320.0, 200.0);
    let at = |t: f64, s: f64| (34.0 + (w - 44.0) * t, h - 24.0 - (h - 40.0) * s);
    html! {
        <svg viewBox={format!("0 0 {w} {h}")} role="img" aria-label="the three singular values of each matrix on the path against t: one stays at 1, two fall from 1 to 0 together">
            <line class="axis" x1="34" y1={format!("{:.1}", h - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", h - 24.0)} />
            <line class="axis" x1="34" y1="12" x2="34" y2={format!("{:.1}", h - 24.0)} />
            <text class="axis-lbl" x="38" y="12">{ "singular values, 0 to 1" }</text>
            <text class="axis-lbl" x="34" y={format!("{:.1}", h - 8.0)}>{ "t = 0, the identity" }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 110.0)} y={format!("{:.1}", h - 8.0)}>{ "t = 1, uniform mixing" }</text>
            { for m.path.iter().enumerate().flat_map(|(k, pt)| pt.singular.iter().enumerate().map(move |(i, s)| {
                let (x, y) = at(pt.t, *s);
                html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r={if k == chosen { "5" } else { "3" }} fill={if i == 0 { "#7048e8" } else { "#e8590c" }} stroke={if k == chosen { "#212529" } else { "none" }} /> }
            }).collect::<Vec<_>>()) }
        </svg>
    }
}

fn path_section(m: &Mhc, chosen: usize, pick: Callback<usize>) -> Html {
    let pt = m.path[chosen];
    let sums = |v: &[f64; 3]| v.iter().map(|x| show(*x)).collect::<Vec<_>>().join(", ");
    html! {
        <section class="space" id="path">
            <h2>{ "The doubly stochastic matrices of size 3, and a path through them" }</h2>
            <div class="worked-out">
                <h3>{ "Counting the free entries" }{ mark(false) }</h3>
                <p>{ format!("Entries: 3 x 3 = {}. Conditions: 3 rows and 3 columns summing to 1, of which one follows from the others (the rows' sums and the columns' sums both total 3): {} independent, which the program checks as Matrix's rank of the six conditions. Free: {} - {} = {}, as (3 - 1)^2 = {} says. Pick the top-left 2 by 2 block and the sums fix the rest.", m.counts[0], m.counts[1], m.counts[0], m.counts[1], m.counts[2], m.counts[3]) }</p>
            </div>
            { p("The path from the identity (t = 0: each stream kept) to uniform mixing (t = 1: each stream replaced by the mean): (1 - t) times the identity plus t times the matrix of all 1/3. Pick a point. In every heat map on this page a square's color is its entry, red for positive and blue for negative, deeper for larger; no entry on the path is negative, and the scale runs to -1 only because it is shared with matrices that have negative entries.") }
            { path_buttons(chosen, m, pick) }
            <div class="figure">
                <figure>
                    { heat("H(t)", &format!("the matrix at t = {:.1}", pt.t), &pt.matrix, 1.0, true) }
                    <table class="numbers">
                        <tbody>
                            <tr><td>{ "row sums" }</td><td>{ sums(&pt.row_sums) }</td></tr>
                            <tr><td>{ "column sums" }</td><td>{ sums(&pt.col_sums) }</td></tr>
                            <tr><td>{ "smallest entry" }</td><td>{ show(pt.smallest) }</td></tr>
                            <tr><td>{ "singular values" }</td><td>{ sums(&pt.singular) }</td></tr>
                            <tr><td>{ "distance from the set (Mixing's)" }</td><td>{ tiny(pt.off) }</td></tr>
                        </tbody>
                    </table>
                    { mark(true) }
                    <p class="narrated">{ narrate::moved(&format!("the factor the differences between streams are multiplied by (the second singular value), from the identity to t = {:.1}", pt.t), m.path[0].singular[1], pt.singular[1]) }</p>
                    <p class="narrated">{ narrate::moved(&format!("the smallest entry, from the identity to t = {:.1}", pt.t), m.path[0].smallest, pt.smallest) }</p>
                </figure>
                <figure class="flat">
                    { singular_chart(m, chosen) }
                    <figcaption>{ "The singular values of every matrix on the path: one stays at 1 (purple, the direction of all-equal streams, which every matrix on the path keeps); two fall together as 1 - t (orange, the differences between streams). The chosen point is outlined." }{ mark(true) }</figcaption>
                </figure>
            </div>
        </section>
    }
}

/// Sinkhorn's errors by sweep on a log scale, row error and column error.
fn error_chart(m: &Mhc) -> Html {
    let (w, h) = (360.0, 220.0);
    let n = m.errors.len();
    let (lo, hi) = (1e-16_f64.log10(), 10.0_f64.log10());
    let at = |k: usize, e: f64| (34.0 + (w - 44.0) * k as f64 / (n - 1) as f64, h - 24.0 - (h - 36.0) * (e.max(1e-16).log10() - lo) / (hi - lo));
    let y_t = at(0, m.tolerance).1;
    let line = |i: usize, color: &str| { let d: Vec<String> = m.errors.iter().enumerate().map(|(k, e)| { let (x, y) = at(k, e[i]); format!("{x:.1},{y:.1}") }).collect(); html! { <polyline points={d.join(" ")} fill="none" stroke={color.to_string()} stroke-width="2" /> } };
    html! {
        <svg viewBox={format!("0 0 {w} {h}")} role="img" aria-label="Sinkhorn's row and column sum errors by sweep on a logarithmic scale: the row error falls steadily, the column error is 0 after every sweep but for rounding; the tolerance as a line">
            <line class="axis" x1="34" y1={format!("{:.1}", h - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", h - 24.0)} />
            <line class="axis" x1="34" y1="12" x2="34" y2={format!("{:.1}", h - 24.0)} />
            <line class="chord" x1="34" y1={format!("{y_t:.1}")} x2={format!("{:.1}", w - 10.0)} y2={format!("{y_t:.1}")} />
            <text class="axis-lbl" x="38" y={format!("{:.1}", y_t - 3.0)}>{ format!("{}, the tolerance", m.tolerance) }</text>
            <text class="axis-lbl" x="38" y="12">{ "sum error, 10 at the top to 1e-16 at the bottom (log scale)" }</text>
            <text class="axis-lbl" x="34" y={format!("{:.1}", h - 8.0)}>{ "before" }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", h - 8.0)}>{ format!("sweep {}", n - 1) }</text>
            { line(0, "#e8590c") }
            { line(1, "#1c7ed6") }
        </svg>
    }
}

fn sinkhorn_section(m: &Mhc) -> Html {
    let v3 = |v: &[f64; 3]| v.iter().map(|x| show(*x)).collect::<Vec<_>>().join(", ");
    let scale = m.a.iter().fold(0.0_f64, |a, v| a.max(v.abs()));
    html! {
        <section class="space" id="sinkhorn">
            <h2>{ "Sinkhorn's iteration: any positive matrix made doubly stochastic" }</h2>
            { p("A free positive matrix A (the exponential of seeded numbers, as mHC makes its matrix from learned numbers): divide each row by its sum, then each column by its sum, and repeat.") }
            <div class="worked-out">
                <h3>{ "One sweep, worked in full" }{ mark(true) }</h3>
                <p>{ format!("A, by rows: ({}), ({}), ({}). The row sums of A: {}. Each row divided by its sum gives the second matrix; its rows now sum to 1. Its column sums: {}. Each column divided by its sum gives the third matrix: its columns sum to 1, and its rows no longer quite do (their sums are now {}).", v3(&[m.a[0], m.a[1], m.a[2]]), v3(&[m.a[3], m.a[4], m.a[5]]), v3(&[m.a[6], m.a[7], m.a[8]]), v3(&m.row_sums), v3(&m.col_sums), v3(&m.row_sums_after)) }</p>
                <p>{ format!("One entry followed through: the top-left entry of A is {}; divided by its row's sum, {}, it is {}; divided by its column's sum in the second matrix, {}, it is {}, the top-left entry of the third matrix.", show(m.a[0]), show(m.row_sums[0]), show(m.by_rows[0]), show(m.col_sums[0]), show(m.first_sweep[0])) }</p>
            </div>
            <div class="figure">
                { heat("A", "A, the free positive matrix", &m.a, scale, true) }
                { heat("A, rows", "each row divided by its sum", &m.by_rows, scale, false) }
                { heat("A, one sweep", "then each column by its sum", &m.first_sweep, scale, false) }
                { heat("S", &format!("after {} sweeps: S", m.sweeps), &m.s, scale, false) }
            </div>
            <div class="figure">
                <figure class="flat">
                    { error_chart(m) }
                    <figcaption>{ format!("The row sum error (orange) and the column sum error (blue) before the first sweep and after each. After every sweep the columns sum to 1 but for rounding; the row error falls at every sweep, under {} from sweep {} on. mHC stops at 20 sweeps.", m.tolerance, m.sweeps_to_tolerance as i64) }{ mark(true) }</figcaption>
                </figure>
            </div>
        </section>
    }
}

fn kinds_section(m: &Mhc) -> Html {
    let scale = m.kinds.iter().flat_map(|k| k.matrix.iter()).fold(0.0_f64, |a, v| a.max(v.abs()));
    html! {
        <section class="space" id="kinds">
            <h2>{ "What each kind of matrix does to a signal" }</h2>
            { p(&format!("One signal, three streams of one number each ({}, {}, {}), mixed by each matrix {}, {} and {} times. The four matrices on one color scale (red positive, blue negative); the doubly stochastic and the orthogonal matrices' entries are small on this scale, so read their numbers. The free matrix is the positive A above; the scaled one is A divided by twice its largest singular value ({}), so that value becomes 0.5; the orthogonal one is the orthogonal matrix nearest A.", show(m.signal[0]), show(m.signal[1]), show(m.signal[2]), m.mixes[0], m.mixes[1], m.mixes[2], show(m.kinds[1].singular[0]))) }
            <div class="figure">
                { for m.kinds.iter().enumerate().map(|(k, kind)| heat(&format!("kind {}", k + 1), KINDS[k], &kind.matrix, scale, k == 0)) }
            </div>
            <table class="numbers">
                <thead><tr><th>{ "matrix" }</th><th>{ "before: mean; spread; length" }</th>{ for m.mixes.iter().map(|c| html! { <th>{ format!("after {c} mixes") }</th> }) }<th>{ "length's rate per mix (measured)" }</th><th>{ "largest singular value" }</th></tr></thead>
                <tbody>{ for m.kinds.iter().enumerate().map(|(k, kind)| html! {
                    <tr><td>{ KINDS[k] }</td>{ for kind.signal.iter().map(|r| html! { <td>{ format!("{}; {}; {}", tiny(r[0]), tiny(r[1]), tiny(r[2])) }</td> }) }<td>{ show(m.rates[k]) }</td><td>{ show(kind.singular[0]) }</td></tr>
                }) }</tbody>
            </table>
            <p class="note">{ format!("Each cell: the streams' mean; their spread (the size of their differences from the mean); the signal's length (the square root of the sum of the squares). The rate is (length after {} / length after {}) to the power 1 / {}: for a free matrix it tends to the size of its largest eigenvalue, which its largest singular value bounds from above. The doubly stochastic matrix's second singular value, {}, bounds the factor each mix shrinks the spread by. Measured.", m.mixes[2], m.mixes[1], m.mixes[2] - m.mixes[1], show(m.kinds[0].singular[1])) }{ mark(true) }</p>
        </section>
    }
}

/// The size-2 doubly stochastic matrices as points (top-left entry, top-right entry): a segment between the two permutations.
fn segment(m: &Mhc) -> Html {
    let (w, h) = (260.0, 240.0);
    let at = |a: f64, b: f64| (40.0 + (w - 60.0) * a, h - 30.0 - (h - 50.0) * b);
    let pts: Vec<String> = m.two_by_two.iter().map(|q| { let (x, y) = at(q[0], q[1]); format!("{x:.1},{y:.1}") }).collect();
    let (sx, sy) = at(0.0, 1.0);
    let (ix, iy) = at(1.0, 0.0);
    html! {
        <svg viewBox={format!("0 0 {w} {h}")} role="img" aria-label="the 2 by 2 doubly stochastic matrices drawn by their top-left and top-right entries: a straight segment from the swap (0, 1) to the identity (1, 0)">
            <line class="axis" x1="40" y1={format!("{:.1}", h - 30.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", h - 30.0)} />
            <line class="axis" x1="40" y1="10" x2="40" y2={format!("{:.1}", h - 30.0)} />
            <polyline points={pts.join(" ")} fill="none" stroke="#7048e8" stroke-width="2.5" />
            { for m.two_by_two.iter().map(|q| { let (x, y) = at(q[0], q[1]); html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="3" fill="#7048e8" /> } }) }
            <text class="lbl" x={format!("{:.1}", sx + 6.0)} y={format!("{:.1}", sy + 4.0)}>{ "the swap (p = 0)" }</text>
            <text class="lbl" x={format!("{:.1}", ix - 70.0)} y={format!("{:.1}", iy - 8.0)}>{ "the identity (p = 1)" }</text>
            <text class="axis-lbl" x="44" y="14">{ "top-right entry, 1 - p" }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 120.0)} y={format!("{:.1}", h - 14.0)}>{ "top-left entry, p" }</text>
        </svg>
    }
}

fn polytope_section(m: &Mhc) -> Html {
    let steps = ["-0.1", "0", "+0.1"];
    html! {
        <section class="space" id="polytope">
            <h2>{ "The set itself: a polytope, not a smooth manifold" }</h2>
            { p("Of size 2 a doubly stochastic matrix has rows (p, 1 - p) and (1 - p, p), p from 0 to 1: the identity at p = 1, the swap at p = 0. One free number, so the set is a segment, drawn whole; its two ends are its corners, the two permutation matrices.") }
            <div class="figure">
                <figure class="flat">
                    { segment(m) }
                    <figcaption>{ "The size-2 set, each matrix drawn by its top-left and top-right entries (the other two repeat them): a straight segment with two corners. Measured: every point printed by the program is doubly stochastic." }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    { p("Of size 3 the set has four free numbers and six corners, the six permutation matrices (Birkhoff's theorem: every doubly stochastic matrix is a mix of them). Each is in the set and orthogonal:") }
                    <div class="figure">{ for m.perms.iter().enumerate().map(|(k, pm)| heat(&format!("P{}", k + 1), &format!("permutation {}: distance {}", k + 1, tiny(m.perm_checks[k])), pm, 1.0, false)) }</div>
                </figure>
            </div>
            { p(&format!("Where the set stops: from the identity, step along D = J / 3 - I (its rows and columns sum to 0, so every sum stays 1) and look at the smallest entry; then from the middle (uniform mixing) along E = P - P', P the cyclic shift (stream 1 to 2, 2 to 3, 3 to 1) and P' that shift reversed. The identity has {} entries at 0, each a condition (entry not negative) holding with equality; the middle has {}.", m.zeros[0], m.zeros[1])) }
            <table class="numbers">
                <thead><tr><th>{ "step" }</th>{ for steps.iter().map(|s| html! { <th>{ s.to_string() }</th> }) }</tr></thead>
                <tbody>
                    <tr><td>{ "from the identity along D: smallest entry" }</td>{ for m.corner.iter().map(|v| html! { <td>{ show(*v) }</td> }) }</tr>
                    <tr><td>{ "from the middle along E: smallest entry" }</td>{ for m.middle.iter().map(|v| html! { <td>{ show(*v) }</td> }) }</tr>
                </tbody>
            </table>
            <p class="note">{ "At the identity a step one way leaves the set (an entry goes negative) and the other way stays in; at the middle both ways stay in. This step shows only that the identity is on the edge of the set, where it stops, as any boundary point would. That the identity is a corner, where faces meet at an angle, is Birkhoff's theorem (the permutation matrices are the corners), matching its six entries at 0: enough active conditions to pin it down in a set of four free numbers. A polytope's corners and edges, where flat faces meet at an angle, are why it is not a smooth manifold; the mHC paper's name says \"manifold-constrained\", and this repository calls the set a polytope." }{ mark(true) }</p>
        </section>
    }
}

fn analysis(m: &Mhc) -> Html {
    let rows: Vec<(&str, Vec<Cell>)> = m.kinds.iter().zip(KINDS).map(|(k, name)| (name, vec![
        Cell::measured(tiny(k.signal[3][0])),
        Cell::measured(tiny(k.signal[3][1])),
        Cell::measured(tiny(k.signal[3][2])),
        Cell::measured(k.singular.iter().map(|s| show(*s)).collect::<Vec<_>>().join(", ")),
        Cell::measured(format!("{}, {}", show(k.gain[0]), show(k.gain[1]))),
        Cell::measured(tiny(k.stochastic_off)),
        Cell::measured(tiny(k.orthogonal_off)),
    ])).collect();
    let (ds, fr, or) = (&m.kinds[0], &m.kinds[1], &m.kinds[3]);
    let sentences = vec![
        format!("Under the doubly stochastic matrix the signal's mean was {} before and {} after {} mixes (kept to rounding); its spread went from {} to {} (zero up to rounding).", show(ds.signal[0][0]), show(ds.signal[3][0]), m.mixes[2], show(ds.signal[0][1]), tiny(ds.signal[3][1])),
        format!("Under the free matrix its length went from {} to {} after {} mixes, at a measured rate of {} per mix against a largest singular value of {}.", show(fr.signal[0][2]), tiny(fr.signal[3][2]), m.mixes[2], show(m.rates[1]), show(fr.singular[0])),
        format!("Under the orthogonal matrix its length was {} before and {} after {} mixes; its mean went from {} to {} and its spread from {} to {}.", show(or.signal[0][2]), show(or.signal[3][2]), m.mixes[2], show(or.signal[0][0]), show(or.signal[3][0]), show(or.signal[0][1]), show(or.signal[3][1])),
    ];
    html! { <>
        { comparison(&[&format!("mean after {}", m.mixes[2]), &format!("spread after {}", m.mixes[2]), &format!("length after {}", m.mixes[2]), "singular values", "gain (forward, backward)", "error against doubly stochastic (the largest sum error or negative entry)", "error against orthogonal (the largest entry of H'H - I)"], &rows, &sentences) }
        { for text::ANALYSIS.iter().map(|s| p(s)) }
    </> }
}

fn library_asides() -> Html {
    let aside = |name: &str, purpose: &str, reading: &str| match definition("Mixing", name) {
        Some(d) => own_function("hc:", &d, p(purpose), html! { <p>{ reading.to_string() }</p> }),
        None => html! {},
    };
    html! { <>
        // hc:l_ayer's aside waits on ask GM10 (its parameter f_ is linked as a built-in); the table below lists it with its doc.
        { aside("b_lend", "The path from the identity to uniform mixing, one matrix per t.", "(1 - t) times the identity plus t / n times the matrix of ones.") }
        { aside("s_tochasticOff", "How far a matrix is from the doubly stochastic ones: the largest of the row sum error, the column sum error and the most negative entry's size.", "The largest of s_umErrors and n_egativity.") }
        { aside("g_ain", "The gain the mHC paper measures: the largest sum of the sizes of a row's entries, and of a column's.", "The sizes of the entries summed along each row and each column; the largest of each.") }
        { aside("s_pread", "The size of the streams' differences from their mean: what a doubly stochastic matrix shrinks.", "The differences (each stream less the mean), squared, summed, square-rooted.") }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_memo((), |_| run());
    // Part 2 runs after the first paint, so the reader has part 1 while
    // the stacks are measured and the page trains again; nothing of part
    // 1 waits on it.
    let deep = use_state(|| None::<Result<Depth, String>>);
    {
        let deep = deep.clone();
        use_effect_with((), move |_| {
            let timer = Timeout::new(60, move || deep.set(Some(depth_run())));
            move || drop(timer)
        });
    }
    let chosen = use_state(|| 5usize);
    // The depth shown and the one shown before it, so a change is narrated from both.
    let depth = use_state(|| (3usize, 3usize));
    let src = program();
    let src2 = depth_program();
    let linker = Linker::of(&src);
    let linker2 = Linker::of(&src2);
    let pick_depth = { let d = depth.clone(); Callback::from(move |k: usize| { let (now, _) = *d; d.set((k, now)); }) };
    let pick = { let c = chosen.clone(); Callback::from(move |k: usize| c.set(k)) };
    let (body, after) = match &*model {
        Err(e) => (html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> }, html! {}),
        Ok(m) => (html! { <>
            { streams(m) }
            { path_section(m, (*chosen).min(m.path.len() - 1), pick) }
            { sinkhorn_section(m) }
            { kinds_section(m) }
            { polytope_section(m) }
        </> }, analysis(m)),
    };
    let (di, before) = *depth;
    let body2 = match &*deep {
        None => html! { <section class="panel"><p class="notice" role="status">{ "Part 2 is running: four stacks measured at four depths from three seeds, every gradient checked, and the first steps of training done again, X_eTaL in your browser; several seconds. Everything above is part 1, finished." }</p></section> },
        Some(Err(e)) => html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped in part 2: {e}") }</p></section> },
        Some(Ok(t)) => html! { <>
            { stack::network(t) }
            { stack::start_section(t, di, before, pick_depth.clone()) }
            { stack::training_section(t, di, before, pick_depth.clone()) }
        </> },
    };
    let after2 = match &*deep {
        Some(Ok(t)) => stack::analysis(t),
        _ => html! { <p class="notice" role="status">{ "Part 2's comparison follows when it has run." }</p> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("mhc-explorer".into())}>
        <header>{ header("mHC explorer: what may the mixing do?", text::LEDE) }</header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ prose(text::HINT) }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>{ for text::BACKGROUND.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Rationale, "The idea: one matrix at a time, then the stack", html! { <>{ for text::RATIONALE.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ for text::PURPOSE.iter().map(|s| p(s)) }</> }) }
            { frame(INTUITION, "The intuition: pouring water between cups", html! { <>{ for text::INTUITION.iter().map(|s| p(s)) }</> }) }
            { frame(MATHEMATICS, "The mathematics: the layer, the count, the path, Sinkhorn, the powers, the kinds and their gradients", html! { <>{ for text::MATHEMATICS.iter().map(|s| p(s)) }</> }) }
            { frame(EXPERIMENT, "The experiment: the matrices one at a time, then four kinds in a stack", html! { <>{ for text::EXPERIMENT.iter().map(|s| p(s)) }</> }) }
            { frame(RUN, "The run: nothing trained in part 1; 48 trainings in part 2", html! { <>{ for text::RUN.iter().map(|s| p(s)) }</> }) }
            { body }
            { body2 }
            { frame(ANALYSIS, "The analysis: four matrices and one signal, then four kinds of mixing in a stack", html! { <>{ after }{ after2 }</> }) }
            { frame(EXTENSION, "Beyond this page", html! { <>{ for text::EXTENSION.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Summary, "What this page showed", html! { <>{ for text::SUMMARY.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Next, "What is next", html! { <ul>{ for text::NEXT.iter().map(|s| li(s)) }</ul> }) }
            <section class="panel">
                <h2>{ "Every idea on this page, explained simply" }</h2>
                <p class="note">{ "The same explanations the underlined words open, in one place. Open any of them." }</p>
                { index(&texts) }
            </section>
            <section class="panel code" id="code">
                <h2>{ "The code, for those who want it" }</h2>
                { for text::CODE.iter().map(|s| p(s)) }
                <h3>{ "The library functions behind the numbers" }</h3>
                { library_asides() }
                <h3>{ "Every library function the program calls" }</h3>
                { functions(&src) }
                <h3>{ "Every library function part 2 calls" }</h3>
                { functions(&src2) }
                <h3>{ "The program, part 1" }</h3>
                { linker.block(&src) }
                <h3>{ "The preset it reads" }</h3>
                <pre class="preset">{ PRESET }</pre>
                <p class="links"><a href="../doc/demos-mhc-explorer-mhc-explorer.xtl.html">{ "This program in the cross-reference" }</a></p>
                <h3>{ "The program, part 2" }</h3>
                { linker2.block(&src2) }
                <h3>{ "The preset it reads" }</h3>
                <pre class="preset">{ DEPTH_PRESET }</pre>
                <h3>{ "The records it reads, and what made them" }</h3>
                <p class="note">{ "The records are 16 files, data/KIND-L.txt, a line per seed of 410 numbers (the loss before each step and after the last, the gradient's size at each step, then the end of the run), written by train/train.sh: mhc-depth.xtl up to the end of its model, followed by one u:r_un line per seed. They are passed to the program where it reads them and are not printed here, being numbers only. Beside them, data/runs.txt says what made them: the steps and seeds, the pins, and the hashes of the program's model, of Mixing and of the preset." }</p>
                <pre class="preset">{ RUNS }</pre>
                <p class="links"><a href="../doc/demos-mhc-explorer-mhc-depth.xtl.html">{ "This program in the cross-reference" }</a></p>
            </section>
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
