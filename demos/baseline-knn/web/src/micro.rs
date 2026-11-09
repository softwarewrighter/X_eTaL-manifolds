//! The model: the demo's own X_eTaL program (baseline-knn.xtl) run
//! once, its core followed by one `r_avel` line per array the page
//! shows. Nothing here computes a number: the page lays X_eTaL's out.

use microscope::run::{numbers, output, section};

pub const SOURCE: &str = include_str!("../../baseline-knn.xtl");

pub fn core() -> &'static str {
    section(SOURCE, "\"kt:\" u_se< \"Kit\"", "# -- end of the core")
}

pub const PRINTED: [(&str, usize); 17] = [
    ("F", 240),
    ("fy", 120),
    ("flatHeld", 30),
    ("flatGuess", 30),
    ("D", 480),
    ("dy", 160),
    ("dirHeld", 40),
    ("dirGuess", 40),
    ("U", 480),
    ("unitGuess", 40),
    ("L", 62),
    ("ty", 31),
    ("edges", 60),
    ("treeHeld", 7),
    ("treeGuess", 7),
    ("record", 20),
    ("distortion", 2),
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

/// One data set as drawn: its points, labels (from 1), the held-out
/// rows (from 1) and the guess for each.
#[derive(Clone, Debug, PartialEq)]
pub struct Set<const D: usize> {
    pub points: Vec<[f64; D]>,
    pub labels: Vec<usize>,
    pub held: Vec<usize>,
    pub guesses: Vec<usize>,
}

impl<const D: usize> Set<D> {
    /// For a held-out row (from 1), whether it was guessed right.
    pub fn right(&self, row: usize) -> Option<bool> {
        self.held.iter().position(|&h| h == row).map(|k| self.guesses[k] == self.labels[row - 1])
    }
    pub fn wrong(&self) -> usize {
        self.held.iter().zip(&self.guesses).filter(|(h, g)| self.labels[**h - 1] != **g).count()
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

#[derive(Clone, Debug, PartialEq)]
pub struct Baseline {
    pub flat: Set<2>,
    pub directions: Set<3>,
    /// The directions put onto the unit sphere, with their guesses.
    pub unit: Set<3>,
    pub tree: Set<2>,
    /// The tree's edges, parent over child (from 1).
    pub edges: Vec<(usize, usize)>,
    /// The five runs: flat (plane), flat (five dimensions), directions, directions on the sphere, the tree's layout.
    pub records: Vec<Record>,
    /// The layout's distortion against the tree: the mean over pairs, the worst pair.
    pub distortion: (f64, f64),
}

pub const RUNS: [&str; 5] = ["flat clusters, the plane", "flat clusters, five dimensions", "directions, lengths kept", "directions, lengths dropped", "the tree's layout"];

fn rows<const D: usize>(v: &[f64]) -> Vec<[f64; D]> {
    v.chunks(D).map(|c| std::array::from_fn(|i| c[i])).collect()
}

pub fn run() -> Result<Baseline, String> {
    let out = output(&program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let ints = |name: &str| v(name).iter().map(|x| *x as usize).collect::<Vec<_>>();
    let edges = v("edges");
    Ok(Baseline {
        flat: Set { points: rows(v("F")), labels: ints("fy"), held: ints("flatHeld"), guesses: ints("flatGuess") },
        directions: Set { points: rows(v("D")), labels: ints("dy"), held: ints("dirHeld"), guesses: ints("dirGuess") },
        unit: Set { points: rows(v("U")), labels: ints("dy"), held: ints("dirHeld"), guesses: ints("unitGuess") },
        tree: Set { points: rows(v("L")), labels: ints("ty"), held: ints("treeHeld"), guesses: ints("treeGuess") },
        edges: (0..30).map(|k| (edges[k] as usize, edges[30 + k] as usize)).collect(),
        records: v("record").chunks(4).map(|r| Record { accuracy: r[0], held: r[1], trained: r[2], operations: r[3] }).collect(),
        distortion: (v("distortion")[0], v("distortion")[1]),
    })
}
