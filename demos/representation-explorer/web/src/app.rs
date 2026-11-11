//! The page, on the shared teaching shell and turning view: the arc of
//! frames; the two data sets two ways (the clusters in the plane with
//! the decision boundary under each distance; the directions turning as
//! they are and on the unit sphere, with the boundary on the sphere's
//! map); the three distances with one pair worked in full; a shortest
//! path and a mean in each space; the four training runs' losses; the
//! record and the distortions as comparison tables; every idea
//! explained; the program. Presentation only: every number, point and
//! cell is one X_eTaL printed; the buttons choose what is shown.

use yew::prelude::*;

use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::definition;
use microscope::turning::{Angle, Point, Scene, Segment, Turning};
use site::{comparison, functions, mark, own_function, Cell, ANALYSIS, EXPERIMENT, EXTENSION, INTUITION, MATHEMATICS, RUN};

use crate::micro::{program, run, Explorer, Set, P2, P3, DISTORTIONS, RUNS};
use crate::text;

fn show(x: f64) -> String {
    format!("{x:.4}")
}

/// A class's color (1 to 4), and its pale version for a boundary cell.
const COLORS: [&str; 4] = ["#e8590c", "#2b8a3e", "#1c7ed6", "#9c6500"];
const PALE: [&str; 4] = ["#ffd8c2", "#c3ebcb", "#c5def7", "#f0dfb0"];
const SEGMENT: &str = "#e8590c";
const ARC: &str = "#2b8a3e";
const CHORD: &str = "#868e96";
const MEAN: &str = "#1c7ed6";
const PLAIN: &str = "#adb5bd";

fn color(label: usize) -> &'static str {
    COLORS[(label.max(1) - 1) % COLORS.len()]
}

fn pale(label: usize) -> &'static str {
    PALE[(label.max(1) - 1) % PALE.len()]
}

const W: f64 = 420.0;

/// The shared view turns about y; the program writes z up, so points
/// are turned a quarter turn about x for drawing, (x, y, z) to
/// (x, z, -y): a rotation, nothing changes meaning.
fn upright(p: P3) -> [f64; 3] {
    [p[0], p[2], -p[1]]
}

/// Which representation a picture shows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Flat,
    Sphere,
}

/// The extent of a set of plane points, with a margin, as a box.
fn fit2(points: &[P2]) -> impl Fn(P2) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for q in points {
        for c in q {
            lo = lo.min(*c);
            hi = hi.max(*c);
        }
    }
    let (lo, hi) = (lo - 0.5, hi + 0.5);
    move |q: P2| (12.0 + (W - 24.0) * (q[0] - lo) / (hi - lo), W - 12.0 - (W - 24.0) * (q[1] - lo) / (hi - lo))
}

/// The flat clusters in the plane: the boundary cells of the chosen
/// distance behind, then the points colored by class, held-out points
/// ringed, a wrong guess (nearest neighbors under that distance)
/// crossed.
fn plane(e: &Explorer, space: Space) -> Html {
    let at = fit2(&e.flat.points);
    let set = match space { Space::Flat => &e.flat, Space::Sphere => &e.flat_unit };
    let classes = match space { Space::Flat => &e.flat_boundary, Space::Sphere => &e.angle_boundary };
    // The grid is 21 by 21 over the set's extent; a cell is drawn round its grid point.
    let (x0, y0) = at(e.grid[0]);
    let (x1, y1) = at(e.grid[21 + 1]);
    let (cw, ch) = ((x1 - x0).abs(), (y1 - y0).abs());
    let label = match space {
        Space::Flat => "the flat clusters in the plane, the decision boundary of nearest neighbors under the straight line shaded by class behind them, held-out points ringed, wrong guesses crossed",
        Space::Sphere => "the flat clusters in the plane, the decision boundary of nearest neighbors under the angle (rays from the origin) shaded by class behind them, held-out points ringed, wrong guesses crossed",
    };
    let (ox, oy) = at([0.0, 0.0]);
    html! {
        <svg viewBox={format!("0 0 {W} {W}")} role="img" aria-label={label}>
            { for e.grid.iter().zip(classes).map(|(g, c)| {
                let (x, y) = at(*g);
                html! { <rect x={format!("{:.1}", x - cw / 2.0)} y={format!("{:.1}", y - ch / 2.0)} width={format!("{cw:.1}")} height={format!("{ch:.1}")} fill={pale(*c)} /> }
            }) }
            <line class="axis" x1="0" y1={format!("{oy:.1}")} x2={format!("{W}")} y2={format!("{oy:.1}")} />
            <line class="axis" x1={format!("{ox:.1}")} y1="0" x2={format!("{ox:.1}")} y2={format!("{W}")} />
            <text class="axis-lbl" x={format!("{:.1}", ox + 4.0)} y={format!("{:.1}", oy - 4.0)}>{ "origin" }</text>
            { for e.flat.points.iter().enumerate().map(|(k, q)| {
                let (x, y) = at(*q);
                let row = k + 1;
                let right = set.right(row);
                html! { <g>
                    <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r={if right.is_some() { "5" } else { "3.5" }} fill={color(set.labels[k])}
                        stroke={if right.is_some() { "var(--fg)" } else { "none" }} stroke-width="1.5" />
                    if right == Some(false) {
                        <text class="lbl" x={format!("{:.1}", x - 4.0)} y={format!("{:.1}", y + 4.5)} font-weight="700">{ "x" }</text>
                    }
                </g> }
            }) }
        </svg>
    }
}

/// The directions as a scene: the points colored by class (as they are,
/// or on the unit sphere with the wire sphere), a wrong guess labeled
/// with what was guessed. On the sphere, the four means on the sphere
/// as arrows and the four plain means of the unit points inside it.
fn directions_scene(e: &Explorer, space: Space) -> Scene {
    let set: &Set<3> = match space { Space::Flat => &e.directions, Space::Sphere => &e.unit };
    let mut points: Vec<Point> = set.points.iter().enumerate().map(|(k, q)| {
        let row = k + 1;
        let label = match set.right(row) {
            Some(false) => format!("x: guessed {}", set.guesses[set.held.iter().position(|&h| h == row).unwrap()]),
            _ => String::new(),
        };
        Point { at: upright(*q), color: color(set.labels[k]).into(), label }
    }).collect();
    let mut segments = vec![];
    let wires: Vec<Vec<[f64; 3]>> = e.parallels.iter().chain(&e.meridians).map(|w| w.iter().map(|q| upright(*q)).collect()).collect();
    if space == Space::Sphere {
        for c in 0..4 {
            segments.push(Segment { from: [0.0; 3], to: upright(e.sphere_mean[c]), color: MEAN.into(), arrow: true });
            points.push(Point { at: upright(e.unit_mean[c]), color: PLAIN.into(), label: format!("plain mean of class {}", c + 1) });
        }
    }
    Scene { points, segments, wires }
}

/// The path scene: p and q as they are with the segment between them;
/// their unit points on the wire sphere with the arc along it and the
/// chord through it.
fn path_scene(e: &Explorer) -> Scene {
    let mut segments = vec![];
    for w in e.segment.windows(2) {
        segments.push(Segment { from: upright(w[0]), to: upright(w[1]), color: SEGMENT.into(), arrow: false });
    }
    for w in e.arc.windows(2) {
        segments.push(Segment { from: upright(w[0]), to: upright(w[1]), color: ARC.into(), arrow: false });
    }
    segments.push(Segment { from: upright(e.chord[0]), to: upright(e.chord[12]), color: CHORD.into(), arrow: false });
    let points = vec![
        Point { at: upright(e.p), color: SEGMENT.into(), label: "p".into() },
        Point { at: upright(e.q), color: SEGMENT.into(), label: "q".into() },
        Point { at: upright(e.arc[0]), color: ARC.into(), label: "u".into() },
        Point { at: upright(e.arc[12]), color: ARC.into(), label: "v".into() },
    ];
    Scene { points, segments, wires: e.parallels.iter().chain(&e.meridians).map(|w| w.iter().map(|q| upright(*q)).collect()).collect() }
}

/// Which boundary the map shows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapRule {
    Angle,
    Half,
    One,
    Two,
}

impl MapRule {
    fn name(self) -> &'static str {
        match self {
            MapRule::Angle => "the angle",
            MapRule::Half => "the straight line, at length 0.5",
            MapRule::One => "the straight line, at length 1",
            MapRule::Two => "the straight line, at length 2",
        }
    }
    fn cells(self, e: &Explorer) -> &[usize] {
        match self {
            MapRule::Angle => &e.map_angle,
            MapRule::Half => &e.map_half,
            MapRule::One => &e.map_one,
            MapRule::Two => &e.map_two,
        }
    }
}

const RULES: [MapRule; 4] = [MapRule::Angle, MapRule::Half, MapRule::One, MapRule::Two];

/// The sphere's map: longitude across, latitude up; the boundary cells
/// of the chosen rule behind, the directions on it colored by class.
fn map(e: &Explorer, rule: MapRule) -> Html {
    let (w, hh) = (W, W / 2.0);
    let at = |q: P2| (w * (q[0] + 180.0) / 360.0, hh / 2.0 - hh * q[1] / 180.0);
    let (cw, ch) = (w / 24.0, hh / 12.0);
    let deg = 180.0 / std::f64::consts::PI;
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label={format!("the sphere's map, longitude across and latitude up, the directions on it colored by class, the decision boundary under {} shaded by class behind them", rule.name())}>
            { for e.map_grid.iter().zip(rule.cells(e)).map(|(g, c)| {
                let (x, y) = at([g[0] * deg, g[1] * deg]);
                html! { <rect x={format!("{:.1}", x - cw / 2.0)} y={format!("{:.1}", y - ch / 2.0)} width={format!("{cw:.1}")} height={format!("{ch:.1}")} fill={pale(*c)} /> }
            }) }
            { for (-150..180).step_by(30).map(|lon| { let (x, _) = at([lon as f64, 0.0]); html! { <line class="wire" x1={format!("{x:.1}")} y1="0" x2={format!("{x:.1}")} y2={format!("{hh}")} /> } }) }
            { for (-60..90).step_by(30).map(|lat| { let (_, y) = at([0.0, lat as f64]); html! { <line class="wire" x1="0" y1={format!("{y:.1}")} x2={format!("{w}")} y2={format!("{y:.1}")} /> } }) }
            { for e.map.iter().enumerate().map(|(k, q)| {
                let (x, y) = at(*q);
                html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="3.5" fill={color(e.unit.labels[k])} /> }
            }) }
            <text class="axis-lbl" x="4" y="12">{ "latitude 90" }</text>
            <text class="axis-lbl" x="4" y={format!("{:.1}", hh - 4.0)}>{ "-90; longitude -180 to 180" }</text>
        </svg>
    }
}

fn space_buttons(space: Space, pick: Callback<Space>, label: &str) -> Html {
    let button = |s: Space, name: &str| {
        let pick = pick.clone();
        html! { <button class={classes!((space == s).then_some("on"))} onclick={Callback::from(move |_| pick.emit(s))} aria-pressed={if space == s { "true" } else { "false" }}>{ name.to_string() }</button> }
    };
    html! {
        <div class="controls" role="group" aria-label={label.to_string()}>
            { button(Space::Flat, "flat: the straight line") }
            { button(Space::Sphere, "the sphere: the angle") }
        </div>
    }
}

fn rule_buttons(rule: MapRule, pick: Callback<MapRule>) -> Html {
    html! {
        <div class="controls" role="group" aria-label="the distance the map's boundary is drawn under">
            { for RULES.iter().map(|r| {
                let pick = pick.clone();
                let r = *r;
                html! { <button class={classes!((rule == r).then_some("on"))} onclick={Callback::from(move |_| pick.emit(r))} aria-pressed={if rule == r { "true" } else { "false" }}>{ r.name() }</button> }
            }) }
        </div>
    }
}

fn key() -> Html {
    html! { <p class="note">{ "Colors are the classes (1 orange, 2 green, 3 blue, 4 brown); the pale cells behind are the decision boundary, each cell colored by the class nearest neighbors gives its middle. A ringed point was held out; a cross on it means its guess, under the distance shown, was wrong." }</p> }
}

fn angle_words(angle: Angle) -> (i32, i32) {
    ((angle.yaw.to_degrees().round() as i32).rem_euclid(360), angle.pitch.to_degrees().round() as i32)
}

/// The two data sets, two ways, with the boundaries.
fn sets(e: &Explorer, space: Space, pick: Callback<Space>, rule: MapRule, pick_rule: Callback<MapRule>, angle: Angle, turned: Callback<Angle>) -> Html {
    let (flat_set, other) = match space { Space::Flat => (&e.flat, &e.flat_unit), Space::Sphere => (&e.flat_unit, &e.flat) };
    let (dir_set, dir_other) = match space { Space::Flat => (&e.directions, &e.unit), Space::Sphere => (&e.unit, &e.directions) };
    let differs = |a: &[usize], b: &[usize]| a.iter().zip(b).filter(|(x, y)| x != y).count();
    let (yaw, pitch) = angle_words(angle);
    html! { <>
        <section class="space" id="clusters">
            <h2>{ "The flat clusters, under each distance" }</h2>
            { p("Three clusters of 40 points in the plane (seed 1), each point its center plus 1.6 times a normal vector. Choose the distance nearest neighbors uses: the straight line between the points as they are, or the angle between their directions from the origin. The points stay where they are; the boundary and the guesses change.") }
            { space_buttons(space, pick.clone(), "the distance for the flat clusters") }
            <div class="figure">
                <figure class="flat">
                    { plane(e, space) }
                    <figcaption>{ format!("Under {}: {} of the 30 held out guessed wrong (the other distance: {}). The boundary is the class of each of 441 grid points, from the 90 training points.", match space { Space::Flat => "the straight line", Space::Sphere => "the angle" }, flat_set.wrong(), other.wrong()) }{ mark(true) }</figcaption>
                </figure>
                <figure>{ key() }{ p("Under the angle every point on a ray from the origin is the same direction, so the boundary is made of rays and two clusters that lie the same way from the origin share a wedge; that is what the sphere costs here.") }</figure>
            </div>
        </section>
        <section class="space" id="directions">
            <h2>{ "The directions, as they are and on the sphere" }</h2>
            { p("Four classes of 40 points in three dimensions (seed 5) that differ by direction and not by length. Choose the representation: the points as they are (lengths 0.5 to 2, the wire sphere showing where length 1 is) or every point put onto the unit sphere, with the four means on it as arrows and the plain means of the unit points, inside, as gray points. The map beside it shows the boundary under the angle, or under the straight line for a point at a given length.") }
            { space_buttons(space, pick, "the representation of the directions") }
            <div class="figure">
                <figure class="turning">
                    <Turning scene={directions_scene(e, space)} {angle} {turned} size={420.0}
                        label={format!("the directional set in three dimensions, {}, colored by class, turned {yaw} degrees round and {pitch} degrees toward you", match space { Space::Flat => "as it is, lengths and all", Space::Sphere => "on the unit sphere" })} />
                    <figcaption>{ format!("{}: {} of the 40 held out guessed wrong by nearest neighbors (the other representation: {}). Turned {yaw} degrees round and {pitch} degrees toward you; turning changes no number.", match space { Space::Flat => "As they are", Space::Sphere => "On the unit sphere" }, dir_set.wrong(), dir_other.wrong()) }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    { rule_buttons(rule, pick_rule) }
                    { map(e, rule) }
                    <figcaption>{ match rule { MapRule::Angle => "The sphere's map, each direction by its longitude and latitude; the boundary under the angle, 288 cells, which the other three buttons are compared with.".to_string(), _ => format!("The sphere's map, each direction by its longitude and latitude; the boundary under {}: {} of the 288 cells differ from the angle's.", rule.name(), differs(rule.cells(e), &e.map_angle)) } }{ mark(true) }</figcaption>
                </figure>
                <figure>{ key() }{ p("In the turning picture only the wrong guesses are labeled (x, and what was guessed). The map's boundary under the straight line is for a point in the cell's direction at the length chosen: at length 0.5 a point is near the short points of every class, at length 2 near the long ones; the angle's boundary does not move.") }</figure>
            </div>
        </section>
    </> }
}

/// The three distances on three pairs, and the worked pair.
fn distances(e: &Explorer) -> Html {
    html! {
        <section class="space" id="distances">
            <h2>{ "Three distances, the same pairs" }</h2>
            { p("Three pairs of the directions: two points of one class (1 and 2), a point and one of the next class (1 and 41), and one of the last class (1 and 121). For each, the straight line between the points as they are, the cosine distance, and the great-circle distance between their unit points, the angle in radians.") }
            <div class="figure">
                <figure>
                    <h3>{ "The three pairs" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "pair" }</th><th>{ "straight line" }</th><th>{ "cosine distance" }</th><th>{ "great circle (radians)" }</th></tr></thead>
                        <tbody>
                            { for e.pairs.iter().zip(&e.distances).map(|((i, j), d)| html! {
                                <tr><td>{ format!("{i} and {j}") }</td><td>{ show(d[0]) }</td><td>{ show(d[1]) }</td><td>{ show(d[2]) }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    { p("The cosine distance and the angle order the three pairs the same way, as they must; the straight line need not, since it mixes the lengths in: see which pair it calls farthest. Points 1 and 2 are of one class and still 1.31 radians (75 degrees) apart: the cone of 0.6 scatters a class widely, which is why the classes overlap and no method gets them all.") }
                </figure>
                <figure>
                    <h3>{ "Points 1 and 41, worked in full" }{ mark(true) }</h3>
                    <div class="worked-out">
                        <p>{ format!("p = ({}, {}, {}); q = ({}, {}, {})", show(e.p[0]), show(e.p[1]), show(e.p[2]), show(e.q[0]), show(e.q[1]), show(e.q[2])) }</p>
                        <p>{ format!("p - q = ({}, {}, {})", show(e.worked_diff[0]), show(e.worked_diff[1]), show(e.worked_diff[2])) }</p>
                        <p>{ format!("squared: {} + {} + {} = {}", show(e.worked_sq[0]), show(e.worked_sq[1]), show(e.worked_sq[2]), show(e.worked_sum)) }</p>
                        <p>{ format!("the straight line: sqrt({}) = {}", show(e.worked_sum), show(e.worked_straight)) }</p>
                        <p>{ format!("p . q = {}; |p| = {}, |q| = {}", show(e.worked_dot), show(e.worked_lengths[0]), show(e.worked_lengths[1])) }</p>
                        <p>{ format!("cos = {} / ({} x {}) = {}", show(e.worked_dot), show(e.worked_lengths[0]), show(e.worked_lengths[1]), show(e.worked_cos)) }</p>
                        <p>{ format!("the cosine distance: 1 - ({}) = {}", show(e.worked_cos), show(e.worked_cosine)) }</p>
                        <p>{ format!("the angle: acos({}) = {} radians", show(e.worked_cos), show(e.worked_angle)) }</p>
                    </div>
                    { p("Every line is the program's; the command-line checks confirm each against the table.") }
                </figure>
            </div>
        </section>
    }
}

/// A shortest path and a mean in each space.
fn paths_and_means(e: &Explorer, angle: Angle, turned: Callback<Angle>) -> Html {
    let (yaw, pitch) = angle_words(angle);
    html! {
        <section class="space" id="paths">
            <h2>{ "A shortest path, and a mean, in each space" }</h2>
            { p("Between points 1 and 41 of the directions: in the flat space the segment from p to q (orange), straight through the room; on the sphere the arc of the great circle between their unit points u and v (green), and the chord between the same two (gray), through the inside. The wire sphere is the unit sphere; p and q sit off it at their own lengths.") }
            <div class="figure">
                <figure class="turning">
                    <Turning scene={path_scene(e)} {angle} {turned} size={420.0} scale={Some(1.8)}
                        label={format!("the segment from p to q through the room, and on the unit sphere the arc between their unit points and the chord between them; turned {yaw} degrees round and {pitch} degrees toward you")} />
                    <figcaption>{ format!("Turned {yaw} degrees round and {pitch} degrees toward you. The arc is 13 points along the great circle, the segment 13 along the straight line.") }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <h3>{ "The lengths" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "path" }</th><th>{ "length" }</th></tr></thead>
                        <tbody>
                            <tr><td>{ "the segment from p to q (the straight line)" }</td><td>{ show(e.worked_straight) }</td></tr>
                            <tr><td>{ "the arc from u to v along the sphere (the angle)" }</td><td>{ show(e.arc_length) }</td></tr>
                            <tr><td>{ "the chord from u to v through the sphere" }</td><td>{ show(e.chord_length) }</td></tr>
                            <tr class="sum"><td>{ "2 sin(angle / 2)" }</td><td>{ show(2.0 * (e.arc_length / 2.0).sin()) }</td></tr>
                        </tbody>
                    </table>
                    <p class="note">{ "The last row is the page's arithmetic on the printed angle, shown to make the relation visible; the three above are the program's." }</p>
                </figure>
                <figure>
                    <h3>{ "The means, per class" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "class" }</th><th>{ "plain mean of the points as they are, length" }</th><th>{ "plain mean of the unit points, length" }</th><th>{ "mean on the sphere: angle from the first mean put onto the sphere (radians)" }</th></tr></thead>
                        <tbody>
                            { for (0..4).map(|c| html! {
                                <tr><td>{ format!("{}", c + 1) }</td><td>{ show(e.mean_lengths[c][0]) }</td><td>{ show(e.mean_lengths[c][1]) }</td><td>{ show(e.mean_apart[c]) }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    { p("The plain mean of a class's points, coordinate by coordinate, is shorter than their lengths average to (about 1.25), since averaging points that point different ways shortens them; the plain mean of its unit points falls inside the sphere (length under 1) for the same reason; the mean on the sphere is on it, and the last column is how far along the sphere it is from the first mean put onto the sphere: class 4's is the largest. The arrows in the directions picture, on the sphere, are these four means.") }
                </figure>
            </div>
        </section>
    }
}

/// The four training runs' losses by step, one chart.
fn loss_chart(e: &Explorer) -> Html {
    let (w, h) = (W, W * 0.6);
    let n = e.losses[0].len();
    let top = e.losses.iter().flatten().cloned().fold(0.0, f64::max);
    let at = |k: usize, v: f64| (30.0 + (w - 40.0) * k as f64 / (n - 1) as f64, h - 24.0 - (h - 36.0) * v / top);
    let names = ["flat clusters, flat", "flat clusters, sphere", "directions, flat", "directions, sphere"];
    let dashes = ["", "6 4", "", "6 4"];
    let colors = ["#7048e8", "#7048e8", "#0ca678", "#0ca678"];
    let (_, y_half) = at(0, 0.5);
    html! {
        <svg viewBox={format!("0 0 {w} {h}")} role="img" aria-label="the training loss by step of the four runs: the flat clusters flat and on the sphere (purple, solid and dashed), the directions flat and on the sphere (teal, solid and dashed); the target 0.5 as a gray line">
            <line class="axis" x1="30" y1={format!("{:.1}", h - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", h - 24.0)} />
            <line class="axis" x1="30" y1="12" x2="30" y2={format!("{:.1}", h - 24.0)} />
            <line class="chord" x1="30" y1={format!("{y_half:.1}")} x2={format!("{:.1}", w - 10.0)} y2={format!("{y_half:.1}")} />
            <text class="axis-lbl" x="34" y={format!("{:.1}", y_half - 3.0)}>{ "loss 0.5, the target" }</text>
            <text class="axis-lbl" x="34" y="12">{ format!("loss, 0 to {:.2}", top) }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", h - 8.0)}>{ format!("step {}", n - 1) }</text>
            <text class="axis-lbl" x="30" y={format!("{:.1}", h - 8.0)}>{ "step 0" }</text>
            { for e.losses.iter().enumerate().map(|(r, l)| {
                let d: Vec<String> = l.iter().enumerate().map(|(k, v)| { let (x, y) = at(k, *v); format!("{x:.1},{y:.1}") }).collect();
                html! { <polyline points={d.join(" ")} fill="none" stroke={colors[r]} stroke-width="2" stroke-dasharray={dashes[r]} /> }
            }) }
            { for names.iter().enumerate().map(|(r, name)| html! { <g>
                <line x1={format!("{:.1}", w - 170.0)} y1={format!("{:.1}", 30.0 + 16.0 * r as f64)} x2={format!("{:.1}", w - 150.0)} y2={format!("{:.1}", 30.0 + 16.0 * r as f64)} stroke={colors[r]} stroke-width="2" stroke-dasharray={dashes[r]} />
                <text class="axis-lbl" x={format!("{:.1}", w - 145.0)} y={format!("{:.1}", 34.0 + 16.0 * r as f64)}>{ name.to_string() }</text>
            </g> }) }
        </svg>
    }
}

fn training(e: &Explorer) -> Html {
    let steps = |c: f64| if c < 0.0 { "not in 300".to_string() } else { format!("{c:.0}") };
    let names = ["flat clusters, flat", "flat clusters, sphere", "directions, flat", "directions, sphere"];
    let sets: [&dyn Fn() -> (usize, usize); 4] = [&|| (e.flat.net_wrong(), 30), &|| (e.flat_unit.net_wrong(), 30), &|| (e.directions.net_wrong(), 40), &|| (e.unit.net_wrong(), 40)];
    html! {
        <section class="space" id="training">
            <h2>{ "The training run: the same network on each representation" }</h2>
            { p("One hidden layer of 8 tanh units then softmax, Adam at rate 0.02, 300 steps, the same seeded starting weights for both representations of a set, on the 90 (clusters) or 120 (directions) training rows. The loss by step, and what each run reached.") }
            <div class="figure">
                <figure class="flat">
                    { loss_chart(e) }
                    <figcaption>{ "The training cross-entropy at every step, four runs. Purple: the flat clusters; teal: the directions (not the class colors); solid: kept flat; dashed: kept on the sphere." }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <h3>{ "Convergence" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "run" }</th><th>{ "loss at step 0" }</th><th>{ "loss at step 300" }</th><th>{ "steps to loss under 0.5" }</th><th>{ "held out wrong" }</th></tr></thead>
                        <tbody>
                            { for e.convergence.iter().enumerate().map(|(r, c)| { let (wrong, of) = sets[r](); html! {
                                <tr><td>{ names[r] }</td><td>{ show(c.start) }</td><td>{ show(c.end) }</td><td>{ steps(c.steps_to_target) }</td><td>{ format!("{wrong} of {of}") }</td></tr>
                            } }) }
                        </tbody>
                    </table>
                    { p("The loss at step 0 differs between the two representations of a set because the same weights see different inputs (the points as they are, or at length 1). The held-out column is the network's guesses on the same held-out rows nearest neighbors was scored on; its accuracy is in the record below.") }
                </figure>
            </div>
        </section>
    }
}

fn analysis(e: &Explorer) -> Html {
    let rows: Vec<(&str, Vec<Cell>)> = RUNS.iter().zip(&e.records).map(|(name, r)| {
        (*name, vec![Cell::measured(show(r.accuracy)), Cell::measured(format!("{:.0}", r.held)), Cell::measured(format!("{:.0}", r.trained)), Cell::measured(format!("{:.0}", r.operations))])
    }).collect();
    let sentences = vec![
        lesson::narrate::moved("nearest neighbors' accuracy on the flat clusters, from flat to the sphere", e.records[0].accuracy, e.records[1].accuracy),
        lesson::narrate::moved("nearest neighbors' accuracy on the directions, from flat to the sphere", e.records[2].accuracy, e.records[3].accuracy),
        lesson::narrate::moved("the network's accuracy on the flat clusters, from flat to the sphere", e.records[4].accuracy, e.records[5].accuracy),
        lesson::narrate::moved("the network's accuracy on the directions, from flat to the sphere", e.records[6].accuracy, e.records[7].accuracy),
        lesson::narrate::moved("the network's training loss at step 300 on the directions, from flat to the sphere", e.convergence[2].end, e.convergence[3].end),
    ];
    let d_rows: Vec<(&str, Vec<Cell>)> = DISTORTIONS.iter().zip(&e.distortion).map(|(name, d)| {
        (*name, vec![Cell::measured(show(d.scale)), Cell::measured(show(d.mean)), Cell::measured(show(d.worst))])
    }).collect();
    let d_sentences = vec![
        format!("The two matching rows are 0 by construction: a set's own distances are its matching representation's. The two crossed rows are the measure: the clusters on the sphere lose {} on average, the directions kept flat {}, and the directions' worst pair is points {} and {}, whose scaled straight-line distance is {} times their angle (the worst-pair column is that factor less 1; measured).", show(e.distortion[1].mean), show(e.distortion[2].mean), e.worst_pair.0, e.worst_pair.1, format!("{:.1}", 1.0 + e.distortion[2].worst)),
    ];
    html! { <>
        <h3>{ "The record" }</h3>
        { comparison(&["accuracy", "held out", "numbers trained", "operations"], &rows, &sentences) }
        <h3>{ "Distortion, each representation against its set's own distances" }</h3>
        { comparison(&["scale fitted", "mean over pairs", "worst pair"], &d_rows, &d_sentences) }
        { for text::ANALYSIS.iter().map(|s| p(s)) }
    </> }
}

fn library_asides() -> Html {
    let own = |lib: &str, alias: &str, name: &str, purpose: &str, reading: &str| match definition(lib, name) {
        Some(d) => own_function(alias, &d, p(purpose), html! { <p>{ reading.to_string() }</p> }),
        None => html! {},
    };
    let shell = |lib: &str, alias: &str, name: &str, purpose: &str, reading: &str| match definition(lib, name) {
        Some(d) => lesson::explain::function(alias, &d, p(purpose), html! { <p>{ reading.to_string() }</p> }),
        None => html! {},
    };
    html! { <>
        { own("Manifold", "mf:", "s_phereProject", "The spherical representation: every point scaled to length 1, its direction kept.", "A set (rank 2) goes row by row; a vector is divided by its length, the zero vector refused.") }
        { own("Manifold", "mf:", "s_phereDistance", "The great-circle distance of the table and the arc's length: the angle between two unit points.", "2 atan2 of |p - q| and |p + q|: accurate near 0 and near pi alike, where acos of the dot product is not.") }
        { own("Manifold", "mf:", "s_phereMean", "The mean on the sphere, per class: the arrows in the directions picture.", "From the plain mean put onto the sphere, thirty steps along the average of the arrows to the points.") }
        { own("Manifold", "mf:", "f_itScale", "The one scale that fits a representation's distances to the set's own before the distortion is measured.", "The sum of the distances to keep over the sum of the representation's, over the pairs that count.") }
        { own("Manifold", "mf:", "d_istortion", "How badly a representation keeps the set's own distances: the mean and the worst of |E / D - 1| over the pairs.", "A mask picks each pair once where D > 0; the ratios less one, in size, are averaged and maxed.") }
        { shell("Learn", "ml:", "k_nn", "Nearest neighbors, under the straight line between whatever points it is given: as they are, or their unit points.", "The squared distances to every training point are found at once; the nearest is taken k times, each shifted out of the way once taken; the labels are counted and the largest count wins.") }
        { shell("NN", "nn:", "c_rossEntropy", "The loss the network is trained to lower, kept at every step for the chart.", "Minus the mean over rows of the one-hot labels times the log of the probabilities.") }
        { own("Kit", "kt:", "n_ormals", "The starting weights: the same seeded normal numbers for both representations of a set.", "The first rows times cols normal numbers of the seed, as a matrix.") }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_memo((), |_| run());
    let space = use_state(|| Space::Flat);
    let rule = use_state(|| MapRule::Angle);
    let angle = use_state(|| Angle { yaw: 0.5, pitch: 0.3 });
    let path_angle = use_state(|| Angle { yaw: -0.6, pitch: 0.4 });
    let pick = { let s = space.clone(); Callback::from(move |n: Space| s.set(n)) };
    let pick_rule = { let r = rule.clone(); Callback::from(move |n: MapRule| r.set(n)) };
    let turned = { let a = angle.clone(); Callback::from(move |n: Angle| a.set(n)) };
    let path_turned = { let a = path_angle.clone(); Callback::from(move |n: Angle| a.set(n)) };
    let src = program();
    let linker = Linker::of(&src);
    let body = match &*model {
        Err(e) => html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> },
        Ok(e) => html! { <>
            { sets(e, *space, pick, *rule, pick_rule, *angle, turned) }
            { distances(e) }
            { paths_and_means(e, *path_angle, path_turned) }
            { training(e) }
        </> },
    };
    let after = match &*model {
        Ok(e) => analysis(e),
        Err(_) => html! {},
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("representation-explorer".into())}>
        <header>{ header("Representation explorer: flat against the sphere", text::LEDE) }</header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ prose(text::HINT) }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>{ for text::BACKGROUND.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Rationale, "The idea: hold everything equal but the space", html! { <>{ for text::RATIONALE.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ for text::PURPOSE.iter().map(|s| p(s)) }</> }) }
            { frame(INTUITION, "The intuition: a sheet of paper and a globe", html! { <>{ for text::INTUITION.iter().map(|s| p(s)) }</> }) }
            { frame(MATHEMATICS, "The mathematics: three distances, paths, means, a classifier", html! { <>{ for text::MATHEMATICS.iter().map(|s| p(s)) }</> }) }
            { frame(EXPERIMENT, "The experiment: two sets by two representations, two methods", html! { <>{ for text::EXPERIMENT.iter().map(|s| p(s)) }</> }) }
            { frame(RUN, "The run: nearest neighbors, and four trainings of one network", html! { <>{ for text::RUN.iter().map(|s| p(s)) }</> }) }
            { body }
            { frame(ANALYSIS, "The analysis: the record, and the distortions", after) }
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
                <h3>{ "The program" }</h3>
                { linker.block(&src) }
                <p class="links"><a href="../doc/demos-representation-explorer-representation-explorer.xtl.html">{ "This program in the cross-reference" }</a></p>
            </section>
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
