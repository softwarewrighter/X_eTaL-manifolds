//! The page, on the shared teaching shell and turning view: the arc of
//! frames; the bowl with its contours, its gradient field and the three
//! paths, the control's a picked by the reader; the loss by step; the
//! control's table and chart; the steepest direction under two rulers
//! worked, with the unit balls; the sphere turning with the three
//! climbs and the eigenvector, the chart and the numbers beside it;
//! the comparisons; every idea explained; the program. Presentation
//! only: every number shown is one X_eTaL printed, and picking a value
//! of a runs nothing.

use yew::prelude::*;

use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::definition;
use lesson::narrate;
use microscope::turning::{Angle, Point, Scene, Segment, Turning};
use site::{comparison, functions, mark, own_function, Cell, ANALYSIS, EXPERIMENT, EXTENSION, INTUITION, MATHEMATICS, RUN};

use gloo_timers::callback::Timeout;

use crate::micro::{program, run, Opt, P2, P3, WAYS};
use crate::model;
use crate::text;
use crate::training::{program as training_program, run as training_run, Training, PRESET as TRAINING_PRESET};

fn show(x: f64) -> String {
    format!("{x:.4}")
}

/// A loss or a distance, small numbers kept readable.
fn tiny(x: f64) -> String {
    if x == 0.0 {
        "0".into()
    } else if x.abs() < 0.001 {
        format!("{x:.2e}")
    } else {
        format!("{x:.4}")
    }
}

fn count(x: f64) -> String {
    if x < 0.0 { "not reached".into() } else { format!("{}", x as i64) }
}

const PLAIN: &str = "#868e96";
const CONTROL: &str = "#e8590c";
const NEWTON: &str = "#2b8a3e";
const RETRACT: &str = "#1c7ed6";
const TOP: &str = "#7048e8";

/// The shared view turns about y; the program writes the sphere with z
/// up, so points are turned a quarter turn about x for drawing,
/// (x, y, z) to (x, z, -y): a rotation, nothing changes meaning.
fn upright(q: P3) -> [f64; 3] {
    [q[0], q[2], -q[1]]
}

/// The preset's a as a fraction: it is given in ninths (and half ninths) so that a third is exact.
fn a_words(a: f64) -> String {
    match (a * 18.0).round() as i64 {
        2 => "1/9".into(),
        3 => "1/6".into(),
        6 => "1/3".into(),
        9 => "1/2".into(),
        18 => "1".into(),
        n if n % 18 == 0 => format!("{}", n / 18),
        _ => format!("{a:.4}"),
    }
}

// -- the bowl

const BW: f64 = 480.0;
const BH: f64 = 320.0;

fn bowl_at(q: P2) -> (f64, f64) {
    (BW / 2.0 + 80.0 * q[0], BH / 2.0 - 80.0 * q[1])
}

fn poly(points: &[P2], color: &str, width: &str, dash: Option<&str>) -> Html {
    let d: Vec<String> = points.iter().map(|q| { let (x, y) = bowl_at(*q); format!("{x:.1},{y:.1}") }).collect();
    html! { <polyline points={d.join(" ")} fill="none" stroke={color.to_string()} stroke-width={width.to_string()} stroke-dasharray={dash.map(|s| s.to_string())} /> }
}

fn bowl(o: &Opt, chosen: usize) -> Html {
    let a = o.control[chosen].a;
    let arrows = o.field.iter().map(|(q, g)| {
        // The field's arrows, each scaled by one factor so the longest fits its cell (the direction and the relative lengths are the gradient's).
        let (x, y) = bowl_at(*q);
        let (dx, dy) = (g[0] * 8.0, -g[1] * 8.0);
        html! { <line class="field" x1={format!("{x:.1}")} y1={format!("{y:.1}")} x2={format!("{:.1}", x + dx)} y2={format!("{:.1}", y + dy)} stroke="#adb5bd" stroke-width="1" marker-end="url(#head)" /> }
    });
    let dots = |path: &[P2], color: &str| path.iter().take(8).map(|q| { let (x, y) = bowl_at(*q); html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="2.5" fill={color.to_string()} /> } }).collect::<Vec<_>>();
    let (sx, sy) = bowl_at(o.preset.start);
    html! {
        <svg viewBox={format!("0 0 {BW} {BH}")} role="img" aria-label={format!("the bowl's contours as ellipses with its gradient field as small arrows, and from the start the three paths: the plain step in gray, the step under diag(a, 1) at a = {} in orange, Newton's direction in green, each marked with a dot at its first points", a_words(a))}>
            <defs><marker id="head" viewBox="0 0 6 6" refX="5" refY="3" markerWidth="5" markerHeight="5" orient="auto"><path d="M0,0 L6,3 L0,6 z" fill="#adb5bd" /></marker></defs>
            <line class="axis" x1="0" y1={format!("{:.1}", BH / 2.0)} x2={format!("{BW}")} y2={format!("{:.1}", BH / 2.0)} />
            <line class="axis" x1={format!("{:.1}", BW / 2.0)} y1="0" x2={format!("{:.1}", BW / 2.0)} y2={format!("{BH}")} />
            { for o.contours.iter().map(|c| poly(c, "#ced4da", "1", None)) }
            { for arrows }
            { poly(&o.plain.path, PLAIN, "2", None) }
            { poly(&o.control_paths[chosen], CONTROL, "2", Some("5 3")) }
            { poly(&o.newton.path, NEWTON, "2.5", None) }
            { for dots(&o.plain.path, PLAIN) }
            { for dots(&o.control_paths[chosen], CONTROL) }
            { for dots(&o.newton.path, NEWTON) }
            <circle cx={format!("{sx:.1}")} cy={format!("{sy:.1}")} r="4" fill="#212529" />
            <text class="lbl" x={format!("{:.1}", sx + 6.0)} y={format!("{:.1}", sy - 6.0)}>{ "start" }</text>
            <text class="lbl" x={format!("{:.1}", BW / 2.0 + 6.0)} y={format!("{:.1}", BH / 2.0 + 14.0)}>{ "floor" }</text>
            { for o.preset.levels.iter().zip(&o.contours).map(|(c, pts)| { let (x, y) = bowl_at(pts[6]); html! { <text class="axis-lbl" x={format!("{:.1}", x + 3.0)} y={format!("{:.1}", y - 2.0)}>{ format!("L = {c}") }</text> } }) }
            <text class="axis-lbl" x="4" y="12">{ "y up, x across; one unit is 80 pixels" }</text>
        </svg>
    }
}

/// The loss by step of the three paths, on a log scale; zeros and values under the floor sit on the floor.
fn loss_chart(o: &Opt, chosen: usize) -> Html {
    let (w, hh) = (420.0, 220.0);
    let n = o.preset.steps + 1;
    let floor = 1e-12_f64;
    let top = o.plain.loss[0].max(o.newton.loss[0]).max(o.control_losses[chosen][0]).log10();
    let bottom = floor.log10();
    let at = |k: usize, loss: f64| {
        let x = 30.0 + (w - 40.0) * k as f64 / (n - 1) as f64;
        let l = loss.max(floor).log10();
        let y = hh - 24.0 - (hh - 36.0) * (l - bottom) / (top - bottom);
        format!("{x:.1},{y:.1}")
    };
    let line = |loss: &[f64], color: &str, dash: Option<&str>| {
        let d: Vec<String> = loss.iter().enumerate().map(|(k, l)| at(k, *l)).collect();
        html! { <polyline points={d.join(" ")} fill="none" stroke={color.to_string()} stroke-width="2" stroke-dasharray={dash.map(|s| s.to_string())} /> }
    };
    let y_t = {
        let l = o.preset.target.log10();
        hh - 24.0 - (hh - 36.0) * (l - bottom) / (top - bottom)
    };
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the loss by step of the three paths on a logarithmic scale, the plain step gray, the control orange dashed, Newton's green, with the target as a line">
            <line class="axis" x1="30" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="30" y1="12" x2="30" y2={format!("{:.1}", hh - 24.0)} />
            <line class="chord" x1="30" y1={format!("{y_t:.1}")} x2={format!("{:.1}", w - 10.0)} y2={format!("{y_t:.1}")} />
            <text class="axis-lbl" x="34" y={format!("{:.1}", y_t - 3.0)}>{ format!("loss {}, the target", o.preset.target) }</text>
            <text class="axis-lbl" x="34" y="12">{ format!("loss, {:.2} at the top down to {floor:e} at the bottom edge (log scale)", o.plain.loss[0]) }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("step {}", n - 1) }</text>
            <text class="axis-lbl" x="30" y={format!("{:.1}", hh - 8.0)}>{ "step 0" }</text>
            { line(&o.plain.loss, PLAIN, None) }
            { line(&o.control_losses[chosen], CONTROL, Some("5 3")) }
            { line(&o.newton.loss, NEWTON, None) }
        </svg>
    }
}

fn a_buttons(chosen: usize, o: &Opt, pick: Callback<usize>) -> Html {
    html! {
        <div class="controls" role="group" aria-label="the anisotropy a of the control's ruler diag(a, 1)">
            { for o.control.iter().enumerate().map(|(k, r)| {
                let pick = pick.clone();
                html! { <button class={classes!(chosen.eq(&k).then_some("on"))} onclick={Callback::from(move |_| pick.emit(k))} aria-pressed={if chosen == k { "true" } else { "false" }}>{ format!("a = {}", a_words(r.a)) }</button> }
            }) }
        </div>
    }
}

fn first_steps(o: &Opt, chosen: usize) -> Html {
    let f = |name: &str, w: crate::micro::First| html! {
        <tr><td>{ name.to_string() }</td><td>{ format!("({}, {})", show(w.d[0]), show(w.d[1])) }</td><td>{ show(w.dot) }</td><td>{ show(w.den) }</td><td>{ show(w.eta) }</td><td>{ format!("({}, {})", show(w.to[0]), show(w.to[1])) }</td></tr>
    };
    let cf = o.control_first[chosen];
    let a = o.control[chosen].a;
    html! { <>
        <h3>{ "The first step under each ruler" }{ mark(true) }</h3>
        <table class="numbers">
            <thead><tr><th>{ "ruler" }</th><th>{ "d = P g (the step is -eta d)" }</th><th>{ "g . d" }</th><th>{ "d . H d" }</th><th>{ "eta = (g . d) / (d . H d)" }</th><th>{ "point reached" }</th></tr></thead>
            <tbody>
                { f("the plain ruler, G = I", o.plain_first) }
                { f(&format!("diag(a, 1) at a = {}", a_words(a)), cf) }
                { f("the bowl's curvature, G = H", o.newton_first) }
            </tbody>
        </table>
        <p class="note">{ format!("From the start ({}, {}) the gradient is g = ({}, {}): the slope along x is x, the slope along y is 3 y. Under the plain ruler the direction is g itself: g . d = g . g = {} and d . H d = g' H g = {} (the x part squared plus three times the y part squared), so eta = {} / {} = {}, and the step lands at ({}, {}). Under H the direction is (x, y), the point itself; both parts are {}, eta = 1, and the step lands on the floor. Under diag(a, 1) the direction is (gx / a, gy): at a = {} an x-step counts {} as long, so x is cheap and the direction leans along x, the valley's length: ({}, {}), with eta = {} / {} = {}.", show(o.preset.start[0]), show(o.preset.start[1]), show(o.g0[0]), show(o.g0[1]), show(o.plain_first.dot), show(o.plain_first.den), show(o.plain_first.dot), show(o.plain_first.den), show(o.plain_first.eta), show(o.plain_first.to[0]), show(o.plain_first.to[1]), show(o.newton_first.dot), a_words(a), a_words(a), show(cf.d[0]), show(cf.d[1]), show(cf.dot), show(cf.den), show(cf.eta)) }</p>
    </> }
}

fn descent_section(o: &Opt, chosen: usize, pick: Callback<usize>) -> Html {
    let a = o.control[chosen].a;
    html! {
        <section class="space" id="bowl">
            <h2>{ "The bowl, three rulers, three paths" }</h2>
            { p("The contours are the loss's level lines, the small arrows its gradient field (neither changes below). From the start, three descents by the same rule: the direction of steepest descent under the ruler, then the exact line search along it. Pick the control's a with the buttons: the orange path is diag(a, 1); a = 1 is the plain ruler and a = 1/3 is Newton's direction.") }
            { a_buttons(chosen, o, pick) }
            <div class="figure">
                <figure class="flat">
                    { bowl(o, chosen) }
                    <figcaption>{ format!("The bowl L = (x^2 + 3 y^2) / 2: contours at the preset's levels (analytical, the ellipses' formula) and the gradient at a grid of points (measured; arrows scaled by one factor to fit). The three paths from the start (measured), dotted at their first points: gray the plain step, orange dashed diag(a, 1) at a = {}, green Newton's. Where paths coincide the later one is drawn on top.", a_words(a)) }{ mark(true) }{ mark(false) }</figcaption>
                </figure>
                <figure class="flat">
                    { loss_chart(o, chosen) }
                    <figcaption>{ "The loss by step of the same three paths, on a logarithmic scale; a loss of 0 (Newton's from step 1) and any loss below the chart's lowest value are drawn on its bottom edge. The target is the dashed horizontal line." }{ mark(true) }</figcaption>
                </figure>
                <figure>{ first_steps(o, chosen) }</figure>
            </div>
        </section>
    }
}

// -- the control

fn control_chart(o: &Opt, chosen: usize) -> Html {
    let (w, hh) = (420.0, 200.0);
    let n = o.control.len();
    let top = o.control.iter().map(|r| r.steps_to_target).fold(0.0, f64::max).max(1.0);
    let at = |k: usize, s: f64| (30.0 + (w - 60.0) * k as f64 / (n - 1) as f64, hh - 24.0 - (hh - 40.0) * s / top);
    let d: Vec<String> = o.control.iter().enumerate().map(|(k, r)| { let (x, y) = at(k, r.steps_to_target); format!("{x:.1},{y:.1}") }).collect();
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the steps to the target under diag(a, 1) at each a of the control, a from 1/9 on the left to 9 on the right, the chosen one filled">
            <line class="axis" x1="30" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="30" y1="12" x2="30" y2={format!("{:.1}", hh - 24.0)} />
            <text class="axis-lbl" x="34" y="12">{ format!("steps to a loss under {}, 0 to {}", o.preset.target, top as i64) }</text>
            <polyline points={d.join(" ")} fill="none" stroke="#868e96" stroke-width="1.5" />
            { for o.control.iter().enumerate().map(|(k, r)| {
                let (x, y) = at(k, r.steps_to_target);
                html! { <g>
                    <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="4.5" fill={if k == chosen { CONTROL } else { "#fff" }} stroke={CONTROL} stroke-width="2" />
                    <text class="axis-lbl" x={format!("{:.1}", x - 8.0)} y={format!("{:.1}", hh - 8.0)}>{ a_words(r.a) }</text>
                </g> }
            }) }
            <text class="axis-lbl" x={format!("{:.1}", w - 120.0)} y={format!("{:.1}", hh - 30.0)}>{ "a (spaced evenly, not to scale)" }</text>
        </svg>
    }
}

fn control_section(o: &Opt, chosen: usize) -> Html {
    let plain = o.control_at(1.0);
    let r = o.control[chosen];
    let sentences: Vec<String> = match plain {
        Some(k) if k != chosen => vec![
            narrate::count(&format!("the steps to the target, from a = 1 (the plain ruler) to a = {}", a_words(r.a)), o.control[k].steps_to_target as usize, r.steps_to_target as usize, None),
            narrate::moved(&format!("the condition number the ruler leaves the bowl with, from a = 1 to a = {}", a_words(r.a)), o.control[k].kappa, r.kappa),
            narrate::moved(&format!("the loss after five steps, from a = 1 to a = {}", a_words(r.a)), o.control[k].after_five, r.after_five),
        ],
        _ => vec!["The chosen a is 1, the plain ruler itself: pick another to read what changes.".into()],
    };
    html! {
        <section class="space" id="control">
            <h2>{ "The ruler as a control: diag(a, 1) from 1/9 to 9" }</h2>
            { p("The same descent at every a of the preset. The ruler counts x-steps a times as heavily; the condition number it leaves the bowl with is the larger of 3 a and 1 / (3 a), which is 1 at a = 1/3, where the direction is Newton's. The chosen a is filled in the chart and marked in the table.") }
            <div class="figure">
                <figure class="flat">
                    { control_chart(o, chosen) }
                    <figcaption>{ "Steps to the target loss at each a (measured). Values of a that leave the same condition number (1/9 and 1, 1/6 and 1/2) take about as many steps, not the same: the start's position relative to the ruler's axes sets the rest." }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <table class="numbers">
                        <thead><tr><th>{ "a" }</th><th>{ "condition number left" }</th><th>{ "steps to target" }</th><th>{ "loss after 5 steps" }</th><th>{ "loss at the end" }</th></tr></thead>
                        <tbody>
                            { for o.control.iter().enumerate().map(|(k, r)| html! {
                                <tr class={classes!((k == chosen).then_some("sum"))}><td>{ a_words(r.a) }{ if k == chosen { " (chosen)" } else { "" } }</td><td>{ show(r.kappa) }</td><td>{ count(r.steps_to_target) }</td><td>{ tiny(r.after_five) }</td><td>{ tiny(r.end) }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    { for sentences.iter().map(|s| html! { <p class="narrated">{ s.clone() }</p> }) }
                </figure>
            </div>
        </section>
    }
}

// -- the steepest direction, worked

fn balls(o: &Opt) -> Html {
    let (w, hh) = (320.0, 320.0);
    let at = |q: P2| (w / 2.0 + 120.0 * q[0], hh / 2.0 - 120.0 * q[1]);
    let ring = |pts: &[P2], color: &str| { let d: Vec<String> = pts.iter().map(|q| { let (x, y) = at(*q); format!("{x:.1},{y:.1}") }).collect(); html! { <polyline points={d.join(" ")} fill="none" stroke={color.to_string()} stroke-width="1.5" /> } };
    let arrow = |v: P2, color: &str| { let (x, y) = at(v); let (cx, cy) = at([0.0, 0.0]); html! { <line x1={format!("{cx:.1}")} y1={format!("{cy:.1}")} x2={format!("{x:.1}")} y2={format!("{y:.1}")} stroke={color.to_string()} stroke-width="2.5" marker-end="url(#ahead)" /> } };
    let (gx, gy) = at(o.plain_dir);
    let (nx, ny) = at(o.newton_dir);
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="at the start, the unit ball of the plain ruler (a circle, gray) and of the bowl's curvature (an ellipse, green), with the steepest-descent direction under each as an arrow from the middle">
            <defs><marker id="ahead" viewBox="0 0 6 6" refX="5" refY="3" markerWidth="4" markerHeight="4" orient="auto"><path d="M0,0 L6,3 L0,6 z" fill="#495057" /></marker></defs>
            { ring(&o.plain_ball, PLAIN) }
            { ring(&o.newton_ball, NEWTON) }
            { arrow(o.plain_dir, PLAIN) }
            { arrow(o.newton_dir, NEWTON) }
            <text class="lbl" x={format!("{:.1}", gx - 4.0)} y={format!("{:.1}", gy + 16.0)}>{ "-g / |g| (plain)" }</text>
            <text class="lbl" x={format!("{:.1}", nx + 6.0)} y={format!("{:.1}", ny - 6.0)}>{ "under H" }</text>
            <text class="axis-lbl" x="4" y="12">{ "the start is at the middle; steps of length 1 under each ruler" }</text>
        </svg>
    }
}

fn steepest_section(o: &Opt) -> Html {
    html! {
        <section class="space" id="steepest">
            <h2>{ "The steepest direction at the start, under two rulers, worked" }</h2>
            { p("Among the steps of length 1 under a ruler (its unit ball), which lowers the loss most? Under the plain ruler the ball is the unit circle and the answer is -g / |g|. Under the bowl's curvature H the ball is the ellipse x^2 + 3 y^2 = 1, wider along x, and the answer tilts toward x. The program checks each answer by trying 360 directions round the ball.") }
            <div class="figure">
                <figure class="flat">
                    { balls(o) }
                    <figcaption>{ "The two unit balls at the start (analytical: each ruler's set of steps of length 1) and the steepest-descent direction within each (measured), gray under the plain ruler, green under H." }{ mark(false) }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <div class="worked-out">
                        <h3>{ "Worked in full" }{ mark(true) }</h3>
                        <p>{ format!("The gradient at the start: g = ({}, {}).", show(o.g0[0]), show(o.g0[1])) }</p>
                        <p>{ format!("Plain ruler, G = I: P g = g; g . P g = g . g = {} x {} + {} x {} = {}, so sqrt(g . P g) = |g| = {}; v = -g / |g| = (-{} / {}, -{} / {}) = ({}, {}). Rate of fall g . v = {} (per unit of plain length).", show(o.g0[0]), show(o.g0[0]), show(o.g0[1]), show(o.g0[1]), show(o.plain_first.dot), show(o.plain_norm), show(o.g0[0]), show(o.plain_norm), show(o.g0[1]), show(o.plain_norm), show(o.plain_dir[0]), show(o.plain_dir[1]), show(o.rates[0])) }</p>
                        <p>{ format!("The bowl's curvature, G = H = diag(1, 3): P = diag(1, 1/3), P g = ({}, {}); g . P g = {} x {} + {} x {} = {}, so sqrt(g . P g) = {}; v = -P g / {} = ({}, {}). Rate of fall g . v = {} (per unit of H's length, a different unit: the two rates are not compared).", show(o.newton_pg[0]), show(o.newton_pg[1]), show(o.g0[0]), show(o.newton_pg[0]), show(o.g0[1]), show(o.newton_pg[1]), show(o.newton_first.dot), show(o.newton_norm), show(o.newton_norm), show(o.newton_dir[0]), show(o.newton_dir[1]), show(o.rates[1])) }</p>
                        <p>{ format!("The check by sampling: of 360 directions round each ball, the one with the smallest g . v is ({}, {}) under I and ({}, {}) under H, within the sampling's half a degree of the formulas.", show(o.sampled_plain[0]), show(o.sampled_plain[1]), show(o.sampled_newton[0]), show(o.sampled_newton[1])) }</p>
                        <p>{ format!("The gradient itself was checked against central differences at three points, the start among them: the largest gap is {}.", tiny(o.gradient_gap)) }</p>
                    </div>
                </figure>
            </div>
        </section>
    }
}

// -- the sphere

fn sphere_scene(o: &Opt) -> Scene {
    let colors = [NEWTON, RETRACT, CONTROL];
    let mut segments = Vec::new();
    for (k, c) in o.climbs.iter().enumerate() {
        for w in c.path.windows(2) {
            segments.push(Segment { from: upright(w[0]), to: upright(w[1]), color: colors[k].into(), arrow: false });
        }
    }
    segments.push(Segment { from: [0.0, 0.0, 0.0], to: upright(o.top), color: TOP.into(), arrow: true });
    let mut points = vec![Point { at: upright(o.climbs[0].path[0]), color: "#212529".into(), label: "start".into() }, Point { at: upright(o.top), color: TOP.into(), label: "the leading eigenvector".into() }];
    for (k, c) in o.climbs.iter().enumerate() {
        for q in c.path.iter().take(6).skip(1) {
            points.push(Point { at: upright(*q), color: colors[k].into(), label: String::new() });
        }
    }
    Scene { points, segments, wires: o.parallels.iter().chain(&o.meridians).map(|w| w.iter().map(|q| upright(*q)).collect()).collect() }
}

fn sphere_chart(o: &Opt) -> Html {
    let (w, hh) = (420.0, 220.0);
    let n = o.preset.sphere_steps + 1;
    let floor = 1e-12_f64;
    let top = o.climbs.iter().map(|c| c.log[0][1]).fold(0.0, f64::max).log10();
    let bottom = floor.log10();
    let at = |k: usize, v: f64| {
        let x = 30.0 + (w - 40.0) * k as f64 / (n - 1) as f64;
        let l = v.max(floor).log10();
        format!("{x:.1},{:.1}", hh - 24.0 - (hh - 36.0) * (l - bottom) / (top - bottom))
    };
    let colors = [NEWTON, RETRACT, CONTROL];
    let dashes = [None, Some("5 3"), None];
    let y_t = { let l = o.preset.tolerance.log10(); hh - 24.0 - (hh - 36.0) * (l - bottom) / (top - bottom) };
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the distance along the sphere from each climb's point to the leading eigenvector, by step, on a logarithmic scale: the exact step green, the retraction blue dashed, the flat step repaired orange; the tolerance as a line">
            <line class="axis" x1="30" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="30" y1="12" x2="30" y2={format!("{:.1}", hh - 24.0)} />
            <line class="chord" x1="30" y1={format!("{y_t:.1}")} x2={format!("{:.1}", w - 10.0)} y2={format!("{y_t:.1}")} />
            <text class="axis-lbl" x="34" y={format!("{:.1}", y_t - 3.0)}>{ format!("distance {}, the tolerance", o.preset.tolerance) }</text>
            <text class="axis-lbl" x="34" y="12">{ format!("distance to the eigenvector along the sphere, {:.2} at the top down to {floor:e} (log scale)", o.climbs[0].log[0][1]) }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("step {}", n - 1) }</text>
            <text class="axis-lbl" x="30" y={format!("{:.1}", hh - 8.0)}>{ "step 0" }</text>
            { for o.climbs.iter().enumerate().map(|(k, c)| {
                let d: Vec<String> = c.log.iter().enumerate().map(|(i, r)| at(i, r[1])).collect();
                html! { <polyline points={d.join(" ")} fill="none" stroke={colors[k]} stroke-width="2" stroke-dasharray={dashes[k].map(|s| s.to_string())} /> }
            }) }
        </svg>
    }
}

fn sphere_numbers(o: &Opt) -> Html {
    html! { <>
        <h3>{ "The matrix, its eigenvalues, and the first steps" }{ mark(true) }</h3>
        <p class="prose">{ format!("A = (4 1 0 / 1 3 1 / 0 1 2). Matrix's eigenvalues: {}, {}, {}; its leading eigenvector ({}, {}, {}). The gradient of the quotient, 2 A p, against central differences at the start: the largest gap is {}.", show(o.eigenvalues[0]), show(o.eigenvalues[1]), show(o.eigenvalues[2]), show(o.top[0]), show(o.top[1]), show(o.top[2]), tiny(o.quotient_gap)) }</p>
        <p class="note">{ "Each cell below: the quotient p' A p at that step; the distance along the sphere from the point to the eigenvector; how far off the sphere the step went before its repair (the exact step lands on the sphere, so 0 but for rounding)." }</p>
        <table class="numbers">
            <thead><tr><th>{ "step" }</th><th>{ "the exact step" }</th><th>{ "the retraction" }</th><th>{ "the flat step, repaired" }</th></tr></thead>
            <tbody>
                { for (0..6).map(|i| html! {
                    <tr><td>{ format!("{i}") }</td>{ for o.climbs.iter().map(|c| html! { <td>{ format!("{}; {}; {}", show(c.log[i][0]), tiny(c.log[i][1]), tiny(c.log[i][2])) }</td> }) }</tr>
                }) }
            </tbody>
        </table>
        <p class="note">{ format!("Analytical, from the eigenvalues: near the eigenvector the flat step repaired shrinks its distance by {} per step (the ratio of the top two eigenvalues of I + 2 eta A) and the step along the sphere by {} (1 - 2 eta (lambda1 - lambda2)); the program checks the measured counts against the steps those factors predict, within three.", show(o.contraction[0]), show(o.contraction[1])) }</p>
    </> }
}

fn sphere_section(o: &Opt, angle: Angle, turned: Callback<Angle>) -> Html {
    let (yaw, pitch) = ((angle.yaw.to_degrees().round() as i32).rem_euclid(360), angle.pitch.to_degrees().round() as i32);
    html! {
        <section class="space" id="sphere">
            <h2>{ "On the sphere: a step that stays, against a step that leaves and is pushed back" }</h2>
            { p("The colors start over here: nothing is carried from the bowl. The unit vector that makes p' A p largest is A's leading eigenvector (purple arrow). From one start, at one rate (the same number for all three, not the same movement along the sphere), three climbs: the exact step along the sphere (green: the gradient's part along the sphere, then the exponential map), the same with the cheap retraction (blue, dashed: add the tangent vector and scale back), and a flat step up the gradient repaired by scaling back to length 1 (orange). The first five points of each are marked.") }
            <div class="figure">
                <figure class="turning">
                    <Turning scene={sphere_scene(o)} angle={angle} {turned} size={420.0}
                        label={format!("the unit sphere with the three climbs from the start to the leading eigenvector, the exact step green, the retraction blue, the flat step repaired orange, and the eigenvector as a purple arrow; turned {yaw} degrees round and {pitch} degrees toward you")} />
                    <figcaption>{ format!("The sphere in three dimensions, turned {yaw} degrees round and {pitch} degrees toward you. Drag, use the arrow keys once it has the focus, or swipe to turn it; turning changes no number. The paths are the points the program printed, joined (measured); the wire lines show where the sphere is (analytical). The flat step's points are drawn after their repair: where it went between repairs is in the table.") }{ mark(true) }{ mark(false) }</figcaption>
                </figure>
                <figure class="flat">
                    { sphere_chart(o) }
                    <figcaption>{ "The distance along the sphere to the eigenvector by step, on a logarithmic scale (measured); a distance below the chart's lowest value is drawn on its bottom edge. The exact step's green line lies under the retraction's dashed blue where the two coincide." }{ mark(true) }</figcaption>
                </figure>
                <figure>{ sphere_numbers(o) }</figure>
            </div>
        </section>
    }
}

// -- the analysis

fn analysis(o: &Opt, chosen: usize) -> Html {
    let r = o.control[chosen];
    let a = a_words(r.a);
    let rec = |name: &str, rc: crate::micro::Record| (name.to_string(), vec![Cell::measured(count(rc.steps_to_target)), Cell::measured(tiny(rc.after_one)), Cell::measured(tiny(rc.after_five)), Cell::measured(tiny(rc.end)), Cell::analytical(format!("{}", rc.numbers as i64)), Cell::analytical(format!("{}", rc.multiplies as i64))]);
    let plain = rec("the plain ruler, G = I", o.plain_record);
    let control = (format!("diag(a, 1) at a = {a}"), vec![Cell::measured(count(r.steps_to_target)), Cell::measured(tiny(o.control_losses[chosen][1])), Cell::measured(tiny(r.after_five)), Cell::measured(tiny(r.end)), Cell::analytical("1"), Cell::analytical("2")]);
    let newton = rec("the bowl's curvature, G = H (Newton's direction)", o.newton_record);
    let rows = [(plain.0.as_str(), plain.1), (control.0.as_str(), control.1), (newton.0.as_str(), newton.1)];
    let sentences = vec![
        narrate::count("the steps to the target, from the plain ruler to Newton's direction", o.plain_record.steps_to_target as usize, o.newton_record.steps_to_target as usize, None),
        narrate::count(&format!("the steps to the target, from the plain ruler to diag(a, 1) at a = {a}"), o.plain_record.steps_to_target as usize, r.steps_to_target as usize, None),
        narrate::moved("the loss after five steps, from the plain ruler to Newton's direction", o.plain_record.after_five, o.newton_record.after_five),
        format!("The loss at the start is the same for every ruler, (x^2 + 3 y^2) / 2 = ({} + {}) / 2 = {}, the first point of the loss-by-step chart; and the contours did not move: the ruler changed the path, not the bowl.", show(o.start_parts[0]), show(o.start_parts[1]), show(o.plain.loss[0])),
    ];
    let srows: Vec<(&str, Vec<Cell>)> = WAYS.iter().zip(&o.climbs).map(|(w, c)| (*w, vec![Cell::measured(show(c.record[0])), Cell::measured(tiny(c.record[1])), Cell::measured(count(c.record[2])), Cell::measured(tiny(c.record[3])), Cell::measured(tiny(c.record[4]))])).collect();
    let ssentences = vec![
        narrate::count("the steps to the tolerance, from the exact step to the flat step repaired", o.climbs[0].record[2] as usize, o.climbs[2].record[2] as usize, None),
        narrate::count("the steps to the tolerance, from the exact step to the retraction", o.climbs[0].record[2] as usize, o.climbs[1].record[2] as usize, None),
        format!("The farthest a step went off the sphere before its repair: {} for the exact step (zero but for rounding), {} for the retraction, {} for the flat step (measured).", tiny(o.climbs[0].record[3]), tiny(o.climbs[1].record[3]), tiny(o.climbs[2].record[3])),
        format!("At step 1 the exact step and the retraction are {} and {} from the eigenvector; at the end {} and {} (measured): they part in the third decimal early and agree at the end.", show(o.climbs[0].log[1][1]), show(o.climbs[1].log[1][1]), tiny(o.climbs[0].record[1]), tiny(o.climbs[1].record[1])),
        format!("All three end within the tolerance of Matrix's eigenvector, at quotients {}, {} and {} against the largest eigenvalue {} (measured).", show(o.climbs[0].record[0]), show(o.climbs[1].record[0]), show(o.climbs[2].record[0]), show(o.eigenvalues[0])),
    ];
    html! { <>
        <h3>{ "On the bowl" }</h3>
        { comparison(&["steps to the target", "loss after 1 step", "loss after 5 steps", "loss at the end", "numbers in the ruler", "multiplies per step on P g"], &rows, &sentences) }
        <h3>{ "On the sphere" }</h3>
        { comparison(&["quotient at the end", "distance to the eigenvector at the end", "steps to the tolerance", "farthest off the sphere before repair", "mean off the sphere per step"], &srows, &ssentences) }
        { for text::ANALYSIS.iter().map(|s| p(s)) }
    </> }
}

fn library_asides() -> Html {
    let aside = |name: &str, purpose: &str, reading: &str| match definition("Manifold", name) {
        Some(d) => own_function("mf:", &d, p(purpose), html! { <p>{ reading.to_string() }</p> }),
        None => html! {},
    };
    html! { <>
        { aside("s_phereDescend", "One gradient step that stays on the sphere: the gradient's part along the sphere at p, then the exact step along it at the rate. The green climb is this, step after step, against -2 A p.", "Read right to left: s_phereTangent drops g's part along p; the result times -lr is the tangent vector; s_phereStep carries p along it by the exponential map.") }
        { aside("s_phereRetract", "The cheap step: add the tangent vector and scale back to length 1. The blue climb uses it in place of s_phereStep; the two differ per step by about a third of the cube of the step's length.", "The rows of p plus the rows of v, each scaled to length 1 (p_rojectRows); a vector comes back as a vector (u_nrows).") }
        { aside("s_phereTangent", "The part of a vector that lies along the sphere at a point: the Riemannian gradient on the unit sphere is the plain gradient's tangent part.", "The dot product of each row of V with the matching row of P, times P, taken away from V.") }
        { aside("s_phereOff", "How far a point is from the sphere: what the flat step's repair undoes, and what the table reports before each repair.", "The length of the point less 1, as a size; a set row by row.") }
    </> }
}

/// The preset's numbers, said where the experiment is described (they are data the program read).
fn preset_words(opt: &Result<Opt, String>) -> Html {
    match opt {
        Ok(o) => p(&format!("The preset's numbers: the start ({}, {}); {} steps on the bowl and a target loss of {}; on the sphere {} steps at rate {} and a tolerance of {} in distance. \"At the end\" means after those steps.", show(o.preset.start[0]), show(o.preset.start[1]), o.preset.steps, o.preset.target, o.preset.sphere_steps, o.preset.sphere_rate, o.preset.tolerance)),
        Err(_) => html! {},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let opt = use_memo((), |_| run());
    // Part 2 runs after the first paint, so the reader has part 1 while
    // the model's trainings run; nothing of part 1 waits on it.
    let trained = use_state(|| None::<Result<Training, String>>);
    {
        let trained = trained.clone();
        use_effect_with((), move |_| {
            let timer = Timeout::new(60, move || trained.set(Some(training_run())));
            move || drop(timer)
        });
    }
    let chosen = use_state(|| 1usize);
    let angle = use_state(|| Angle { yaw: 2.1, pitch: 0.73 });
    let src = program();
    let src2 = training_program();
    let linker = Linker::of(&src);
    let linker2 = Linker::of(&src2);
    let pick = { let c = chosen.clone(); Callback::from(move |k: usize| c.set(k)) };
    let turned = { let a = angle.clone(); Callback::from(move |n: Angle| a.set(n)) };
    let (body, after) = match &*opt {
        Err(e) => (html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> }, html! {}),
        Ok(o) => (html! { <>
            { descent_section(o, *chosen, pick) }
            { control_section(o, *chosen) }
            { steepest_section(o) }
            { sphere_section(o, *angle, turned) }
        </> }, analysis(o, *chosen)),
    };
    let body2 = match &*trained {
        None => html! { <section class="panel"><p class="notice" role="status">{ "Part 2 is running: a small model trained ten ways and checked nine more, X_eTaL in your browser; a few seconds. Everything above is part 1, finished." }</p></section> },
        Some(Err(e)) => html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped in part 2: {e}") }</p></section> },
        Some(Ok(t)) => html! { <>{ model::training(t) }{ model::geometry(t) }</> },
    };
    let after2 = match &*trained {
        Some(Ok(t)) => model::analysis(t),
        _ => html! { <p class="notice" role="status">{ "Part 2's comparison follows when it has run." }</p> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("optimization-explorer".into())}>
        <header>{ header("Optimization explorer: is downhill one direction, and does a ruler pay?", text::LEDE) }</header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ prose(text::HINT) }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>{ for text::BACKGROUND.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Rationale, "The idea: keep the loss, change the ruler", html! { <>{ for text::RATIONALE.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ for text::PURPOSE.iter().map(|s| p(s)) }</> }) }
            { frame(INTUITION, "The intuition: a narrow valley and a strange ruler", html! { <>{ for text::INTUITION.iter().map(|s| p(s)) }</> }) }
            { frame(MATHEMATICS, "The mathematics: a metric, a direction, a step, and the sphere", html! { <>{ for text::MATHEMATICS.iter().map(|s| p(s)) }</> }) }
            { frame(EXPERIMENT, "The experiment: three rulers, eight values of a, three climbs", html! { <>{ for text::EXPERIMENT.iter().map(|s| p(s)) }{ preset_words(&opt) }</> }) }
            { frame(RUN, "The run: descent on a formula", html! { <>{ for text::RUN.iter().map(|s| p(s)) }</> }) }
            { body }
            { body2 }
            { frame(ANALYSIS, "The analysis: the rulers compared, the climbs compared, and whether the metric paid on a model", html! { <>{ after }{ after2 }</> }) }
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
                <pre class="preset">{ crate::micro::PRESET }</pre>
                <p class="links"><a href="../doc/demos-optimization-explorer-optimization-explorer.xtl.html">{ "This program in the cross-reference" }</a></p>
                <h3>{ "The program, part 2" }</h3>
                { linker2.block(&src2) }
                <h3>{ "The preset it reads" }</h3>
                <pre class="preset">{ TRAINING_PRESET }</pre>
                <p class="links"><a href="../doc/demos-optimization-explorer-optimization-training.xtl.html">{ "This program in the cross-reference" }</a></p>
            </section>
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
