//! The model: the demo's own X_eTaL program (representation-explorer.xtl)
//! run once, its core followed by one `r_avel` line per array the page
//! shows. Nothing here computes a number: the page lays X_eTaL's out.

use microscope::run::{numbers, output, section};

pub const SOURCE: &str = include_str!("../../representation-explorer.xtl");

pub fn core() -> &'static str {
    section(SOURCE, "\"kt:\" u_se< \"Kit\"", "# -- end of the core")
}

/// Every array the page reads back, with its count.
pub const PRINTED: [(&str, usize); 56] = [
    ("F", 240),
    ("fy", 120),
    ("Fu", 240),
    ("D", 480),
    ("dy", 160),
    ("Du", 480),
    ("I", 3),
    ("J", 3),
    ("distances", 9),
    ("p", 3),
    ("q", 3),
    ("workedDiff", 3),
    ("workedSq", 3),
    ("workedSum", 1),
    ("workedStraight", 1),
    ("workedDot", 1),
    ("workedLengths", 2),
    ("workedCos", 1),
    ("workedCosine", 1),
    ("workedAngle", 1),
    ("segment", 39),
    ("arc", 39),
    ("chord", 39),
    ("arcLength", 1),
    ("chordLength", 1),
    ("plainMean", 12),
    ("unitMean", 12),
    ("sphereMean", 12),
    ("meanLengths", 8),
    ("meanApart", 4),
    ("fHeld", 30),
    ("fRawGuess", 30),
    ("fUnitGuess", 30),
    ("dHeld", 40),
    ("dRawGuess", 40),
    ("dUnitGuess", 40),
    ("gridF", 882),
    ("flatBoundary", 441),
    ("angleBoundary", 441),
    ("gridLL", 576),
    ("mapAngle", 288),
    ("mapHalf", 288),
    ("mapOne", 288),
    ("mapTwo", 288),
    ("lossFlatRaw", 301),
    ("lossFlatUnit", 301),
    ("lossDirRaw", 301),
    ("lossDirUnit", 301),
    ("netFlatRawGuess", 30),
    ("netFlatUnitGuess", 30),
    ("netDirRawGuess", 40),
    ("netDirUnitGuess", 40),
    ("convergence", 12),
    ("distortion", 12),
    ("worstPair", 2),
    ("record", 32),
];

/// The wire lines, read back after the arrays above.
pub const WIRES: [(&str, usize); 3] = [("parallels", 375), ("meridians", 900), ("dMap", 320)];

pub fn tail() -> String {
    let mut s = String::from("# -- what the page reads back, one array per line\n");
    for (name, _) in PRINTED.iter().chain(&WIRES) {
        s += &format!("r_avel {name}\n");
    }
    s
}

pub fn program() -> String {
    format!("{}{}", core(), tail())
}

pub type P2 = [f64; 2];
pub type P3 = [f64; 3];

/// One data set in one representation, as drawn: its points, labels
/// (from 1), the held-out rows (from 1) and the guess for each by
/// nearest neighbors and by the network.
#[derive(Clone, Debug, PartialEq)]
pub struct Set<const D: usize> {
    pub points: Vec<[f64; D]>,
    pub labels: Vec<usize>,
    pub held: Vec<usize>,
    pub guesses: Vec<usize>,
    pub net_guesses: Vec<usize>,
}

impl<const D: usize> Set<D> {
    /// For a held-out row (from 1), whether nearest neighbors guessed it right.
    pub fn right(&self, row: usize) -> Option<bool> {
        self.held.iter().position(|&h| h == row).map(|k| self.guesses[k] == self.labels[row - 1])
    }
    pub fn wrong(&self) -> usize {
        self.held.iter().zip(&self.guesses).filter(|(h, g)| self.labels[**h - 1] != **g).count()
    }
    pub fn net_wrong(&self) -> usize {
        self.held.iter().zip(&self.net_guesses).filter(|(h, g)| self.labels[**h - 1] != **g).count()
    }
}

/// One run's record: accuracy, held out, numbers trained, operations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Record {
    pub accuracy: f64,
    pub held: f64,
    pub trained: f64,
    pub operations: f64,
}

/// One training run's convergence: the loss before the first step,
/// after the last, and the steps to a loss under the target (-1: never).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Convergence {
    pub start: f64,
    pub end: f64,
    pub steps_to_target: f64,
}

/// One row of the distortion table: the fitted scale, the mean over
/// pairs, the worst pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distortion {
    pub scale: f64,
    pub mean: f64,
    pub worst: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Explorer {
    pub flat: Set<2>,
    pub flat_unit: Set<2>,
    pub directions: Set<3>,
    pub unit: Set<3>,
    /// The three pairs (rows of the directions set) and their straight-line, cosine and great-circle distances.
    pub pairs: Vec<(usize, usize)>,
    pub distances: Vec<[f64; 3]>,
    /// The worked pair: p, q, the difference, its squares, their sum, the straight line; p . q, the lengths, the cosine, the cosine distance, the angle.
    pub p: P3,
    pub q: P3,
    pub worked_diff: P3,
    pub worked_sq: P3,
    pub worked_sum: f64,
    pub worked_straight: f64,
    pub worked_dot: f64,
    pub worked_lengths: [f64; 2],
    pub worked_cos: f64,
    pub worked_cosine: f64,
    pub worked_angle: f64,
    /// The segment from p to q in the flat space; the arc and the chord between the unit points; their lengths.
    pub segment: Vec<P3>,
    pub arc: Vec<P3>,
    pub chord: Vec<P3>,
    pub arc_length: f64,
    pub chord_length: f64,
    /// Per class of the directions: the plain mean, the plain mean of the unit points, the mean on the sphere; the lengths of the two plain means; the angle between the mean on the sphere and the plain mean put onto it.
    pub plain_mean: Vec<P3>,
    pub unit_mean: Vec<P3>,
    pub sphere_mean: Vec<P3>,
    pub mean_lengths: Vec<[f64; 2]>,
    pub mean_apart: Vec<f64>,
    /// The plane's grid (441 points) and its class under the straight line and under the angle.
    pub grid: Vec<P2>,
    pub flat_boundary: Vec<usize>,
    pub angle_boundary: Vec<usize>,
    /// The map's grid (288 longitude, latitude pairs, radians) and its class under the angle and under the straight line at length 0.5, 1 and 2.
    pub map_grid: Vec<P2>,
    pub map_angle: Vec<usize>,
    pub map_half: Vec<usize>,
    pub map_one: Vec<usize>,
    pub map_two: Vec<usize>,
    /// The training loss by step (301 each): flat raw, flat unit, directions raw, directions unit.
    pub losses: Vec<Vec<f64>>,
    pub convergence: Vec<Convergence>,
    /// Flat on flat, sphere on flat, flat on directions, sphere on directions.
    pub distortion: Vec<Distortion>,
    /// The pair of directions the flat representation keeps worst.
    pub worst_pair: (usize, usize),
    /// Nearest neighbors on the four (set, representation) pairs, then the network on the same four.
    pub records: Vec<Record>,
    pub parallels: Vec<Vec<P3>>,
    pub meridians: Vec<Vec<P3>>,
    /// The directions on the sphere's map: longitude and latitude, degrees.
    pub map: Vec<P2>,
}

pub const RUNS: [&str; 8] = [
    "nearest neighbors, flat clusters, flat",
    "nearest neighbors, flat clusters, sphere",
    "nearest neighbors, directions, flat",
    "nearest neighbors, directions, sphere",
    "the network, flat clusters, flat",
    "the network, flat clusters, sphere",
    "the network, directions, flat",
    "the network, directions, sphere",
];

pub const DISTORTIONS: [&str; 4] = [
    "flat clusters kept flat",
    "flat clusters kept on the sphere",
    "directions kept flat",
    "directions kept on the sphere",
];

fn rows<const D: usize>(v: &[f64]) -> Vec<[f64; D]> {
    v.chunks(D).map(|c| std::array::from_fn(|i| c[i])).collect()
}

fn lines(v: &[f64], n: usize) -> Vec<Vec<P3>> {
    rows::<3>(v).chunks(n).map(|c| c.to_vec()).collect()
}

pub fn run() -> Result<Explorer, String> {
    let out = output(&program(), PRINTED.len() + WIRES.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().chain(&WIRES).enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let ints = |name: &str| v(name).iter().map(|x| *x as usize).collect::<Vec<_>>();
    let one = |name: &str| v(name)[0];
    let p3 = |name: &str| -> P3 { let w = v(name); [w[0], w[1], w[2]] };
    let (i, j) = (ints("I"), ints("J"));
    let wp = ints("worstPair");
    Ok(Explorer {
        flat: Set { points: rows(v("F")), labels: ints("fy"), held: ints("fHeld"), guesses: ints("fRawGuess"), net_guesses: ints("netFlatRawGuess") },
        flat_unit: Set { points: rows(v("Fu")), labels: ints("fy"), held: ints("fHeld"), guesses: ints("fUnitGuess"), net_guesses: ints("netFlatUnitGuess") },
        directions: Set { points: rows(v("D")), labels: ints("dy"), held: ints("dHeld"), guesses: ints("dRawGuess"), net_guesses: ints("netDirRawGuess") },
        unit: Set { points: rows(v("Du")), labels: ints("dy"), held: ints("dHeld"), guesses: ints("dUnitGuess"), net_guesses: ints("netDirUnitGuess") },
        pairs: i.iter().zip(&j).map(|(a, b)| (*a, *b)).collect(),
        distances: rows(v("distances")),
        p: p3("p"),
        q: p3("q"),
        worked_diff: p3("workedDiff"),
        worked_sq: p3("workedSq"),
        worked_sum: one("workedSum"),
        worked_straight: one("workedStraight"),
        worked_dot: one("workedDot"),
        worked_lengths: [v("workedLengths")[0], v("workedLengths")[1]],
        worked_cos: one("workedCos"),
        worked_cosine: one("workedCosine"),
        worked_angle: one("workedAngle"),
        segment: rows(v("segment")),
        arc: rows(v("arc")),
        chord: rows(v("chord")),
        arc_length: one("arcLength"),
        chord_length: one("chordLength"),
        plain_mean: rows(v("plainMean")),
        unit_mean: rows(v("unitMean")),
        sphere_mean: rows(v("sphereMean")),
        mean_lengths: (0..4).map(|c| [v("meanLengths")[c], v("meanLengths")[4 + c]]).collect(),
        mean_apart: v("meanApart").to_vec(),
        grid: rows(v("gridF")),
        flat_boundary: ints("flatBoundary"),
        angle_boundary: ints("angleBoundary"),
        map_grid: rows(v("gridLL")),
        map_angle: ints("mapAngle"),
        map_half: ints("mapHalf"),
        map_one: ints("mapOne"),
        map_two: ints("mapTwo"),
        losses: ["lossFlatRaw", "lossFlatUnit", "lossDirRaw", "lossDirUnit"].iter().map(|n| v(n).to_vec()).collect(),
        convergence: v("convergence").chunks(3).map(|r| Convergence { start: r[0], end: r[1], steps_to_target: r[2] }).collect(),
        distortion: v("distortion").chunks(3).map(|r| Distortion { scale: r[0], mean: r[1], worst: r[2] }).collect(),
        worst_pair: (wp[0], wp[1]),
        records: v("record").chunks(4).map(|r| Record { accuracy: r[0], held: r[1], trained: r[2], operations: r[3] }).collect(),
        parallels: lines(v("parallels"), 25),
        meridians: lines(v("meridians"), 25),
        map: rows(v("dMap")),
    })
}
