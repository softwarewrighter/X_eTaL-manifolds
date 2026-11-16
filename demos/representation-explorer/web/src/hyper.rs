//! The model of part 2: the demo's second X_eTaL program
//! (representation-hyperbolic.xtl) run once, its core followed by one
//! `r_avel` line per array the page shows, with the preset put where the
//! program's `[]N_GET` reads it. Nothing here computes a number: the page
//! lays X_eTaL's out.

use microscope::run::{numbers, output_with, section};

use crate::micro::{P2, P3};

pub const SOURCE: &str = include_str!("../../representation-hyperbolic.xtl");
pub const PRESET: &str = include_str!("../../presets/tree.txt");
pub const PRESET_PATH: &str = "presets/tree.txt";

pub fn core() -> &'static str {
    section(SOURCE, "\"kt:\" u_se< \"Kit\"", "# -- end of the core")
}

/// Every array the page reads back, with its count.
pub const PRINTED: [(&str, usize); 44] = [
    ("preset", 20),
    ("edges", 60),
    ("ty", 31),
    ("layout", 62),
    ("radii", 5),
    ("fromMiddle", 5),
    ("wu", 2),
    ("wv", 2),
    ("workedM2", 1),
    ("workedA", 1),
    ("workedB", 1),
    ("workedZ", 1),
    ("workedDisk", 1),
    ("workedAcosh", 1),
    ("workedPlain", 1),
    ("geodesic", 26),
    ("straight", 26),
    ("pathLengths", 2),
    ("room", 25),
    ("sphereP", 93),
    ("flatP", 93),
    ("ballP", 93),
    ("sphereLog", 804),
    ("flatLog", 804),
    ("ballLog", 804),
    ("edgeDistortion", 90),
    ("worstPairs", 6),
    ("spreadRows", 12),
    ("treePath", 26),
    ("neighbors", 3),
    ("held", 7),
    ("knnGuess", 21),
    ("logisticGuess", 21),
    ("accuracy", 6),
    ("record", 27),
    ("baseline", 3),
    ("curvature", 40),
    ("curvePoints", 744),
    ("parallels", 375),
    ("meridians", 900),
    ("sphereMap", 62),
    ("train", 24),
    ("T", 961),
    ("otherMeans", 1206),
];

pub fn tail() -> String {
    let mut s = String::from("# -- what the page reads back, one array per line\n");
    for (name, _) in PRINTED.iter() {
        s += &format!("r_avel {name}\n");
    }
    s
}

pub fn program() -> String {
    format!("{}{}", core(), tail())
}

/// The preset's numbers, named as the program names them.
#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    pub depth: usize,
    pub branching: usize,
    pub seeds: [usize; 3],
    pub spread: f64,
    pub steps: usize,
    pub control_steps: usize,
    pub rate: f64,
    pub target: f64,
    pub k: usize,
    pub split_seed: usize,
    pub curvatures: Vec<f64>,
}

/// One row of a fit's log: the loss, the mean distortion, the worst
/// pair, how far past the rim the step went.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogRow {
    pub loss: f64,
    pub mean: f64,
    pub worst: f64,
    pub off: f64,
}

/// One fitted embedding of the tree: its curvature, the points (31 rows
/// of 3, the third 0 but on the sphere), its log by step, the
/// distortion of each edge, the worst pair, and its record.
#[derive(Clone, Debug, PartialEq)]
pub struct Fit {
    pub curvature: f64,
    pub points: Vec<P3>,
    pub log: Vec<LogRow>,
    pub edge_distortion: Vec<f64>,
    pub worst_pair: (usize, usize),
    pub record: FitRecord,
    pub knn_guess: Vec<usize>,
    pub logistic_guess: Vec<usize>,
}

/// One space's record: the end distortion, the worst pair, the steps to
/// the target (-1: never), the distortion with the scale forgiven, the
/// numbers trained, the operations, the constraint's violation, and the
/// two held-out accuracies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FitRecord {
    pub mean: f64,
    pub worst: f64,
    pub steps_to_target: f64,
    pub forgiven: f64,
    pub trained: f64,
    pub operations: f64,
    pub off: f64,
    pub knn: f64,
    pub logistic: f64,
}

/// One row of the curvature control: K, the mean distortion at the
/// control's last step, the worst pair, the steps to the target, how far
/// past the rim; and the embedding at that curvature at that step.
#[derive(Clone, Debug, PartialEq)]
pub struct Curve {
    pub k: f64,
    pub mean: f64,
    pub worst: f64,
    pub steps_to_target: f64,
    pub off: f64,
    pub points: Vec<P3>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hyper {
    pub preset: Preset,
    /// The tree: its edges (parent, child; from 1), each node's class, the kit's plain layout, its distances.
    pub edges: Vec<(usize, usize)>,
    pub classes: Vec<usize>,
    pub layout: Vec<P2>,
    pub tree_distances: Vec<f64>,
    /// The disk: the distance from the middle at a few plain radii.
    pub radii: Vec<f64>,
    pub from_middle: Vec<f64>,
    /// The worked pair: u, v, |u - v|^2, 1 - |u|^2, 1 - |v|^2, z, the distance (asinh and acosh forms), the plain distance.
    pub u: P2,
    pub v: P2,
    pub worked_m2: f64,
    pub worked_a: f64,
    pub worked_b: f64,
    pub worked_z: f64,
    pub worked_disk: f64,
    pub worked_acosh: f64,
    pub worked_plain: f64,
    /// The geodesic and the straight segment between u and v, and their lengths along the disk.
    pub geodesic: Vec<P2>,
    pub straight: Vec<P2>,
    pub path_lengths: [f64; 2],
    /// The room by level: nodes, the plane's circle, the disk's circle, the spacing in the plane, in the disk.
    pub room: Vec<[f64; 5]>,
    /// The three fits of the first start: the sphere (+1), the plane (0), the disk (-1).
    pub fits: Vec<Fit>,
    /// Per space: the end distortion of each of the three starts, and the spread.
    pub spread_rows: Vec<[f64; 4]>,
    /// The other two starts' mean distortion by step: sphere, sphere, plane, plane, disk, disk.
    pub other_means: Vec<Vec<f64>>,
    /// The geodesic from the root to the last leaf in the disk; the three nodes nearest node 2 along the disk.
    pub tree_path: Vec<P2>,
    pub neighbors: Vec<usize>,
    /// The held-out nodes (from 1) and the training nodes.
    pub held: Vec<usize>,
    pub train: Vec<usize>,
    /// The baseline's row: the plain layout's distortion (mean, worst) and nearest neighbors on it.
    pub baseline: [f64; 3],
    /// The curvature control, one per curvature of the preset.
    pub curves: Vec<Curve>,
    pub parallels: Vec<Vec<P3>>,
    pub meridians: Vec<Vec<P3>>,
    /// The sphere embedding on the sphere's map: longitude and latitude, degrees.
    pub sphere_map: Vec<P2>,
}

pub const SPACES: [&str; 3] = ["the sphere (curvature +1)", "the plane (curvature 0)", "the disk (curvature -1)"];
pub const SPACES_SHORT: [&str; 3] = ["sphere", "plane", "disk"];

impl Hyper {
    pub fn n(&self) -> usize {
        self.classes.len()
    }
    /// Whether the held-out node (from 1) was guessed right by a guess list.
    pub fn right(&self, guesses: &[usize], node: usize) -> Option<bool> {
        self.held.iter().position(|&h| h == node).map(|k| guesses[k] == self.classes[node - 1])
    }
    pub fn wrong(&self, guesses: &[usize]) -> usize {
        self.held.iter().zip(guesses).filter(|(h, g)| self.classes[**h - 1] != **g).count()
    }
    /// The tree distance between two nodes (from 1).
    pub fn tree_distance(&self, a: usize, b: usize) -> f64 {
        self.tree_distances[(a - 1) * self.n() + (b - 1)]
    }
}

fn rows<const D: usize>(v: &[f64]) -> Vec<[f64; D]> {
    v.chunks(D).map(|c| std::array::from_fn(|i| c[i])).collect()
}

fn lines(v: &[f64], n: usize) -> Vec<Vec<P3>> {
    rows::<3>(v).chunks(n).map(|c| c.to_vec()).collect()
}

pub fn run() -> Result<Hyper, String> {
    let (out, _) = output_with(&[(PRESET_PATH.to_string(), PRESET.to_string())], &program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    let ints = |name: &str| v(name).iter().map(|x| *x as usize).collect::<Vec<_>>();
    let one = |name: &str| v(name)[0];
    let p2 = |name: &str| -> P2 { let w = v(name); [w[0], w[1]] };
    let pr = v("preset");
    let preset = Preset {
        depth: pr[0] as usize,
        branching: pr[1] as usize,
        seeds: [pr[2] as usize, pr[3] as usize, pr[4] as usize],
        spread: pr[5],
        steps: pr[6] as usize,
        control_steps: pr[7] as usize,
        rate: pr[8],
        target: pr[9],
        k: pr[10] as usize,
        split_seed: pr[11] as usize,
        curvatures: pr[12..].to_vec(),
    };
    let n = v("ty").len();
    let edges_v = ints("edges");
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|e| (edges_v[e], edges_v[n - 1 + e])).collect();
    let logs = |name: &str| v(name).chunks(4).map(|r| LogRow { loss: r[0], mean: r[1], worst: r[2], off: r[3] }).collect::<Vec<_>>();
    let ed = v("edgeDistortion");
    let wp = ints("worstPairs");
    let rec = v("record");
    let kg = ints("knnGuess");
    let lg = ints("logisticGuess");
    let held_n = v("held").len();
    let fits = [("sphereP", "sphereLog", 1.0), ("flatP", "flatLog", 0.0), ("ballP", "ballLog", -1.0)]
        .iter()
        .enumerate()
        .map(|(s, (p, l, k))| Fit {
            curvature: *k,
            points: rows(v(p)),
            log: logs(l),
            edge_distortion: ed[s * (n - 1)..(s + 1) * (n - 1)].to_vec(),
            worst_pair: (wp[2 * s], wp[2 * s + 1]),
            record: FitRecord {
                mean: rec[9 * s],
                worst: rec[9 * s + 1],
                steps_to_target: rec[9 * s + 2],
                forgiven: rec[9 * s + 3],
                trained: rec[9 * s + 4],
                operations: rec[9 * s + 5],
                off: rec[9 * s + 6],
                knn: rec[9 * s + 7],
                logistic: rec[9 * s + 8],
            },
            knn_guess: kg[s * held_n..(s + 1) * held_n].to_vec(),
            logistic_guess: lg[s * held_n..(s + 1) * held_n].to_vec(),
        })
        .collect();
    let cv = v("curvature");
    let cp = rows::<3>(v("curvePoints"));
    let curves = (0..preset.curvatures.len())
        .map(|i| Curve { k: cv[5 * i], mean: cv[5 * i + 1], worst: cv[5 * i + 2], steps_to_target: cv[5 * i + 3], off: cv[5 * i + 4], points: cp[i * n..(i + 1) * n].to_vec() })
        .collect();
    let b = v("baseline");
    let steps_plus = preset.steps + 1;
    Ok(Hyper {
        preset,
        edges,
        classes: ints("ty"),
        layout: rows(v("layout")),
        tree_distances: v("T").to_vec(),
        radii: v("radii").to_vec(),
        from_middle: v("fromMiddle").to_vec(),
        u: p2("wu"),
        v: p2("wv"),
        worked_m2: one("workedM2"),
        worked_a: one("workedA"),
        worked_b: one("workedB"),
        worked_z: one("workedZ"),
        worked_disk: one("workedDisk"),
        worked_acosh: one("workedAcosh"),
        worked_plain: one("workedPlain"),
        geodesic: rows(v("geodesic")),
        straight: rows(v("straight")),
        path_lengths: [v("pathLengths")[0], v("pathLengths")[1]],
        room: rows(v("room")),
        fits,
        spread_rows: rows(v("spreadRows")),
        other_means: v("otherMeans").chunks(steps_plus).map(|c| c.to_vec()).collect(),
        tree_path: rows(v("treePath")),
        neighbors: ints("neighbors"),
        held: ints("held"),
        train: ints("train"),
        baseline: [b[0], b[1], b[2]],
        curves,
        parallels: lines(v("parallels"), 25),
        meridians: lines(v("meridians"), 25),
        sphere_map: rows(v("sphereMap")),
    })
}
