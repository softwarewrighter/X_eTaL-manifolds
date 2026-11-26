//! The views of part 2: the network; the stack at the start drawn as a
//! strip per layer for each kind, colored by the signal's size, the
//! gradient's size and the streams' spread on one scale, a depth picked
//! by the reader and the change narrated from measured values; the
//! constraint's violation; training by depth (loss and gradient size by
//! step, the end over the seeds, the cost); the H2 table; the analysis.
//! Presentation only: every number is one the program printed.

use yew::prelude::*;

use lesson::eli5::p;
use lesson::narrate;
use site::{comparison, mark, Cell};

use crate::depth::{Depth, Entry, Three, KINDS};

const COLORS: [&str; 4] = ["#868e96", "#e8590c", "#1c7ed6", "#2b8a3e"];
const SHORT: [&str; 4] = ["plain", "free", "doubly stochastic", "orthogonal"];

fn show(x: f64) -> String {
    format!("{x:.4}")
}

fn tiny(x: f64) -> String {
    if x.is_nan() {
        "not a number".into()
    } else if x == 0.0 {
        "0".into()
    } else if x.abs() < 0.001 || x.abs() >= 100000.0 {
        format!("{x:.2e}")
    } else {
        format!("{x:.4}")
    }
}

fn spread3(t: Three) -> String {
    if t[0] < 0.0 { "no seed finite".into() } else { format!("{} ({} to {})", tiny(t[0]), tiny(t[1]), tiny(t[2])) }
}

fn acc3(t: Three) -> String {
    if t[0] < 0.0 { "no seed finite".into() } else { format!("{:.3} ({:.3} to {:.3})", t[0], t[1], t[2]) }
}

/// A ratio's color on one logarithmic scale shared by the whole figure: white at 1, red deeper toward 1000 times, blue deeper toward 1/100.
fn color(r: f64) -> String {
    if !r.is_finite() || r <= 0.0 {
        return "#adb5bd".into();
    }
    let x = r.log10();
    let (a, cold) = if x >= 0.0 { ((x / 3.0).min(1.0), false) } else { ((-x / 2.0).min(1.0), true) };
    let fade = (255.0 * (1.0 - 0.85 * a)) as u8;
    if cold { format!("rgb({fade},{fade},255)") } else { format!("rgb(255,{fade},{fade})") }
}

pub fn depth_buttons(t: &Depth, chosen: usize, pick: Callback<usize>) -> Html {
    html! {
        <div class="controls" role="group" aria-label="the depth of the stack: the number of layers">
            { for t.preset.depths.iter().enumerate().map(|(k, l)| {
                let pick = pick.clone();
                html! { <button class={classes!(chosen.eq(&k).then_some("on"))} onclick={Callback::from(move |_| pick.emit(k))} aria-pressed={if chosen == k { "true" } else { "false" }}>{ format!("{l} layers") }</button> }
            }) }
        </div>
    }
}

pub fn network(t: &Depth) -> Html {
    let pr = &t.preset;
    html! {
        <section class="space" id="network">
            <h2>{ "Part 2: from one matrix to a stack of layers" }</h2>
            { p(&format!("A small network: each example is a point of the plane (two numbers) from three seeded clouds of points, one per class ({} classes, the repository's kit of data sets); it goes into {} streams of {} numbers each, each stream by its own trained weights; then through L layers, each the layer of part 1 over the streams (the read r = a' X, the layer's output F = tanh(r W + beta) of a small dense layer, the streams mixed by H and the output written back with weights b); then read by its own weights into {} scores, turned into probabilities by a softmax (as in a [logistic classifier|logistic-classifier]), scored by the [cross-entropy|cross-entropy]. {} points: {} trained on, {} [held out|held-out].", pr.k, pr.n, pr.d, pr.k, pr.k * pr.per, t.trained_on, t.held_out)) }
            { p(&format!("Four kinds of mixing, each H made from the same n by n trained numbers T of its layer, so that T = 0 is the identity or as near it as the kind allows: the **plain residual** (H is the identity, T unused); **free** mixing, H = I + T ([hyper-connections|hyper-connections]); **doubly stochastic** mixing, {} sweeps of [Sinkhorn's iteration|sinkhorn] on exp(z I + T) with z = {}, a pull toward the identity ([mHC|mhc]; at T = 0 the matrix is already doubly stochastic, each diagonal entry e^z / (e^z + {}), as near the identity as a positive matrix gets with this pull); and **orthogonal** mixing, the [Cayley transform|cayley-transform] (I - K)^-1 (I + K) of K = (T - T') / 2. Each layer has {} trained numbers: T ({}), W ({}), beta ({}), a and b ({} each, trained, starting at 1/{} and 1); the read-in ({} numbers) and the read-out ({} weights and {} constants, with {} more for the final read of the streams) add {}. The plain residual does not use its T, so it trains {} fewer per layer. The kinds also differ in how many independent ways their H can change (free directions): free mixing all {}; doubly stochastic {} ((n - 1)^2 at n = {}, part 1's formula: adding a number to a whole row or column of T changes nothing after Sinkhorn); orthogonal {} (only T's skew part K counts).", pr.sweeps, pr.pull, pr.n - 1, t.per_layer, pr.n * pr.n, pr.d * pr.d, pr.d, pr.n, pr.n, 2 * pr.n * pr.d, pr.d * pr.k, pr.k, pr.n, t.outside, pr.n * pr.n, pr.n * pr.n, (pr.n - 1) * (pr.n - 1), pr.n, pr.n * (pr.n - 1) / 2)) }
            { p(&format!("The start, for each of {} seeds: every T drawn at random with standard deviation {} (the same numbers for every kind, so the four stacks start from the same weights and differ only in how H is made from them), every W with standard deviation {} / sqrt({}), beta = 0, a = 1/{} (the read takes the mean), b = 1 (the output is written to every stream). The layers of a shallow stack are the first layers of the deeper ones. Training: plain gradient descent (the step of X_eTaL-ML's Optim library) at the rate {} on all the training rows, {} steps, the same for every kind and depth.", pr.seeds, pr.mix_spread, pr.weight_scale, pr.d, pr.n, pr.rate, pr.steps)) }
            <div class="worked-out">
                <h3>{ "The gradient, checked" }{ mark(true) }</h3>
                <p>{ format!("Training follows the gradient of the loss through every layer, written out by hand in Mixing (the layer's own derivative and each kind of mixing's: through the {} sweeps for the doubly stochastic kind, through the Cayley transform for the orthogonal one). Ten of the trained numbers of a {}-layer stack, one from each part of it, were each moved a millionth up and down and the loss's change compared with the gradient: the largest gap over the largest gradient was {} for the plain residual, {} free, {} doubly stochastic, {} orthogonal.", pr.sweeps, pr.depths[0], tiny(t.gradient_gaps[0]), tiny(t.gradient_gaps[1]), tiny(t.gradient_gaps[2]), tiny(t.gradient_gaps[3])) }</p>
            </div>
        </section>
    }
}

/// The stack at the start for one depth: four kinds side by side, three columns each (signal, gradient, spread), a row per state from the bottom (the streams going in) to the top.
fn strips(t: &Depth, di: usize) -> Html {
    let l = t.preset.depths[di];
    let rows = l + 1;
    let (w, cell_w, gap) = (900.0, 64.0, 32.0);
    let cell_h = (380.0 / rows as f64).min(30.0);
    let h = cell_h * rows as f64 + 60.0;
    let group = 3.0 * cell_w + gap;
    html! {
        <svg viewBox={format!("0 0 {w} {h:.0}")} role="img" aria-label={format!("the {l}-layer stack at the start for each kind of mixing, a row per state from the bottom up, three columns per kind: the signal's size over its size going in, the gradient's size over its size at the top, the streams' spread over theirs going in; white is 1, red larger, blue smaller, on one logarithmic scale")}>
            { for t.cells.iter().enumerate().map(|(k, row)| {
                let c = &row[di].start;
                let x0 = 12.0 + group * k as f64;
                let cols: [Vec<f64>; 3] = [
                    c.sizes.iter().map(|s| s / c.sizes[0]).collect(),
                    c.gradients.iter().map(|g| g / c.gradients[l]).collect(),
                    c.spreads.iter().map(|s| s / c.spreads[0]).collect(),
                ];
                html! { <g>
                    <text class="lbl" x={format!("{x0:.1}")} y="14">{ SHORT[k] }</text>
                    { for ["signal", "gradient", "spread"].iter().enumerate().map(|(q, name)| html! {
                        <text class="axis-lbl" x={format!("{:.1}", x0 + cell_w * q as f64)} y={format!("{:.1}", h - 8.0)}>{ name.to_string() }</text>
                    }) }
                    { for cols.iter().enumerate().flat_map(|(q, col)| col.iter().enumerate().map(move |(i, r)| {
                        let y = 24.0 + cell_h * (rows - 1 - i) as f64;
                        let what = ["signal size over the size going in", "gradient size over the size at the top", "spread over the spread going in"][q];
                        html! { <rect x={format!("{:.1}", x0 + cell_w * q as f64)} y={format!("{y:.1}")} width={format!("{:.1}", cell_w - 2.0)} height={format!("{:.1}", (cell_h - 1.0).max(1.0))} fill={color(*r)}><title>{ format!("{}, after layer {i} of {l}: {what}, {}", SHORT[k], tiny(*r)) }</title></rect> }
                    }).collect::<Vec<_>>()) }
                </g> }
            }) }
        </svg>
    }
}

fn start_rows(t: &Depth, di: usize) -> Vec<(&'static str, Vec<Cell>)> {
    t.cells.iter().zip(KINDS).map(|(row, name)| {
        let c = &row[di].start;
        (name, vec![
            Cell::measured(spread3(c.up)),
            Cell::measured(spread3(c.down)),
            Cell::measured(spread3(c.spread)),
            Cell::measured(format!("{}, {}", show(c.gain[0]), show(c.gain[1]))),
            Cell::measured(tiny(c.off[0])),
            Cell::measured(tiny(c.off[1])),
        ])
    }).collect()
}

pub fn start_section(t: &Depth, di: usize, before: usize, pick: Callback<usize>) -> Html {
    let l = t.preset.depths[di];
    let lb = t.preset.depths[before];
    let sentences: Vec<String> = if before == di {
        vec![format!("Pick another depth: the sentences here then say what changed, from the depth shown before to the one picked, in measured values.")]
    } else {
        (0..4).flat_map(|k| {
            let (a, b) = (&t.cells[k][before].start, &t.cells[k][di].start);
            [
                narrate::moved(&format!("Under {} mixing the signal's growth up the stack (mean over the seeds), from {lb} layers to {l}", SHORT[k]), a.up[0], b.up[0]),
                narrate::moved(&format!("and the gradient's growth down it"), a.down[0], b.down[0]),
            ]
        }).collect()
    };
    html! {
        <section class="space" id="start">
            <h2>{ "At the start of training: what the stack does to a signal, layer by layer" }</h2>
            { p("Before any training step, one pass over the training rows. Going up: the size of the streams after each layer (the square root of the sum of the squares of all their numbers, per example). Coming down: the size of the gradient of the loss with respect to the streams at each state, the training signal on its way back ([signal propagation|signal-propagation]). And the streams' spread: how much the streams still differ from their mean, the channel part 1 showed doubly stochastic mixing shrinking. Pick a depth.") }
            { depth_buttons(t, di, pick) }
            <div class="figure">
                <figure class="flat">
                    { strips(t, di) }
                    <figcaption>{ format!("The {l}-layer stack at the start, first seed, a row per state from the streams going in (bottom) to the top: for each kind three columns, the signal's size over its size going in, the gradient's size over its size at the top, and the spread over the spread going in. White is 1 (unchanged); red is larger, deepest at 1000 times; blue is smaller, deepest at 1/100; one scale for every column and kind. Hover or tap a cell for its value. The table below gives the top against the bottom for every seed (mean, smallest to largest), all measured in your browser when part 2 ran.") }{ mark(true) }</figcaption>
                </figure>
            </div>
            { comparison(&["signal growth up the stack: top over bottom, mean (smallest to largest) over the seeds", "gradient growth down the stack: bottom over top", "spread kept: top over bottom", "composite gain (forward, backward): the product of the layers' H, as mHC measures it", "largest distance of an H from doubly stochastic", "largest distance of an H from orthogonal"], &start_rows(t, di), &sentences) }

            <p class="note">{ format!("The plain residual's stack grows too: each layer adds its output to every stream, so the signal grows up the stack and the gradient down it in every kind, by amounts H changes. The plain residual keeps the spread exactly (its H is the identity and it writes the same F to every stream, so the differences are untouched). An orthogonal H keeps the streams' length but not their mean, so size can move from the mean into the differences: its spread grows. The composite gain is the mixing's part alone (forward, the largest row sum of sizes of the product of the {l} matrices H); the growth columns are the whole stack's. The doubly stochastic H are within {} of the set: from T's random start, {} sweeps on a matrix this near the identity do not finish the iteration (part 1's matrix, far from the identity, converged).", tiny(t.cells[2][di].start.off[0]), t.preset.sweeps) }{ mark(true) }</p>
            <p class="note">{ format!("The doubly stochastic stack and the plain residual are the same network at the start, exactly: a matrix whose columns sum to 1 keeps the streams' mean; the read takes the mean, the output is written to every stream alike, and the read-out takes the mean too; so the loss sees only the mean, which follows the plain residual's path. Their gradients' growth agrees to rounding ({} and {} at {l} layers) and their first losses are equal. Training makes a, b and the read-out unequal across the streams, and the two part only slightly: this limits what comparing them can show.", show(t.cells[0][di].start.down[0]), show(t.cells[2][di].start.down[0])) }</p>
        </section>
    }
}

/// Loss or gradient size by step for every kind and seed at one depth, on one logarithmic scale; a number that is not finite ends its line.
fn by_step(t: &Depth, di: usize, gradient: bool) -> Html {
    let (w, hh) = (440.0, 240.0);
    let steps = t.preset.steps;
    let (lo, hi) = if gradient { (-3.0, 6.0) } else { (-2.0, 2.0) };
    let at = |k: usize, v: f64| (40.0 + (w - 50.0) * k as f64 / steps as f64, hh - 24.0 - (hh - 40.0) * (v.log10().clamp(lo, hi) - lo) / (hi - lo));
    let what = if gradient { "the size of the gradient of every trained number" } else { "the training loss" };
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label={format!("{what} by step for the four kinds at {} layers, every seed a line, on a logarithmic scale; plain gray, free orange, doubly stochastic blue, orthogonal green", t.preset.depths[di])}>
            <line class="axis" x1="40" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="40" y1="14" x2="40" y2={format!("{:.1}", hh - 24.0)} />
            <text class="axis-lbl" x="44" y="12">{ format!("{what}, 1e{hi} at the top to 1e{lo} at the bottom (log scale)") }</text>
            <text class="axis-lbl" x="40" y={format!("{:.1}", hh - 8.0)}>{ "step 0" }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("step {steps}") }</text>
            { for t.cells.iter().enumerate().flat_map(|(k, row)| row[di].records.iter().map(move |r| {
                let vals = if gradient { &r.gradients } else { &r.losses };
                let d: Vec<String> = vals.iter().take_while(|v| v.is_finite()).enumerate().map(|(i, v)| { let (x, y) = at(i, *v); format!("{x:.1},{y:.1}") }).collect();
                html! { <polyline points={d.join(" ")} fill="none" stroke={COLORS[k]} stroke-width="1.6" opacity="0.85" /> }
            }).collect::<Vec<_>>()) }
        </svg>
    }
}

fn after_rows(t: &Depth, di: usize) -> Vec<(&'static str, Vec<Cell>)> {
    t.cells.iter().zip(KINDS).map(|(row, name)| {
        let e: &Entry = &row[di];
        let a = &e.after;
        (name, vec![
            Cell::measured(format!("{} of {} ({} finite)", a.trained, t.preset.seeds, a.finite)),
            Cell::measured(spread3(a.loss)),
            Cell::measured(acc3(a.held)),
            Cell::measured(if a.finite == 0 { "no seed finite".into() } else { tiny(a.largest_gradient) }),
            Cell::measured(if a.finite == 0 { "no seed finite".into() } else { tiny(a.spread) }),
            Cell::analytical(format!("{}", e.cost as i64)),
        ])
    }).collect()
}

pub fn training_section(t: &Depth, di: usize, before: usize, pick: Callback<usize>) -> Html {
    let pr = &t.preset;
    let l = pr.depths[di];
    let lb = pr.depths[before];
    let sentences: Vec<String> = if before == di {
        vec!["Pick another depth: the sentences here then say what changed, in measured values.".to_string()]
    } else {
        (0..4).map(|k| narrate::count(&format!("Under {} mixing the seeds trained, from {lb} layers to {l},", SHORT[k]), t.cells[k][before].after.trained, t.cells[k][di].after.trained, None)).collect()
    };
    html! {
        <section class="space" id="training">
            <h2>{ "Trained: the same task at four depths" }</h2>
            { p(&format!("Each kind at each depth trained from each seed's start for {} steps, offline by the same program (train/train.sh runs mhc-depth.xtl up to its model, then one training per seed; the records are in data/, read back by the page). A seed counts as trained when every number stayed finite and its loss at the end is below its loss at the start. The same depth control:", pr.steps)) }
            { depth_buttons(t, di, pick) }
            <div class="figure">
                <figure class="flat">
                    { by_step(t, di, false) }
                    <figcaption>{ format!("The training loss by step at {l} layers, every seed a line: plain gray, free orange, doubly stochastic blue, orthogonal green. A loss flat near 27.6, below the top of the scale, is a run whose predictions saturated (a probability that rounds to 0 is counted as 1e-12, whose cross-entropy is about 27.6).") }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    { by_step(t, di, true) }
                    <figcaption>{ format!("The size of the gradient of every trained number at each step, {l} layers. A line that stops early is a run whose gradient stopped being a number (it overflowed); nothing after that step is finite.") }{ mark(true) }</figcaption>
                </figure>
            </div>
            { comparison(&["seeds trained", "loss at the end, mean (smallest to largest) over the finite seeds", "held-out accuracy, the same", "largest gradient size met", "spread kept at the end: top over bottom", "multiplications per step (counted)"], &after_rows(t, di), &sentences) }
            <p class="note">{ format!("{} held-out points, so one point is 1/{} of the accuracy. The cost is counted, not timed: every layer's read, dense layer and write, forward and back (a product's backward counted as two of its forward); the read-in and read-out, the same for every kind, are left out; the mixing's product H X for every kind but the plain residual; making H, nothing for free mixing, {} sweeps forward and back for doubly stochastic, a solve of {} by {} forward and back for orthogonal. At this width ({} streams of {} numbers) the product H X costs four times the dense layer (n^2 d against d^2 multiplications per example); in a real network the width d is in the thousands and it is small, while Sinkhorn's cost does not grow with the width or the batch.", t.held_out, t.held_out, pr.sweeps, pr.n, pr.n, pr.n, pr.d) }</p>
            <div class="worked-out">
                <h3>{ "The page trained again" }{ mark(true) }</h3>
                <p>{ format!("When this page opened, your browser trained each kind for the first {} steps at {} layers from the first seed, as the offline run did, and compared: the largest gap between its losses and the record's was {}. The records the page reads are what this program computes. Their first and last loss here, by kind: {}. The starting losses differ by kind although the weights are the same, because each kind makes a different H from the same T; the plain residual's and the doubly stochastic one's are equal, as said above.", pr.page_steps, pr.depths[0], tiny(t.page_gap), t.page_losses.iter().enumerate().map(|(k, r)| format!("{} {} to {}", SHORT[k], show(r[0]), show(r[pr.page_steps]))).collect::<Vec<_>>().join("; ")) }</p>
            </div>
        </section>
    }
}

/// The H2 table: every kind at every depth.
pub fn h2_table(t: &Depth) -> Html {
    let names: Vec<String> = (0..4).flat_map(|k| t.preset.depths.iter().map(move |l| format!("{}, {} layers", SHORT[k], l))).collect();
    let rows: Vec<(&str, Vec<Cell>)> = names.iter().enumerate().map(|(j, name)| {
        let e = &t.cells[j / 4][j % 4];
        (name.as_str(), vec![
            Cell::measured(tiny(e.start.up[0])),
            Cell::measured(tiny(e.start.down[0])),
            Cell::measured(tiny(e.start.spread[0])),
            Cell::measured(show(e.start.gain[0])),
            Cell::measured(format!("{} of {} ({} finite)", e.after.trained, t.preset.seeds, e.after.finite)),
            Cell::measured(spread3(e.after.loss)),
            Cell::measured(acc3(e.after.held)),
            Cell::measured(if e.after.finite == 0 { "no seed finite".into() } else { tiny(e.after.spread) }),
            Cell::analytical(format!("{}", e.cost as i64)),
        ])
    }).collect();
    comparison(&["signal growth at the start (mean)", "gradient growth at the start (mean)", "spread kept at the start", "composite gain at the start (forward, of the product of the H)", "seeds trained", "loss at the end, mean (smallest to largest)", "held-out accuracy, the same", "spread kept at the end", "multiplications per step"], &rows, &[])
}

pub fn analysis(t: &Depth) -> Html {
    let top = 3;
    let lt = t.preset.depths[top];
    let c = |k: usize| &t.cells[k][top];
    let sentences = vec![
        format!("At {lt} layers the composite gain of the free mixing at the start was {} (mean over the seeds), of the doubly stochastic {} and of the orthogonal {}; the free stack's signal grew {} times up the stack against the plain residual's {}.", show(c(1).start.gain[0]), show(c(2).start.gain[0]), show(c(3).start.gain[0]), spread3(c(1).start.up), spread3(c(0).start.up)),
        format!("At {lt} layers the seeds trained were {} of {} for the free mixing, {} for the plain residual, {} for the doubly stochastic and {} for the orthogonal.", c(1).after.trained, t.preset.seeds, c(0).after.trained, c(2).after.trained, c(3).after.trained),
        format!("At {lt} layers the held-out accuracy was {} for the plain residual, {} doubly stochastic and {} orthogonal; the doubly stochastic stack kept {} of its streams' spread at the top at the end, the orthogonal {}.", acc3(c(0).after.held), acc3(c(2).after.held), acc3(c(3).after.held), tiny(c(2).after.spread), tiny(c(3).after.spread)),
    ];
    let gains = |k: usize| t.cells[k].iter().map(|e| show(e.start.gain[0])).collect::<Vec<_>>().join(", ");
    let depths = t.preset.depths.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", ");
    let cost = |k: usize| c(k).cost / c(1).cost;
    let o8 = &t.cells[3][1].after.held;
    let p8 = &t.cells[0][1].after.held;
    let reading = [
        format!("Against what was written before the run, the first half: the free mixing's composite gain at the start was {} at {depths} layers, past 10 at 32 as the count for H2 asked; the doubly stochastic one's {} and the orthogonal one's {}, inside 0.5 to 2 at every depth. That counts for H2's first half at these depths. The free growth is set by the start chosen before the run (T drawn with standard deviation {}); a smaller start would grow less, and was not tried.", gains(1), gains(2), gains(3), t.preset.mix_spread),
        format!("The second half, against free mixing: at {} layers {} of {} free seeds stayed finite, at {lt} layers {}; every constrained seed trained, and a step cost {:.2} times the free one's under doubly stochastic mixing and {:.2} times under orthogonal, within the twice written down. The criterion written before the run compared losses and accuracies; it did not foresee runs that stop being finite, and counting them as not trained is a reading made after the run. One rate served all; a smaller rate for the free stack was not tried.", t.preset.depths[2], t.cells[1][2].after.finite, t.preset.seeds, c(1).after.finite, cost(2), cost(3)),
        format!("Against the plain residual, which has no mixing to hold: at {} layers the orthogonal stack's held-out accuracy, {}, lies above the plain residual's whole range, {}, by one held-out point over its best seed; at the other depths the ranges overlap. At {lt} layers the orthogonal end loss, {}, lies at the low end of the plain residual's range, {}; the doubly stochastic stack ended with the highest mean loss of the finite stacks there, {}, its ranges overlapping the plain residual's. The plain residual's held-out accuracy fell with depth, from {} at {} layers to {} at {lt}, which this page does not explain. On a task solved at {} layers, these differences are a point or two of 18; whether a constraint helps a stack that needs its depth is not tested here.", t.preset.depths[1], acc3(*o8), acc3(*p8), spread3(c(3).after.loss), spread3(c(0).after.loss), spread3(c(2).after.loss), format!("{:.3}", t.cells[0][0].after.held[0]), t.preset.depths[0], format!("{:.3}", c(0).after.held[0]), t.preset.depths[0]),
        format!("The count against H2 on collapse: the doubly stochastic stack shrank its streams' spread with depth, to {} of the start at the top of {lt} layers at the start and {} at the end of training, the difference channel fading as the Birkhoff-geometry paper predicts; its held-out accuracy at {lt} layers, {}, lies within the plain residual's range {}. The collapse is measured; a loss of accuracy beyond the seeds' range is not. The orthogonal stack kept and enlarged its spread ({} at the end).", tiny(c(2).start.spread[0]), tiny(c(2).after.spread), acc3(c(2).after.held), acc3(c(0).after.held), tiny(c(3).after.spread)),
    ];
    html! { <>
        <h3>{ "Part 2: the H2 table, every kind at every depth" }</h3>
        { h2_table(t) }
        { for sentences.iter().map(|s| html! { <p class="narrated">{ s.clone() }</p> }) }
        { for reading.iter().map(|s| p(s)) }
        { for crate::text::ANALYSIS2.iter().map(|s| p(s)) }
    </> }
}
