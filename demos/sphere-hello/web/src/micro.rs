//! The model: the demo's own X_eTaL program (sphere-hello.xtl) run
//! once, its core followed by one `r_avel` line per array the page
//! shows. Nothing here knows about the browser, and nothing here
//! computes a number: the page lays X_eTaL's out and lets the reader
//! pick which pair to read.

use microscope::run::{numbers, output, section};

pub const SOURCE: &str = include_str!("../../sphere-hello.xtl");

/// The program's core: from its imports to the end-of-core marker.
pub fn core() -> &'static str {
    section(SOURCE, "\"mf:\" u_se< \"Manifold\"", "# -- end of the core")
}

/// What the page reads back, in order: (name, items).
pub const PRINTED: [(&str, usize); 13] = [
    ("P", 18),
    ("chord", 36),
    ("arc", 36),
    ("byChord", 36),
    ("byArc", 36),
    ("plain", 3),
    ("plainLength", 1),
    ("onSphere", 3),
    ("apart", 1),
    ("arcs", 195 * 3),
    ("map", 12),
    ("parallels", 125 * 3),
    ("meridians", 156 * 3),
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

/// The six points as X_eTaL measured them.
#[derive(Clone, Debug, PartialEq)]
pub struct Hello {
    pub p: Vec<P3>,
    /// The two distance matrices, 6 by 6, row by row.
    pub chord: Vec<f64>,
    pub arc: Vec<f64>,
    /// Each point's neighbors in order (the point itself first), 6 rows of 6, points from 1.
    pub by_chord: Vec<usize>,
    pub by_arc: Vec<usize>,
    pub plain: P3,
    pub plain_length: f64,
    pub on_sphere: P3,
    /// The distance along the sphere between the mean on it and the plain mean put onto it.
    pub apart: f64,
    /// The great-circle arc of each pair, 15 arcs of 13 points, pairs in order 1-2, 1-3, ..., 5-6.
    pub arcs: Vec<Vec<P3>>,
    /// Each point's longitude and latitude, degrees.
    pub map: Vec<P2>,
    pub parallels: Vec<Vec<P3>>,
    pub meridians: Vec<Vec<P3>>,
}

fn rows<const D: usize>(v: &[f64]) -> Vec<[f64; D]> {
    v.chunks(D).map(|c| std::array::from_fn(|i| c[i])).collect()
}

fn wires<const D: usize>(v: &[f64], per: usize) -> Vec<Vec<[f64; D]>> {
    rows::<D>(v).chunks(per).map(|c| c.to_vec()).collect()
}

pub fn run() -> Result<Hello, String> {
    let out = output(&program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let ints = |name: &str| v(name).iter().map(|x| *x as usize).collect::<Vec<_>>();
    Ok(Hello {
        p: rows(v("P")),
        chord: v("chord").to_vec(),
        arc: v("arc").to_vec(),
        by_chord: ints("byChord"),
        by_arc: ints("byArc"),
        plain: [v("plain")[0], v("plain")[1], v("plain")[2]],
        plain_length: v("plainLength")[0],
        on_sphere: [v("onSphere")[0], v("onSphere")[1], v("onSphere")[2]],
        apart: v("apart")[0],
        arcs: wires(v("arcs"), 13),
        map: rows(v("map")),
        parallels: wires(v("parallels"), 25),
        meridians: wires(v("meridians"), 13),
    })
}

/// The pairs, in the order the program lists their arcs (points from 1).
pub const PAIRS: [(usize, usize); 15] = [(1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (2, 3), (2, 4), (2, 5), (2, 6), (3, 4), (3, 5), (3, 6), (4, 5), (4, 6), (5, 6)];

impl Hello {
    /// The chord and the arc of a pair (points from 1).
    pub fn distances(&self, i: usize, j: usize) -> (f64, f64) {
        (self.chord[(i - 1) * 6 + (j - 1)], self.arc[(i - 1) * 6 + (j - 1)])
    }
    /// A point's neighbors nearest first, itself left out (points from 1).
    pub fn neighbors(&self, i: usize, by_arc: bool) -> Vec<usize> {
        let order = if by_arc { &self.by_arc } else { &self.by_chord };
        order[(i - 1) * 6..i * 6].iter().copied().filter(|&j| j != i).collect()
    }
    /// The widest pair by each distance, and the widest gap between the two distances.
    pub fn widest(&self) -> (f64, f64, f64) {
        let chord = self.chord.iter().cloned().fold(0.0, f64::max);
        let arc = self.arc.iter().cloned().fold(0.0, f64::max);
        let gap = self.arc.iter().zip(&self.chord).map(|(a, c)| a - c).fold(0.0, f64::max);
        (chord, arc, gap)
    }
    /// Whether every point's neighbors come in the same order by both.
    pub fn same_orders(&self) -> usize {
        (1..=6).filter(|&i| self.neighbors(i, false) == self.neighbors(i, true)).count()
    }
}
