//! The model: the demo's own X_eTaL program (manifold-tour.xtl) run
//! once, its core followed by one `r_avel` line per array the page
//! shows; what comes back is every coordinate, wire line and distance
//! on the page. Nothing here knows about the browser, and nothing
//! here computes a number: the page lays X_eTaL's out.

use microscope::run::{numbers, output, section};

/// The command-line program.
pub const SOURCE: &str = include_str!("../../manifold-tour.xtl");

/// The program's core: from its imports to the end-of-core marker.
pub fn core() -> &'static str {
    section(SOURCE, "\"mf:\" u_se< \"Manifold\"", "# -- end of the core")
}

/// What the page reads back, in order: (name, items). The counts are
/// fixed by the program (72 segments round a circle, five parallels
/// of 49 points, ...); `run` checks each.
pub const PRINTED: [(&str, usize); 53] = [
    ("circleAngles", 73),
    ("circle", 73 * 2),
    ("circleNearA", 2),
    ("circleNear", 4),
    ("circleNearD", 2),
    ("circleFarA", 2),
    ("circleFar", 4),
    ("circleFarD", 2),
    ("parallelLL", 245 * 2),
    ("parallels", 245 * 3),
    ("meridianLL", 300 * 2),
    ("meridians", 300 * 3),
    ("sphereDeg", 6),
    ("spherePts", 9),
    ("sphereNearD", 2),
    ("sphereFarD", 2),
    ("sphereNearArc", 13 * 3),
    ("sphereNearArcLL", 13 * 2),
    ("sphereFarArc", 49 * 3),
    ("sphereFarArcLL", 49 * 2),
    ("torusRingsUV", 294 * 2),
    ("torusRings", 294 * 3),
    ("torusTubesUV", 300 * 2),
    ("torusTubes", 300 * 3),
    ("torusRimLengths", 2),
    ("rollAlongMap", 305 * 2),
    ("rollAlong", 305 * 3),
    ("rollAcrossMap", 26 * 2),
    ("rollAcross", 26 * 3),
    ("rollTH", 6),
    ("rollPts", 9),
    ("rollMap", 6),
    ("rollNearD", 2),
    ("rollFarD", 2),
    ("rim", 91 * 2),
    ("ruler", 12),
    ("rulerD", 6),
    ("diskFar", 4),
    ("diskFarD", 2),
    ("diskNear", 4),
    ("diskNearD", 2),
    ("diskFarGeo", 25 * 2),
    ("diskNearGeo", 25 * 2),
    ("geo1", 25 * 2),
    ("geo2", 25 * 2),
    ("geo3", 25 * 2),
    ("radii", 2),
    ("rollT", 61),
    ("dims", 10),
    ("workedDiff", 3),
    ("workedSq", 3),
    ("workedSum", 1),
    ("workedPlus", 1),
];

/// The lines after the core: each array the page shows, raveled.
pub fn tail() -> String {
    let mut s = String::from("# -- what the page reads back, one array per line (the maps in degrees)\n");
    for (name, _) in PRINTED {
        match name {
            "parallelLL" | "meridianLL" | "torusRingsUV" | "torusTubesUV" => s += &format!("r_avel {name} / deg\n"),
            _ => s += &format!("r_avel {name}\n"),
        }
    }
    s
}

/// Everything the page runs.
pub fn program() -> String {
    format!("{}{}", core(), tail())
}

/// A point in the plane (a map's coordinates, or the disk's).
pub type P2 = [f64; 2];
/// A point in three dimensions.
pub type P3 = [f64; 3];

/// A straight-line distance through the surrounding space and the
/// distance along the manifold, between the same two points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pair {
    pub straight: f64,
    pub along: f64,
}

impl Pair {
    fn of(v: &[f64]) -> Pair {
        Pair { straight: v[0], along: v[1] }
    }
    /// How much longer the way along the manifold is, as a ratio.
    pub fn ratio(&self) -> f64 {
        self.along / self.straight
    }
}

/// The tour, as X_eTaL computed it.
#[derive(Clone, Debug, PartialEq)]
pub struct Tour {
    // The circle.
    pub circle_angles: Vec<f64>,
    pub circle: Vec<P2>,
    pub circle_near_a: P2,
    pub circle_near: [P2; 2],
    pub circle_near_d: Pair,
    pub circle_far_a: P2,
    pub circle_far: [P2; 2],
    pub circle_far_d: Pair,
    // The sphere: wire lines in three dimensions and on the map (longitude, latitude in degrees).
    pub parallels: Vec<Vec<P3>>,
    pub parallels_ll: Vec<Vec<P2>>,
    pub meridians: Vec<Vec<P3>>,
    pub meridians_ll: Vec<Vec<P2>>,
    /// p, q, r by longitude and latitude in degrees, and in three dimensions.
    pub sphere_deg: [P2; 3],
    pub sphere_pts: [P3; 3],
    pub sphere_near_d: Pair,
    pub sphere_far_d: Pair,
    pub sphere_near_arc: Vec<P3>,
    pub sphere_near_arc_ll: Vec<P2>,
    pub sphere_far_arc: Vec<P3>,
    pub sphere_far_arc_ll: Vec<P2>,
    // The torus.
    pub radii: P2,
    pub torus_rings: Vec<Vec<P3>>,
    /// The rings and tubes on the map, (u, v) in degrees.
    pub torus_rings_uv: Vec<Vec<P2>>,
    pub torus_tubes: Vec<Vec<P3>>,
    pub torus_tubes_uv: Vec<Vec<P2>>,
    /// The outer ring's length and the inner ring's, along the wire.
    pub torus_rim_lengths: P2,
    // The Swiss roll.
    pub roll_t: Vec<f64>,
    pub roll_along: Vec<Vec<P3>>,
    pub roll_along_map: Vec<Vec<P2>>,
    pub roll_across: Vec<Vec<P3>>,
    pub roll_across_map: Vec<Vec<P2>>,
    /// a, b, c as (t, h), in three dimensions, and on the unrolled sheet.
    pub roll_th: [P2; 3],
    pub roll_pts: [P3; 3],
    pub roll_map: [P2; 3],
    pub roll_near_d: Pair,
    pub roll_far_d: Pair,
    // The Poincare disk.
    pub rim: Vec<P2>,
    pub ruler: Vec<P2>,
    pub ruler_d: Vec<f64>,
    pub disk_far: [P2; 2],
    pub disk_far_d: Pair,
    pub disk_near: [P2; 2],
    pub disk_near_d: Pair,
    pub disk_far_geo: Vec<P2>,
    pub disk_near_geo: Vec<P2>,
    pub geos: Vec<Vec<P2>>,
    /// Each space's (intrinsic, ambient) dimension, read off the arrays: the circle, the sphere, the torus, the roll, the disk.
    pub dims: [P2; 5],
    /// The chord from p to q worked in full: the differences, their squares, the sum, and |p + q|.
    pub worked_diff: P3,
    pub worked_sq: P3,
    pub worked_sum: f64,
    pub worked_plus: f64,
}

/// A raveled matrix back as rows of `d`.
fn rows<const D: usize>(v: &[f64]) -> Vec<[f64; D]> {
    v.chunks(D).map(|c| std::array::from_fn(|i| c[i])).collect()
}

/// Rows cut into wire lines of `per` points each.
fn wires<const D: usize>(v: &[f64], per: usize) -> Vec<Vec<[f64; D]>> {
    rows::<D>(v).chunks(per).map(|c| c.to_vec()).collect()
}

fn fixed<const D: usize, const N: usize>(v: &[f64]) -> [[f64; D]; N] {
    let r = rows::<D>(v);
    std::array::from_fn(|i| r[i])
}

/// Run the program and read the tour back.
pub fn run() -> Result<Tour, String> {
    let out = output(&program(), PRINTED.len())?;
    let mut got = std::collections::HashMap::new();
    for (i, (name, n)) in PRINTED.iter().enumerate() {
        got.insert(*name, numbers::<f64>(&out[i], *n).map_err(|e| format!("{name}: {e}"))?);
    }
    let v = |name: &str| got[name].as_slice();
    Ok(Tour {
        circle_angles: v("circleAngles").to_vec(),
        circle: rows(v("circle")),
        circle_near_a: [v("circleNearA")[0], v("circleNearA")[1]],
        circle_near: fixed(v("circleNear")),
        circle_near_d: Pair::of(v("circleNearD")),
        circle_far_a: [v("circleFarA")[0], v("circleFarA")[1]],
        circle_far: fixed(v("circleFar")),
        circle_far_d: Pair::of(v("circleFarD")),
        parallels: wires(v("parallels"), 49),
        parallels_ll: wires(v("parallelLL"), 49),
        meridians: wires(v("meridians"), 25),
        meridians_ll: wires(v("meridianLL"), 25),
        sphere_deg: fixed(v("sphereDeg")),
        sphere_pts: fixed(v("spherePts")),
        sphere_near_d: Pair::of(v("sphereNearD")),
        sphere_far_d: Pair::of(v("sphereFarD")),
        sphere_near_arc: rows(v("sphereNearArc")),
        sphere_near_arc_ll: rows(v("sphereNearArcLL")),
        sphere_far_arc: rows(v("sphereFarArc")),
        sphere_far_arc_ll: rows(v("sphereFarArcLL")),
        radii: [v("radii")[0], v("radii")[1]],
        torus_rings: wires(v("torusRings"), 49),
        torus_rings_uv: wires(v("torusRingsUV"), 49),
        torus_tubes: wires(v("torusTubes"), 25),
        torus_tubes_uv: wires(v("torusTubesUV"), 25),
        torus_rim_lengths: [v("torusRimLengths")[0], v("torusRimLengths")[1]],
        roll_t: v("rollT").to_vec(),
        roll_along: wires(v("rollAlong"), 61),
        roll_along_map: wires(v("rollAlongMap"), 61),
        roll_across: wires(v("rollAcross"), 2),
        roll_across_map: wires(v("rollAcrossMap"), 2),
        roll_th: fixed(v("rollTH")),
        roll_pts: fixed(v("rollPts")),
        roll_map: fixed(v("rollMap")),
        roll_near_d: Pair::of(v("rollNearD")),
        roll_far_d: Pair::of(v("rollFarD")),
        rim: rows(v("rim")),
        ruler: rows(v("ruler")),
        ruler_d: v("rulerD").to_vec(),
        disk_far: fixed(v("diskFar")),
        disk_far_d: Pair::of(v("diskFarD")),
        disk_near: fixed(v("diskNear")),
        disk_near_d: Pair::of(v("diskNearD")),
        disk_far_geo: rows(v("diskFarGeo")),
        disk_near_geo: rows(v("diskNearGeo")),
        geos: ["geo1", "geo2", "geo3"].iter().map(|g| rows(v(g))).collect(),
        dims: fixed(v("dims")),
        worked_diff: [v("workedDiff")[0], v("workedDiff")[1], v("workedDiff")[2]],
        worked_sq: [v("workedSq")[0], v("workedSq")[1], v("workedSq")[2]],
        worked_sum: v("workedSum")[0],
        worked_plus: v("workedPlus")[0],
    })
}

/// The spaces, in the order of `dims`.
pub const SPACES: [&str; 5] = ["circle", "sphere", "torus", "Swiss roll", "Poincare disk"];
