//! The views of part 2: the data in both scales, the three loss curves
//! on one scale for each data set, the record and the cost, the plain
//! rate sweep, the Fisher's eigenvalues, one change of the weights
//! measured two ways, the invariance check, and the comparison.
//! Presentation only: every number is one the program printed.

use yew::prelude::*;

use lesson::eli5::p;
use lesson::narrate;
use site::{comparison, mark, Cell};

use crate::training::{Run, Training, METHODS};

const COLORS: [&str; 3] = ["#868e96", "#1c7ed6", "#2b8a3e"];
const DASH: [Option<&str>; 3] = [None, Some("5 3"), None];
const CLASS: [&str; 3] = ["#e8590c", "#2b8a3e", "#7048e8"];

fn show(x: f64) -> String {
    format!("{x:.4}")
}

fn tiny(x: f64) -> String {
    if x == 0.0 {
        "0".into()
    } else if x.abs() < 0.001 || x.abs() >= 10000.0 {
        format!("{x:.2e}")
    } else {
        format!("{x:.4}")
    }
}

fn count(x: f64) -> String {
    if x < 0.0 { "not reached".into() } else { format!("{}", x as i64) }
}

fn stays(x: f64) -> String {
    if x < 0.0 { "not under at the end".into() } else { format!("{}", x as i64) }
}

/// The data, as they are and with the first input times the scale (the same points: the picture's x axis is relabeled).
fn data_picture(t: &Training, bad: bool) -> Html {
    let (w, h) = (300.0, 240.0);
    let (lo, hi) = t.x.iter().fold(([f64::MAX; 2], [f64::MIN; 2]), |(lo, hi), q| ([lo[0].min(q[0]), lo[1].min(q[1])], [hi[0].max(q[0]), hi[1].max(q[1])]));
    let s = if bad { t.preset.bad_scale } else { 1.0 };
    // One unit of x1 and of x2 drawn at the same length, so the badly scaled picture is as wide as the scale makes it; its width is clipped to the box and said.
    let span = (hi[0] - lo[0]).max(hi[1] - lo[1]) * 1.1;
    let unit = (h - 30.0) / span;
    let at = |q: [f64; 2]| (20.0 + (q[0] * s - lo[0] * s) * unit, h - 15.0 - (q[1] - lo[1]) * unit);
    let width = (hi[0] - lo[0]) * s * unit;
    html! {
        <svg viewBox={format!("0 0 {w} {h}")} role="img" aria-label={if bad { format!("the flat clusters with the first input times {}: the same points stretched along x1 so far that one unit of x2 is a sliver; most of the picture runs off to the right", t.preset.bad_scale) } else { "the three flat clusters as they are, x1 across and x2 up, colored by class, the held-out points ringed".to_string() }}>
            <line class="axis" x1="20" y1={format!("{:.1}", h - 15.0)} x2={format!("{w}")} y2={format!("{:.1}", h - 15.0)} />
            <line class="axis" x1="20" y1="0" x2="20" y2={format!("{:.1}", h - 15.0)} />
            { for t.x.iter().enumerate().map(|(i, q)| {
                let (x, y) = at(*q);
                let ring = t.held.contains(&(i + 1));
                html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r={if ring { "3.5" } else { "2.5" }} fill={if ring { "none" } else { CLASS[t.y[i] - 1] }} stroke={CLASS[t.y[i] - 1]} stroke-width="1.2" /> }
            }) }
            <text class="axis-lbl" x="24" y="12">{ if bad { format!("x1 times {}, drawn to the same unit as x2: {:.0} units of picture wide, {:.0} shown", t.preset.bad_scale, width, w - 20.0) } else { "x1 across, x2 up, one unit the same on both".to_string() } }</text>
        </svg>
    }
}

/// The three runs' training loss by step on one logarithmic scale; losses above the top are drawn at the top.
fn loss_chart(t: &Training, runs: &[Run; 3], label: &str) -> Html {
    let (w, hh) = (420.0, 230.0);
    let n = t.preset.steps + 1;
    let (lo, hi) = (0.1_f64.log10(), 100.0_f64.log10());
    let at = |k: usize, l: f64| {
        let x = 34.0 + (w - 44.0) * k as f64 / (n - 1) as f64;
        let y = hh - 24.0 - (hh - 36.0) * ((l.max(0.1).min(100.0)).log10() - lo) / (hi - lo);
        (x, y)
    };
    let y_t = at(0, t.preset.target).1;
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label={format!("{label}: the training loss by step of the three methods on one logarithmic scale from 0.1 to 100, plain gray, diagonal blue dashed, natural green, with the target as a line")}>
            <line class="axis" x1="34" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="34" y1="12" x2="34" y2={format!("{:.1}", hh - 24.0)} />
            <line class="chord" x1="34" y1={format!("{y_t:.1}")} x2={format!("{:.1}", w - 10.0)} y2={format!("{y_t:.1}")} />
            <text class="axis-lbl" x="38" y={format!("{:.1}", y_t - 3.0)}>{ format!("loss {}, the target", t.preset.target) }</text>
            <text class="axis-lbl" x="38" y="12">{ "training loss, 100 at the top to 0.1 at the bottom (log scale)" }</text>
            <text class="axis-lbl" x="34" y={format!("{:.1}", hh - 8.0)}>{ "step 0" }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("step {}", n - 1) }</text>
            { for runs.iter().enumerate().map(|(m, r)| {
                let d: Vec<String> = r.log.iter().enumerate().map(|(k, row)| { let (x, y) = at(k, row.loss); format!("{x:.1},{y:.1}") }).collect();
                html! { <polyline points={d.join(" ")} fill="none" stroke={COLORS[m]} stroke-width="2" stroke-dasharray={DASH[m].map(|s| s.to_string())} /> }
            }) }
        </svg>
    }
}

fn record_rows(runs: &[Run; 3]) -> Vec<(&'static str, Vec<Cell>)> {
    runs.iter().zip(METHODS).map(|(r, name)| {
        let c = r.record;
        (name, vec![
            Cell::measured(count(c.first_under)),
            Cell::measured(stays(c.stays_under)),
            Cell::measured(tiny(c.end_loss)),
            Cell::measured(format!("{:.3}", c.end_accuracy)),
            Cell::measured(tiny(c.mean_update)),
            Cell::measured(tiny(c.largest_update)),
            Cell::analytical(format!("{}", c.numbers as i64)),
            Cell::analytical(format!("{}", c.multiplies as i64)),
        ])
    }).collect()
}

const MEASURES: [&str; 8] = ["first under the target", "stays under from", "loss at the end", "held-out accuracy", "mean update", "largest update", "numbers kept per step", "multiplies per step"];

pub fn training(t: &Training) -> Html {
    let pr = &t.preset;
    html! {
        <section class="space" id="training">
            <h2>{ "Part 2: from the bowl to a model" }</h2>
            { p(&format!("A softmax model (a [logistic classifier|logistic-classifier] for three classes) with six weights, two classes' rows of a bias and two input weights, learns the three flat clusters: {} points, {} trained on and {} [held out|held-out]. The third class's score is fixed at 0, so that no change of the six weights leaves the predictions alone (with a row per class, adding one number to every score would, and the Fisher matrix would be singular). Every run starts at all weights 0 (every class 1/3, the loss ln 3 = {}) and takes {} steps. The target, a loss of {}, was set before the run as mostly learned (the lowest loss any run reaches here is {}).", t.x.len(), t.trained_on, t.held.len(), show(t.good[0].log[0].loss), pr.steps, pr.target, show(t.good[2].record.end_loss))) }
            { p(&format!("Each method has its own rate, because a rate is measured in its own ruler's units (part 1): plain {}, diagonal {}, natural {} (1 is Newton's full step); the damping {}. The three were set before the first run and not tuned afterwards; only the plain rate is swept below, on the badly scaled data.", pr.rates[0], pr.rates[1], pr.rates[2], pr.damping)) }
            <div class="figure">
                <figure class="flat">
                    { data_picture(t, false) }
                    <figcaption>{ "The data as they are: three clusters, x1 and x2 on one unit. Ringed points are held out." }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    { data_picture(t, true) }
                    <figcaption>{ format!("The same points with x1 times {}, as if one input were recorded in different units. Nothing about the classes changed; only the numbers the model is given for x1.", pr.bad_scale) }{ mark(true) }</figcaption>
                </figure>
            </div>
            <h3>{ format!("With x1 times {}: the badly scaled version", pr.bad_scale) }</h3>
            <div class="figure">
                <figure class="flat">
                    { loss_chart(t, &t.bad, "badly scaled") }
                    <figcaption>{ format!("The training loss by step, x1 times {}: plain gray, diagonal blue dashed, natural green (the two lie on each other). A loss above 100 is drawn at the top. The plain step thrashes: each step overshoots along x1, whose slope is the scale times larger, and throws the loss far up the bowl's wall (its updates reach {}); at step {} it dips under the target once and jumps back out.", pr.bad_scale, tiny(t.bad[0].record.largest_update), count(t.bad[0].record.first_under)) }{ mark(true) }</figcaption>
                </figure>
                <figure>{ comparison(&MEASURES, &record_rows(&t.bad), &[]) }</figure>
            </div>
            <h3>{ "With the inputs as they are" }</h3>
            <div class="figure">
                <figure class="flat">
                    { loss_chart(t, &t.good, "as they are") }
                    <figcaption>{ "The same three methods, the same start, rates and steps, on the data as they are." }{ mark(true) }</figcaption>
                </figure>
                <figure>{ comparison(&MEASURES, &record_rows(&t.good), &[]) }</figure>
            </div>
            <p class="note">{ format!("{} held-out points, so one point is 1/{} of the accuracy: the diagonal's {:.3} and the natural step's {:.3} on the data as they are differ by one point. The update's flat length depends on how the inputs are written (x1's weight is {} times smaller on the badly scaled version), so the same predictions can come from updates of different flat length, as the natural step's rows show.", t.held.len(), t.held.len(), t.good[1].record.end_accuracy, t.good[2].record.end_accuracy, pr.bad_scale) }</p>
            <p class="note">{ "First under the target: the first step whose loss is under it. Stays under from: the step after which the loss stays below it to the end (a dip below and back out is not convergence). An update's size is the flat length of the change of the six weights. The cost columns are counted, not timed, for what each step forms: numbers a step keeps beyond the weights (the gradient, then the Fisher's diagonal, then the whole Fisher) and multiplies per step over the training rows, the solve of the 6 by 6 system by elimination counted as 108; exponentials, divisions and additions are not counted (the exponentials cost the same per row for every method)." }</p>
            <h3>{ "Is the plain step only mistuned? Its rate swept on the badly scaled data" }{ mark(true) }</h3>
            <table class="numbers">
                <thead><tr><th>{ "plain rate" }</th><th>{ "loss at the end" }</th><th>{ "first under the target" }</th><th>{ "stays under from" }</th><th>{ "largest update" }</th></tr></thead>
                <tbody>{ for t.sweep.iter().map(|r| html! { <tr><td>{ r.rate }</td><td>{ tiny(r.end_loss) }</td><td>{ count(r.first_under) }</td><td>{ stays(r.stays_under) }</td><td>{ tiny(r.largest_update) }</td></tr> }) }</tbody>
            </table>
            <p class="note">{ format!("Within 60 steps and at these four rates, a factor of ten apart: a rate between them, or more steps, was not tried, and at 0.005 the loss is still falling. A large rate overshoots along x1, whose slope is the scale times larger; a rate small enough for x1 crawls along everything else. The Fisher's eigenvalues at the start (below: {} against {}) suggest that no single plain rate is fast along every direction; one rate serves them all only if the ruler gives each direction its own length, which is what the diagonal and the Fisher do.", tiny(t.eigen[0][0]), tiny(t.eigen[0][5])) }</p>
        </section>
    }
}

pub fn geometry(t: &Training) -> Html {
    let e = &t.eigen;
    let ch = &t.changes;
    let names = ["0.1 added to class 1's bias weight", "0.1 added to class 1's x1 weight", "the plain step's first update", "the natural step's first update"];
    html! {
        <section class="space" id="fisher">
            <h2>{ "The Fisher metric as a geometry" }</h2>
            { p("The Fisher matrix F is a ruler on the six weights: a change d of the weights has Fisher length sqrt(d' F d), which says how much the model's predicted probabilities move, where the flat length sqrt(d . d) says how much the numbers move. At the start, on the badly scaled data:") }
            <div class="figure">
                <figure>
                    <table class="numbers">
                        <thead><tr><th>{ "the change of the weights" }</th><th>{ "flat length" }</th><th>{ "Fisher length" }</th></tr></thead>
                        <tbody>{ for names.iter().zip(ch).map(|(n, c)| html! { <tr><td>{ n.to_string() }</td><td>{ show(c[0]) }</td><td>{ show(c[1]) }</td></tr> }) }</tbody>
                    </table>
                    <p class="note">{ format!("The same 0.1 is {} by the Fisher on the bias and {} on x1's weight: x1 is {} times larger in the badly scaled data, so a change of its weight moves the predictions far more. The plain step's first update is {} long in numbers and {} by the start's Fisher, a ruler valid only for small changes (probabilities cannot move by more than 1): so far outside its range is a sign of a step much too large. The natural step's is {} and {}.", show(ch[0][1]), show(ch[1][1]), t.preset.bad_scale, show(ch[2][0]), show(ch[2][1]), show(ch[3][0]), show(ch[3][1])) }{ mark(true) }</p>
                </figure>
                <figure>
                    <table class="numbers">
                        <thead><tr><th>{ "data" }</th><th>{ "the Fisher's six eigenvalues at the start, largest first (each the Fisher length squared of a unit change of the weights along its own direction)" }</th><th>{ "condition number" }</th></tr></thead>
                        <tbody>
                            { for [format!("x1 times {}", t.preset.bad_scale), "as they are".to_string()].iter().enumerate().map(|(i, n)| html! { <tr><td>{ n.to_string() }</td><td>{ e[i][..6].iter().map(|x| tiny(*x)).collect::<Vec<_>>().join(", ") }</td><td>{ format!("{:.1}", e[i][6]) }</td></tr> }) }
                        </tbody>
                    </table>
                    <p class="note">{ format!("The condition number, the ratio of the largest eigenvalue to the smallest, is the bowl's shape from part 1 again, here at the start (the Fisher changes as the weights move): the bad scaling made it {:.0} times larger. For this model the Fisher is also the loss's curvature, a known fact of softmax regression, checked here at the start on the data as they are against central differences of the gradient (the largest gap {}); the Fisher of one row matches kron(M, x x') written out to {}, and the gradient matches central differences to {}.", t.condition_ratio, tiny(t.gaps[2]), tiny(t.gaps[1]), tiny(t.gaps[0])) }{ mark(true) }</p>
                </figure>
            </div>
            <h3>{ "The natural step does not care how the inputs are written" }{ mark(true) }</h3>
            { p("Rewrite the inputs in two ways the classes cannot notice: x1 times the scale, and x1 replaced by x1 + x2. From the start (all weights 0, so every version predicts 1/3 for every class), run ten steps of each method on each version, undamped, and compare the predicted probabilities on the training rows with those of the same method on the data as they are. The largest gap:") }
            <table class="numbers">
                <thead><tr><th>{ "method" }</th><th>{ "x1 rescaled" }</th><th>{ "x1 mixed with x2" }</th></tr></thead>
                <tbody>{ for METHODS.iter().zip(&t.invariance).map(|(m, g)| html! { <tr><td>{ m.to_string() }</td><td>{ tiny(g[0]) }</td><td>{ tiny(g[1]) }</td></tr> }) }</tbody>
            </table>
            <p class="note">{ format!("The plain step learns a different model after either rewriting ({} and {}). The diagonal ruler gives each weight its own length, so it undoes a rescaling of one input to rounding ({}) and not a mixing of two ({}). The natural step undoes both ({} and {}, rounding): the Fisher is defined by what the model predicts, not by how its inputs are written. The damping (adding {} times the identity to F, as the training runs above do) gives the property up in proportion to {} against the Fisher's smallest eigenvalue ({} at the start on the data as they are): the identity is a flat ruler, so a damped step is partly a plain one. Here that proportion is small, and the damped runs' end losses on the two versions agree to four decimals ({} and {}); this check is undamped.", show(t.invariance[0][0]), show(t.invariance[0][1]), tiny(t.invariance[1][0]), show(t.invariance[1][1]), tiny(t.invariance[2][0]), tiny(t.invariance[2][1]), t.preset.damping, t.preset.damping, tiny(t.eigen[1][5]), show(t.good[2].record.end_loss), show(t.bad[2].record.end_loss)) }</p>
        </section>
    }
}

pub fn analysis(t: &Training) -> Html {
    let (pb, nb) = (t.bad[0].record, t.bad[2].record);
    let (pg, dg, ng) = (t.good[0].record, t.good[1].record, t.good[2].record);
    let spent = |i: usize| if t.spent[i] < 0.0 { "not under at the end".to_string() } else { format!("{}", t.spent[i] as i64) };
    let rows = [
        ("plain gradient descent", vec![Cell::measured(stays(pb.stays_under)), Cell::measured(stays(pg.stays_under)), Cell::analytical(format!("{}", pg.multiplies as i64)), Cell::measured(spent(3))]),
        ("the diagonal preconditioner", vec![Cell::measured(stays(t.bad[1].record.stays_under)), Cell::measured(stays(dg.stays_under)), Cell::analytical(format!("{}", dg.multiplies as i64)), Cell::measured(spent(4))]),
        ("the natural-gradient step", vec![Cell::measured(stays(nb.stays_under)), Cell::measured(stays(ng.stays_under)), Cell::analytical(format!("{}", ng.multiplies as i64)), Cell::measured(spent(5))]),
    ];
    let sentences = vec![
        narrate::moved("the loss at the end under plain gradient descent, from the inputs as they are to x1 badly scaled", pg.end_loss, pb.end_loss),
        format!("The loss at the end under the natural step is {} on the data as they are and {} with x1 badly scaled; under the diagonal {} and {} (measured): equal to four decimals, because both rulers undo a rescaling of one input.", show(ng.end_loss), show(nb.end_loss), show(dg.end_loss), show(t.bad[1].record.end_loss)),
        narrate::count("multiplies spent to stay under the target on the data as they are, from plain gradient descent to the natural step", t.spent[3] as usize, t.spent[5] as usize, None),
    ];
    html! { <>
        <h3>{ "Part 2: does the metric pay?" }</h3>
        { comparison(&["stays under the target from (x1 badly scaled)", "stays under from (as they are)", "multiplies per step", "multiplies to stay under (as they are): steps times per step"], &rows, &sentences) }
        { p("Read against what was written before the run. On the badly scaled data the metric paid: the plain step at its rate did not settle in the steps given, and no plain rate tried settled in the steps given, while the diagonal and the natural step took the same few steps they take on the clean data. On the data as they are the metric did not pay in multiplies: the natural step took 2 steps against plain's 5, and spent more in all, because each step costs the Fisher and its solve; the gap is under a factor of two, and counting the exponentials (the same per row for every method) would narrow it a little without reversing it. That is a null result for the metric on well scaled data, and the page records it as one.") }
        { p("The limits. A model this small hides the real cost: the Fisher has as many rows as there are weights, so it holds the square of their number and its solve grows with the cube; at a million weights it cannot be formed, which is why practical natural-gradient methods approximate it (by blocks, for instance). Adam's per-weight scaling is a cousin of the diagonal preconditioner here (it divides by a running average of squared gradients, not by the Fisher's diagonal), not the natural gradient. And for softmax regression the Fisher equals the loss's curvature, so the natural step is Newton's; in a network the two differ.") }
    </> }
}
