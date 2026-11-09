//! The page, on the shared teaching shell and turning view: the arc of
//! frames; the three data sets drawn (the clusters flat, the directions
//! turning, the tree's layout with its edges), the held-out points
//! ringed and the wrong guesses crossed; the record of every run as the
//! comparison table; every idea explained; the program. Presentation
//! only: every number and point is one X_eTaL printed.

use yew::prelude::*;

use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::definition;
use microscope::turning::{Angle, Point, Scene, Turning};
use site::{comparison, functions, mark, own_function, Cell, ANALYSIS, EXPERIMENT, EXTENSION, INTUITION, MATHEMATICS, RUN};

use crate::micro::{program, run, Baseline, Set, P2, RUNS};
use crate::text;

fn show(x: f64) -> String {
    format!("{x:.4}")
}

/// A class's color (1 to 4).
const COLORS: [&str; 4] = ["#e8590c", "#2b8a3e", "#1c7ed6", "#9c6500"];

fn color(label: usize) -> &'static str {
    COLORS[(label.max(1) - 1) % COLORS.len()]
}

const W: f64 = 420.0;

/// A flat set: a scatter of its points colored by class; held-out
/// points ringed; a wrong guess crossed.
fn scatter<const D: usize>(set: &Set<D>, extra: Html, label: &str, at: &dyn Fn([f64; D]) -> (f64, f64)) -> Html {
    html! {
        <svg viewBox={format!("0 0 {W} {W}")} role="img" aria-label={label.to_string()}>
            { extra }
            { for set.points.iter().enumerate().map(|(k, q)| {
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

/// The extent of a set's first two coordinates, with a margin, as a box.
fn fit2<const D: usize>(points: &[[f64; D]]) -> impl Fn([f64; D]) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for q in points {
        for c in &q[..2] {
            lo = lo.min(*c);
            hi = hi.max(*c);
        }
    }
    let (lo, hi) = (lo - 0.5, hi + 0.5);
    move |q: [f64; D]| (12.0 + (W - 24.0) * (q[0] - lo) / (hi - lo), W - 12.0 - (W - 24.0) * (q[1] - lo) / (hi - lo))
}

fn tree_edges(b: &Baseline, at: &dyn Fn(P2) -> (f64, f64)) -> Html {
    html! { <>{ for b.edges.iter().map(|(a, c)| {
        let (x1, y1) = at(b.tree.points[a - 1]);
        let (x2, y2) = at(b.tree.points[c - 1]);
        html! { <line class="wire" x1={format!("{x1:.1}")} y1={format!("{y1:.1}")} x2={format!("{x2:.1}")} y2={format!("{y2:.1}")} /> }
    }) }</> }
}

/// The directions as a scene for the shared view (the program writes
/// them with z up; the view turns about y, so (x, y, z) goes to
/// (x, z, -y), a rotation). A wrong guess is labeled with what was
/// guessed; nothing else is.
fn directions_scene(set: &Set<3>) -> Scene {
    let points = set.points.iter().enumerate().map(|(k, q)| {
        let row = k + 1;
        let label = match set.right(row) {
            Some(false) => format!("x: guessed {}", set.guesses[set.held.iter().position(|&h| h == row).unwrap()]),
            _ => String::new(),
        };
        Point { at: [q[0], q[2], -q[1]], color: color(set.labels[k]).into(), label }
    }).collect();
    Scene { points, segments: vec![], wires: vec![] }
}

fn key() -> Html {
    html! { <p class="note">{ "Colors are the classes (1 orange, 2 green, 3 blue, 4 brown). A ringed point was held out; a cross on it means its guess was wrong." }</p> }
}

fn sets(b: &Baseline, angle: Angle, turned: Callback<Angle>, unit_angle: Angle, unit_turned: Callback<Angle>) -> Html {
    let flat_at = fit2(&b.flat.points);
    let tree_at = fit2(&b.tree.points);
    let (yaw, pitch) = ((angle.yaw.to_degrees().round() as i32).rem_euclid(360), angle.pitch.to_degrees().round() as i32);
    html! { <>
        <section class="space" id="flat">
            <h2>{ "Flat clusters" }</h2>
            { p("Three clusters of 40 points in the plane (seed 1), made by the Kit library: each point its center plus 1.6 times a normal vector, so the clusters touch. The five-dimensional set (4 clusters of 30, seed 2) is in the record below and not drawn.") }
            <div class="figure">
                <figure class="flat">
                    { scatter(&b.flat, html! {}, "the flat clusters in the plane, colored by class, the held-out points ringed and wrong guesses crossed", &flat_at) }
                    <figcaption>{ format!("The 120 points; {} of the 30 held out were guessed wrong.", b.flat.wrong()) }{ mark(true) }</figcaption>
                </figure>
                <figure>{ key() }</figure>
            </div>
        </section>
        <section class="space" id="directions">
            <h2>{ "Directions" }</h2>
            { p("Four classes of 40 points in three dimensions (seed 5) that differ by direction, not by length: each class points one way, each point is that direction plus 0.6 times a normal vector put back onto the sphere, then scaled by a length between 0.5 and 2. Left, the points as they are; right, the same with their lengths dropped (put onto the unit sphere). Both turn.") }
            <div class="figure">
                <figure class="turning">
                    <Turning scene={directions_scene(&b.directions)} {angle} {turned} size={420.0}
                        label={format!("the directional set in three dimensions, colored by class, turned {yaw} degrees round and {pitch} degrees toward you")} />
                    <figcaption>{ format!("Lengths kept: turned {yaw} degrees round and {pitch} degrees toward you. {} of the 40 held out guessed wrong.", b.directions.wrong()) }{ mark(true) }</figcaption>
                </figure>
                <figure class="turning">
                    <Turning scene={directions_scene(&b.unit)} angle={unit_angle} turned={unit_turned} size={420.0}
                        label="the same points put onto the unit sphere, colored by class" />
                    <figcaption>{ format!("Lengths dropped: every point on the unit sphere. {} of the 40 held out guessed wrong.", b.unit.wrong()) }{ mark(true) }</figcaption>
                </figure>
                <figure>{ key() }{ p("Drag either picture, use the arrow keys once it has the focus, or swipe, to turn it; the two turn separately; turning changes no number. In these two pictures only the wrong guesses are labeled (x, and what was guessed); the held-out points are not ringed, as the shared view draws points by color and label only.") }</figure>
            </div>
        </section>
        <section class="space" id="tree">
            <h2>{ "A tree, and its layout" }</h2>
            { p("A tree of depth 4 and branching 2: 31 nodes, the root at the middle, each level a ring of radius its depth, the nodes of a level spread round it in order (the Kit library's plain layout). The lines are the tree's edges; a node's class is the branch of the root it is under. Nearest neighbors here is on the layout's coordinates, k = 3.") }
            <div class="figure">
                <figure class="flat">
                    { scatter(&b.tree, tree_edges(b, &tree_at), "the tree's layout in the plane, its edges drawn, nodes colored by branch, held-out nodes ringed", &tree_at) }
                    <figcaption>{ format!("The 31 nodes; {} of the 7 held out guessed wrong. The layout keeps the tree's distances to {} on average over the pairs and {} at the worst pair (distortion, Manifold's).", b.tree.wrong(), show(b.distortion.0), show(b.distortion.1)) }{ mark(true) }</figcaption>
                </figure>
                <figure>{ key() }{ p("Two leaves of neighboring branches sit side by side on the page and are 8 steps apart along the tree: that is the worst pair.") }</figure>
            </div>
        </section>
    </> }
}

fn analysis(b: &Baseline) -> Html {
    let rows: Vec<(&str, Vec<Cell>)> = RUNS.iter().zip(&b.records).map(|(name, r)| {
        (*name, vec![Cell::measured(show(r.accuracy)), Cell::measured(format!("{:.0}", r.held)), Cell::measured(format!("{:.0}", r.trained)), Cell::measured(format!("{:.0}", r.operations))])
    }).collect();
    let sentences = vec![
        lesson::narrate::moved("the accuracy on the directions, from the lengths kept to dropped", b.records[2].accuracy, b.records[3].accuracy),
        lesson::narrate::moved("the accuracy on the flat clusters, from the plane to five dimensions", b.records[0].accuracy, b.records[1].accuracy),
        format!("The tree's layout keeps the tree's distances to {} on average and {} at the worst pair (measured).", show(b.distortion.0), show(b.distortion.1)),
    ];
    html! { <>
        { comparison(&["accuracy", "held out", "numbers trained", "operations"], &rows, &sentences) }
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
        { shell("Learn", "ml:", "k_nn", "The method of every run: for each held-out point, the label most of its k nearest training points have.", "The squared distances to every training point are found at once; the nearest is taken k times, each shifted out of the way once taken; the labels are counted and the largest count wins.") }
        { own("Kit", "kt:", "f_lat", "The flat clusters: k centers from the seed, each point its center plus the spread times a normal vector.", "Two seeded draws (the centers, the offsets), the labels in order, the points as the offsets plus the center each label selects.") }
        { own("Kit", "kt:", "d_irections", "The directional set: class directions on the unit sphere, points near them put back on the sphere, then scaled by a length that carries nothing.", "s_phereProject (Manifold) twice: the directions, and the perturbed points; then each row scaled by a seeded length.") }
        { own("Kit", "kt:", "t_reeLayout", "The tree's plain picture: each node on the ring of its depth, spread evenly in order.", "The depth of each node is its distance from the root (t_reeDistances); its place on the ring is its number less the nodes above its level.") }
        { own("Manifold", "mf:", "d_istortion", "How badly the layout keeps the tree's distances: the mean and the worst of |E / T - 1| over the pairs.", "A mask picks each pair once where T > 0; the ratios less one, in size, are averaged and maxed.") }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let base = use_memo((), |_| run());
    let angle = use_state(|| Angle { yaw: 0.5, pitch: 0.3 });
    let unit_angle = use_state(|| Angle { yaw: 0.5, pitch: 0.3 });
    let turned = { let a = angle.clone(); Callback::from(move |n: Angle| a.set(n)) };
    let unit_turned = { let a = unit_angle.clone(); Callback::from(move |n: Angle| a.set(n)) };
    let src = program();
    let linker = Linker::of(&src);
    let body = match &*base {
        Err(e) => html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> },
        Ok(b) => sets(b, *angle, turned, *unit_angle, unit_turned),
    };
    let after = match &*base {
        Ok(b) => analysis(b),
        Err(_) => html! {},
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("baseline-knn".into())}>
        <header>{ header("Baseline: nearest neighbors", text::LEDE) }</header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ prose(text::HINT) }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>{ for text::BACKGROUND.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Rationale, "The idea: fix everything but the geometry, then run the plainest one", html! { <>{ for text::RATIONALE.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ for text::PURPOSE.iter().map(|s| p(s)) }</> }) }
            { frame(INTUITION, "The intuition: three kinds of structure", html! { <>{ for text::INTUITION.iter().map(|s| p(s)) }</> }) }
            { frame(MATHEMATICS, "The mathematics: nearest neighbors, the split, distortion", html! { <>{ for text::MATHEMATICS.iter().map(|s| p(s)) }</> }) }
            { frame(EXPERIMENT, "The experiment: five runs, one record each", html! { <>{ for text::EXPERIMENT.iter().map(|s| p(s)) }</> }) }
            { frame(RUN, "The run: nearest neighbors on each held-out quarter", html! { <>{ for text::RUN.iter().map(|s| p(s)) }</> }) }
            { body }
            { frame(ANALYSIS, "The analysis: the record of every run", after) }
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
                <h3>{ "The library functions behind the runs" }</h3>
                { library_asides() }
                <h3>{ "Every library function the program calls" }</h3>
                { functions(&src) }
                <h3>{ "The program" }</h3>
                { linker.block(&src) }
                <p class="links"><a href="../doc/demos-baseline-knn-baseline-knn.xtl.html">{ "This program in the cross-reference" }</a></p>
            </section>
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
