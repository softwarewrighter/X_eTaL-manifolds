//! The page: the question, what to know first, the idea and what the
//! page is; then the five spaces, each with its turning view, its flat
//! view and its numbers, and what it is for here; the summary against
//! the question, what is next, and the program. Presentation only:
//! every number shown is one X_eTaL printed.

use std::rc::Rc;

use yew::prelude::*;

use microscope::source::{between, block};
use site::{footer, frame, header, mark, p, Frame};

use crate::micro::{program, run, Pair, Tour, P2, P3, SPACES};
use crate::text;
use crate::turn::{project, radius, segments, Turn, Up, SIZE};

/// A number as the page quotes it: four decimals (the run's full
/// digits are in the program's output).
fn show(x: f64) -> String {
    format!("{x:.4}")
}

fn f1(x: f64) -> String {
    format!("{x:.1}")
}

// -- the turning view (the page's projection: the GM3 workaround) --

/// A line drawn over the wires: its points, its class, its name.
#[derive(Clone, Debug, PartialEq)]
pub struct Path3 {
    pub points: Vec<P3>,
    pub class: &'static str,
}

/// A labeled point.
#[derive(Clone, Debug, PartialEq)]
pub struct Point3 {
    pub at: P3,
    pub label: &'static str,
    pub class: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct TurningProps {
    pub name: &'static str,
    pub wires: Rc<Vec<Vec<P3>>>,
    pub paths: Rc<Vec<Path3>>,
    pub chords: Rc<Vec<(P3, P3)>>,
    pub points: Rc<Vec<Point3>>,
    pub up: Up,
    pub start: (f64, f64),
}

/// A picture in three dimensions, turned by dragging, the arrow keys
/// or a finger. Its angle is this component's state and nothing else's.
#[function_component(Turning)]
pub fn turning(props: &TurningProps) -> Html {
    let turn = use_state(|| Turn { yaw: props.start.0, pitch: props.start.1 });
    let drag = use_state(|| None::<(f64, f64)>);
    let r = radius(&props.wires);
    let t = *turn;
    let segs = segments(&props.wires, props.up, t, r);
    let down = {
        let drag = drag.clone();
        Callback::from(move |e: PointerEvent| drag.set(Some((e.client_x() as f64, e.client_y() as f64))))
    };
    let mv = {
        let (drag, turn) = (drag.clone(), turn.clone());
        Callback::from(move |e: PointerEvent| {
            if let Some((x0, y0)) = *drag {
                let (x, y) = (e.client_x() as f64, e.client_y() as f64);
                turn.set(turn.tipped((x - x0) * 0.01, (y - y0) * 0.01));
                drag.set(Some((x, y)));
            }
        })
    };
    let up = {
        let drag = drag.clone();
        Callback::from(move |_: PointerEvent| drag.set(None))
    };
    let leave = {
        let drag = drag.clone();
        Callback::from(move |_: PointerEvent| drag.set(None))
    };
    let key = {
        let turn = turn.clone();
        Callback::from(move |e: KeyboardEvent| {
            let step = 5f64.to_radians();
            let (dy, dp) = match e.key().as_str() {
                "ArrowLeft" => (-step, 0.0),
                "ArrowRight" => (step, 0.0),
                "ArrowUp" => (0.0, step),
                "ArrowDown" => (0.0, -step),
                _ => return,
            };
            e.prevent_default();
            turn.set(turn.tipped(dy, dp));
        })
    };
    let (yaw, pitch) = t.degrees();
    let c = |d: f64| format!("{d:.1}");
    html! {
        <figure class="turning">
            <svg viewBox={format!("0 0 {SIZE} {SIZE}")} role="img" tabindex="0"
                aria-label={format!("the {} in three dimensions, turned {yaw} degrees round and {pitch} degrees toward you; drag, use the arrow keys or swipe to turn it", props.name)}
                onpointerdown={down} onpointermove={mv} onpointerup={up} onpointerleave={leave} onkeydown={key}>
                { for segs.iter().map(|s| html! {
                    <line class="wire" x1={c(s.a.x)} y1={c(s.a.y)} x2={c(s.b.x)} y2={c(s.b.y)} opacity={format!("{:.2}", 0.18 + 0.62 * s.near)} />
                }) }
                { for props.chords.iter().map(|(a, b)| {
                    let (a, b) = (project(*a, props.up, t, r), project(*b, props.up, t, r));
                    html! { <line class="chord" x1={c(a.x)} y1={c(a.y)} x2={c(b.x)} y2={c(b.y)} /> }
                }) }
                { for props.paths.iter().map(|path| {
                    let pts: Vec<String> = path.points.iter().map(|q| { let d = project(*q, props.up, t, r); format!("{:.1},{:.1}", d.x, d.y) }).collect();
                    html! { <polyline class={path.class} points={pts.join(" ")} /> }
                }) }
                { for props.points.iter().map(|q| {
                    let d = project(q.at, props.up, t, r);
                    html! { <g>
                        <circle class={classes!("pt", q.class)} cx={c(d.x)} cy={c(d.y)} r="4.5" />
                        <text class="lbl" x={c(d.x + 7.0)} y={c(d.y - 6.0)}>{ q.label }</text>
                    </g> }
                }) }
            </svg>
            <figcaption>
                { format!("The {} in three dimensions, turned ", props.name) }
                <span class="state">{ format!("{yaw} degrees round and {pitch} degrees toward you") }</span>
                { ". Drag it, press the arrow keys once it has the focus (click or tab to it), or swipe, to turn it; turning changes no number on this page, only the side you see. Nothing nearer is drawn bigger: the view is orthographic." }
                { mark(false) }
            </figcaption>
        </figure>
    }
}

// -- the flat views --

/// A polyline of map points, (x, y) mapped to the SVG by `at`.
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

/// The circle unrolled: its map, the angle from 0 to 2 pi, with the
/// near pair and the far pair marked where the program put them.
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

/// The sphere's map: longitude 0 to 360 across, latitude -90 to 90 up.
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

/// The torus's map: u across, v up, both 0 to 360.
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

/// The Swiss roll unrolled: the length along the sheet across, the
/// height up, in the sheet's own units (the only map here to scale).
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

/// The Poincare disk, flat: the rim, the ruler, the geodesics, the
/// two pairs.
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

/// The two distances of a pair, as a table row.
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

/// A space's section: its heading, its dimensions, its prose, the figure row, what it is for.
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
        <div class="worked">
            <p><b>{ "Worked in full: the chord from p to q." }</b>{ " Every number below is printed by the program (its workedDiff, workedSq, workedSum lines)." }</p>
            <p>{ format!("p = ({}, {}, {}); q = ({}, {}, {})", show(pp[0]), show(pp[1]), show(pp[2]), show(qq[0]), show(qq[1]), show(qq[2])) }</p>
            <p>{ format!("the differences p - q: {}, {}, {}", show(t.worked_diff[0]), show(t.worked_diff[1]), show(t.worked_diff[2])) }</p>
            <p>{ format!("their squares: {}, {}, {}; their sum: {}", show(t.worked_sq[0]), show(t.worked_sq[1]), show(t.worked_sq[2]), show(t.worked_sum)) }</p>
            <p>{ format!("the chord, the square root of the sum: {}", show(t.sphere_near_d.straight)) }</p>
            <p>{ format!("the great circle, 2 atan2(|p - q|, |p + q|) with |p - q| = {} and |p + q| = {}: {}", show(t.sphere_near_d.straight), show(t.worked_plus), show(t.sphere_near_d.along)) }</p>
        </div>
    }
}

fn source() -> Html {
    let src = program();
    let range = between(&src, "sphereFarArc := ", "\n");
    block(&src, range)
}

#[function_component(App)]
pub fn app() -> Html {
    let tour = use_memo((), |_| run());
    let body = match &*tour {
        Err(e) => html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> },
        Ok(t) => spaces(t),
    };
    html! {
        <>
        <header>
            { header("Manifold tour", "What a manifold is, on five of them: the circle, the sphere, the torus, the Swiss roll and the Poincare disk, each drawn by X_eTaL from its own map and turning in your browser, with its flat view and one measurement each of near and far: the straight line through the space against the distance along it. Nothing here is trained.") }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ text::HINT }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>
                { p(text::BACKGROUND[0]) }
                <ul>{ for text::BACKGROUND[1..4].iter().map(|s| html! { <li>{ s.to_string() }</li> }) }</ul>
                { p(text::BACKGROUND[4]) }
            </> }) }
            { frame(Frame::Rationale, "The idea: a space that is flat up close", html! { <>
                { for text::RATIONALE[0..3].iter().map(|s| p(s)) }
                <ul>{ for text::RATIONALE[3].split(". A representation").enumerate().map(|(i, s)| {
                    let s = if i == 0 { s.to_string() } else { format!("A representation{s}") };
                    html! { <li>{ s }</li> }
                }) }</ul>
                { for text::RATIONALE[4..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>
                { for text::PURPOSE.iter().map(|s| p(s)) }
            </> }) }
            { body }
            { frame(Frame::Summary, "What this page showed", html! { <>
                { for text::SUMMARY.iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Next, "What is next", html! { <>
                <ul>{ for text::NEXT.iter().map(|s| html! { <li>{ s.to_string() }</li> }) }</ul>
            </> }) }
            <section class="panel code">
                <h2>{ "The program" }</h2>
                { for text::CODE.iter().map(|s| html! { <p class="note">{ s.to_string() }</p> }) }
                { source() }
            </section>
        </main>
        { footer() }
        </>
    }
}

/// The five spaces, from the tour the program computed.
fn spaces(t: &Tour) -> Html {
    let no_chords: Rc<Vec<(P3, P3)>> = Rc::new(vec![]);
    let no_paths: Rc<Vec<Path3>> = Rc::new(vec![]);
    // The circle, in three dimensions as a flat ring (z = 0), so it turns like the rest.
    let lift = |q: P2| [q[0], q[1], 0.0];
    let circle3: Vec<P3> = t.circle.iter().map(|q| lift(*q)).collect();
    // The arcs along the circle: the circle's own points up to the pair's angle, then the pair's point.
    let arc = |upto: f64, end: P2| -> Vec<P3> {
        t.circle_angles.iter().zip(&t.circle).filter(|(a, _)| **a <= upto).map(|(_, q)| lift(*q)).chain([lift(end)]).collect()
    };
    let circle_paths = Rc::new(vec![
        Path3 { points: arc(t.circle_far_a[1], t.circle_far[1]), class: "far" },
        Path3 { points: arc(t.circle_near_a[1], t.circle_near[1]), class: "near" },
    ]);
    let circle_figure = html! { <>
        <Turning name="circle" wires={Rc::new(vec![circle3])} paths={circle_paths}
            chords={Rc::new(vec![(lift(t.circle_near[0]), lift(t.circle_near[1])), (lift(t.circle_far[0]), lift(t.circle_far[1]))])}
            points={Rc::new(vec![Point3 { at: lift(t.circle_near[0]), label: "0", class: "" }, Point3 { at: lift(t.circle_near[1]), label: "0.1", class: "near" }, Point3 { at: lift(t.circle_far[1]), label: "2.5", class: "far" }])}
            up={Up::Y} start={(0.0, 0.0)} />
        { flat("The circle's map: one number, the angle. The near pair (green) and the far pair (orange) are marked; the dashed lines in the picture are the chords through the plane.", circle_flat(t), false) }
    </> };
    let circle_numbers = pairs(&[("0 and 0.1 (near)", &t.circle_near_d), ("0 and 2.5 (far)", &t.circle_far_d)], "along the circle");

    let sphere_figure = html! { <>
        <Turning name="sphere" wires={Rc::new(t.parallels.iter().chain(&t.meridians).cloned().collect::<Vec<_>>())}
            paths={Rc::new(vec![Path3 { points: t.sphere_far_arc.clone(), class: "far" }, Path3 { points: t.sphere_near_arc.clone(), class: "near" }])}
            chords={Rc::new(vec![(t.sphere_pts[0], t.sphere_pts[2])])}
            points={Rc::new(vec![Point3 { at: t.sphere_pts[0], label: "p", class: "" }, Point3 { at: t.sphere_pts[1], label: "q", class: "near" }, Point3 { at: t.sphere_pts[2], label: "r", class: "far" }])}
            up={Up::Z} start={(-0.9, 0.35)} />
        { flat("The sphere's map, longitude across and latitude up, with the parallels and meridians of the picture, p, q and r, and the two great-circle arcs (which curve on the map). Positions can be read off it; lengths cannot.", sphere_map(t), false) }
    </> };
    let sphere_numbers = html! { <>
        { pairs(&[("p and q (near)", &t.sphere_near_d), ("p and r (far)", &t.sphere_far_d)], "great circle") }
        <table class="numbers">
            <thead><tr><th>{ "point" }</th><th>{ "longitude" }</th><th>{ "latitude" }</th></tr></thead>
            <tbody>{ for ["p", "q", "r"].iter().zip(&t.sphere_deg).map(|(n, d)| html! { <tr><td>{ n.to_string() }</td><td>{ f1(d[0]) }</td><td>{ f1(d[1]) }</td></tr> }) }</tbody>
        </table>
    </> };

    let torus_figure = html! { <>
        <Turning name="torus" wires={Rc::new(t.torus_rings.iter().chain(&t.torus_tubes).cloned().collect::<Vec<_>>())}
            paths={Rc::new(vec![Path3 { points: t.torus_rings[0].clone(), class: "far" }, Path3 { points: t.torus_rings[3].clone(), class: "near" }])}
            chords={no_chords.clone()} points={Rc::new(vec![])} up={Up::Z} start={(0.5, 0.9)} />
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

    let roll_figure = html! { <>
        <Turning name="Swiss roll" wires={Rc::new(t.roll_along.iter().chain(&t.roll_across).cloned().collect::<Vec<_>>())}
            paths={no_paths.clone()}
            chords={Rc::new(vec![(t.roll_pts[0], t.roll_pts[2]), (t.roll_pts[0], t.roll_pts[1])])}
            points={Rc::new(vec![Point3 { at: t.roll_pts[0], label: "a", class: "" }, Point3 { at: t.roll_pts[1], label: "b", class: "near" }, Point3 { at: t.roll_pts[2], label: "c", class: "far" }])}
            up={Up::Y} start={(0.4, 0.5)} />
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
            { p("Each space has an intrinsic dimension (the number of coordinates of its map, the directions an ant on it can move in) and an ambient dimension (the number of coordinates of the space it is drawn in). The program reads both off the shapes of its arrays:") }
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
