//! The model of part 2: the demo's second X_eTaL program
//! (mhc-depth.xtl) run once, its core followed by one `r_avel` line per
//! array the page shows, with the preset and the offline runs' records
//! (data/, written by train/train.sh from the same program) put where
//! the program's `[]N_GET` reads them. Nothing here computes a number:
//! the page lays X_eTaL's out and lets the reader pick a depth.

use microscope::run::{numbers, output_with, section};

pub const SOURCE: &str = include_str!("../../mhc-depth.xtl");
pub const PRESET: &str = include_str!("../../presets/depth.txt");
pub const PRESET_PATH: &str = "presets/depth.txt";
pub const RUNS: &str = include_str!("../../data/runs.txt");

/// The offline runs' records, a file per kind and depth, as the program reads them.
pub const RECORDS: [(&str, &str); 16] = [
    ("data/plain-4.txt", include_str!("../../data/plain-4.txt")),
    ("data/plain-8.txt", include_str!("../../data/plain-8.txt")),
    ("data/plain-16.txt", include_str!("../../data/plain-16.txt")),
    ("data/plain-32.txt", include_str!("../../data/plain-32.txt")),
    ("data/free-4.txt", include_str!("../../data/free-4.txt")),
    ("data/free-8.txt", include_str!("../../data/free-8.txt")),
    ("data/free-16.txt", include_str!("../../data/free-16.txt")),
    ("data/free-32.txt", include_str!("../../data/free-32.txt")),
    ("data/stochastic-4.txt", include_str!("../../data/stochastic-4.txt")),
    ("data/stochastic-8.txt", include_str!("../../data/stochastic-8.txt")),
    ("data/stochastic-16.txt", include_str!("../../data/stochastic-16.txt")),
    ("data/stochastic-32.txt", include_str!("../../data/stochastic-32.txt")),
    ("data/orthogonal-4.txt", include_str!("../../data/orthogonal-4.txt")),
    ("data/orthogonal-8.txt", include_str!("../../data/orthogonal-8.txt")),
    ("data/orthogonal-16.txt", include_str!("../../data/orthogonal-16.txt")),
    ("data/orthogonal-32.txt", include_str!("../../data/orthogonal-32.txt")),
];

pub fn core() -> &'static str {
    section(SOURCE, "\"hc:\" u_se< \"Mixing\"", "# -- end of the core")
}

/// What the page reads back, in order, with its count (from the preset:
/// 4 kinds, depths 4 8 16 32, 3 seeds, 200 steps, 20 on the page).
pub const PRINTED: [(&str, usize); 14] = [
    ("preset", 19),
    ("B", 1),
    ("p", 1),
    ("g", 1),
    ("gradientGaps", 4),
    ("start", 16 * 13),
    ("profiles", 4 * 3 * (5 + 9 + 17 + 33)),
    ("cost", 16),
    ("pageRuns", 4 * 50),
    ("pageGap", 1),
    ("after", 16 * 15),
    ("records", 48 * 410),
    ("h2", 16 * 12),
    ("held", 18),
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

pub const KINDS: [&str; 4] = ["plain residual", "free (hyper-connections)", "doubly stochastic (mHC)", "orthogonal (Cayley)"];

/// The preset, as the program read it.
#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    pub n: usize,
    pub d: usize,
    pub data_seed: usize,
    pub k: usize,
    pub per: usize,
    pub spread: f64,
    pub split_seed: usize,
    pub sweeps: usize,
    pub pull: f64,
    pub mix_spread: f64,
    pub weight_scale: f64,
    pub rate: f64,
    pub steps: usize,
    pub seeds: usize,
    pub depths: [usize; 4],
    pub page_steps: usize,
}

/// A number over the seeds: mean, smallest, largest.
pub type Three = [f64; 3];

/// One kind at one depth at the start of training, over the seeds.
#[derive(Clone, Debug, PartialEq)]
pub struct Start {
    /// The signal's size at the top over the bottom.
    pub up: Three,
    /// The gradient's size at the bottom over the top.
    pub down: Three,
    /// The streams' spread at the top over the bottom.
    pub spread: Three,
    /// The composite's gain (mean forward, backward).
    pub gain: [f64; 2],
    /// The largest distance of a mixing matrix from doubly stochastic, from orthogonal.
    pub off: [f64; 2],
    /// The first seed, state by state from the bottom: the signal's size, the gradient's size, the spread.
    pub sizes: Vec<f64>,
    pub gradients: Vec<f64>,
    pub spreads: Vec<f64>,
}

/// One kind at one depth after training, over the seeds (-1 where no seed stayed finite).
#[derive(Clone, Debug, PartialEq)]
pub struct After {
    pub trained: usize,
    pub finite: usize,
    pub loss: Three,
    pub held: Three,
    pub largest_gradient: f64,
    pub train_accuracy: f64,
    pub off: [f64; 2],
    pub gain: f64,
    pub size: f64,
    pub spread: f64,
}

/// One seed's record: the loss before each step and after the last; the gradient's size at each step; the end.
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub losses: Vec<f64>,
    pub gradients: Vec<f64>,
    pub train_accuracy: f64,
    pub held: f64,
    pub finite: bool,
}

/// One kind at one depth: the start, the end, the records (a seed each), the cost of a step.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub start: Start,
    pub after: After,
    pub records: Vec<Record>,
    pub cost: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Depth {
    pub preset: Preset,
    pub trained_on: usize,
    pub held_out: usize,
    /// Trained numbers in one layer's row, and outside the layers.
    pub per_layer: usize,
    pub outside: usize,
    pub gradient_gaps: [f64; 4],
    /// cells[kind][depth index].
    pub cells: Vec<Vec<Entry>>,
    /// The page's own run, a kind each: the losses before each step and after the last.
    pub page_losses: Vec<Vec<f64>>,
    pub page_gap: f64,
}

fn three(v: &[f64]) -> Three {
    [v[0], v[1], v[2]]
}

pub fn run() -> Result<Depth, String> {
    let mut files = vec![(PRESET_PATH.to_string(), PRESET.to_string())];
    files.extend(RECORDS.iter().map(|(p, t)| (p.to_string(), t.to_string())));
    let (out, _) = output_with(&files, &program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let pr = v("preset");
    let preset = Preset {
        n: pr[0] as usize,
        d: pr[1] as usize,
        data_seed: pr[2] as usize,
        k: pr[3] as usize,
        per: pr[4] as usize,
        spread: pr[5],
        split_seed: pr[6] as usize,
        sweeps: pr[7] as usize,
        pull: pr[8],
        mix_spread: pr[9],
        weight_scale: pr[10],
        rate: pr[11],
        steps: pr[12] as usize,
        seeds: pr[13] as usize,
        depths: [pr[14] as usize, pr[15] as usize, pr[16] as usize, pr[17] as usize],
        page_steps: pr[18] as usize,
    };
    let (steps, seeds) = (preset.steps, preset.seeds);
    let rec_len = 2 * steps + 10;
    let (st, af, recs, cost, prof) = (v("start"), v("after"), v("records"), v("cost"), v("profiles"));
    let mut at = 0;
    let mut cells = Vec::new();
    for kind in 0..4 {
        let mut row = Vec::new();
        for (di, &l) in preset.depths.iter().enumerate() {
            let j = kind * 4 + di;
            let s = &st[13 * j..13 * j + 13];
            let e = l + 1;
            let pf = &prof[at..at + 3 * e];
            at += 3 * e;
            let a = &af[15 * j..15 * j + 15];
            let records = (0..seeds).map(|sd| {
                let r = &recs[rec_len * (j * seeds + sd)..rec_len * (j * seeds + sd + 1)];
                Record { losses: r[..steps + 1].to_vec(), gradients: r[steps + 1..2 * steps + 1].to_vec(), train_accuracy: r[2 * steps + 1], held: r[2 * steps + 2], finite: r[2 * steps + 3] == 1.0 }
            }).collect();
            row.push(Entry {
                start: Start { up: three(&s[0..]), down: three(&s[3..]), spread: three(&s[6..]), gain: [s[9], s[10]], off: [s[11], s[12]], sizes: pf[..e].to_vec(), gradients: pf[e..2 * e].to_vec(), spreads: pf[2 * e..].to_vec() },
                after: After { trained: a[0] as usize, finite: a[1] as usize, loss: three(&a[2..]), held: three(&a[5..]), largest_gradient: a[8], train_accuracy: a[9], off: [a[10], a[11]], gain: a[12], size: a[13], spread: a[14] },
                records,
                cost: cost[j],
            });
        }
        cells.push(row);
    }
    let pw = 2 * preset.page_steps + 10;
    Ok(Depth {
        trained_on: v("B")[0] as usize,
        held_out: v("held").len(),
        per_layer: v("p")[0] as usize,
        outside: v("g")[0] as usize,
        gradient_gaps: std::array::from_fn(|i| v("gradientGaps")[i]),
        cells,
        page_losses: v("pageRuns").chunks(pw).map(|r| r[..preset.page_steps + 1].to_vec()).collect(),
        page_gap: v("pageGap")[0],
        preset,
    })
}
