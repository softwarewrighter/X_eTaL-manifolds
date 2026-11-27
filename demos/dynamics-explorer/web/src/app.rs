//! The page, on the shared teaching shell: the arc of frames; the five
//! fields with their arrows and paths, the field picked by the reader;
//! every field's measures and rest point side by side; the square
//! worked in full; Euler's step against Runge-Kutta's on the rotation,
//! the step size and the steps shown picked by the reader; descent and
//! the game; the comparisons; every idea explained; the program.
//! Presentation only: every number shown is one X_eTaL printed, and
//! picking runs nothing.

use yew::prelude::*;

use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::narrate;
use site::{comparison, functions, mark, Cell, ANALYSIS, EXPERIMENT, EXTENSION, INTUITION, MATHEMATICS, RUN};

use crate::model::{program, run, Dynamics, P2, FIELDS, PRESET, RULES};
use crate::text;

fn show(x: f64) -> String {
    format!("{x:.4}")
}

/// A gap or a distance, small and large numbers kept readable.
fn tiny(x: f64) -> String {
    if x == 0.0 {
        "0".into()
    } else if x.abs() < 0.001 || x.abs() >= 100000.0 {
        format!("{x:.2e}")
    } else {
        format!("{x:.4}")
    }
}

/// A 2 by 2 matrix by rows, as the page writes it.
fn matrix(a: &[f64; 4]) -> String {
    let n = |x: f64| if x == x.round() { format!("{}", x as i64) } else { format!("{x}") };
    format!("({} {} / {} {})", n(a[0]), n(a[1]), n(a[2]), n(a[3]))
}

const STARTS: [&str; 4] = ["#1c7ed6", "#e8590c", "#2b8a3e", "#7048e8"];
const EULER: &str = "#e8590c";
const RK: &str = "#2b8a3e";
const RULE_COLORS: [&str; 4] = ["#868e96", "#e03131", "#1c7ed6", "#2b8a3e"];
const RULE_DASH: [Option<&str>; 4] = [None, None, Some("5 3"), None];
const COUNTS: [usize; 4] = [25, 50, 100, 200];

// -- the plane

/// A square picture of the plane from -range to range on both axes.
struct Plane {
    size: f64,
    range: f64,
}

impl Plane {
    fn at(&self, q: P2) -> (f64, f64) {
        let k = self.size / (2.0 * self.range);
        (self.size / 2.0 + k * q[0], self.size / 2.0 - k * q[1])
    }
    fn line(&self, pts: &[P2], color: &str, width: &str, dash: Option<&str>) -> Html {
        let d: Vec<String> = pts.iter().map(|q| { let (x, y) = self.at(*q); format!("{x:.1},{y:.1}") }).collect();
        html! { <polyline points={d.join(" ")} fill="none" stroke={color.to_string()} stroke-width={width.to_string()} stroke-dasharray={dash.map(|s| s.to_string())} /> }
    }
    fn dot(&self, q: P2, color: &str, r: &str) -> Html {
        let (x, y) = self.at(q);
        html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r={r.to_string()} fill={color.to_string()} /> }
    }
    fn axes(&self) -> Html {
        let s = self.size;
        html! { <>
            <line class="axis" x1="0" y1={format!("{:.1}", s / 2.0)} x2={format!("{s}")} y2={format!("{:.1}", s / 2.0)} />
            <line class="axis" x1={format!("{:.1}", s / 2.0)} y1="0" x2={format!("{:.1}", s / 2.0)} y2={format!("{s}")} />
        </> }
    }
    /// The arrows at the points, every one scaled by one factor so the longest is `longest` units long.
    fn arrows(&self, at: &[P2], v: &[P2], longest: f64, id: &str) -> Html {
        let top = v.iter().map(|w| (w[0] * w[0] + w[1] * w[1]).sqrt()).fold(0.0, f64::max);
        let f = if top > 0.0 { longest / top } else { 0.0 };
        html! { <>
            <defs><marker id={id.to_string()} viewBox="0 0 6 6" refX="5" refY="3" markerWidth="5" markerHeight="5" orient="auto"><path d="M0,0 L6,3 L0,6 z" fill="#adb5bd" /></marker></defs>
            { for at.iter().zip(v).map(|(q, w)| {
                let (x, y) = self.at(*q);
                let (x2, y2) = self.at([q[0] + f * w[0], q[1] + f * w[1]]);
                html! { <line x1={format!("{x:.1}")} y1={format!("{y:.1}")} x2={format!("{x2:.1}")} y2={format!("{y2:.1}")} stroke="#adb5bd" stroke-width="1" marker-end={format!("url(#{id})")} /> }
            }) }
        </> }
    }
}

fn buttons(names: &[String], chosen: usize, label: &str, pick: Callback<usize>) -> Html {
    html! {
        <div class="controls" role="group" aria-label={label.to_string()}>
            { for names.iter().enumerate().map(|(k, n)| {
                let pick = pick.clone();
                html! { <button class={classes!((chosen == k).then_some("on"))} onclick={Callback::from(move |_| pick.emit(k))} aria-pressed={if chosen == k { "true" } else { "false" }}>{ n.clone() }</button> }
            }) }
        </div>
    }
}

// -- the five fields

fn field_picture(d: &Dynamics, k: usize) -> Html {
    let pl = Plane { size: 360.0, range: 3.0 };
    let f = &d.fields[k];
    html! {
        <svg viewBox="0 0 360 360" role="img" aria-label={format!("the {} field as arrows on a grid from -2 to 2, and from four starts the Runge-Kutta paths in four colors with the exact paths as thin dashed gray lines under them; the rest point at the middle", FIELDS[k])}>
            <defs><clipPath id="plane"><rect x="0" y="0" width="360" height="360" /></clipPath></defs>
            { pl.axes() }
            { pl.arrows(&d.grid, &f.arrows, 0.4, "fhead") }
            <g clip-path="url(#plane)">
                { for f.exact.iter().map(|e| pl.line(e, "#495057", "1", Some("2 3"))) }
                { for f.rk.iter().enumerate().map(|(j, r)| pl.line(r, STARTS[j], "2", None)) }
            </g>
            { for d.preset.starts.iter().enumerate().map(|(j, q)| pl.dot(*q, STARTS[j], "4")) }
            { pl.dot([0.0, 0.0], "#212529", "3") }
            <text class="axis-lbl" x="4" y="12">{ "x across, y up, -3 to 3; the grid of arrows spans -2 to 2" }</text>
        </svg>
    }
}

fn field_numbers(d: &Dynamics, k: usize) -> Html {
    let f = &d.fields[k];
    let (m, s, l) = (f.measures, f.stability, f.loop_);
    html! { <>
        <h3>{ format!("The {}: A = {}", FIELDS[k], matrix(&f.a)) }{ mark(true) }</h3>
        <table class="numbers">
            <thead><tr><th>{ "measure" }</th><th>{ "exact, from A" }</th><th>{ "by differences, mean over the grid" }</th><th>{ "largest gap at a grid point" }</th></tr></thead>
            <tbody>
                <tr><td>{ "divergence, the trace a + d" }</td><td>{ show(m.div) }</td><td>{ show(m.div_mean) }</td><td>{ tiny(m.div_gap) }</td></tr>
                <tr><td>{ "curl, c - b" }</td><td>{ show(m.curl) }</td><td>{ show(m.curl_mean) }</td><td>{ tiny(m.curl_gap) }</td></tr>
            </tbody>
        </table>
        <p class="note">{ format!("The sum round the loop on the unit circle: {} (the curl times the area inside, {}: Green's theorem). The flow out across it: {} (the divergence times the area, {}). The field is {}symmetric, so it {} minus the gradient of a loss.", tiny(l.sum), tiny(l.curl_area), tiny(l.out), tiny(l.div_area), if f.symmetric { "" } else { "not " }, if f.symmetric { "is" } else { "is not" }) }</p>
        <p class="note">{ format!("At the rest point: trace {}, determinant {}, eigenvalues with real parts {} and {}{}; {}. What the paths did: {} (their distance at the end over at the start, from {} to {}).", show(s.trace), show(s.det), show(s.re[0]), show(s.re[1]), if s.im > 0.0 { format!(" and imaginary parts +- {}", show(s.im)) } else { String::new() }, d.says(k), d.did(k), tiny(s.smallest), tiny(s.largest)) }</p>
        <p class="note">{ format!("The Runge-Kutta paths against the exact ones: the largest gap, relative, is {} (the tolerance is {}).", tiny(f.path_error), d.preset.tol) }</p>
    </> }
}

fn fields_section(d: &Dynamics, chosen: usize, before: usize, pick: Callback<usize>) -> Html {
    let names: Vec<String> = FIELDS.iter().map(|s| s.to_string()).collect();
    let sentences: Vec<String> = if before == chosen {
        vec![format!("Showing the {}. Pick another field to read what changes", FIELDS[chosen])]
    } else {
        let (a, b) = (&d.fields[before], &d.fields[chosen]);
        let from = format!("from the {} to the {}", FIELDS[before], FIELDS[chosen]);
        vec![
            narrate::moved(&format!("The divergence, {from},"), a.measures.div, b.measures.div),
            narrate::moved(&format!("The curl, {from},"), a.measures.curl, b.measures.curl),
            narrate::moved(&format!("The sum round the loop, {from},"), a.loop_.sum, b.loop_.sum),
            narrate::moved(&format!("The largest real part of an eigenvalue, {from},"), a.stability.re[0], b.stability.re[0]),
            narrate::moved(&format!("The largest end-over-start distance of a path, {from},"), a.stability.largest, b.stability.largest),
        ]
    };
    html! {
        <section class="space" id="fields">
            <h2>{ "Five fields, their arrows and their paths" }</h2>
            { p("Pick a field. The arrows show it at a grid of points; from four starts (the dots), the paths of a state that follows it, by Runge-Kutta steps, with the exact paths under them. The table beside says what the field's matrix predicts, and what the paths did.") }
            { buttons(&names, chosen, "the field to show", pick) }
            <div class="figure">
                <figure class="flat">
                    { field_picture(d, chosen) }
                    <figcaption>{ format!("The {} v(x) = A x with A = {}: arrows at a 9 by 9 grid (measured: the field at each point, all scaled by one factor so the longest fits), the paths from four starts by Runge-Kutta, {} steps of {} (measured), and the exact paths dashed under them (analytical, from exp(t A); where the two agree the dashes are hidden). A path that leaves the picture is cut at its edge.", FIELDS[chosen], matrix(&d.fields[chosen].a), d.preset.path_steps, d.preset.h) }{ mark(true) }{ mark(false) }</figcaption>
                </figure>
                <figure>
                    { field_numbers(d, chosen) }
                    { for sentences.iter().map(|s| html! { <p class="narrated">{ format!("{s}.") }</p> }) }
                </figure>
            </div>
        </section>
    }
}

fn all_fields(d: &Dynamics) -> Html {
    html! {
        <section class="space" id="rest-points">
            <h2>{ "All five side by side: the measures and the rest point" }</h2>
            { p("Each field's divergence and curl (exact, from the matrix; the differences agree, as the table above shows field by field), the sum round the loop beside the curl times the area, and the rest point: what the trace and determinant predict, against what the paths did.") }
            <table class="numbers">
                <thead><tr><th>{ "field" }</th><th>{ "divergence" }</th><th>{ "curl" }</th><th>{ "sum round the loop" }</th><th>{ "curl times area" }</th><th>{ "flow out" }</th><th>{ "divergence times area" }</th></tr></thead>
                <tbody>
                    { for d.fields.iter().enumerate().map(|(k, f)| html! {
                        <tr><td>{ FIELDS[k] }</td><td>{ show(f.measures.div) }</td><td>{ show(f.measures.curl) }</td><td>{ tiny(f.loop_.sum) }</td><td>{ tiny(f.loop_.curl_area) }</td><td>{ tiny(f.loop_.out) }</td><td>{ tiny(f.loop_.div_area) }</td></tr>
                    }) }
                </tbody>
            </table>
            <table class="numbers">
                <thead><tr><th>{ "field" }</th><th>{ "trace T" }</th><th>{ "determinant D" }</th><th>{ "real parts" }</th><th>{ "imaginary part" }</th><th>{ "what they say" }</th><th>{ "end over start, smallest to largest" }</th><th>{ "what the paths did" }</th></tr></thead>
                <tbody>
                    { for d.fields.iter().enumerate().map(|(k, f)| html! {
                        <tr><td>{ FIELDS[k] }</td><td>{ show(f.stability.trace) }</td><td>{ show(f.stability.det) }</td><td>{ format!("{}, {}", show(f.stability.re[0]), show(f.stability.re[1])) }</td><td>{ if f.stability.im > 0.0 { format!("+- {}", show(f.stability.im)) } else { "0 (real)".into() } }</td><td>{ d.says(k) }</td><td>{ format!("{} to {}", tiny(f.stability.smallest), tiny(f.stability.largest)) }</td><td>{ d.did(k) }</td></tr>
                    }) }
                </tbody>
            </table>
            <p class="note">{ format!("All measured but the area, {} (the polygon of {} pieces on the unit circle: a little under pi). The eigenvalues are T / 2 +- sqrt(T^2 / 4 - D), complex when T^2 / 4 < D. The saddle's paths: the start on the y axis comes in along it, the others leave along x. The rotation's paths end as far out as they started to within Runge-Kutta's slow spiral in, under a part in a million over these steps (the program's check allows a part in a thousand).", show(d.area), d.preset.loop_n) }{ mark(true) }</p>
        </section>
    }
}

// -- the square, worked

fn square_picture(d: &Dynamics) -> Html {
    let pl = Plane { size: 300.0, range: 1.6 };
    let rot = &d.fields[1];
    let inside: Vec<(P2, P2)> = d.grid.iter().zip(&rot.arrows).filter(|(q, _)| q[0].abs() <= 1.0 && q[1].abs() <= 1.0).map(|(q, v)| (*q, *v)).collect();
    let (at, v): (Vec<P2>, Vec<P2>) = inside.into_iter().unzip();
    html! {
        <svg viewBox="0 0 300 300" role="img" aria-label="the square with corners (-1, -1) and (1, 1) walked anticlockwise, the rotation's arrows inside it, and each side labeled with what it adds to the sum round the loop">
            { pl.axes() }
            { pl.arrows(&at, &v, 0.22, "shead") }
            { pl.line(&d.square, "#7048e8", "2.5", None) }
            { for d.square_work.iter().zip(&d.sides_rotation).map(|(w, s)| {
                let (x, y) = pl.at([w[0] * 1.25, w[1] * 1.25]);
                html! { <text class="lbl" x={format!("{:.1}", x - 12.0)} y={format!("{:.1}", y + 4.0)}>{ format!("+{}", show(*s)) }</text> }
            }) }
            <text class="axis-lbl" x="4" y="12">{ "the rotation, walked anticlockwise" }</text>
        </svg>
    }
}

fn square_section(d: &Dynamics) -> Html {
    let w = &d.square_work;
    html! {
        <section class="space" id="loop">
            <h2>{ "Round a loop, worked in full: the rotation has no height" }</h2>
            { p("If the rotation were minus the gradient of some loss, walking round a loop and adding up v . dx would bring the \"height\" back to where it started. Walk the square with corners (-1, -1), (1, -1), (1, 1), (-1, 1) anticlockwise, one straight piece per side; for a field this simple the arrow at the middle of a side, dotted with the side, is the side's whole sum.") }
            <div class="figure">
                <figure class="flat">
                    { square_picture(d) }
                    <figcaption>{ "The square and the rotation's arrows inside it (measured), each side labeled with its v . dx (measured)." }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <div class="worked-out">
                        <h3>{ "Worked in full" }{ mark(true) }</h3>
                        <table class="numbers">
                            <thead><tr><th>{ "side" }</th><th>{ "its middle" }</th><th>{ "the side as a step" }</th><th>{ "the rotation's arrow (-y, x) at the middle" }</th><th>{ "v . step" }</th><th>{ "height after it" }</th><th>{ "the attractor's arrow (-x, -y / 2)" }</th><th>{ "v . step" }</th></tr></thead>
                            <tbody>
                                { for (0..4).map(|i| html! {
                                    <tr><td>{ ["bottom", "right", "top", "left"][i] }</td><td>{ format!("({}, {})", show(w[i][0]), show(w[i][1])) }</td><td>{ format!("({}, {})", show(w[i][2]), show(w[i][3])) }</td><td>{ format!("({}, {})", show(w[i][4]), show(w[i][5])) }</td><td>{ show(d.sides_rotation[i]) }</td><td>{ show(d.height_rotation[i]) }</td><td>{ format!("({}, {})", show(w[i][6]), show(w[i][7])) }</td><td>{ show(d.sides_attractor[i]) }</td></tr>
                                }) }
                            </tbody>
                        </table>
                        <p>{ format!("The bottom side, for the rotation: its middle is ({}, {}), where the arrow is (-y, x) = ({}, {}); the side as a step is ({}, {}); the dot product is {} x {} + {} x {} = {}. Every side adds the same, since at the middle of each side the arrow points along the side, the way the walk goes, so the \"height\" (minus the running sum) ends the loop at {}, not where it started: no loss has this field as its downhill direction. Green's theorem gives the same total: the curl, {}, times the area inside, 4.", show(w[0][0]), show(w[0][1]), show(w[0][4]), show(w[0][5]), show(w[0][2]), show(w[0][3]), show(w[0][4]), show(w[0][2]), show(w[0][5]), show(w[0][3]), show(d.sides_rotation[0]), show(d.height_rotation[3]), show(d.fields[1].measures.curl)) }</p>
                        <p>{ format!("The attractor is minus the gradient of L = x^2 / 2 + y^2 / 4. Its sides add {}, {}, {} and {}: each side adds L at its start minus L at its end, and on this square L is the same at both ends of every side. Round any loop it adds 0 for the same reason. Checked over the grid: the attractor's arrows are minus the gradient of that L, and the saddle's minus the gradient of L = -(x^2 - y^2) / 2, by central differences, to {} and {}.", show(d.sides_attractor[0]), show(d.sides_attractor[1]), show(d.sides_attractor[2]), show(d.sides_attractor[3]), tiny(d.gradient_gaps[0]), tiny(d.gradient_gaps[1])) }</p>
                    </div>
                </figure>
            </div>
        </section>
    }
}

// -- Euler against Runge-Kutta

fn rotation_picture(d: &Dynamics, s: usize, count: usize) -> Html {
    let pl = Plane { size: 320.0, range: 2.0 };
    let r = &d.rotations[s];
    html! {
        <svg viewBox="0 0 320 320" role="img" aria-label={format!("the unit circle, and from (1, 0) the first {count} Euler steps of size {} in orange and Runge-Kutta steps in green", r.size)}>
            <defs><clipPath id="rot"><rect x="0" y="0" width="320" height="320" /></clipPath></defs>
            { pl.axes() }
            { pl.line(&d.circle, "#ced4da", "6", None) }
            <g clip-path="url(#rot)">
                { pl.line(&r.euler[..=count], EULER, "1.5", None) }
                { pl.line(&r.rk[..=count], RK, "2", Some("5 3")) }
            </g>
            { pl.dot(r.euler[count], EULER, "3.5") }
            { pl.dot(r.rk[count], RK, "3.5") }
            { pl.dot(d.preset.r0, "#212529", "3.5") }
            <text class="axis-lbl" x="4" y="12">{ "x across, y up, -2 to 2; the wide gray ring is the unit circle" }</text>
        </svg>
    }
}

/// The distance from the circle by step on a log scale, for both integrators at one size, up to the steps shown.
fn off_chart(d: &Dynamics, s: usize, count: usize) -> Html {
    let (w, hh) = (420.0, 230.0);
    let r = &d.rotations[s];
    let (lo, hi) = (-10.0_f64, 7.0_f64);
    let at = |k: usize, v: f64| {
        let x = 34.0 + (w - 44.0) * k as f64 / count as f64;
        let l = v.abs().max(1e-10).log10().min(hi);
        format!("{x:.1},{:.1}", hh - 24.0 - (hh - 36.0) * (l - lo) / (hi - lo))
    };
    let line = |v: &[f64], color: &str, dash: Option<&str>| {
        let pts: Vec<String> = v[1..=count].iter().enumerate().map(|(i, x)| at(i + 1, *x)).collect();
        html! { <polyline points={pts.join(" ")} fill="none" stroke={color.to_string()} stroke-width="2" stroke-dasharray={dash.map(|s| s.to_string())} /> }
    };
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label={format!("the distance from the circle by step, on a logarithmic scale from 1e-10 to 1e7, for Euler steps of size {} (orange) and Runge-Kutta steps (green dashed), the first {count} steps", r.size)}>
            <line class="axis" x1="34" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="34" y1="12" x2="34" y2={format!("{:.1}", hh - 24.0)} />
            <text class="axis-lbl" x="38" y="12">{ "distance from the circle, 1e7 at the top to 1e-10 at the bottom (log scale)" }</text>
            <text class="axis-lbl" x="34" y={format!("{:.1}", hh - 8.0)}>{ "step 1" }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("step {count}") }</text>
            { line(&r.euler_off, EULER, None) }
            { line(&r.rk_off, RK, Some("5 3")) }
        </svg>
    }
}

fn rotation_section(d: &Dynamics, s: usize, s_before: usize, c: usize, c_before: usize, pick_s: Callback<usize>, pick_c: Callback<usize>) -> Html {
    let count = COUNTS[c];
    let r = &d.rotations[s];
    let size_names: Vec<String> = d.preset.sizes.iter().map(|x| format!("step {x}")).collect();
    let count_names: Vec<String> = COUNTS.iter().map(|n| format!("{n} steps")).collect();
    let mut sentences = Vec::new();
    if s != s_before {
        let b = &d.rotations[s_before];
        sentences.push(narrate::moved(&format!("Euler's distance from the circle after {count} steps, from step size {} to {},", b.size, r.size), b.euler_off[count], r.euler_off[count]));
        sentences.push(narrate::moved(&format!("Runge-Kutta's, the same,"), b.rk_off[count], r.rk_off[count]));
    } else if c != c_before {
        let b = COUNTS[c_before];
        sentences.push(narrate::moved(&format!("Euler's distance from the circle at step size {}, from {b} steps to {count},", r.size), r.euler_off[b], r.euler_off[count]));
        sentences.push(narrate::moved("Runge-Kutta's, the same,", r.rk_off[b], r.rk_off[count]));
    } else {
        sentences.push("Pick another step size or number of steps to read what changes.".into());
    }
    let sw = d.same_work;
    html! {
        <section class="space" id="integrators">
            <h2>{ "Following a field: Euler's step against Runge-Kutta's on the rotation" }</h2>
            { p("The rotation's exact paths are circles, so a path's distance from the circle it started on is its error, readable at every step. From (1, 0), on the unit circle, both integrators at the chosen step size; the second row of buttons chooses how many of the steps are drawn.") }
            { buttons(&size_names, s, "the step size of both integrators", pick_s) }
            { buttons(&count_names, c, "how many steps to show", pick_c) }
            <div class="figure">
                <figure class="flat">
                    { rotation_picture(d, s, count) }
                    <figcaption>{ format!("Euler (orange) and Runge-Kutta (green, dashed) at step {}, the first {count} steps (measured); the unit circle (analytical: the polygon of the loop, {} pieces). An Euler path that leaves the picture is cut at its edge; the dots are where each stands after {count} steps.", r.size, d.preset.loop_n) }{ mark(true) }{ mark(false) }</figcaption>
                </figure>
                <figure class="flat">
                    { off_chart(d, s, count) }
                    <figcaption>{ "The size of the distance from the circle by step, on a logarithmic scale (measured): Euler's is outward, Runge-Kutta's inward. A distance below the chart's bottom is drawn on it." }{ mark(true) }</figcaption>
                </figure>
            </div>
            <div class="figure">
                <figure>
                    { for sentences.iter().map(|x| html! { <p class="narrated">{ if x.ends_with('.') { x.clone() } else { format!("{x}.") } }</p> }) }
                    <table class="numbers">
                        <thead><tr><th>{ "step size" }</th><th>{ "Euler's factor per step, sqrt(1 + s^2)" }</th><th>{ "Runge-Kutta's" }</th><th>{ format!("Euler's distance after {} steps: from the factor, measured", d.preset.rot_steps) }</th><th>{ "Runge-Kutta's: from the factor, measured" }</th></tr></thead>
                        <tbody>
                            { for d.rotations.iter().enumerate().map(|(j, x)| html! {
                                <tr class={classes!((j == s).then_some("sum"))}><td>{ x.size }</td><td>{ format!("{:.6}", x.euler_factor) }</td><td>{ format!("{:.12}", x.rk_factor) }</td><td>{ format!("{}; {}", tiny(x.predicted[0]), tiny(x.measured[0])) }</td><td>{ format!("{}; {}", tiny(x.predicted[1]), tiny(x.measured[1])) }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    <p class="note">{ format!("The factors are analytical (the formulas above, computed by the program), the distances measured (positive outside the circle, negative inside); they agree to rounding. With the same work: Euler at step {} for {} steps and Runge-Kutta at step {} for {} steps both cover time {} and look at the field {} times; Euler ends {} from the circle, Runge-Kutta {}.", d.preset.sizes[1], sw[2] as i64, d.preset.sizes[3], sw[6] as i64, show(sw[0]), sw[3] as i64, tiny(sw[4]), tiny(sw[5])) }{ mark(true) }</p>
                </figure>
            </div>
        </section>
    }
}

// -- descent and the game

fn game_picture(d: &Dynamics) -> Html {
    let pl = Plane { size: 340.0, range: 2.2 };
    html! {
        <svg viewBox="0 0 340 340" role="img" aria-label="from the start (1, 1) the four rules' paths: gradient descent on the loss gray, straight in; simultaneous steps on the game red, spiraling out; taking turns blue dashed, circling; the look-ahead step green, spiraling in">
            <defs><clipPath id="game"><rect x="0" y="0" width="340" height="340" /></clipPath></defs>
            { pl.axes() }
            <g clip-path="url(#game)">
                { for d.rules.iter().enumerate().map(|(k, r)| pl.line(&r.path, RULE_COLORS[k], "1.6", RULE_DASH[k])) }
            </g>
            { pl.dot(d.preset.game0, "#212529", "4") }
            { pl.dot([0.0, 0.0], "#212529", "2.5") }
            <text class="lbl" x="176" y="184">{ "solution" }</text>
            <text class="axis-lbl" x="4" y="12">{ "x (player one) across, y (player two) up, -2.2 to 2.2" }</text>
        </svg>
    }
}

fn game_chart(d: &Dynamics) -> Html {
    let (w, hh) = (420.0, 230.0);
    let n = d.preset.game_steps;
    let (lo, hi) = (-10.0_f64, 2.0_f64);
    let at = |k: usize, v: f64| {
        let x = 34.0 + (w - 44.0) * k as f64 / n as f64;
        let l = v.max(1e-10).log10().min(hi);
        format!("{x:.1},{:.1}", hh - 24.0 - (hh - 36.0) * (l - lo) / (hi - lo))
    };
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the distance from the solution by step for the four rules on a logarithmic scale from 1e-10 to 100: descent gray falling fast, simultaneous red rising, turns blue dashed level, look-ahead green falling slowly">
            <line class="axis" x1="34" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="34" y1="12" x2="34" y2={format!("{:.1}", hh - 24.0)} />
            <text class="axis-lbl" x="38" y="12">{ "distance from the solution, 100 at the top to 1e-10 at the bottom (log scale)" }</text>
            <text class="axis-lbl" x="34" y={format!("{:.1}", hh - 8.0)}>{ "step 0" }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("step {n}") }</text>
            { for d.rules.iter().enumerate().map(|(k, r)| {
                let pts: Vec<String> = r.distance.iter().enumerate().map(|(i, v)| at(i, *v)).collect();
                html! { <polyline points={pts.join(" ")} fill="none" stroke={RULE_COLORS[k]} stroke-width="2" stroke-dasharray={RULE_DASH[k].map(|s| s.to_string())} /> }
            }) }
        </svg>
    }
}

fn game_section(d: &Dynamics) -> Html {
    let e = &d.turns_energy;
    let pr = &d.preset;
    html! {
        <section class="space" id="game">
            <h2>{ "As machine learning meets it: descent on a loss, and a two-player game" }</h2>
            { p(&format!("From the start ({}, {}) at rate {}, {} steps of each rule. Gradient descent on the loss L = (x^2 + y^2) / 2 follows a gradient field. The other three play the game f(x, y) = x y: player one lowers f with x, player two raises it with y. The game's field, the two players' velocities together, is the rotation: the program checked it against central differences of f at the grid (largest gap {}).", pr.game0[0], pr.game0[1], pr.eta, pr.game_steps, tiny(d.game_field_gap))) }
            <div class="figure">
                <figure class="flat">
                    { game_picture(d) }
                    <figcaption>{ "The four paths from the start (measured): descent gray, simultaneous steps red, taking turns blue dashed, the look-ahead step green. A path that leaves the picture is cut at its edge." }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    { game_chart(d) }
                    <figcaption>{ "The distance from the solution (0, 0) by step, on a logarithmic scale (measured); a distance below the chart's bottom is drawn on it." }{ mark(true) }</figcaption>
                </figure>
            </div>
            <div class="figure">
                <figure>
                    <table class="numbers">
                        <thead><tr><th>{ "rule" }</th><th>{ "its matrix M (p <- M p)" }</th><th>{ "factor per step, from M's trace and determinant" }</th><th>{ "the point after one step" }</th><th>{ format!("distance after {} steps", pr.game_steps) }</th></tr></thead>
                        <tbody>
                            { for d.rules.iter().enumerate().map(|(k, r)| html! {
                                <tr><td>{ RULES[k] }</td><td>{ matrix(&r.matrix) }</td><td>{ format!("{:.6}", r.factor) }</td><td>{ format!("({}, {})", show(r.path[1][0]), show(r.path[1][1])) }</td><td>{ tiny(r.distance[pr.game_steps]) }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    <p class="note">{ format!("The first step worked, from (1, 1) at rate {}: simultaneous, x = 1 - {} x 1 and y = 1 + {} x 1, both players answering the old point; taking turns, x first, then y = 1 + {} times the new x; the look-ahead step goes first to the simultaneous point, then from (1, 1) along the slopes found there: x = 1 - {} times the trial y, y = 1 + {} times the trial x. Taking turns keeps E = x^2 + y^2 - eta x y fixed: {} at the start, {} at the end (measured), so its points stay on one ellipse round the solution and its distance moves within a band, between {} and {} (measured): its factor 1 is the size of its matrix's eigenvalues, not a fixed distance. The factors are analytical; the distances at the end are those factors to the power of the steps times the start's distance, sqrt 2, which the program checks for the three rules whose matrix is a multiple of a rotation.", pr.eta, pr.eta, pr.eta, pr.eta, pr.eta, pr.eta, show(e[0]), show(e[e.len() - 1]), tiny(d.turns_band[0]), tiny(d.turns_band[1])) }{ mark(true) }{ mark(false) }</p>
                </figure>
            </div>
        </section>
    }
}

// -- the analysis

fn analysis(d: &Dynamics) -> Html {
    let sw = d.same_work;
    let r2 = &d.rotations[1];
    let irows = [
        ("Euler's step", vec![Cell::analytical("1"), Cell::analytical(format!("{:.6}", r2.euler_factor)), Cell::measured(tiny(r2.measured[0])), Cell::measured(tiny(sw[4]))]),
        ("fourth-order Runge-Kutta", vec![Cell::analytical("4"), Cell::analytical(format!("{:.12}", r2.rk_factor)), Cell::measured(tiny(r2.measured[1])), Cell::measured(tiny(sw[5]))]),
    ];
    let isent = vec![
        format!("At step {} for {} steps, Euler ends {} from the circle and Runge-Kutta {} (measured).", r2.size, d.preset.rot_steps, tiny(r2.measured[0]), tiny(r2.measured[1])),
        format!("With the same {} looks at the field over the same time {}, Euler at step {} ends {} from the circle and Runge-Kutta at step {} ends {} (measured).", sw[3] as i64, show(sw[0]), d.preset.sizes[1], tiny(sw[4]), d.preset.sizes[3], tiny(sw[5])),
    ];
    let looks = ["1", "1", "1", "2"];
    let grows: Vec<(&str, Vec<Cell>)> = d.rules.iter().enumerate().map(|(k, r)| (RULES[k], vec![Cell::analytical(looks[k]), Cell::analytical(format!("{:.6}", r.factor)), Cell::measured(tiny(r.distance[d.preset.game_steps]))])).collect();
    let g = &d.rules;
    let n = d.preset.game_steps;
    let gsent = vec![
        format!("After {n} steps from the same start, gradient descent on the loss is {} from the solution and simultaneous steps on the game {} (measured): the same eta, one field from a loss and one that turns.", tiny(g[0].distance[n]), tiny(g[1].distance[n])),
        format!("Taking turns ends {} from the solution, having stayed between {} and {} throughout; the look-ahead step ends {} (measured).", tiny(g[2].distance[n]), tiny(d.turns_band[0]), tiny(d.turns_band[1]), tiny(g[3].distance[n])),
        format!("The start was {} from the solution for every rule (measured).", tiny(g[0].distance[0])),
    ];
    html! { <>
        <h3>{ "Two integrators on the rotation" }</h3>
        { comparison(&["looks at the field per step", "factor per step at step 0.1", "distance from the circle at the end, step 0.1", "distance at the end with the same work"], &irows, &isent) }
        <h3>{ "Four rules from one start at one rate" }</h3>
        { comparison(&["looks at the slopes per step", "factor per step", "distance from the solution at the end"], &grows, &gsent) }
        { for text::ANALYSIS.iter().map(|s| p(s)) }
    </> }
}

fn preset_words(d: &Result<Dynamics, String>) -> Html {
    match d {
        Ok(d) => { let pr = &d.preset; p(&format!("The preset's numbers: the four starts ({}, {}), ({}, {}), ({}, {}), ({}, {}); Runge-Kutta steps of {} for {} steps; a grid of {} by {} arrows from -{} to {}; central differences with h = {}; a loop of {} pieces; the rotation from ({}, {}) at step sizes {}, {}, {} and {}, {} steps each; the game from ({}, {}) at rate {} for {} steps; a tolerance of {} for the paths.", pr.starts[0][0], pr.starts[0][1], pr.starts[1][0], pr.starts[1][1], pr.starts[2][0], pr.starts[2][1], pr.starts[3][0], pr.starts[3][1], pr.h, pr.path_steps, pr.side, pr.side, pr.half, pr.half, pr.dh, pr.loop_n, pr.r0[0], pr.r0[1], pr.sizes[0], pr.sizes[1], pr.sizes[2], pr.sizes[3], pr.rot_steps, pr.game0[0], pr.game0[1], pr.eta, pr.game_steps, pr.tol)) }
        Err(_) => html! {},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let dyn_ = use_memo((), |_| run());
    let field = use_state(|| (1usize, 1usize));
    let size = use_state(|| (1usize, 1usize));
    let count = use_state(|| (1usize, 1usize));
    let src = program();
    let linker = Linker::of(&src);
    let pick_field = { let f = field.clone(); Callback::from(move |k: usize| f.set((k, f.0))) };
    let pick_size = { let s = size.clone(); let c = count.clone(); Callback::from(move |k: usize| { s.set((k, s.0)); c.set((c.0, c.0)); }) };
    let pick_count = { let c = count.clone(); let s = size.clone(); Callback::from(move |k: usize| { c.set((k, c.0)); s.set((s.0, s.0)); }) };
    let (body, after) = match &*dyn_ {
        Err(e) => (html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> }, html! {}),
        Ok(d) => (html! { <>
            { fields_section(d, field.0, field.1, pick_field) }
            { all_fields(d) }
            { square_section(d) }
            { rotation_section(d, size.0, size.1, count.0, count.1, pick_size, pick_count) }
            { game_section(d) }
        </> }, analysis(d)),
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("dynamics-explorer".into())}>
        <header>{ header("Dynamics explorer: must a state head downhill?", text::LEDE) }</header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ prose(text::HINT) }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>{ for text::BACKGROUND.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Rationale, "The idea: change the field, then change how it is followed", html! { <>{ for text::RATIONALE.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ for text::PURPOSE.iter().map(|s| p(s)) }</> }) }
            { frame(INTUITION, "The intuition: a leaf on a pond, a ball on a hill, two hands on one dial", html! { <>{ for text::INTUITION.iter().map(|s| p(s)) }</> }) }
            { frame(MATHEMATICS, "The mathematics: a field, its exact paths, two integrators, divergence, curl, a loop, a rest point, a game", html! { <>{ for text::MATHEMATICS.iter().map(|s| p(s)) }</> }) }
            { frame(EXPERIMENT, "The experiment: five fields, two integrators at four step sizes, four rules", html! { <>{ for text::EXPERIMENT.iter().map(|s| p(s)) }{ preset_words(&dyn_) }</> }) }
            { frame(RUN, "The run: the fields followed, the loops walked, the game played", html! { <>{ for text::RUN.iter().map(|s| p(s)) }</> }) }
            { body }
            { frame(ANALYSIS, "The analysis: the integrators compared, the rules compared", after) }
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
                <h3>{ "Every library function the program calls" }</h3>
                { functions(&src) }
                <h3>{ "The program" }</h3>
                { linker.block(&src) }
                <h3>{ "The preset it reads" }</h3>
                <pre class="preset">{ PRESET }</pre>
                <p class="links"><a href="../doc/demos-dynamics-explorer-dynamics-explorer.xtl.html">{ "This program in the cross-reference" }</a></p>
            </section>
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
