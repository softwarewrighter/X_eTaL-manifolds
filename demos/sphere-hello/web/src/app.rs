//! The page, on the shared teaching shell and turning view: the arc of
//! frames; the sphere with its six points, the picked pair's chord and
//! arc, and the two means, turning; the map beside it; the pair's
//! numbers and each point's neighbors two ways; the comparison; every
//! idea explained; the program. Presentation only: every number shown
//! is one X_eTaL printed, and picking a pair runs nothing.

use yew::prelude::*;

use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::definition;
use microscope::turning::{Angle, Point, Scene, Segment, Turning};
use site::{comparison, functions, mark, own_function, Cell, ANALYSIS, EXPERIMENT, EXTENSION, INTUITION, MATHEMATICS, RUN};

use crate::micro::{program, run, Hello, P2, P3, PAIRS};
use crate::text;

fn show(x: f64) -> String {
    format!("{x:.4}")
}

const INK: &str = "#74c0fc";
const CHORD: &str = "#e8590c";
const ARC: &str = "#2b8a3e";
const PLAIN: &str = "#9c6500";
const MEAN: &str = "#1c7ed6";

/// The shared view turns about y; the program writes the sphere with z
/// up, so points are turned a quarter turn about x for drawing,
/// (x, y, z) to (x, z, -y): a rotation, nothing changes meaning.
fn upright(p: P3) -> [f64; 3] {
    [p[0], p[2], -p[1]]
}

/// The scene: the wire sphere, the six points, the picked pair's chord
/// (a straight segment through the inside) and arc (along the skin),
/// the plain mean (inside) and the mean on the sphere.
fn scene(h: &Hello, pair: usize) -> Scene {
    let (i, j) = PAIRS[pair];
    let mut segments = vec![Segment { from: upright(h.p[i - 1]), to: upright(h.p[j - 1]), color: CHORD.into(), arrow: false }];
    for w in h.arcs[pair].windows(2) {
        segments.push(Segment { from: upright(w[0]), to: upright(w[1]), color: ARC.into(), arrow: false });
    }
    segments.push(Segment { from: [0.0, 0.0, 0.0], to: upright(h.plain), color: PLAIN.into(), arrow: true });
    segments.push(Segment { from: [0.0, 0.0, 0.0], to: upright(h.on_sphere), color: MEAN.into(), arrow: true });
    let mut points: Vec<Point> = h.p.iter().enumerate().map(|(k, q)| Point { at: upright(*q), color: if k + 1 == i || k + 1 == j { CHORD.into() } else { INK.into() }, label: format!("{}", k + 1) }).collect();
    points.push(Point { at: upright(h.plain), color: PLAIN.into(), label: "plain mean".into() });
    points.push(Point { at: upright(h.on_sphere), color: MEAN.into(), label: "mean on the sphere".into() });
    Scene { points, segments, wires: h.parallels.iter().chain(&h.meridians).map(|w| w.iter().map(|q| upright(*q)).collect()).collect() }
}

/// The map: longitude across, latitude up; the six points and the picked pair's arc.
fn map(h: &Hello, pair: usize) -> Html {
    let (w, hh) = (420.0, 210.0);
    let at = |q: P2| (w * (q[0] + 180.0) / 360.0, hh / 2.0 - hh * q[1] / 180.0);
    let (i, j) = PAIRS[pair];
    // The arc's points back onto the map: the program printed the arcs in three dimensions; their longitude and latitude
    // are not printed, so the map shows the pair's endpoints joined by a straight line of the map, marked as such.
    let (a, b) = (at(h.map[i - 1]), at(h.map[j - 1]));
    html! {
        <svg viewBox={format!("0 0 {w} {hh}")} role="img" aria-label="the sphere's map, longitude across and latitude up, with the six points; the picked pair is joined">
            { for (-150..180).step_by(30).map(|lon| { let (x, _) = at([lon as f64, 0.0]); html! { <line class="wire" x1={format!("{x:.1}")} y1="0" x2={format!("{x:.1}")} y2={format!("{hh}")} /> } }) }
            { for (-60..90).step_by(30).map(|lat| { let (_, y) = at([0.0, lat as f64]); html! { <line class="wire" x1="0" y1={format!("{y:.1}")} x2={format!("{w}")} y2={format!("{y:.1}")} /> } }) }
            <line class="chord" x1={format!("{:.1}", a.0)} y1={format!("{:.1}", a.1)} x2={format!("{:.1}", b.0)} y2={format!("{:.1}", b.1)} />
            { for h.map.iter().enumerate().map(|(k, q)| {
                let (x, y) = at(*q);
                let picked = k + 1 == i || k + 1 == j;
                html! { <g>
                    <circle class={classes!("pt", picked.then_some("far"))} cx={format!("{x:.1}")} cy={format!("{y:.1}")} r="4.5" />
                    <text class="lbl" x={format!("{:.1}", x + 6.0)} y={format!("{:.1}", y - 5.0)}>{ format!("{}", k + 1) }</text>
                </g> }
            }) }
            <text class="axis-lbl" x="4" y="12">{ "latitude 90" }</text>
            <text class="axis-lbl" x="4" y={format!("{:.1}", hh - 4.0)}>{ "-90; longitude -180 to 180" }</text>
        </svg>
    }
}

fn pair_buttons(picked: usize, pick: Callback<usize>) -> Html {
    html! {
        <div class="controls" role="group" aria-label="the pair of points to read">
            { for PAIRS.iter().enumerate().map(|(k, (i, j))| {
                let pick = pick.clone();
                html! { <button class={classes!(picked.eq(&k).then_some("on"))} onclick={Callback::from(move |_| pick.emit(k))} aria-pressed={if picked == k { "true" } else { "false" }}>{ format!("{i} and {j}") }</button> }
            }) }
        </div>
    }
}

/// The picked pair's two distances, and each of its points' neighbors two ways.
fn numbers(h: &Hello, pair: usize) -> Html {
    let (i, j) = PAIRS[pair];
    let (chord, arc) = h.distances(i, j);
    let list = |v: Vec<usize>| v.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", ");
    html! { <>
        <h3>{ format!("Points {i} and {j}") }{ mark(true) }</h3>
        <table class="numbers">
            <thead><tr><th>{ "distance" }</th><th>{ "value" }</th></tr></thead>
            <tbody>
                <tr><td>{ "the chord (straight line through)" }</td><td>{ show(chord) }</td></tr>
                <tr><td>{ "the great circle (along, radians)" }</td><td>{ show(arc) }</td></tr>
                <tr class="sum"><td>{ "ratio, along to through" }</td><td>{ format!("{:.4}", arc / chord) }</td></tr>
                <tr><td>{ "2 sin(arc / 2)" }</td><td>{ show(2.0 * (arc / 2.0).sin()) }</td></tr>
            </tbody>
        </table>
        <h3>{ "Neighbors, nearest first" }{ mark(true) }</h3>
        <table class="numbers">
            <thead><tr><th>{ "point" }</th><th>{ "by the chord" }</th><th>{ "by the great circle" }</th></tr></thead>
            <tbody>
                <tr><td>{ format!("{i}") }</td><td>{ list(h.neighbors(i, false)) }</td><td>{ list(h.neighbors(i, true)) }</td></tr>
                <tr><td>{ format!("{j}") }</td><td>{ list(h.neighbors(j, false)) }</td><td>{ list(h.neighbors(j, true)) }</td></tr>
            </tbody>
        </table>
        <p class="note">{ "The last row's 2 sin(arc / 2) is the page's arithmetic on the printed arc, shown only to make the program's check visible; the chord and the arc above it are the program's." }</p>
    </> }
}

fn means(h: &Hello) -> Html {
    html! { <>
        <h3>{ "The two means" }{ mark(true) }</h3>
        <table class="numbers">
            <thead><tr><th>{ "mean" }</th><th>{ "x" }</th><th>{ "y" }</th><th>{ "z" }</th><th>{ "length" }</th></tr></thead>
            <tbody>
                <tr><td>{ "plain (coordinate by coordinate)" }</td><td>{ show(h.plain[0]) }</td><td>{ show(h.plain[1]) }</td><td>{ show(h.plain[2]) }</td><td>{ show(h.plain_length) }</td></tr>
                <tr><td>{ "on the sphere" }</td><td>{ show(h.on_sphere[0]) }</td><td>{ show(h.on_sphere[1]) }</td><td>{ show(h.on_sphere[2]) }</td><td>{ "1" }</td></tr>
            </tbody>
        </table>
        <p class="prose">{ format!("The plain mean is {} inside the sphere (1 less its length). Put onto the sphere, it is {} radians along the sphere from the mean on the sphere: the two are different points.", show(1.0 - h.plain_length), show(h.apart)) }</p>
    </> }
}

fn analysis(h: &Hello, pair: usize) -> Html {
    let (i, j) = PAIRS[pair];
    let (chord, arc) = h.distances(i, j);
    let (wc, wa, gap) = h.widest();
    let same = h.same_orders();
    let rows = [
        ("the straight line (the chord)", vec![Cell::measured(show(chord)), Cell::measured(show(wc)), Cell::measured(format!("{same} of 6"))]),
        ("along the sphere (the great circle)", vec![Cell::measured(show(arc)), Cell::measured(show(wa)), Cell::measured(format!("{same} of 6"))]),
    ];
    let mean_rows = [
        ("the plain mean", vec![Cell::measured(show(h.plain_length)), Cell::measured(show(1.0 - h.plain_length))]),
        ("the mean on the sphere", vec![Cell::measured("1.0000"), Cell::measured("0.0000")]),
    ];
    let sentences = vec![
        lesson::narrate::moved(&format!("the distance between points {i} and {j}, from the chord to the great circle"), chord, arc),
        lesson::narrate::moved("the widest distance over all pairs, from the chord to the great circle", wc, wa),
        format!("The widest gap between the two distances over all pairs is {} (measured).", show(gap)),
        lesson::narrate::moved("the distance from the middle of the room, from the plain mean to the mean on the sphere", h.plain_length, 1.0),
    ];
    html! { <>
        { comparison(&[&format!("points {i} and {j}"), "widest pair", "points whose neighbors come in the same order by both"], &rows, &sentences) }
        { comparison(&["length", "how far inside the sphere"], &mean_rows, &[format!("Along the sphere, the mean on the sphere is {} radians from the plain mean put onto the sphere (measured).", show(h.apart))]) }
        { for text::ANALYSIS.iter().map(|s| p(s)) }
    </> }
}

fn library_asides() -> Html {
    let aside = |name: &str, purpose: &str, reading: &str| match definition("Manifold", name) {
        Some(d) => own_function("mf:", &d, p(purpose), html! { <p>{ reading.to_string() }</p> }),
        None => html! {},
    };
    html! { <>
        { aside("s_phereProject", "Puts a direction onto the unit sphere by scaling it to length 1; given a set, every row. The six points are made with it.", "Read right to left: a set (rank 2) goes to the rows version; a vector is divided by its length, the zero vector refused.") }
        { aside("s_phereDistances", "The great-circle distance between every pair of rows: the matrix the arcs and the neighbor orders come from.", "a_partPairs gives |r - r'| and |r + r'| for every pair, summed a coordinate at a time; a_ngle gives 2 atan2 of them.") }
        { aside("s_phereMean", "The mean on the sphere: from the plain mean put onto the sphere, thirty steps along the average of the arrows to the points.", "p_ower applies the step thirty times; each step is s_phereStep along the mean of the arrows (a_rrows, the logarithmic maps).") }
        { aside("s_phereArc", "The great-circle arc between two points as 13 points, for the green arc in the picture.", "The arrow from p to q, then the steps along 0, 1/12, ..., 1 of it, each put back on the sphere.") }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let hello = use_memo((), |_| run());
    let picked = use_state(|| 0usize);
    let angle = use_state(|| Angle { yaw: -0.6, pitch: 0.4 });
    let src = program();
    let linker = Linker::of(&src);
    let pick = { let pk = picked.clone(); Callback::from(move |k: usize| pk.set(k)) };
    // A point clicked in the picture: it becomes one of the pair (with the pair's other point, or point 1).
    let point_picked = {
        let pk = picked.clone();
        Callback::from(move |idx: usize| {
            if idx >= 6 { return; }
            let n = idx + 1;
            let (i, j) = PAIRS[*pk];
            let other = if n == i { j } else if n == j { i } else { i };
            if let Some(k) = PAIRS.iter().position(|&(a, b)| (a == n && b == other) || (a == other && b == n)) {
                pk.set(k);
            }
        })
    };
    let turned = { let a = angle.clone(); Callback::from(move |n: Angle| a.set(n)) };
    let body = match &*hello {
        Err(e) => html! { <section class="panel"><p class="notice" role="status">{ format!("X_eTaL stopped: {e}") }</p></section> },
        Ok(h) => {
            let (yaw, pitch) = ((angle.yaw.to_degrees().round() as i32).rem_euclid(360), angle.pitch.to_degrees().round() as i32);
            let (i, j) = PAIRS[*picked];
            html! {
                <section class="space" id="points">
                    <h2>{ "The six points, and a pair of them" }</h2>
                    { p("Pick a pair with the buttons, or click a point in the picture and then another. Orange is the pair and its chord through the inside; green is its great circle along the skin; the two arrows from the middle are the plain mean (ending inside the sphere) and the mean on the sphere.") }
                    { pair_buttons(*picked, pick) }
                    <div class="figure">
                        <figure class="turning">
                            <Turning scene={scene(h, *picked)} angle={*angle} {turned} size={420.0} picked={Some(point_picked)}
                                label={format!("the unit sphere with six points, points {i} and {j} joined by their chord and their great circle, and the two means; turned {yaw} degrees round and {pitch} degrees toward you")} />
                            <figcaption>{ format!("The sphere in three dimensions, turned {yaw} degrees round and {pitch} degrees toward you. Drag, use the arrow keys once it has the focus, or swipe to turn it; turning changes no number. The wire lines show where the sphere is.") }{ mark(false) }</figcaption>
                        </figure>
                        <figure class="flat">
                            { map(h, *picked) }
                            <figcaption>{ format!("The sphere's map: each point by its longitude and latitude (the program's, in degrees). Points {i} and {j} are joined by a straight line of the map, which is neither the chord nor the great circle: the map is for reading positions.") }{ mark(false) }</figcaption>
                        </figure>
                        <figure>{ numbers(h, *picked) }</figure>
                    </div>
                    { means(h) }
                </section>
            }
        }
    };
    let after = match &*hello {
        Ok(h) => analysis(h, *picked),
        Err(_) => html! {},
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("sphere-hello".into())}>
        <header>{ header("Sphere hello", text::LEDE) }</header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                <p class="hint">{ prose(text::HINT) }</p>
                <p class="prose"><b>{ text::QUESTION[0] }</b></p>
                { for text::QUESTION[1..].iter().map(|s| p(s)) }
            </> }) }
            { frame(Frame::Background, "What you need to know first", html! { <>{ for text::BACKGROUND.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Rationale, "The idea: measure the same points two ways", html! { <>{ for text::RATIONALE.iter().map(|s| p(s)) }</> }) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ for text::PURPOSE.iter().map(|s| p(s)) }</> }) }
            { frame(INTUITION, "The intuition: two cities on the globe", html! { <>{ for text::INTUITION.iter().map(|s| p(s)) }</> }) }
            { frame(MATHEMATICS, "The mathematics: a chord, a great circle, two means", html! { <>{ for text::MATHEMATICS.iter().map(|s| p(s)) }</> }) }
            { frame(EXPERIMENT, "The experiment: every pair, every neighbor order, two means", html! { <>{ for text::EXPERIMENT.iter().map(|s| p(s)) }</> }) }
            { frame(RUN, "The run: nothing is trained", html! { <>{ for text::RUN.iter().map(|s| p(s)) }</> }) }
            { body }
            { frame(ANALYSIS, "The analysis: the two distances and the two means, compared", after) }
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
                <p class="links"><a href="../doc/demos-sphere-hello-sphere-hello.xtl.html">{ "This program in the cross-reference" }</a></p>
            </section>
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
