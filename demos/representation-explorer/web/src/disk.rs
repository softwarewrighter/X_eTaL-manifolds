//! Part 2's views: the Poincare disk (the worked pair, its geodesic,
//! the distance from the middle, the room by level); the tree fitted in
//! the sphere, the plane and the disk, side by side on one scale of
//! edge distortion, the sphere turning with its map beside; the
//! distortion by step; curvature as a control; the tree's record, the
//! spread, and the H1 table. Presentation only: every number, point and
//! color is one X_eTaL printed; the buttons choose what is shown.

use yew::prelude::*;

use lesson::eli5::p;
use microscope::turning::{Angle, Point, Scene, Segment, Turning};
use site::{comparison, mark, Cell};

use crate::hyper::{Curve, Fit, Hyper, SPACES, SPACES_SHORT};
use crate::micro::{Explorer, P2, P3};

fn show(x: f64) -> String {
    format!("{x:.4}")
}

const W: f64 = 420.0;
/// The branches' colors (class 1, class 2), as part 1's first two classes.
const COLORS: [&str; 2] = ["#e8590c", "#2b8a3e"];
const GEODESIC: &str = "#2b8a3e";
const STRAIGHT: &str = "#868e96";
const RIM: &str = "#adb5bd";
const PATH: &str = "#1c7ed6";
const UNCOLORED: &str = "#868e96";

fn color(class: usize) -> &'static str {
    COLORS[(class.max(1) - 1) % 2]
}

/// An edge's color on the one scale: gray at distortion 0 to red at 1
/// or more (the fraction by which the edge's length is off 1).
fn edge_color(d: f64) -> String {
    let t = d.clamp(0.0, 1.0);
    let mix = |a: f64, b: f64| (a + (b - a) * t).round() as u8;
    format!("#{:02x}{:02x}{:02x}", mix(0xad as f64, 0xc9 as f64), mix(0xb5 as f64, 0x2a as f64), mix(0xbd as f64, 0x2a as f64))
}

/// The shared view turns about y; the program writes z up: a quarter
/// turn about x for drawing, a rotation.
fn upright(q: P3) -> [f64; 3] {
    [q[0], q[2], -q[1]]
}

fn angle_words(angle: Angle) -> (i32, i32) {
    ((angle.yaw.to_degrees().round() as i32).rem_euclid(360), angle.pitch.to_degrees().round() as i32)
}

/// A box fitted round plane points with a margin.
fn fit(points: &[P3]) -> impl Fn(P2) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for q in points {
        for c in &q[..2] {
            lo = lo.min(*c);
            hi = hi.max(*c);
        }
    }
    let (lo, hi) = (lo - 0.3, hi + 0.3);
    move |q: P2| (12.0 + (W - 24.0) * (q[0] - lo) / (hi - lo), W - 12.0 - (W - 24.0) * (q[1] - lo) / (hi - lo))
}

/// The unit disk's box: -1.05 to 1.05 each way.
fn disk_at(q: P2) -> (f64, f64) {
    (W / 2.0 + (W / 2.0 - 10.0) * q[0] / 1.05, W / 2.0 - (W / 2.0 - 10.0) * q[1] / 1.05)
}

fn rim() -> Html {
    let (cx, cy) = disk_at([0.0, 0.0]);
    let r = (W / 2.0 - 10.0) / 1.05;
    html! { <circle cx={format!("{cx:.1}")} cy={format!("{cy:.1}")} r={format!("{r:.1}")} fill="none" stroke={RIM} stroke-width="1.5" /> }
}

/// The disk with the worked pair: the geodesic between u and v, the
/// straight segment, and the circles at the plain radii whose distance
/// from the middle the table gives.
fn worked_disk(h: &Hyper) -> Html {
    let path = |pts: &[P2], color: &str, dash: &str| {
        let d: Vec<String> = pts.iter().map(|q| { let (x, y) = disk_at(*q); format!("{x:.1},{y:.1}") }).collect();
        html! { <polyline points={d.join(" ")} fill="none" stroke={color.to_string()} stroke-width="2.5" stroke-dasharray={dash.to_string()} /> }
    };
    let (cx, cy) = disk_at([0.0, 0.0]);
    let scale = (W / 2.0 - 10.0) / 1.05;
    let (ux, uy) = disk_at(h.u);
    let (vx, vy) = disk_at(h.v);
    html! {
        <svg viewBox={format!("0 0 {W} {W}")} role="img" aria-label="the Poincare disk: the rim, circles at plain radius 0.5, 0.9 and 0.99 labeled with their distance from the middle along the disk, and between the points u and v the geodesic (green, bowed toward the middle) and the straight segment (gray)">
            { rim() }
            { for h.radii.iter().zip(&h.from_middle).take(3).map(|(r, d)| html! { <g>
                <circle cx={format!("{cx:.1}")} cy={format!("{cy:.1}")} r={format!("{:.1}", scale * r)} fill="none" stroke={RIM} stroke-width="0.8" stroke-dasharray="3 3" />
                <text class="axis-lbl" x={format!("{:.1}", cx + 3.0)} y={format!("{:.1}", cy - scale * r + 10.0)}>{ format!("radius {r}: {} from the middle", format!("{d:.2}")) }</text>
            </g> }) }
            { path(&h.straight, STRAIGHT, "5 4") }
            { path(&h.geodesic, GEODESIC, "") }
            <circle cx={format!("{ux:.1}")} cy={format!("{uy:.1}")} r="5" fill={GEODESIC} />
            <circle cx={format!("{vx:.1}")} cy={format!("{vy:.1}")} r="5" fill={GEODESIC} />
            <text class="lbl" x={format!("{:.1}", ux + 7.0)} y={format!("{:.1}", uy + 4.0)}>{ "u" }</text>
            <text class="lbl" x={format!("{:.1}", vx + 7.0)} y={format!("{:.1}", vy + 4.0)}>{ "v" }</text>
            <text class="axis-lbl" x="6" y={format!("{:.1}", W - 6.0)}>{ "the rim: infinitely far from every point" }</text>
        </svg>
    }
}

/// The disk: what it is, one distance worked, a path, and the room.
pub fn disk(h: &Hyper) -> Html {
    html! {
        <section class="space" id="disk">
            <h2>{ "The Poincare disk: a space with room to spare" }</h2>
            { p("The third space: the inside of the unit circle, measured so that a step near the rim counts for far more than the same step near the middle. Two points u = (0.6, 0) and v = (0, 0.6), their shortest path along the disk (a circle's arc bowed toward the middle) beside the straight segment, and how far from the middle a point at a given plain radius is.") }
            <div class="figure">
                <figure class="flat">
                    { worked_disk(h) }
                    <figcaption>{ format!("The geodesic from u to v is {} long along the disk, the straight segment {} (summed over its 12 pieces): the bowed path is the shorter one, because it keeps away from the rim. The plain distance between u and v is {}.", show(h.path_lengths[0]), show(h.path_lengths[1]), show(h.worked_plain)) }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <h3>{ "The distance from the middle" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "plain radius" }</th><th>{ "distance along the disk" }</th></tr></thead>
                        <tbody>
                            { for h.radii.iter().zip(&h.from_middle).map(|(r, d)| html! { <tr><td>{ format!("{r}") }</td><td>{ show(*d) }</td></tr> }) }
                        </tbody>
                    </table>
                    { p("Twice atanh of the radius: near the middle twice the plain distance, then more and more. The last row is the rim's radius the program keeps every point to, 1 - 0.00001, as the source papers do (Nickel and Kiela; Ganea, Becigneul and Hofmann): about 12.2 from the middle, as far as any point can be, so two points of the disk are at most about 24.4 apart. At that radius 1 - r^2 is 0.00002 and keeps about eleven of sixteen digits; nearer the rim it would lose more, and at the rim itself the distance is infinite.") }
                </figure>
                <figure>
                    <h3>{ "u and v, worked in full" }{ mark(true) }</h3>
                    <div class="worked-out">
                        <p>{ format!("u = ({}, {}); v = ({}, {})", show(h.u[0]), show(h.u[1]), show(h.v[0]), show(h.v[1])) }</p>
                        <p>{ format!("|u - v|^2 = {}", show(h.worked_m2)) }</p>
                        <p>{ format!("1 - |u|^2 = {}; 1 - |v|^2 = {}", show(h.worked_a), show(h.worked_b)) }</p>
                        <p>{ format!("z = sqrt({}) / sqrt({} x {}) = {}", show(h.worked_m2), show(h.worked_a), show(h.worked_b), show(h.worked_z)) }</p>
                        <p>{ format!("the distance: 2 asinh({}) = {}", show(h.worked_z), show(h.worked_disk)) }</p>
                        <p>{ format!("the same as acosh(1 + 2 z^2) = {}", show(h.worked_acosh)) }</p>
                        <p>{ format!("the plain distance, for comparison: sqrt({}) = {}", show(h.worked_m2), show(h.worked_plain)) }</p>
                    </div>
                    { p("Every line is the program's; the command-line checks confirm the distance against Manifold's and the two forms against each other.") }
                </figure>
            </div>
            <h3>{ "Why a tree wants this space: the room, counted" }{ mark(true) }</h3>
            { p("Level L of a tree of branching 2 has 2^L nodes, every one L edges from the root. Put them evenly round the circle of radius L in the plane and in the disk (L measured along the disk, as the table above measures it: plain radius tanh(L / 2)), and measure the distance between neighbors along each space.") }
            <table class="numbers">
                <thead><tr><th>{ "level" }</th><th>{ "nodes" }</th><th>{ "the plane's circle, length 2 pi L" }</th><th>{ "the disk's circle, length 2 pi sinh L" }</th><th>{ "neighbors apart in the plane" }</th><th>{ "neighbors apart in the disk" }</th></tr></thead>
                <tbody>
                    { for h.room.iter().enumerate().map(|(l, r)| html! {
                        <tr><td>{ l + 1 }</td><td>{ format!("{:.0}", r[0]) }</td><td>{ format!("{:.2}", r[1]) }</td><td>{ format!("{:.2}", r[2]) }</td><td>{ show(r[3]) }</td><td>{ show(r[4]) }</td></tr>
                    }) }
                </tbody>
            </table>
            { p(&format!("In the plane the circle grows with L and the nodes double, so from level 2 on the neighbors crowd: by level 5 they are {} apart, less than the edge of length 1 between a node and its parent, and the levels below would crowd worse. In the disk the circle's length grows like sinh L, doubling about every 0.7 of radius, as the nodes do: the neighbors on level 5 are {} apart along the disk and the spacing grows with every level. (The spacing is the shortest path between neighbors, which in the disk is much shorter than the arc round the circle; the circle's length says how much room there is, the spacing how it is used.) This is the counting behind the hypothesis: a tree's levels grow exponentially, and only a space of negative curvature has room that grows the same way. The tree fitted below stops at level 4, where this count still finds room in the plane ({} apart); what the plane loses on it is subtler than crowding on one circle, since the branches must also keep their distances across the root, and is measured by the fit, not counted here.", show(h.room[4][3]), show(h.room[4][4]), show(h.room[3][3]))) }
        </section>
    }
}

/// A fitted embedding drawn flat: the edges colored by distortion on the
/// one scale, the nodes by branch, held-out nodes ringed, a wrong guess
/// by nearest neighbors crossed. For the disk, the rim and the geodesic
/// from the root to the last leaf, and node 2's three nearest ringed in
/// blue.
fn flat_embedding(h: &Hyper, f: &Fit, points: &[P3], in_disk: bool, extras: bool, colored: bool, label: &str) -> Html {
    let box_at = fit(points);
    let at = |q: P3| if in_disk { disk_at([q[0], q[1]]) } else { box_at([q[0], q[1]]) };
    let path: Vec<String> = h.tree_path.iter().map(|q| { let (x, y) = disk_at(*q); format!("{x:.1},{y:.1}") }).collect();
    html! {
        <svg viewBox={format!("0 0 {W} {W}")} role="img" aria-label={label.to_string()}>
            if in_disk { { rim() } }
            { for h.edges.iter().zip(&f.edge_distortion).map(|((a, b), d)| {
                let (x1, y1) = at(points[a - 1]);
                let (x2, y2) = at(points[b - 1]);
                html! { <line x1={format!("{x1:.1}")} y1={format!("{y1:.1}")} x2={format!("{x2:.1}")} y2={format!("{y2:.1}")} stroke={if colored { edge_color(*d) } else { UNCOLORED.into() }} stroke-width={if colored { "2.5" } else { "1.5" }} stroke-dasharray={if colored { "" } else { "4 3" }} /> }
            }) }
            if extras {
                <polyline points={path.join(" ")} fill="none" stroke={PATH} stroke-width="2" stroke-dasharray="6 3" />
                { for h.neighbors.iter().map(|k| { let (x, y) = at(points[k - 1]); html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="9" fill="none" stroke={PATH} stroke-width="1.5" /> } }) }
            }
            { for points.iter().enumerate().map(|(k, q)| {
                let (x, y) = at(*q);
                let node = k + 1;
                let right = if colored { h.right(&f.knn_guess, node) } else { None };
                html! { <g>
                    <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r={if right.is_some() { "5.5" } else { "4" }} fill={color(h.classes[k])}
                        stroke={if right.is_some() { "var(--fg)" } else { "none" }} stroke-width="1.5" />
                    if node == 1 { <text class="lbl" x={format!("{:.1}", x + 6.0)} y={format!("{:.1}", y - 4.0)}>{ "root" }</text> }
                    if node == h.n() { <text class="lbl" x={format!("{:.1}", x + 6.0)} y={format!("{:.1}", y - 4.0)}>{ format!("{node}") }</text> }
                    if node == 2 && extras { <text class="lbl" x={format!("{:.1}", x + 10.0)} y={format!("{:.1}", y + 4.0)}>{ "2" }</text> }
                    if right == Some(false) { <text class="lbl" x={format!("{:.1}", x - 4.0)} y={format!("{:.1}", y + 4.5)} font-weight="700">{ "x" }</text> }
                </g> }
            }) }
        </svg>
    }
}

/// The sphere embedding as a scene: the wire sphere, the edges as
/// segments colored by distortion, the nodes by branch.
fn sphere_scene(h: &Hyper, f: &Fit, points: &[P3], colored: bool) -> Scene {
    let pts: Vec<Point> = points.iter().enumerate().map(|(k, q)| {
        let node = k + 1;
        let label = match if colored { h.right(&f.knn_guess, node) } else { None } { Some(false) => "x: guessed wrong".to_string(), Some(true) => "held out".to_string(), None => String::new() };
        Point { at: upright(*q), color: color(h.classes[k]).into(), label }
    }).collect();
    let segments = h.edges.iter().zip(&f.edge_distortion).map(|((a, b), d)| Segment { from: upright(points[a - 1]), to: upright(points[b - 1]), color: if colored { edge_color(*d) } else { UNCOLORED.into() }, arrow: false }).collect();
    Scene { points: pts, segments, wires: h.parallels.iter().chain(&h.meridians).map(|w| w.iter().map(|q| upright(*q)).collect()).collect() }
}

/// The sphere embedding on the sphere's map, the edges drawn where they
/// do not wrap round.
fn sphere_map(h: &Hyper, f: &Fit) -> Html {
    let (w, hh) = (W, W / 2.0);
    let at = |q: P2| (w * (q[0] + 180.0) / 360.0, hh / 2.0 - hh * q[1] / 180.0);
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the sphere's map, longitude across and latitude up, the tree's nodes on it colored by branch and its edges colored by distortion, an edge that wraps round the back left out">
            { for (-150..180).step_by(30).map(|lon| { let (x, _) = at([lon as f64, 0.0]); html! { <line class="wire" x1={format!("{x:.1}")} y1="0" x2={format!("{x:.1}")} y2={format!("{hh}")} /> } }) }
            { for (-60..90).step_by(30).map(|lat| { let (_, y) = at([0.0, lat as f64]); html! { <line class="wire" x1="0" y1={format!("{y:.1}")} x2={format!("{w}")} y2={format!("{y:.1}")} /> } }) }
            { for h.edges.iter().zip(&f.edge_distortion).filter(|((a, b), _)| (h.sphere_map[a - 1][0] - h.sphere_map[b - 1][0]).abs() < 180.0).map(|((a, b), d)| {
                let (x1, y1) = at(h.sphere_map[a - 1]);
                let (x2, y2) = at(h.sphere_map[b - 1]);
                html! { <line x1={format!("{x1:.1}")} y1={format!("{y1:.1}")} x2={format!("{x2:.1}")} y2={format!("{y2:.1}")} stroke={edge_color(*d)} stroke-width="2" /> }
            }) }
            { for h.sphere_map.iter().enumerate().map(|(k, q)| { let (x, y) = at(*q); html! { <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="3.5" fill={color(h.classes[k])} /> } }) }
            <text class="axis-lbl" x="4" y="12">{ "latitude 90" }</text>
            <text class="axis-lbl" x="4" y={format!("{:.1}", hh - 4.0)}>{ "-90; longitude -180 to 180" }</text>
        </svg>
    }
}

fn scale_key() -> Html {
    html! { <p class="note">{ "Node colors are the branches (orange: under the root's first child, the root with it; green: under the second). Edge colors are one scale for all three pictures: gray for an edge whose length in the space is the tree's 1, through to red for one off by a whole edge or more (a stretched edge and a squeezed one color alike). A ringed node was held out; a cross means nearest neighbors under the space's distance guessed its branch wrong. In the control's pictures further down the edges are dashed and uncolored: their distortion per edge is not printed." }</p> }
}

/// The three embeddings of the tree side by side.
pub fn embeddings(h: &Hyper, angle: Angle, turned: Callback<Angle>) -> Html {
    let (yaw, pitch) = angle_words(angle);
    let caption = |f: &Fit| format!("mean distortion {} over the 465 pairs, worst {} (nodes {} and {}); {} of the 7 held out guessed wrong by nearest neighbors", show(f.record.mean), show(f.record.worst), f.worst_pair.0, f.worst_pair.1, h.wrong(&f.knn_guess));
    html! {
        <section class="space" id="embeddings">
            <h2>{ "The tree fitted in each space, from the same start" }</h2>
            { p(&format!("The tree of the kit, depth {} and branching {}: {} nodes, {} edges, distances along it from 1 to {}. Fitted in the unit sphere (curvature +1), the plane (0) and the unit disk (-1) from the same seeded start, {} steps at rate {} each, the scale not fitted (an edge should come out 1 in every space). Each picture colors the edges by how far off 1 they came out, on one scale.", h.preset.depth, h.preset.branching, h.n(), h.edges.len(), 2 * h.preset.depth, h.preset.steps, h.preset.rate)) }
            <div class="figure">
                <figure class="turning">
                    <h3>{ SPACES[0] }</h3>
                    <Turning scene={sphere_scene(h, &h.fits[0], &h.fits[0].points, true)} {angle} {turned} size={420.0}
                        label={format!("the tree fitted on the unit sphere, its nodes colored by branch and its edges by distortion, turned {yaw} degrees round and {pitch} degrees toward you")} />
                    <figcaption>{ format!("On the unit sphere: {}. Turned {yaw} degrees round and {pitch} degrees toward you; turning changes no number.", caption(&h.fits[0])) }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    <h3>{ "the sphere's map" }</h3>
                    { sphere_map(h, &h.fits[0]) }
                    <figcaption>{ "The same embedding read flat: each node by its longitude and latitude. The sphere is too small for the tree: its farthest points are pi apart and the tree needs 8, so the edges are squeezed everywhere." }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    <h3>{ SPACES[1] }</h3>
                    { flat_embedding(h, &h.fits[1], &h.fits[1].points, false, false, true, "the tree fitted in the plane, its nodes colored by branch and its edges by distortion, held-out nodes ringed, wrong guesses crossed") }
                    <figcaption>{ format!("In the plane: {}.", caption(&h.fits[1])) }{ mark(true) }</figcaption>
                </figure>
                <figure class="flat">
                    <h3>{ SPACES[2] }</h3>
                    { flat_embedding(h, &h.fits[2], &h.fits[2].points, true, true, true, "the tree fitted in the Poincare disk, its nodes colored by branch and its edges by distortion, the geodesic from the root to node 31 dashed blue, node 2's three nearest nodes along the disk ringed blue") }
                    <figcaption>{ format!("In the disk: {} (k = 3, since a node of this tree has at most three neighbors along it, its parent and two children; the baseline page used the same). The dashed blue path is the geodesic from the root to node {} ({} edges apart in the tree); the blue rings are the three nodes nearest node 2 along the disk, {}, {} and {} (its parent and children are 1, 4 and 5).", caption(&h.fits[2]), h.n(), h.tree_distance(1, h.n()), h.neighbors[0], h.neighbors[1], h.neighbors[2]) }{ mark(true) }</figcaption>
                </figure>
                <figure>{ scale_key() }{ p("The disk's picture is the disk's own coordinates: an edge near the rim that looks short is as long along the disk as one near the middle that looks long. The plane's picture is scaled to fit its box, so its edge lengths can be compared with each other and not with the other pictures'.") }</figure>
            </div>
        </section>
    }
}

/// The three fits' mean distortion by step, one chart, the target as a line.
fn fit_chart(h: &Hyper) -> Html {
    let (w, hh) = (W, W * 0.6);
    let n = h.fits[0].log.len();
    let top = h.fits.iter().flat_map(|f| f.log.iter().map(|r| r.mean)).chain(h.other_means.iter().flatten().cloned()).fold(0.0, f64::max);
    let at = |k: usize, v: f64| (30.0 + (w - 40.0) * k as f64 / (n - 1) as f64, hh - 24.0 - (hh - 36.0) * v / top);
    let colors = ["#7048e8", "#868e96", "#0ca678"];
    let (_, y_t) = at(0, h.preset.target);
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the mean distortion by step of the three fits from the first start (solid: the sphere purple, the plane gray, the disk teal) and from the other two starts (dashed, the same colors); the target as a line">
            <line class="axis" x1="30" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="30" y1="12" x2="30" y2={format!("{:.1}", hh - 24.0)} />
            <line class="chord" x1="30" y1={format!("{y_t:.1}")} x2={format!("{:.1}", w - 10.0)} y2={format!("{y_t:.1}")} />
            <text class="axis-lbl" x="34" y={format!("{:.1}", y_t - 3.0)}>{ format!("distortion {}, the target", h.preset.target) }</text>
            <text class="axis-lbl" x="34" y="12">{ format!("mean distortion, 0 to {:.2}", top) }</text>
            <text class="axis-lbl" x={format!("{:.1}", w - 60.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("step {}", n - 1) }</text>
            <text class="axis-lbl" x="30" y={format!("{:.1}", hh - 8.0)}>{ "step 0" }</text>
            { for h.other_means.iter().enumerate().map(|(i, m)| {
                let d: Vec<String> = m.iter().enumerate().map(|(k, v)| { let (x, y) = at(k, *v); format!("{x:.1},{y:.1}") }).collect();
                html! { <polyline points={d.join(" ")} fill="none" stroke={colors[i / 2]} stroke-width="1" stroke-dasharray="3 3" opacity="0.7" /> }
            }) }
            { for h.fits.iter().enumerate().map(|(s, f)| {
                let d: Vec<String> = f.log.iter().enumerate().map(|(k, r)| { let (x, y) = at(k, r.mean); format!("{x:.1},{y:.1}") }).collect();
                html! { <polyline points={d.join(" ")} fill="none" stroke={colors[s]} stroke-width="2" /> }
            }) }
            { for SPACES_SHORT.iter().enumerate().map(|(s, name)| html! { <g>
                <line x1={format!("{:.1}", w - 120.0)} y1={format!("{:.1}", 30.0 + 16.0 * s as f64)} x2={format!("{:.1}", w - 100.0)} y2={format!("{:.1}", 30.0 + 16.0 * s as f64)} stroke={colors[s]} stroke-width="2" />
                <text class="axis-lbl" x={format!("{:.1}", w - 95.0)} y={format!("{:.1}", 34.0 + 16.0 * s as f64)}>{ name.to_string() }</text>
            </g> }) }
        </svg>
    }
}

/// The fits' convergence and spread.
pub fn fitting(h: &Hyper) -> Html {
    let steps = |c: f64| if c < 0.0 { format!("not in {}", h.preset.steps) } else { format!("{c:.0}") };
    html! {
        <section class="space" id="fitting">
            <h2>{ "The fit, step by step, and from three starts" }</h2>
            { p(&format!("Each fit lowers the mean over the {} pairs of (E / D - 1)^2, E the distance along the space and D the tree's, by stepping every node along the space against the gradient: {} steps at rate {}. The mean distortion (the mean of |E / D - 1|) at every step, for the first start; then every start's end.", h.n() * (h.n() - 1) / 2, h.preset.steps, h.preset.rate)) }
            <div class="figure">
                <figure class="flat">
                    { fit_chart(h) }
                    <figcaption>{ "The mean distortion at every step: solid, the three fits from the first start; dashed, the same from the other two starts. Purple: the sphere; gray: the plane; teal: the disk (not the branch colors)." }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <h3>{ "Convergence" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "space" }</th><th>{ "distortion at step 0" }</th><th>{ format!("at step {}", h.preset.steps) }</th><th>{ format!("steps to under {}", h.preset.target) }</th><th>{ "worst pair at the end" }</th><th>{ "past the rim, farthest" }</th></tr></thead>
                        <tbody>
                            { for h.fits.iter().enumerate().map(|(s, f)| html! {
                                <tr><td>{ SPACES[s] }</td><td>{ show(f.log[0].mean) }</td><td>{ show(f.record.mean) }</td><td>{ steps(f.record.steps_to_target) }</td><td>{ show(f.record.worst) }</td><td>{ if s == 2 { show(f.record.off) } else { "no rim".to_string() } }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    { p("The last column is the constraint's violation: how far past the rim's radius any step in the disk went before it was brought back (0: none did; the fit never needed the rim); the sphere and the plane have no rim. The three starts land at different distortions at step 0 (the same tangent vector, bent differently by each space). The sphere's curve flattens early: it has found the best it can do at that size. The plane's flattens by step 100; the disk's is still falling at the end.") }
                </figure>
                <figure>
                    <h3>{ "Three starts" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "space" }</th><th>{ format!("end distortion, seed {}", h.preset.seeds[0]) }</th><th>{ format!("seed {}", h.preset.seeds[1]) }</th><th>{ format!("seed {}", h.preset.seeds[2]) }</th><th>{ "spread (largest less smallest)" }</th></tr></thead>
                        <tbody>
                            { for h.spread_rows.iter().enumerate().map(|(s, r)| html! {
                                <tr><td>{ SPACES[s] }</td><td>{ show(r[0]) }</td><td>{ show(r[1]) }</td><td>{ show(r[2]) }</td><td>{ show(r[3]) }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    { p(&format!("The same fit from three seeded starts; the chart's dashed curves are the other two starts. A fit of distances by gradient descent can stop in a local minimum (an arrangement every small move makes worse), and which one depends on the start: the plane's second start is flat from step 50, a minimum; the disk's second flattens by step 150 and its third is still falling at step 200, slow rather than stuck. The rule written before the saga compares the difference between the two spaces on the first start, the disk under the plane by {}, with the spread of the disk's three ends, {} (the plane's is {}): the difference is within the spread. On those starts the baseline page's plain layout, no fit, at {} is below both fits' ends.", show(h.spread_rows[1][0] - h.spread_rows[2][0]), show(h.spread_rows[2][3]), show(h.spread_rows[1][3]), show(h.baseline[0]))) }
                </figure>
            </div>
        </section>
    }
}

fn curve_buttons(chosen: usize, curves: &[Curve], pick: Callback<usize>) -> Html {
    html! {
        <div class="controls" role="group" aria-label="the curvature the fit is shown at">
            { for curves.iter().enumerate().map(|(i, c)| {
                let pick = pick.clone();
                html! { <button class={classes!((chosen == i).then_some("on"))} onclick={Callback::from(move |_| pick.emit(i))} aria-pressed={if chosen == i { "true" } else { "false" }}>{ format!("{}", c.k) }</button> }
            }) }
        </div>
    }
}

/// Distortion against curvature, the control's rows as dots on a
/// categorical axis.
fn curve_chart(h: &Hyper, chosen: usize) -> Html {
    let (w, hh) = (W, W * 0.5);
    let n = h.curves.len();
    let top = h.curves.iter().map(|c| c.mean).fold(0.0, f64::max);
    let at = |i: usize, v: f64| (30.0 + (w - 60.0) * (i as f64 + 0.5) / n as f64, hh - 24.0 - (hh - 36.0) * v / top);
    let d: Vec<String> = h.curves.iter().enumerate().map(|(i, c)| { let (x, y) = at(i, c.mean); format!("{x:.1},{y:.1}") }).collect();
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the mean distortion of the fit at each curvature of the control, positive on the left through zero to negative on the right, the chosen one filled">
            <line class="axis" x1="30" y1={format!("{:.1}", hh - 24.0)} x2={format!("{:.1}", w - 10.0)} y2={format!("{:.1}", hh - 24.0)} />
            <line class="axis" x1="30" y1="12" x2="30" y2={format!("{:.1}", hh - 24.0)} />
            <text class="axis-lbl" x="34" y="12">{ format!("mean distortion at step {}, 0 to {:.2}", h.preset.control_steps, top) }</text>
            <polyline points={d.join(" ")} fill="none" stroke="#868e96" stroke-width="1.5" />
            { for h.curves.iter().enumerate().map(|(i, c)| {
                let (x, y) = at(i, c.mean);
                html! { <g>
                    <circle cx={format!("{x:.1}")} cy={format!("{y:.1}")} r={if i == chosen { "6" } else { "4" }} fill={if i == chosen { "#1c7ed6" } else { "var(--bg, white)" }} stroke="#1c7ed6" stroke-width="2" />
                    <text class="axis-lbl" x={format!("{:.1}", x - 10.0)} y={format!("{:.1}", hh - 8.0)}>{ format!("{}", c.k) }</text>
                </g> }
            }) }
            <text class="axis-lbl" x={format!("{:.1}", w - 90.0)} y={format!("{:.1}", hh - 30.0)}>{ "curvature K" }</text>
        </svg>
    }
}

/// Curvature as a control: the fit at each curvature of the preset.
pub fn control(h: &Hyper, chosen: usize, pick: Callback<usize>, angle: Angle, turned: Callback<Angle>) -> Html {
    let c = &h.curves[chosen];
    let plane = h.curves.iter().position(|c| c.k == 0.0).unwrap_or(0);
    let (yaw, pitch) = angle_words(angle);
    // The control's pictures are every fit at the control's step; the distortion per edge is not printed there, so the edges are drawn dashed, uncolored.
    let shown: &Fit = if c.k > 0.0 { &h.fits[0] } else if c.k < 0.0 { &h.fits[2] } else { &h.fits[1] };
    let steps = |s: f64| if s < 0.0 { format!("not in {}", h.preset.control_steps) } else { format!("{s:.0}") };
    let when = format!("the embedding at step {}; the edges are dashed, their distortion per edge not printed", h.preset.control_steps);
    let picture = if c.k > 0.0 {
        html! { <figure class="turning">
            <Turning scene={sphere_scene(h, shown, &c.points, false)} {angle} {turned} size={420.0}
                label={format!("the tree fitted on a sphere of curvature {}, drawn on the unit sphere, turned {yaw} degrees round and {pitch} degrees toward you", c.k)} />
            <figcaption>{ format!("Curvature {}: a sphere of radius {:.2}, drawn as the unit sphere (every length on it is {:.2} times what is drawn); {}. Turned {yaw} degrees round and {pitch} degrees toward you.", c.k, 1.0 / c.k.sqrt(), 1.0 / c.k.sqrt(), when) }{ mark(true) }</figcaption>
        </figure> }
    } else if c.k < 0.0 {
        html! { <figure class="flat">
            { flat_embedding(h, shown, &c.points, true, false, false, &format!("the tree fitted in a disk of curvature {}, drawn in the unit disk", c.k)) }
            <figcaption>{ format!("Curvature {}: a disk whose lengths are {:.2} times the unit disk's, drawn in the unit disk; {}. The nearer the rim the tree's leaves sit, the more room they have.", c.k, 1.0 / (-c.k).sqrt(), when) }{ mark(true) }</figcaption>
        </figure> }
    } else {
        html! { <figure class="flat">
            { flat_embedding(h, shown, &c.points, false, false, false, "the tree fitted in the plane") }
            <figcaption>{ format!("Curvature 0: the plane; {}.", when) }{ mark(true) }</figcaption>
        </figure> }
    };
    let moved = lesson::narrate::moved(&format!("the mean distortion at step {} (the control's step for every row), from the plane (curvature 0) to curvature {}", h.preset.control_steps, c.k), h.curves[plane].mean, c.mean);
    html! {
        <section class="space" id="curvature">
            <h2>{ "Curvature as a control" }</h2>
            { p(&format!("The same fit from the first start at each of {} curvatures, from +1 (the unit sphere) through 0 (the plane) to -4, each for {} steps at rate {}: a sphere of curvature K is the unit sphere with every length times 1 / sqrt K, a disk of curvature K the unit disk with every length divided by sqrt -K, and the step along each is the unit space's step at rate {} sqrt |K|. Choose a curvature; the picture shows its embedding and the sentence says what moved against the plane's.", h.curves.len(), h.preset.control_steps, h.preset.rate, h.preset.rate)) }
            { curve_buttons(chosen, &h.curves, pick) }
            <div class="figure">
                { picture }
                <figure class="flat">
                    { curve_chart(h, chosen) }
                    <figcaption>{ format!("The mean distortion at step {} against the curvature, positive on the left, negative on the right; the chosen curvature filled.", h.preset.control_steps) }{ mark(true) }</figcaption>
                </figure>
                <figure>
                    <h3>{ "The control's row" }{ mark(true) }</h3>
                    <table class="numbers">
                        <thead><tr><th>{ "curvature" }</th><th>{ format!("mean distortion at step {}", h.preset.control_steps) }</th><th>{ "worst pair" }</th><th>{ format!("steps to under {}", h.preset.target) }</th><th>{ "past the rim" }</th></tr></thead>
                        <tbody>
                            { for h.curves.iter().enumerate().map(|(i, r)| html! {
                                <tr class={classes!((i == chosen).then_some("sum"))}><td>{ format!("{}", r.k) }</td><td>{ show(r.mean) }</td><td>{ show(r.worst) }</td><td>{ steps(r.steps_to_target) }</td><td>{ show(r.off) }</td></tr>
                            }) }
                        </tbody>
                    </table>
                    <p class="prose">{ moved }</p>
                    { p(&format!("Every row and picture here is at step {}, the control's budget: the rows at +1, 0 and -1 are the three fits above read at that step, and their pictures the embeddings those fits kept there (the same start, rate and steps give the same numbers; the record's end values at step {} differ a little from these). From either side, the fit at a curvature near 0 comes out near the plane's: the plane is the limit of both families. On this start at this step the lowest of the eight rows is at -1/4, but the three rows from -1/16 to -1 lie within {} of each other, less than the spread over starts measured above, so which negative curvature is best is not settled here; what the control does show is the fall from +1 to the plane and the rise at -4, where the tree is pushed toward the rim.", h.preset.control_steps, h.preset.steps, show((h.curves[4].mean.max(h.curves[5].mean).max(h.curves[6].mean)) - h.curves[4].mean.min(h.curves[5].mean).min(h.curves[6].mean)))) }
                </figure>
            </div>
        </section>
    }
}

/// The tree's record as a comparison, the baseline's row beside it, and
/// the H1 table: three data sets by three spaces, from both programs.
pub fn analysis(e: &Explorer, h: &Hyper) -> Html {
    let steps = |c: f64| if c < 0.0 { "not reached".to_string() } else { format!("{c:.0}") };
    let rows: Vec<(&str, Vec<Cell>)> = SPACES.iter().zip(&h.fits).map(|(name, f)| {
        let r = f.record;
        (*name, vec![Cell::measured(show(r.mean)), Cell::measured(show(r.worst)), Cell::measured(steps(r.steps_to_target)), Cell::measured(show(r.forgiven)), Cell::measured(show(r.knn)), Cell::measured(show(r.logistic)), Cell::measured(format!("{:.0}", r.trained)), Cell::measured(format!("{:.0}", r.operations))])
    }).collect();
    let sentences = vec![
        lesson::narrate::moved("the tree's mean distortion, from the plane to the disk", h.fits[1].record.mean, h.fits[2].record.mean),
        lesson::narrate::moved("the tree's mean distortion, from the plane to the unit sphere", h.fits[1].record.mean, h.fits[0].record.mean),
        lesson::narrate::moved("the tree's worst pair, from the plane to the disk", h.fits[1].record.worst, h.fits[2].record.worst),
        format!("The baseline page's plain layout of the same tree, no fit: mean distortion {}, worst {}, nearest neighbors {} (measured there and again here).", show(h.baseline[0]), show(h.baseline[1]), show(h.baseline[2])),
    ];
    let acc = |a: f64| Cell::measured(show(a));
    let dist = |m: f64, w: f64| Cell::measured(format!("{} mean, {} worst", show(m), show(w)));
    let h1: Vec<(&str, Vec<Cell>)> = vec![
        ("flat clusters, the plane", vec![dist(e.distortion[0].mean, e.distortion[0].worst), acc(e.records[0].accuracy), acc(e.records[4].accuracy), Cell::measured(format!("{:.0}", e.convergence[0].steps_to_target)), Cell::measured(format!("{:.0}", e.records[4].operations))]),
        ("flat clusters, the sphere", vec![dist(e.distortion[1].mean, e.distortion[1].worst), acc(e.records[1].accuracy), acc(e.records[5].accuracy), Cell::measured(format!("{:.0}", e.convergence[1].steps_to_target)), Cell::measured(format!("{:.0}", e.records[5].operations))]),
        ("directions, the plane", vec![dist(e.distortion[2].mean, e.distortion[2].worst), acc(e.records[2].accuracy), acc(e.records[6].accuracy), Cell::measured(format!("{:.0}", e.convergence[2].steps_to_target)), Cell::measured(format!("{:.0}", e.records[6].operations))]),
        ("directions, the sphere", vec![dist(e.distortion[3].mean, e.distortion[3].worst), acc(e.records[3].accuracy), acc(e.records[7].accuracy), Cell::measured(if e.convergence[3].steps_to_target < 0.0 { "not reached".to_string() } else { format!("{:.0}", e.convergence[3].steps_to_target) }), Cell::measured(format!("{:.0}", e.records[7].operations))]),
        ("the tree, the plane", vec![dist(h.fits[1].record.mean, h.fits[1].record.worst), acc(h.fits[1].record.knn), acc(h.fits[1].record.logistic), Cell::measured(steps(h.fits[1].record.steps_to_target)), Cell::measured(format!("{:.0}", h.fits[1].record.operations))]),
        ("the tree, the sphere", vec![dist(h.fits[0].record.mean, h.fits[0].record.worst), acc(h.fits[0].record.knn), acc(h.fits[0].record.logistic), Cell::measured(steps(h.fits[0].record.steps_to_target)), Cell::measured(format!("{:.0}", h.fits[0].record.operations))]),
        ("the tree, the disk", vec![dist(h.fits[2].record.mean, h.fits[2].record.worst), acc(h.fits[2].record.knn), acc(h.fits[2].record.logistic), Cell::measured(steps(h.fits[2].record.steps_to_target)), Cell::measured(format!("{:.0}", h.fits[2].record.operations))]),
    ];
    let h1_sentences = vec![
        "The first two sets' rows are part 1's: representations chosen by a rule (the points as they are; their directions), the distortion after one scale is fitted, the accuracy of nearest neighbors (k = 5) and of the network, the network's steps to a training loss under 0.5 and its operations. The tree's rows are part 2's: fitted embeddings at a fixed scale, the accuracy of nearest neighbors (k = 3) and of the logistic classifier on the embedding, the fit's steps to a mean distortion under 0.2 and its operations. So the column accuracy, trained classifier is the network for the first two sets and the logistic classifier for the tree, and steps to target is to a loss for the first two and to a distortion for the tree: a number is compared down a set's rows and not across sets.".to_string(),
        "Two cells of the nine have no row: the flat clusters and the directions in the disk were not fitted (120 or 160 points make 7140 or 12720 pairs against the tree's 465, so the nine fits would take minutes in a page, not seconds); they are open, and the table is seven cells of nine.".to_string(),
    ];
    html! { <>
        <h3>{ "The tree's record" }</h3>
        { comparison(&["mean distortion", "worst pair", "steps to under 0.2", "distortion, scale forgiven", "nearest neighbors", "logistic", "numbers fitted", "operations"], &rows, &sentences) }
        <h3>{ "H1: three data sets by three spaces" }</h3>
        { comparison(&["distortion", "accuracy, nearest neighbors", "accuracy, trained classifier", "steps to target", "operations"], &h1, &h1_sentences) }
    </> }
}
