//! The model: the demo's own X_eTaL program (mhc-explorer.xtl) run
//! once, its core followed by one `r_avel` line per array the page
//! shows, with the preset put where the program's `[]N_GET` reads it.
//! Nothing here computes a number: the page lays X_eTaL's out and lets
//! the reader pick which point of the path to read.

use microscope::run::{numbers, output_with, section};

pub const SOURCE: &str = include_str!("../../mhc-explorer.xtl");
pub const PRESET: &str = include_str!("../../presets/matrices.txt");
pub const PRESET_PATH: &str = "presets/matrices.txt";

pub fn core() -> &'static str {
    section(SOURCE, "\"hc:\" u_se< \"Mixing\"", "# -- end of the core")
}

/// What the page reads back, in order, with its count (from the preset:
/// 3 streams of 4, 11 points on the path, 20 sweeps).
pub const PRINTED: [(&str, usize); 29] = [
    ("preset", 15),
    ("x", 4),
    ("one", 4),
    ("copies", 12),
    ("X", 12),
    ("mixedX", 12),
    ("counts", 4),
    ("ts", 11),
    ("pathMatrices", 99),
    ("pathTable", 132),
    ("A", 9),
    ("rowSums", 3),
    ("byRows", 9),
    ("colSums", 3),
    ("firstSweep", 9),
    ("rowSumsAfter", 3),
    ("errors", 42),
    ("sweepsToTolerance", 1),
    ("S", 9),
    ("kinds", 36),
    ("signalTable", 48),
    ("kindTable", 28),
    ("rates", 4),
    ("twoByTwo", 44),
    ("permChecks", 6),
    ("perms", 54),
    ("corner", 3),
    ("middle", 3),
    ("zeros", 2),
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

/// A square matrix of size 3, by rows.
pub type M3 = [f64; 9];

/// A point of the path: t, its matrix, the row and column sums, the smallest entry, the singular values, the distance from the set.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PathPoint {
    pub t: f64,
    pub matrix: M3,
    pub row_sums: [f64; 3],
    pub col_sums: [f64; 3],
    pub smallest: f64,
    pub singular: [f64; 3],
    pub off: f64,
}

/// One kind of matrix and what it did: its matrix; the signal's (mean, spread, length) before and after each number of mixes; its singular values, gain (forward, backward), distance from doubly stochastic and from orthogonal.
#[derive(Clone, Debug, PartialEq)]
pub struct Kind {
    pub matrix: M3,
    /// (mean, spread, length): before, then after each number of mixes of the preset.
    pub signal: [[f64; 3]; 4],
    pub singular: [f64; 3],
    pub gain: [f64; 2],
    pub stochastic_off: f64,
    pub orthogonal_off: f64,
}

pub const KINDS: [&str; 4] = ["doubly stochastic (Sinkhorn's result)", "free (the positive matrix it came from)", "free, scaled down", "orthogonal (the nearest to the free one)"];

#[derive(Clone, Debug, PartialEq)]
pub struct Mhc {
    pub n: usize,
    pub d: usize,
    pub sweeps: usize,
    pub tolerance: f64,
    pub mixes: [usize; 3],
    pub signal: [f64; 3],
    pub x: Vec<f64>,
    pub one: Vec<f64>,
    pub copies: Vec<Vec<f64>>,
    pub streams: Vec<Vec<f64>>,
    pub mixed: Vec<Vec<f64>>,
    /// Entries, independent conditions (the rank of the conditions), free entries, and (n - 1)^2.
    pub counts: [f64; 4],
    pub path: Vec<PathPoint>,
    pub a: M3,
    pub row_sums: [f64; 3],
    pub by_rows: M3,
    pub col_sums: [f64; 3],
    pub first_sweep: M3,
    /// The row sums after the first sweep.
    pub row_sums_after: [f64; 3],
    /// (row error, column error) before the first sweep and after each.
    pub errors: Vec<[f64; 2]>,
    pub sweeps_to_tolerance: f64,
    pub s: M3,
    pub kinds: Vec<Kind>,
    /// The rate per mix each signal's length changed at, measured between the last two numbers of mixes.
    pub rates: [f64; 4],
    /// Entries at 0 at the identity and at the middle.
    pub zeros: [f64; 2],
    /// The size-2 matrices along p: (p, 1 - p, 1 - p, p).
    pub two_by_two: Vec<[f64; 4]>,
    pub perm_checks: Vec<f64>,
    pub perms: Vec<M3>,
    /// The smallest entry at steps -0.1, 0, +0.1 from the identity along D, and from the middle along E.
    pub corner: [f64; 3],
    pub middle: [f64; 3],
}

fn m3(v: &[f64]) -> M3 {
    std::array::from_fn(|i| v[i])
}

pub fn run() -> Result<Mhc, String> {
    let (out, _) = output_with(&[(PRESET_PATH.to_string(), PRESET.to_string())], &program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let pr = v("preset");
    let (n, d) = (pr[0] as usize, pr[1] as usize);
    let rows = |name: &str, w: usize| v(name).chunks(w).map(|c| c.to_vec()).collect::<Vec<_>>();
    let mats = v("pathMatrices");
    let path = v("pathTable").chunks(12).enumerate().map(|(i, r)| PathPoint {
        t: r[0],
        matrix: m3(&mats[9 * i..]),
        row_sums: [r[1], r[2], r[3]],
        col_sums: [r[4], r[5], r[6]],
        smallest: r[7],
        singular: [r[8], r[9], r[10]],
        off: r[11],
    }).collect();
    let kinds_m = v("kinds");
    let sig = v("signalTable");
    let kt = v("kindTable");
    let kinds = (0..4).map(|k| Kind {
        matrix: m3(&kinds_m[9 * k..]),
        signal: std::array::from_fn(|j| [sig[12 * k + 3 * j], sig[12 * k + 3 * j + 1], sig[12 * k + 3 * j + 2]]),
        singular: [kt[7 * k], kt[7 * k + 1], kt[7 * k + 2]],
        gain: [kt[7 * k + 3], kt[7 * k + 4]],
        stochastic_off: kt[7 * k + 5],
        orthogonal_off: kt[7 * k + 6],
    }).collect();
    Ok(Mhc {
        n,
        d,
        sweeps: pr[7] as usize,
        tolerance: pr[8],
        mixes: [pr[9] as usize, pr[10] as usize, pr[11] as usize],
        signal: [pr[12], pr[13], pr[14]],
        x: v("x").to_vec(),
        one: v("one").to_vec(),
        copies: rows("copies", d),
        streams: rows("X", d),
        mixed: rows("mixedX", d),
        counts: [v("counts")[0], v("counts")[1], v("counts")[2], v("counts")[3]],
        path,
        a: m3(v("A")),
        row_sums: [v("rowSums")[0], v("rowSums")[1], v("rowSums")[2]],
        by_rows: m3(v("byRows")),
        col_sums: [v("colSums")[0], v("colSums")[1], v("colSums")[2]],
        first_sweep: m3(v("firstSweep")),
        row_sums_after: [v("rowSumsAfter")[0], v("rowSumsAfter")[1], v("rowSumsAfter")[2]],
        errors: v("errors").chunks(2).map(|c| [c[0], c[1]]).collect(),
        sweeps_to_tolerance: v("sweepsToTolerance")[0],
        s: m3(v("S")),
        kinds,
        rates: std::array::from_fn(|i| v("rates")[i]),
        zeros: [v("zeros")[0], v("zeros")[1]],
        two_by_two: v("twoByTwo").chunks(4).map(|c| [c[0], c[1], c[2], c[3]]).collect(),
        perm_checks: v("permChecks").to_vec(),
        perms: v("perms").chunks(9).map(m3).collect(),
        corner: [v("corner")[0], v("corner")[1], v("corner")[2]],
        middle: [v("middle")[0], v("middle")[1], v("middle")[2]],
    })
}
