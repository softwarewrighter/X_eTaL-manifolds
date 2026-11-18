//! The model: the demo's own X_eTaL program (optimization-explorer.xtl)
//! run once, its core followed by one `r_avel` line per array the page
//! shows, with the preset put where the program's `[]N_GET` reads it.
//! Nothing here knows about the browser, and nothing here computes a
//! number: the page lays X_eTaL's out and lets the reader pick which
//! anisotropy of the control to read.

use microscope::run::{numbers, output_with, section};

pub const SOURCE: &str = include_str!("../../optimization-explorer.xtl");
pub const PRESET: &str = include_str!("../../presets/metrics.txt");
pub const PRESET_PATH: &str = "presets/metrics.txt";

/// The program's core: from its imports to the end-of-core marker.
pub fn core() -> &'static str {
    section(SOURCE, "\"mf:\" u_se< \"Manifold\"", "# -- end of the core")
}

/// What the page reads back, in order: (name, items). The counts follow
/// the preset (30 steps, 60 on the sphere, 8 anisotropies).
pub const PRINTED: [(&str, usize); 42] = [
    ("preset", 20),
    ("anis", 8),
    ("gradientGap", 1),
    ("contours", 245 * 2),
    ("field", 35 * 4),
    ("plainPath", 31 * 2),
    ("plainLoss", 31),
    ("newtonPath", 31 * 2),
    ("newtonLoss", 31),
    ("plainFirst", 7),
    ("newtonFirst", 7),
    ("startParts", 2),
    ("record", 12),
    ("control", 8 * 5),
    ("controlPaths", 8 * 31 * 2),
    ("controlLosses", 8 * 31),
    ("controlFirst", 8 * 7),
    ("g0", 2),
    ("plainNorm", 1),
    ("newtonPg", 2),
    ("newtonNorm", 1),
    ("plainDir", 2),
    ("newtonDir", 2),
    ("rates", 2),
    ("plainBall", 49 * 2),
    ("newtonBall", 49 * 2),
    ("sampledPlain", 2),
    ("sampledNewton", 2),
    ("eigenvalues", 3),
    ("top", 3),
    ("quotientGap", 1),
    ("expPath", 61 * 3),
    ("expLog", 61 * 3),
    ("retractPath", 61 * 3),
    ("retractLog", 61 * 3),
    ("flatPath", 61 * 3),
    ("flatLog", 61 * 3),
    ("sphereRecord", 15),
    ("parallels", 125 * 3),
    ("meridians", 300 * 3),
    ("checkAt", 6),
    ("contraction", 2),
];

pub fn tail() -> String {
    let mut s = String::from("# -- what the page reads back, one array per line\n");
    for (name, _) in PRINTED {
        s += &format!("r_avel {name}\n");
    }
    s
}

pub fn program() -> String {
    format!("{}{}", core(), tail())
}

pub type P2 = [f64; 2];
pub type P3 = [f64; 3];

/// The preset, as the program read it.
#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    pub start: P2,
    pub steps: usize,
    pub target: f64,
    pub levels: Vec<f64>,
    pub sphere_rate: f64,
    pub sphere_steps: usize,
    pub tolerance: f64,
    /// The anisotropies a of the control, as the program divided them.
    pub anis: Vec<f64>,
}

/// One descent under a metric: its path (the start first) and the loss at each point.
#[derive(Clone, Debug, PartialEq)]
pub struct Descent {
    pub path: Vec<P2>,
    pub loss: Vec<f64>,
}

/// The record of a descent: the program's row.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Record {
    pub steps_to_target: f64,
    pub after_one: f64,
    pub after_five: f64,
    pub end: f64,
    pub numbers: f64,
    pub multiplies: f64,
}

/// A row of the control: a, the condition number it leaves, the steps to the target, the loss after five steps and at the end.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlRow {
    pub a: f64,
    pub kappa: f64,
    pub steps_to_target: f64,
    pub after_five: f64,
    pub end: f64,
}

/// A first step worked: the direction d = P g, eta, the point reached, and eta's two parts g . d and d . H d.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct First {
    pub d: P2,
    pub eta: f64,
    pub to: P2,
    pub dot: f64,
    pub den: f64,
}

/// A way up the sphere: its path and, per point, the quotient, the distance to the eigenvector, how far off the sphere the step went.
#[derive(Clone, Debug, PartialEq)]
pub struct Climb {
    pub path: Vec<P3>,
    pub log: Vec<[f64; 3]>,
    /// The record: the quotient at the end, the distance at the end, the steps to the tolerance, the farthest and the mean off the sphere.
    pub record: [f64; 5],
}

/// Everything the program measured, as the page reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct Opt {
    pub preset: Preset,
    pub gradient_gap: f64,
    /// The three points the gradient was checked at.
    pub check_at: Vec<P2>,
    /// The contours, one per level, 49 points each.
    pub contours: Vec<Vec<P2>>,
    /// The gradient field: (point, gradient) at 35 grid points.
    pub field: Vec<(P2, P2)>,
    pub plain: Descent,
    pub newton: Descent,
    pub plain_first: First,
    pub newton_first: First,
    /// The loss at the start in its two parts, x^2 and 3 y^2.
    pub start_parts: P2,
    /// Analytical: the factor per step each way shrinks its distance by near the eigenvector (the flat step repaired; the step along the sphere).
    pub contraction: P2,
    pub plain_record: Record,
    pub newton_record: Record,
    pub control: Vec<ControlRow>,
    pub control_paths: Vec<Vec<P2>>,
    pub control_losses: Vec<Vec<f64>>,
    /// The first step under each a of the control.
    pub control_first: Vec<First>,
    pub g0: P2,
    pub plain_norm: f64,
    pub newton_pg: P2,
    pub newton_norm: f64,
    pub plain_dir: P2,
    pub newton_dir: P2,
    pub rates: P2,
    pub plain_ball: Vec<P2>,
    pub newton_ball: Vec<P2>,
    pub sampled_plain: P2,
    pub sampled_newton: P2,
    pub eigenvalues: P3,
    pub top: P3,
    pub quotient_gap: f64,
    /// The exact step, the retraction, the flat step repaired.
    pub climbs: [Climb; 3],
    pub parallels: Vec<Vec<P3>>,
    pub meridians: Vec<Vec<P3>>,
}

fn rows<const D: usize>(v: &[f64]) -> Vec<[f64; D]> {
    v.chunks(D).map(|c| std::array::from_fn(|i| c[i])).collect()
}

fn wires<const D: usize>(v: &[f64], per: usize) -> Vec<Vec<[f64; D]>> {
    rows::<D>(v).chunks(per).map(|c| c.to_vec()).collect()
}

pub fn run() -> Result<Opt, String> {
    let (out, _) = output_with(&[(PRESET_PATH.to_string(), PRESET.to_string())], &program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let one = |name: &str| v(name)[0];
    let p2 = |name: &str| -> P2 { let w = v(name); [w[0], w[1]] };
    let p3 = |name: &str| -> P3 { let w = v(name); [w[0], w[1], w[2]] };
    let pr = v("preset");
    let preset = Preset {
        start: [pr[0], pr[1]],
        steps: pr[2] as usize,
        target: pr[3],
        levels: pr[4..9].to_vec(),
        sphere_rate: pr[9],
        sphere_steps: pr[10] as usize,
        tolerance: pr[11],
        anis: v("anis").to_vec(),
    };
    let descent = |p: &str, l: &str| Descent { path: rows(v(p)), loss: v(l).to_vec() };
    let first = |w: &[f64]| First { d: [w[0], w[1]], eta: w[2], to: [w[3], w[4]], dot: w[5], den: w[6] };
    let record = |r: &[f64]| Record { steps_to_target: r[0], after_one: r[1], after_five: r[2], end: r[3], numbers: r[4], multiplies: r[5] };
    let rec = v("record");
    let climb = |p: &str, l: &str, k: usize| Climb { path: rows(v(p)), log: rows(v(l)), record: std::array::from_fn(|i| v("sphereRecord")[5 * k + i]) };
    Ok(Opt {
        gradient_gap: one("gradientGap"),
        check_at: rows(v("checkAt")),
        contours: wires(v("contours"), 49),
        field: rows::<4>(v("field")).iter().map(|r| ([r[0], r[1]], [r[2], r[3]])).collect(),
        plain: descent("plainPath", "plainLoss"),
        newton: descent("newtonPath", "newtonLoss"),
        plain_first: first(v("plainFirst")),
        newton_first: first(v("newtonFirst")),
        start_parts: p2("startParts"),
        contraction: p2("contraction"),
        plain_record: record(&rec[..6]),
        newton_record: record(&rec[6..]),
        control: v("control").chunks(5).map(|r| ControlRow { a: r[0], kappa: r[1], steps_to_target: r[2], after_five: r[3], end: r[4] }).collect(),
        control_paths: wires(v("controlPaths"), preset.steps + 1),
        control_losses: v("controlLosses").chunks(preset.steps + 1).map(|c| c.to_vec()).collect(),
        control_first: v("controlFirst").chunks(7).map(first).collect(),
        g0: p2("g0"),
        plain_norm: one("plainNorm"),
        newton_pg: p2("newtonPg"),
        newton_norm: one("newtonNorm"),
        plain_dir: p2("plainDir"),
        newton_dir: p2("newtonDir"),
        rates: p2("rates"),
        plain_ball: rows(v("plainBall")),
        newton_ball: rows(v("newtonBall")),
        sampled_plain: p2("sampledPlain"),
        sampled_newton: p2("sampledNewton"),
        eigenvalues: p3("eigenvalues"),
        top: p3("top"),
        quotient_gap: one("quotientGap"),
        climbs: [climb("expPath", "expLog", 0), climb("retractPath", "retractLog", 1), climb("flatPath", "flatLog", 2)],
        parallels: wires(v("parallels"), 25),
        meridians: wires(v("meridians"), 25),
        preset,
    })
}

/// The three ways up the sphere, in the program's order.
pub const WAYS: [&str; 3] = ["the step that stays on the sphere (exact)", "the same with the cheap retraction", "a flat step, repaired"];

impl Opt {
    /// The control's row whose a is the plain step's (1) and Newton's (1/3).
    pub fn control_at(&self, a: f64) -> Option<usize> {
        self.control.iter().position(|r| (r.a - a).abs() < 1e-9)
    }
    /// The last loss of a path's log that is above zero, for a chart on a log scale.
    pub fn smallest_positive(&self) -> f64 {
        self.plain.loss.iter().chain(&self.newton.loss).chain(self.control_losses.iter().flatten()).copied().filter(|&x| x > 0.0).fold(f64::INFINITY, f64::min)
    }
}
