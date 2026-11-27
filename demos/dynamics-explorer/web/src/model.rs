//! The model: the demo's own X_eTaL program (dynamics-explorer.xtl)
//! run once, its core followed by one `r_avel` line per array the page
//! shows, with the preset put where the program's `[]N_GET` reads it.
//! Nothing here knows about the browser, and nothing here computes a
//! number: the page lays X_eTaL's out and lets the reader pick a
//! field, a step size and how many steps to show.

use microscope::run::{numbers, output_with, section};

pub const SOURCE: &str = include_str!("../../dynamics-explorer.xtl");
pub const PRESET: &str = include_str!("../../presets/fields.txt");
pub const PRESET_PATH: &str = "presets/fields.txt";

/// The program's core: from its imports to the end-of-core marker.
pub fn core() -> &'static str {
    section(SOURCE, "\"mx:\" u_se< \"Matrix\"", "# -- end of the core")
}

/// What the page reads back, in order: (name, items). The counts follow
/// the preset: 5 fields, 4 starts, 40 steps, a 9 by 9 grid, a loop of
/// 64 pieces, 4 step sizes of 200 steps, 100 steps of the game.
pub const PRINTED: [(&str, usize); 34] = [
    ("preset", 46),
    ("grid", 81 * 2),
    ("arrows", 5 * 81 * 2),
    ("rkPaths", 5 * 4 * 41 * 2),
    ("exactPaths", 5 * 4 * 41 * 2),
    ("pathErrors", 5),
    ("measures", 5 * 6),
    ("circle", 65 * 2),
    ("area", 1),
    ("loops", 5 * 4),
    ("sidesRotation", 4),
    ("heightRotation", 4),
    ("sidesAttractor", 4),
    ("gradientGaps", 2),
    ("symmetric", 5),
    ("stability", 5 * 7),
    ("rotPaths", 8 * 201 * 2),
    ("rotOff", 8 * 201),
    ("eulerFactor", 4),
    ("rkFactor", 4),
    ("predictedEnd", 8),
    ("measuredEnd", 8),
    ("sameWork", 7),
    ("gamePaths", 4 * 101 * 2),
    ("gameDistance", 4 * 101),
    ("gameMatrices", 16),
    ("gameFactors", 4),
    ("turnsEnergy", 101),
    ("gameFieldGap", 1),
    ("ticks", 9),
    ("square", 5 * 2),
    ("tol", 1),
    ("squareWork", 4 * 8),
    ("turnsBand", 2),
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

/// The five fields, in the preset's order.
pub const FIELDS: [&str; 5] = ["attractor", "rotation", "spiral", "saddle", "expanding spiral"];

/// The four rules of the game section, in the program's order.
pub const RULES: [&str; 4] = ["gradient descent on the loss", "simultaneous descent and ascent", "taking turns", "the look-ahead step (extragradient)"];

/// The preset, as the program read it.
#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    /// Each field's matrix by rows (a b / c d).
    pub fields: [[f64; 4]; 5],
    pub starts: [P2; 4],
    pub h: f64,
    pub path_steps: usize,
    pub half: f64,
    pub side: usize,
    pub dh: f64,
    pub loop_n: usize,
    pub r0: P2,
    pub sizes: [f64; 4],
    pub rot_steps: usize,
    pub game0: P2,
    pub eta: f64,
    pub game_steps: usize,
    pub tol: f64,
}

/// One field's measures: the exact divergence and curl, their means over the grid by differences, the largest gaps.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Measures {
    pub div: f64,
    pub curl: f64,
    pub div_mean: f64,
    pub curl_mean: f64,
    pub div_gap: f64,
    pub curl_gap: f64,
}

/// One field's loop: the sum of v . dx round the circle, the flow out, and the curl and the divergence times the area.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Loop {
    pub sum: f64,
    pub out: f64,
    pub curl_area: f64,
    pub div_area: f64,
}

/// One field's rest point: trace, determinant, the two real parts (largest first), the imaginary part's size; the largest and smallest end-over-start distance of the paths.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stability {
    pub trace: f64,
    pub det: f64,
    pub re: P2,
    pub im: f64,
    pub largest: f64,
    pub smallest: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub a: [f64; 4],
    /// The field's arrows at the grid points (the same order as `Dynamics::grid`).
    pub arrows: Vec<P2>,
    /// From each start: Runge-Kutta's points and the exact ones, at the same times.
    pub rk: Vec<Vec<P2>>,
    pub exact: Vec<Vec<P2>>,
    pub path_error: f64,
    pub measures: Measures,
    pub loop_: Loop,
    pub symmetric: bool,
    pub stability: Stability,
}

/// One step size on the rotation: Euler's and Runge-Kutta's points and distances from the circle by step.
#[derive(Clone, Debug, PartialEq)]
pub struct Rotation {
    pub size: f64,
    pub euler: Vec<P2>,
    pub rk: Vec<P2>,
    pub euler_off: Vec<f64>,
    pub rk_off: Vec<f64>,
    pub euler_factor: f64,
    pub rk_factor: f64,
    /// Analytical and measured distance from the circle at the end: Euler, Runge-Kutta.
    pub predicted: P2,
    pub measured: P2,
}

/// One rule of the game: its points, its distance from the solution by step, its matrix by rows, its factor.
#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub path: Vec<P2>,
    pub distance: Vec<f64>,
    pub matrix: [f64; 4],
    pub factor: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dynamics {
    pub preset: Preset,
    pub grid: Vec<P2>,
    pub ticks: Vec<f64>,
    pub fields: Vec<Field>,
    /// The loop: the polygon on the unit circle, its area.
    pub circle: Vec<P2>,
    pub area: f64,
    /// The square worked: each side's v . dx for the rotation and the attractor; the rotation's running height.
    pub square: Vec<P2>,
    pub sides_rotation: [f64; 4],
    pub height_rotation: [f64; 4],
    pub sides_attractor: [f64; 4],
    /// The square worked, a row per side: the middle, the side as a step, the rotation's arrow at the middle, the attractor's.
    pub square_work: Vec<[f64; 8]>,
    /// The attractor's and the saddle's largest gap from minus the gradient of their loss.
    pub gradient_gaps: P2,
    pub rotations: Vec<Rotation>,
    /// The same work: time covered by Euler and Runge-Kutta, looks at the field by each, distance from the circle at the end of each, Runge-Kutta's steps.
    pub same_work: [f64; 7],
    pub rules: Vec<Rule>,
    pub turns_energy: Vec<f64>,
    /// Taking turns: the smallest and the largest distance from the solution over the steps.
    pub turns_band: P2,
    pub game_field_gap: f64,
}

fn rows(v: &[f64]) -> Vec<P2> {
    v.chunks(2).map(|c| [c[0], c[1]]).collect()
}

fn four(v: &[f64]) -> [f64; 4] {
    [v[0], v[1], v[2], v[3]]
}

pub fn run() -> Result<Dynamics, String> {
    let (out, _) = output_with(&[(PRESET_PATH.to_string(), PRESET.to_string())], &program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let pr = v("preset");
    let preset = Preset {
        fields: std::array::from_fn(|k| four(&pr[4 * k..])),
        starts: std::array::from_fn(|j| [pr[20 + 2 * j], pr[21 + 2 * j]]),
        h: pr[28],
        path_steps: pr[29] as usize,
        half: pr[30],
        side: pr[31] as usize,
        dh: pr[32],
        loop_n: pr[33] as usize,
        r0: [pr[34], pr[35]],
        sizes: four(&pr[36..]),
        rot_steps: pr[40] as usize,
        game0: [pr[41], pr[42]],
        eta: pr[43],
        game_steps: pr[44] as usize,
        tol: pr[45],
    };
    let np = preset.side * preset.side;
    let n = preset.path_steps + 1;
    let (arrows, rk, exact) = (rows(v("arrows")), rows(v("rkPaths")), rows(v("exactPaths")));
    let (me, lo, st) = (v("measures"), v("loops"), v("stability"));
    let fields = (0..5).map(|k| Field {
        a: preset.fields[k],
        arrows: arrows[k * np..(k + 1) * np].to_vec(),
        rk: (0..4).map(|j| rk[(k * 4 + j) * n..(k * 4 + j + 1) * n].to_vec()).collect(),
        exact: (0..4).map(|j| exact[(k * 4 + j) * n..(k * 4 + j + 1) * n].to_vec()).collect(),
        path_error: v("pathErrors")[k],
        measures: { let m = &me[6 * k..]; Measures { div: m[0], curl: m[1], div_mean: m[2], curl_mean: m[3], div_gap: m[4], curl_gap: m[5] } },
        loop_: { let l = &lo[4 * k..]; Loop { sum: l[0], out: l[1], curl_area: l[2], div_area: l[3] } },
        symmetric: v("symmetric")[k] == 0.0,
        stability: { let s = &st[7 * k..]; Stability { trace: s[0], det: s[1], re: [s[2], s[3]], im: s[4], largest: s[5], smallest: s[6] } },
    }).collect();
    let r = preset.rot_steps + 1;
    let (rp, ro) = (rows(v("rotPaths")), v("rotOff"));
    let rotations = (0..4).map(|j| Rotation {
        size: preset.sizes[j],
        euler: rp[2 * j * r..(2 * j + 1) * r].to_vec(),
        rk: rp[(2 * j + 1) * r..(2 * j + 2) * r].to_vec(),
        euler_off: ro[2 * j * r..(2 * j + 1) * r].to_vec(),
        rk_off: ro[(2 * j + 1) * r..(2 * j + 2) * r].to_vec(),
        euler_factor: v("eulerFactor")[j],
        rk_factor: v("rkFactor")[j],
        predicted: [v("predictedEnd")[j], v("predictedEnd")[4 + j]],
        measured: [v("measuredEnd")[j], v("measuredEnd")[4 + j]],
    }).collect();
    let g = preset.game_steps + 1;
    let (gp, gd) = (rows(v("gamePaths")), v("gameDistance"));
    let rules = (0..4).map(|k| Rule {
        path: gp[k * g..(k + 1) * g].to_vec(),
        distance: gd[k * g..(k + 1) * g].to_vec(),
        matrix: four(&v("gameMatrices")[4 * k..]),
        factor: v("gameFactors")[k],
    }).collect();
    Ok(Dynamics {
        grid: rows(v("grid")),
        ticks: v("ticks").to_vec(),
        fields,
        circle: rows(v("circle")),
        area: v("area")[0],
        square: rows(v("square")),
        sides_rotation: four(v("sidesRotation")),
        height_rotation: four(v("heightRotation")),
        sides_attractor: four(v("sidesAttractor")),
        square_work: v("squareWork").chunks(8).map(|c| std::array::from_fn(|i| c[i])).collect(),
        gradient_gaps: [v("gradientGaps")[0], v("gradientGaps")[1]],
        rotations,
        same_work: std::array::from_fn(|i| v("sameWork")[i]),
        rules,
        turns_energy: v("turnsEnergy").to_vec(),
        turns_band: [v("turnsBand")[0], v("turnsBand")[1]],
        game_field_gap: v("gameFieldGap")[0],
        preset,
    })
}

impl Dynamics {
    /// What the paths did, in words the page uses: from the measured end-over-start distances.
    pub fn did(&self, k: usize) -> &'static str {
        let s = self.fields[k].stability;
        if (s.largest - 1.0).abs() < 1e-6 && (s.smallest - 1.0).abs() < 1e-6 {
            "every path ended as far from the rest point as it started, to under a part in a million"
        } else if s.largest < 1.0 {
            "every path ended nearer the rest point"
        } else if s.smallest > 1.0 {
            "every path ended farther from the rest point"
        } else {
            "some paths ended nearer and some farther"
        }
    }
    /// What the eigenvalues say, from the program's real parts.
    pub fn says(&self, k: usize) -> &'static str {
        let s = self.fields[k].stability;
        if s.re[0] < 0.0 {
            "both real parts negative: paths fall in"
        } else if s.re[1] > 0.0 {
            "both real parts positive: paths leave"
        } else if s.re[0] > 0.0 {
            "one real part positive, one negative: in along one direction, out along another"
        } else {
            "real parts zero: paths neither fall in nor leave"
        }
    }
}
