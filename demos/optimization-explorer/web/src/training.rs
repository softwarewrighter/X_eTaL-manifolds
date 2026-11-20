//! The model of part 2: the demo's second X_eTaL program
//! (optimization-training.xtl) run once, its core followed by one
//! `r_avel` line per array the page shows, with the preset put where the
//! program's `[]N_GET` reads it. Nothing here computes a number: the
//! page lays X_eTaL's out.

use microscope::run::{numbers, output_with, section};

use crate::micro::P2;

pub const SOURCE: &str = include_str!("../../optimization-training.xtl");
pub const PRESET: &str = include_str!("../../presets/training.txt");
pub const PRESET_PATH: &str = "presets/training.txt";

pub fn core() -> &'static str {
    section(SOURCE, "\"kt:\" u_se< \"Kit\"", "# -- end of the core")
}

/// Every array the page reads back, with its count (from the preset:
/// 120 points, 90 trained on, 60 steps, four plain rates swept).
pub const PRINTED: [(&str, usize); 19] = [
    ("preset", 18),
    ("X", 240),
    ("y", 120),
    ("held", 30),
    ("plainBadLog", 183),
    ("diagBadLog", 183),
    ("naturalBadLog", 183),
    ("plainLog", 183),
    ("diagLog", 183),
    ("naturalLog", 183),
    ("record", 48),
    ("plainSweep", 20),
    ("fisherEigen", 14),
    ("changes", 8),
    ("invariance", 6),
    ("gaps", 3),
    ("nt", 1),
    ("conditionRatio", 1),
    ("spent", 6),
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

/// The preset, as the program read it.
#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    pub data_seed: usize,
    pub k: usize,
    pub per: usize,
    pub spread: f64,
    pub split_seed: usize,
    pub steps: usize,
    pub target: f64,
    /// The rate of each method: plain, diagonal, natural.
    pub rates: [f64; 3],
    pub damping: f64,
    pub bad_scale: f64,
    pub start_seed: usize,
    pub start_spread: f64,
    pub sweep: Vec<f64>,
}

/// One step's row of a run: the training loss after it, the held-out accuracy, the update's flat length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogRow {
    pub loss: f64,
    pub accuracy: f64,
    pub update: f64,
}

/// A run's record: the program's row.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Record {
    pub end_loss: f64,
    pub end_accuracy: f64,
    pub first_under: f64,
    pub stays_under: f64,
    pub mean_update: f64,
    pub largest_update: f64,
    pub numbers: f64,
    pub multiplies: f64,
}

/// A run: its log (the start's row first) and its record.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub log: Vec<LogRow>,
    pub record: Record,
}

/// A row of the plain rate sweep on the badly scaled data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SweepRow {
    pub rate: f64,
    pub end_loss: f64,
    pub first_under: f64,
    pub stays_under: f64,
    pub largest_update: f64,
}

/// Everything part 2 measured.
#[derive(Clone, Debug, PartialEq)]
pub struct Training {
    pub preset: Preset,
    pub x: Vec<P2>,
    pub y: Vec<usize>,
    pub held: Vec<usize>,
    /// Plain, diagonal, natural on the badly scaled data.
    pub bad: [Run; 3],
    /// Plain, diagonal, natural on the data as they are.
    pub good: [Run; 3],
    pub sweep: Vec<SweepRow>,
    /// The Fisher's eigenvalues at the start, largest first, and the condition number: badly scaled, as they are.
    pub eigen: [[f64; 7]; 2],
    /// Four changes of the weights, each (flat length, Fisher length): 0.1 on class 1's bias; 0.1 on its x1 weight; the plain first step; the natural first step.
    pub changes: [P2; 4],
    /// The largest gap in any probability after ten undamped steps against the data as they are, per method (plain, diagonal, natural): x1 rescaled, x1 mixed with x2.
    pub invariance: [[f64; 2]; 3],
    /// The factor by which the bad scaling raised the Fisher's condition number.
    pub condition_ratio: f64,
    /// The multiplies each run spent to stay under the target (-1 if it never did): badly scaled plain, diagonal, natural, then as they are.
    pub spent: [f64; 6],
    /// The gradient against central differences; the Fisher against kron(M, x x'); the Fisher against the gradient's central differences.
    pub gaps: [f64; 3],
    pub trained_on: usize,
}

pub const METHODS: [&str; 3] = ["plain gradient descent", "the diagonal preconditioner", "the natural-gradient step"];

pub fn run() -> Result<Training, String> {
    let (out, _) = output_with(&[(PRESET_PATH.to_string(), PRESET.to_string())], &program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let pr = v("preset");
    let preset = Preset {
        data_seed: pr[0] as usize,
        k: pr[1] as usize,
        per: pr[2] as usize,
        spread: pr[3],
        split_seed: pr[4] as usize,
        steps: pr[5] as usize,
        target: pr[6],
        rates: [pr[7], pr[8], pr[9]],
        damping: pr[10],
        bad_scale: pr[11],
        start_seed: pr[12] as usize,
        start_spread: pr[13],
        sweep: pr[14..].to_vec(),
    };
    let rec = v("record");
    let record = |i: usize| { let r = &rec[8 * i..8 * i + 8]; Record { end_loss: r[0], end_accuracy: r[1], first_under: r[2], stays_under: r[3], mean_update: r[4], largest_update: r[5], numbers: r[6], multiplies: r[7] } };
    let log = |name: &str| v(name).chunks(3).map(|r| LogRow { loss: r[0], accuracy: r[1], update: r[2] }).collect::<Vec<_>>();
    let runs = |names: [&str; 3], from: usize| -> [Run; 3] { std::array::from_fn(|i| Run { log: log(names[i]), record: record(from + i) }) };
    let e = v("fisherEigen");
    let ch = v("changes");
    Ok(Training {
        x: v("X").chunks(2).map(|r| [r[0], r[1]]).collect(),
        y: v("y").iter().map(|c| *c as usize).collect(),
        held: v("held").iter().map(|c| *c as usize).collect(),
        bad: runs(["plainBadLog", "diagBadLog", "naturalBadLog"], 0),
        good: runs(["plainLog", "diagLog", "naturalLog"], 3),
        sweep: v("plainSweep").chunks(5).map(|r| SweepRow { rate: r[0], end_loss: r[1], first_under: r[2], stays_under: r[3], largest_update: r[4] }).collect(),
        eigen: [std::array::from_fn(|i| e[i]), std::array::from_fn(|i| e[7 + i])],
        changes: std::array::from_fn(|i| [ch[2 * i], ch[2 * i + 1]]),
        invariance: std::array::from_fn(|i| [v("invariance")[2 * i], v("invariance")[2 * i + 1]]),
        condition_ratio: v("conditionRatio")[0],
        spent: std::array::from_fn(|i| v("spent")[i]),
        gaps: std::array::from_fn(|i| v("gaps")[i]),
        trained_on: v("nt")[0] as usize,
        preset,
    })
}
