//! The page, on the shared teaching shell (X_eTaL-ML's lesson, ask GM1)
//! and its turning view (microscope::turning, ask GM3): the question,
//! what to know first, the idea, what the page is; the intuition, the
//! mathematics, the experiment, the run (none), the analysis, the
//! extension; the five spaces, each with its turning view, its flat
//! view and its numbers; the summary, what is next, every idea
//! explained, the program. Words open plain explanations; every number
//! shown is one X_eTaL printed. Presentation only.

use yew::prelude::*;

use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::definition;
use microscope::turning::{Angle, Point, Scene, Segment, Turning};
use site::{comparison, functions, mark, own_function, Cell, ANALYSIS, EXPERIMENT, EXTENSION, INTUITION, MATHEMATICS, RUN};

use crate::micro::{program, run, Pair, Tour, P2, P3, SPACES};
use crate::text;

/// A number as the page quotes it: four decimals (the run's full
/// digits are in the program's output).
fn show(x: f64) -> String {
    format!("{x:.4}")
}

fn f1(x: f64) -> String {
    format!("{x:.1}")
}

// -- the turning views, through the shared view --

/// The shared view turns about the vertical axis, y. The sphere and
/// the torus are written with z up, so their points are turned a
/// quarter turn about x for drawing, (x, y, z) to (x, z, -y): a
/// rotation, so the picture keeps its handedness and no number changes
/// its meaning. The Swiss roll has its height as y already.
#[derive(Clone, Copy, PartialEq)]
pub enum Up {
    Z,
    Y,
}

pub fn upright(p: P3, up: Up) -> [f64; 3] {
    match up {
        Up::Z => [p[0], p[2], -p[1]],
        Up::Y => p,
    }
}

/// A space's scene: its wire lines (analytical), its paths (the arcs,
/// measured), its chords (straight lines through the room) and its
/// labeled points, all from the run.
fn scene(wires: &[Vec<P3>], paths: &[(&[P3], &str)], chords: &[(P3, P3)], points: &[(P3, &str, &str)], up: Up) -> Scene {
    let mut segments: Vec<Segment> = chords.iter().map(|(a, b)| Segment { from: upright(*a, up), to: upright(*b, up), color: "#868e96".into(), arrow: false }).collect();
    for (path, color) in paths {
        for w in path.windows(2) {
            segments.push(Segment { from: upright(w[0], up), to: upright(w[1], up), color: color.to_string(), arrow: false });
        }
    }
    Scene {
        points: points.iter().map(|(at, label, color)| Point { at: upright(*at, up), color: color.to_string(), label: label.to_string() }).collect(),
        segments,
        wires: wires.iter().map(|w| w.iter().map(|p| upright(*p, up)).collect()).collect(),
    }
}

const NEAR: &str = "#2b8a3e";
const FAR: &str = "#e8590c";
const INK: &str = "#868e96";

#[derive(Properties, PartialEq)]
struct ViewProps {
    name: &'static str,
    scene: Scene,
    start: Angle,
}

/// One turning view with its own angle (page state) and its caption.
#[function_component(View)]
fn view(props: &ViewProps) -> Html {
    let angle = use_state(|| props.start);
    let turned = { let a = angle.clone(); Callback::from(move |n: Angle| a.set(n)) };
    let (yaw, pitch) = ((angle.yaw.to_degrees().round() as i32).rem_euclid(360), angle.pitch.to_degrees().round() as i32);
    html! {
        <figure class="turning">
            <Turning scene={props.scene.clone()} angle={*angle} {turned} size={420.0}
                label={format!("the {} in three dimensions, turned {yaw} degrees round and {pitch} degrees toward you; drag, use the arrow keys or swipe to turn it", props.name)} />
            <figcaption>
                { format!("The {} in three dimensions, turned ", props.name) }
                <span class="state">{ format!("{yaw} degrees round and {pitch} degrees toward you") }</span>
                { ". Drag it, press the arrow keys once it has the focus (click or tab to it), or swipe, to turn it; turning changes no number on this page, only the side you see. The shared view of these repositories: a mild perspective, the nearest drawn last." }
                { mark(false) }
            </figcaption>
        </figure>
    }
}

// -- the flat views --

fn polyline(points: &[P2], class: &str, at: &dyn Fn(P2) -> (f64, f64)) -> Html {
    let pts: Vec<String> = points.iter().map(|q| { let (x, y) = at(*q); format!("{x:.1},{y:.1}") }).collect();
    html! { <polyline class={class.to_string()} points={pts.join(" ")} /> }
}

fn dot(q: P2, label: &str, class: &str, at: &dyn Fn(P2) -> (f64, f64)) -> Html {
    let (x, y) = at(q);
    html! { <g>
        <circle class={classes!("pt", class.to_string())} cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="4" />
        <text class="lbl" x={format!("{:.1}", x + 6.0)} y={format!("{:.1}", y - 5.0)}>{ label.to_string() }</text>
    </g> }
}

const W: f64 = 420.0;

fn circle_flat(t: &Tour) -> Html {
    let two_pi = t.circle_angles[t.circle_angles.len() - 1];
    let h = 120.0;
    let at = |q: P2| (30.0 + (W - 60.0) * q[0] / two_pi, h / 2.0 + q[1]);
    let ticks = t.circle_angles.iter().step_by(6).map(|a| {
        let (x, y) = at([*a, 0.0]);
        html! { <line class="axis" x1={format!("{x:.1}")} y1={format!("{:.1}", y - 4.0)} x2={format!("{x:.1}")} y2={format!("{:.1}", y + 4.0)} /> }
    });
    html! {
        <svg viewBox={format!("0 0 {W} {h}")} role="img" aria-label="the circle's map: the angle from 0 to two pi, with the near pair and the far pair marked">
            <line class="axis" x1={format!("{:.1}", at([0.0, 0.0]).0)} y1={format!("{:.1}", h / 2.0)} x2={format!("{:.1}", at([two_pi, 0.0]).0)} y2={format!("{:.1}", h / 2.0)} />
            { for ticks }
            <text class="axis-lbl" x="30" y={format!("{:.1}", h / 2.0 + 20.0)}>{ "0" }</text>
            <text class="axis-lbl" x={format!("{:.1}", W - 46.0)} y={format!("{:.1}", h / 2.0 + 20.0)}>{ "2 pi" }</text>
            { polyline(&[[t.circle_far_a[0], -14.0], [t.circle_far_a[1], -14.0]], "far", &at) }
            { polyline(&[[t.circle_near_a[0], 14.0], [t.circle_near_a[1], 14.0]], "near", &at) }
            { dot([t.circle_near_a[0], 0.0], "", "", &at) }
            { dot([t.circle_near_a[1], 0.0], &format!("{} (near)", t.circle_near_a[1]), "near", &at) }
            { dot([t.circle_far_a[1], 0.0], &format!("{} (far)", t.circle_far_a[1]), "far", &at) }
            <text class="axis-lbl" x="30" y="16">{ "the angle in radians: along the circle, distance is length on this line" }</text>
        </svg>
    }
}

fn sphere_map(t: &Tour) -> Html {
    let h = W / 2.0;
    let at = |q: P2| (W * q[0] / 360.0, h / 2.0 - h * q[1] / 180.0);
    let labels = [("p", ""), ("q", "near"), ("r", "far")];
    html! {
        <svg viewBox={format!("0 0 {W} {h}")} role="img" aria-label="the sphere's map: longitude across, latitude up, with the parallels, the meridians, the three points and the two arcs">
            { for t.parallels_ll.iter().map(|l| polyline(l, "wire", &at)) }
            { for t.meridians_ll.iter().map(|l| polyline(l, "wire", &at)) }
            { polyline(&t.sphere_far_arc_ll, "far", &at) }
            { polyline(&t.sphere_near_arc_ll, "near", &at) }
            { for t.sphere_deg.iter().zip(labels).map(|(q, (name, class))| dot(*q, name, class, &at)) }
            <text class="axis-lbl" x="4" y="12">{ "latitude 90" }</text>
            <text class="axis-lbl" x="4" y={format!("{:.1}", h - 4.0)}>{ "-90" }</text>
            <text class="axis-lbl" x={format!("{:.1}", W - 90.0)} y={format!("{:.1}", h / 2.0 - 4.0)}>{ "longitude 360" }</text>
        </svg>
    }
}

fn torus_map(t: &Tour) -> Html {
    let h = W;
    let at = |q: P2| (12.0 + (W - 24.0) * q[0] / 360.0, 22.0 + (h - 44.0) * (1.0 - q[1] / 360.0));
    html! {
        <svg viewBox={format!("0 0 {W} {h}")} role="img" aria-label="the torus's map: the angle round the hole across, the angle round the tube up, both 0 to 360 degrees; the rings are its rows and the tubes its columns">
            { for t.torus_rings_uv.iter().map(|l| polyline(l, "wire", &at)) }
            { for t.torus_tubes_uv.iter().map(|l| polyline(l, "wire", &at)) }
            { polyline(&t.torus_rings_uv[0], "far", &at) }
            { polyline(&t.torus_rings_uv[3], "near", &at) }
            <text class="axis-lbl" x="4" y={format!("{:.1}", h - 6.0)}>{ "u round the hole, 0 to 360" }</text>
            <text class="axis-lbl" x="4" y="12">{ "v round the tube: 0 is the outer ring (orange), 180 the inner (green)" }</text>
        </svg>
    }
}

fn roll_map(t: &Tour) -> Html {
    let (x0, x1) = (t.roll_along_map[0][0][0], t.roll_along_map[0][60][0]);
    let (y0, y1) = (-6.0, 6.0);
    let h = W * (y1 - y0) / (x1 - x0) + 40.0;
    let at = |q: P2| (W * (q[0] - x0) / (x1 - x0), 20.0 + (h - 40.0) * (y1 - q[1]) / (y1 - y0));
    let labels = [("a", ""), ("b", "near"), ("c", "far")];
    html! {
        <svg viewBox={format!("0 0 {W} {h:.0}")} role="img" aria-label="the Swiss roll unrolled: the length along the sheet across, the height up; the three points a, b and c">
            { for t.roll_along_map.iter().map(|l| polyline(l, "wire", &at)) }
            { for t.roll_across_map.iter().map(|l| polyline(l, "wire", &at)) }
            { polyline(&[t.roll_map[0], t.roll_map[2]], "far", &at) }
            { polyline(&[t.roll_map[0], t.roll_map[1]], "near", &at) }
            { for t.roll_map.iter().zip(labels).map(|(q, (name, class))| dot(*q, name, class, &at)) }
            <text class="axis-lbl" x="4" y="12">{ format!("along the sheet, {} to {}; height -6 to 6: this map is to scale", f1(x0), f1(x1)) }</text>
        </svg>
    }
}

fn disk(t: &Tour) -> Html {
    let at = |q: P2| (W / 2.0 + 0.44 * W * q[0], W / 2.0 - 0.44 * W * q[1]);
    html! {
        <svg viewBox={format!("0 0 {W} {W}")} role="img" aria-label="the Poincare disk: its rim, the disk's ruler along a diameter, five geodesics, and two pairs of points, one near the rim and one near the middle">
            { polyline(&t.rim, "rim", &at) }
            { for t.geos.iter().map(|g| polyline(g, "geo", &at)) }
            { polyline(&t.disk_far_geo, "far", &at) }
            { polyline(&t.disk_near_geo, "near", &at) }
            { for t.ruler.iter().zip(&t.ruler_d).enumerate().map(|(i, (q, d))| {
                let (x, y) = at(*q);
                let dy = if i % 2 == 0 { 20.0 } else { -12.0 };
                html! { <g>
                    <line class="ruler" x1={format!("{x:.1}")} y1={format!("{:.1}", y - 7.0)} x2={format!("{x:.1}")} y2={format!("{:.1}", y + 7.0)} />
                    <text class="axis-lbl" x={format!("{:.1}", x - 3.0)} y={format!("{:.1}", y + dy)}>{ format!("{d:.0}") }</text>
                </g> }
            }) }
            { dot(t.disk_far[0], "s", "", &at) }
            { dot(t.disk_far[1], "t", "far", &at) }
            { dot(t.disk_near[0], "s'", "", &at) }
            { dot(t.disk_near[1], "t'", "near", &at) }
        </svg>
    }
}

// -- the numbers --

fn pair_row(name: &str, p: &Pair) -> Html {
    html! { <tr><td>{ name.to_string() }</td><td>{ show(p.straight) }</td><td>{ show(p.along) }</td><td>{ format!("{:.4}", p.ratio()) }</td></tr> }
}

fn pairs(rows: &[(&str, &Pair)], along: &str) -> Html {
    html! {
        <table class="numbers">
            <thead><tr><th>{ "between" }</th><th>{ "straight line" }</th><th>{ along }</th><th>{ "ratio" }</th></tr></thead>
            <tbody>{ for rows.iter().map(|(n, p)| pair_row(n, p)) }</tbody>
        </table>
    }
}

fn space(name: &str, dims: P2, intro: &str, measured: &str, after: &[&str], figure: Html, numbers: Html, extra: Html, for_here: &str) -> Html {
    html! {
        <section class="space" id={name.replace(' ', "-")}>
            <h2>{ format!("The {name}") }</h2>
            <p class="dims">{ format!("intrinsic dimension {}, ambient dimension {} (read off the arrays by the program)", dims[0], dims[1]) }</p>
            { p(intro) }
            <div class="figure">
                { figure }
                <figure>
                    <h3>{ "The numbers" }{ mark(true) }</h3>
                    { numbers }
                    <figcaption>{ measured }</figcaption>
                </figure>
            </div>
            { extra }
            { for after.iter().map(|s| p(s)) }
            <h3>{ "What it is for here" }</h3>
            { p(for_here) }
        </section>
    }
}

fn flat(caption: &str, svg: Html, measured: bool) -> Html {
    html! { <figure class="flat">{ svg }<figcaption>{ caption.to_string() }{ mark(measured) }</figcaption></figure> }
}

fn dims_table(t: &Tour) -> Html {
    html! {
        <table class="dims">
            <thead><tr><th>{ "space" }</th><th>{ "intrinsic dimension" }</th><th>{ "ambient dimension" }</th></tr></thead>
            <tbody>{ for SPACES.iter().zip(&t.dims).map(|(s, d)| html! { <tr><td>{ s.to_string() }</td><td>{ format!("{}", d[0]) }</td><td>{ format!("{}", d[1]) }</td></tr> }) }</tbody>
        </table>
    }
}

/// The chord from p to q worked in full, from the numbers the program printed.
fn worked(t: &Tour) -> Html {
    let (pp, qq) = (t.sphere_pts[0], t.sphere_pts[1]);
    html! {
        <div class="worked-out">
            <p><b>{ "Worked in full: the chord from p to q." }</b>{ " Every number below is printed by the program (its workedDiff, workedSq, workedSum lines)." }</p>
            <p>{ format!("p = ({}, {}, {}); q = ({}, {}, {})", show(pp[0]), show(pp[1]), show(pp[2]), show(qq[0]), show(qq[1]), show(qq[2])) }</p>
            <p>{ format!("the differences p - q: {}, {}, {}", show(t.worked_diff[0]), show(t.worked_diff[1]), show(t.worked_diff[2])) }</p>
            <p>{ format!("their squares: {}, {}, {}; their sum: {}", show(t.worked_sq[0]), show(t.worked_sq[1]), show(t.worked_sq[2]), show(t.worked_sum)) }</p>
            <p>{ format!("the chord, the square root of the sum: {}", show(t.sphere_near_d.straight)) }</p>
            <p>{ format!("the great circle, 2 atan2(|p - q|, |p + q|) with |p - q| = {} and |p + q| = {}: {}", show(t.sphere_near_d.straight), show(t.worked_plus), show(t.sphere_near_d.along)) }</p>
        </div>
    }
}

/// The comparison of the page (plan G5): the pairs against the two
/// distances and their ratio, all measured.
fn analysis(t: &Tour) -> Html {
    let row = |name: &'static str, p: &Pair| (name, vec![Cell::measured(show(p.straight)), Cell::measured(show(p.along)), Cell::measured(format!("{:.4}", p.ratio()))]);
    let rows = [
        row("circle, 0.1 radians apart (near)", &t.circle_near_d),
        row("circle, 2.5 radians apart (far)", &t.circle_far_d),
        row("sphere, p to q (near)", &t.sphere_near_d),
        row("sphere, p to r (far)", &t.sphere_far_d),
        row("Swiss roll, a to b (near)", &t.roll_near_d),
        row("Swiss roll, a to c (a turn apart)", &t.roll_far_d),
        row("Poincare disk, near the middle", &t.disk_near_d),
        row("Poincare disk, near the rim", &t.disk_far_d),
    ];
    let sentences = vec![
        lesson::narrate::moved("the ratio along to through, circle, near to far", t.circle_near_d.ratio(), t.circle_far_d.ratio()),
        lesson::narrate::moved("the ratio along to through, sphere, near to far", t.sphere_near_d.ratio(), t.sphere_far_d.ratio()),
        lesson::narrate::moved("the ratio along to through, Swiss roll, near to a turn apart", t.roll_near_d.ratio(), t.roll_far_d.ratio()),
        lesson::narrate::moved("the ratio by the disk to on paper, from the middle to the rim", t.disk_near_d.ratio(), t.disk_far_d.ratio()),
        lesson::narrate::moved("the torus's ring length, from the inner ring to the outer", t.torus_rim_lengths[1], t.torus_rim_lengths[0]),
    ];
    html! { <>
        { comparison(&["straight line", "along the space", "ratio"], &rows, &sentences) }
        { for text::ANALYSIS.iter().map(|s| p(s)) }
    </> }
}

/// The Manifold functions the program uses, each explained from the
/// library's own source, in an aside.
fn library_asides(linker: &Linker) -> Html {
    let aside = |name: &str, purpose: &str, reading: &str| match definition("Manifold", name) {
        Some(d) => own_function("mf:", &d, p(purpose), html! { <p>{ reading.to_string() }</p> }),
        None => html! {},
    };
    html! { <>
        { aside("s_pherePoints", "Turns rows of (longitude, latitude) into points of the unit sphere: the sphere's map, read backward. Every wire line of the sphere and its three points come from it.", "Read the body right to left: the two columns are taken out of the map, and three lists (x, y, z) are made from cosines and sines and put side by side as columns.") }
        { aside("s_phereArc", "The great-circle arc from p to q as n + 1 points, evenly spaced along it: what the orange and green arcs on the sphere are.", "The arrow from p to q is found first (the tangent vector a step would follow), then the points at the fractions 0, 1/n, ..., 1 of it, each put back on the sphere.") }
        { aside("s_phereDistance", "The great-circle distance between two points of the sphere: the angle between them, from the lengths of their difference and their sum.", "Two helpers: a_part gives |p - q| and |p + q|; a_ngle gives 2 atan2 of them.") }
        { aside("r_ollDistance", "The distance along the Swiss roll's sheet between two points given on its map: Pythagoras on (the length along the spiral, the height), because the sheet is flat.", "Each point's length along the spiral comes from r_ollAlong; the differences in that and in height are squared, summed and rooted.") }
        { aside("b_allDistance", "The Poincare disk's distance: 2 atanh of the length of q seen from p, by the disk's own addition (Mobius addition, the private helper m_obius).", "Read right to left: q seen from p, its length, atanh, times 2.") }
        <p class="note">{ "The program's own small functions (pairs, repeats, a wire's length, the straight line) are written out at its top; see " }{ linker.code("u:p_airs") }{ " and the others in the program below." }</p>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let tour = use_memo((), |_| run());
    let src = program();
    let linker = Linker::of(&src);
    let body = match &*tour {
        Err(e) => html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> },
        Ok(t) => spaces(t),
    };
    let after = match &*tour {
        Ok(t) => analysis(t),
        Err(_) => html! {},
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("manifold-tour".into())}>
        <header>
            { header("Manifold tour", "What a manifold is, on five of them: the circle, the sphere, the torus, the Swiss roll and the Poincare disk, each drawn by X_eTaL from its own map and turning in your browser, with its flat view and one measurement each of near and far: the straight line through the space against the distance along it. Nothing here is trained.") }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ prose(text::HINT) }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>
                { p(text::BACKGROUND[0]) }
                <ul>{ for text::BACKGROUND[1..4].iter().map(|s| li(s)) }</ul>
                { p(text::BACKGROUND[4]) }
            </> }) }
            { frame(Frame::Rationale, "The idea: a space that is flat up close", html! { <>
                { for text::RATIONALE[0..3].iter().map(|s| p(s)) }
                <ul>{ for text::RATIONALE[3].split(". A representation").enumerate().map(|(i, s)| {
                    let s = if i == 0 { s.to_string() } else { format!("A representation{s}") };
                    li(&s)
                }) }</ul>
                { for text::RATIONALE[4..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ for text::PURPOSE.iter().map(|s| p(s)) }</> }) }
            { frame(INTUITION, "The intuition: an ant on a ball", html! { <>{ for text::INTUITION.iter().map(|s| p(s)) }</> }) }
            { frame(MATHEMATICS, "The mathematics: two distances, and a map", html! { <>{ for text::MATHEMATICS.iter().map(|s| p(s)) }</> }) }
            { frame(EXPERIMENT, "The experiment: a near pair and a far pair on each space", html! { <>{ for text::EXPERIMENT.iter().map(|s| p(s)) }</> }) }
            { frame(RUN, "The run: nothing is trained", html! { <>{ for text::RUN.iter().map(|s| p(s)) }</> }) }
            { body }
            { frame(ANALYSIS, "The analysis: the ratios, compared", after) }
            { frame(EXTENSION, "Beyond this page: what it rests on, what is open", html! { <>{ for text::EXTENSION.iter().map(|s| p(s)) }</> }) }
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
                <h3>{ "The library functions behind the pictures" }</h3>
                { library_asides(&linker) }
                <h3>{ "Every library function the program calls" }</h3>
                { functions(&src) }
                <h3>{ "The program" }</h3>
                { linker.block(&src) }
                <p class="links"><a href="../doc/demos-manifold-tour-manifold-tour.xtl.html">{ "This program in the cross-reference" }</a></p>
            </section>
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}

/// The five spaces, from the tour the program computed.
fn spaces(t: &Tour) -> Html {
    let lift = |q: P2| [q[0], q[1], 0.0];
    let circle3: Vec<P3> = t.circle.iter().map(|q| lift(*q)).collect();
    let arc = |upto: f64, end: P2| -> Vec<P3> {
        t.circle_angles.iter().zip(&t.circle).filter(|(a, _)| **a <= upto).map(|(_, q)| lift(*q)).chain([lift(end)]).collect()
    };
    let (far_arc, near_arc) = (arc(t.circle_far_a[1], t.circle_far[1]), arc(t.circle_near_a[1], t.circle_near[1]));
    let circle_figure = html! { <>
        <View name="circle" scene={scene(&[circle3], &[(&far_arc, FAR), (&near_arc, NEAR)],
            &[(lift(t.circle_near[0]), lift(t.circle_near[1])), (lift(t.circle_far[0]), lift(t.circle_far[1]))],
            &[(lift(t.circle_near[0]), "0", INK), (lift(t.circle_near[1]), "0.1", NEAR), (lift(t.circle_far[1]), "2.5", FAR)], Up::Y)}
            start={Angle { yaw: 0.0, pitch: 0.0 }} />
        { flat("The circle's map: one number, the angle. The near pair (green) and the far pair (orange) are marked; the gray lines in the picture are the chords through the plane.", circle_flat(t), false) }
    </> };
    let circle_numbers = pairs(&[("0 and 0.1 (near)", &t.circle_near_d), ("0 and 2.5 (far)", &t.circle_far_d)], "along the circle");

    let sphere_wires: Vec<Vec<P3>> = t.parallels.iter().chain(&t.meridians).cloned().collect();
    let sphere_figure = html! { <>
        <View name="sphere" scene={scene(&sphere_wires, &[(&t.sphere_far_arc, FAR), (&t.sphere_near_arc, NEAR)],
            &[(t.sphere_pts[0], t.sphere_pts[2])],
            &[(t.sphere_pts[0], "p", INK), (t.sphere_pts[1], "q", NEAR), (t.sphere_pts[2], "r", FAR)], Up::Z)}
            start={Angle { yaw: -0.9, pitch: 0.35 }} />
        { flat("The sphere's map, longitude across and latitude up, with the parallels and meridians of the picture, p, q and r, and the two great-circle arcs (which curve on the map). Positions can be read off it; lengths cannot.", sphere_map(t), false) }
    </> };
    let sphere_numbers = html! { <>
        { pairs(&[("p and q (near)", &t.sphere_near_d), ("p and r (far)", &t.sphere_far_d)], "great circle") }
        <table class="numbers">
            <thead><tr><th>{ "point" }</th><th>{ "longitude" }</th><th>{ "latitude" }</th></tr></thead>
            <tbody>{ for ["p", "q", "r"].iter().zip(&t.sphere_deg).map(|(n, d)| html! { <tr><td>{ n.to_string() }</td><td>{ f1(d[0]) }</td><td>{ f1(d[1]) }</td></tr> }) }</tbody>
        </table>
    </> };

    let torus_wires: Vec<Vec<P3>> = t.torus_rings.iter().chain(&t.torus_tubes).cloned().collect();
    let torus_figure = html! { <>
        <View name="torus" scene={scene(&torus_wires, &[(&t.torus_rings[0], FAR), (&t.torus_rings[3], NEAR)], &[], &[], Up::Z)}
            start={Angle { yaw: 0.5, pitch: 0.9 }} />
        { flat("The torus's map: a square of two angles, its opposite edges joined. The outer ring (orange) and the inner ring (green) are two rows of the same width here, and twice as long one as the other on the torus.", torus_map(t), false) }
    </> };
    let torus_numbers = html! {
        <table class="numbers">
            <thead><tr><th>{ "ring" }</th><th>{ "length along the wire" }</th></tr></thead>
            <tbody>
                <tr><td>{ "outer (v = 0)" }</td><td>{ show(t.torus_rim_lengths[0]) }</td></tr>
                <tr><td>{ "inner (v = 180)" }</td><td>{ show(t.torus_rim_lengths[1]) }</td></tr>
                <tr class="sum"><td>{ "ratio" }</td><td>{ format!("{:.3}", t.torus_rim_lengths[0] / t.torus_rim_lengths[1]) }</td></tr>
            </tbody>
        </table>
    };

    let roll_wires: Vec<Vec<P3>> = t.roll_along.iter().chain(&t.roll_across).cloned().collect();
    let roll_figure = html! { <>
        <View name="Swiss roll" scene={scene(&roll_wires, &[], &[(t.roll_pts[0], t.roll_pts[2]), (t.roll_pts[0], t.roll_pts[1])],
            &[(t.roll_pts[0], "a", INK), (t.roll_pts[1], "b", NEAR), (t.roll_pts[2], "c", FAR)], Up::Y)}
            start={Angle { yaw: 0.4, pitch: 0.5 }} />
        { flat("The sheet unrolled: the length along it across, the height up. This map is to scale, because the sheet is flat: a and c, a layer apart in the picture, are far apart here.", roll_map(t), false) }
    </> };
    let roll_numbers = html! { <>
        { pairs(&[("a and b (near)", &t.roll_near_d), ("a and c (a turn apart)", &t.roll_far_d)], "along the sheet") }
        <table class="numbers">
            <thead><tr><th>{ "point" }</th><th>{ "t" }</th><th>{ "h" }</th><th>{ "along" }</th></tr></thead>
            <tbody>{ for ["a", "b", "c"].iter().zip(t.roll_th.iter().zip(&t.roll_map)).map(|(n, (th, m))| html! { <tr><td>{ n.to_string() }</td><td>{ show(th[0]) }</td><td>{ f1(th[1]) }</td><td>{ show(m[0]) }</td></tr> }) }</tbody>
        </table>
    </> };

    let disk_figure = html! { <>
        { flat("The Poincare disk, flat as it must be: the rim (dashed, not part of the disk), the disk's ruler along a diameter (its marks 1 to 5 apart by the disk's distance), five geodesics (blue; a diameter among them), and the two pairs: s and t near the rim (orange), s' and t' the same two drawn in toward the middle (green).", disk(t), false) }
        <figure class="flat">
            <h3>{ "The ruler" }{ mark(true) }</h3>
            <table class="numbers">
                <thead><tr><th>{ "mark" }</th><th>{ "x on paper" }</th><th>{ "disk distance from the middle" }</th></tr></thead>
                <tbody>{ for t.ruler.iter().zip(&t.ruler_d).map(|(q, d)| html! { <tr><td>{ format!("{d:.0}") }</td><td>{ show(q[0]) }</td><td>{ show(*d) }</td></tr> }) }</tbody>
            </table>
            <figcaption>{ "Equal steps by the disk's ruler, from the middle out: on paper each step is shorter than the last, and the rim is never reached." }</figcaption>
        </figure>
    </> };
    let disk_numbers = pairs(&[("s and t (near the rim)", &t.disk_far_d), ("s' and t' (near the middle)", &t.disk_near_d)], "by the disk");

    html! { <>
        <section class="space" id="dimensions">
            <h2>{ "Five spaces, two dimensions each" }</h2>
            { p("Each space has an [intrinsic dimension|intrinsic-dimension] (the number of coordinates of its map, the directions an ant on it can move in) and an [ambient dimension|ambient-dimension] (the number of coordinates of the space it is drawn in). The program reads both off the shapes of its arrays:") }
            { dims_table(t) }
            { p("Only the disk's agree: it is drawn in its own two dimensions, with a different ruler. The others are curved inside a room with one more dimension than they have.") }
        </section>
        { space("circle", t.dims[0], text::CIRCLE[0], "Measured by the program: the straight line (the chord, Pythagoras in the plane) and the distance along the circle (the angle, in radians), for each pair.", &[text::CIRCLE[1]], circle_figure, circle_numbers, html! {}, text::CIRCLE[2]) }
        { space("sphere", t.dims[1], text::SPHERE[0], "Measured by the program: the chord through the sphere and the great-circle distance along it, p to q and p to r; the points' longitude and latitude are the program's inputs, in degrees.", &[text::SPHERE[1], text::SPHERE[2]], sphere_figure, sphere_numbers, worked(t), text::SPHERE[3]) }
        { space("torus", t.dims[2], text::TORUS[0], "Measured by the program: the length of each ring summed along its 48 segments. The exact circumferences are 2 pi (R + r) and 2 pi (R - r); the wire is a little shorter, as a polygon is shorter than its circle.", &[text::TORUS[1]], torus_figure, torus_numbers, html! {}, text::TORUS[2]) }
        { space("Swiss roll", t.dims[3], text::ROLL[0], "Measured by the program: the straight line through the air between the points of the roll, and the distance along the sheet from their unrolled coordinates; t and h are the inputs, along is computed.", &[text::ROLL[1], text::ROLL[2]], roll_figure, roll_numbers, html! {}, text::ROLL[3]) }
        { space("Poincare disk", t.dims[4], text::DISK[0], "Measured by the program: the straight line on paper between the two points and the disk's distance between them, for the pair near the rim and the pair near the middle.", &[text::DISK[1], text::DISK[2]], disk_figure, disk_numbers, html! {}, text::DISK[3]) }
    </> }
}
