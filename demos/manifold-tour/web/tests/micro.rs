//! The page's model natively: the page runs the command line's core,
//! what it reads back is what the command line prints, and the
//! numbers say what the page says they say (near pairs nearly agree,
//! far pairs do not; every drawn point is on its space).

use manifold_tour_web::micro::{core, program, run, SOURCE, SPACES};

fn len3(p: [f64; 3]) -> f64 {
    (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()
}

#[test]
fn the_page_runs_the_command_lines_core() {
    assert!(SOURCE.contains(core()));
    assert!(core().starts_with("\"mf:\" u_se< \"Manifold\""));
    assert!(!core().contains("ch:t_est"), "the core prints and checks nothing");
    assert!(program().starts_with(core()));
    assert!(program().contains("r_avel sphereFarArc\n"));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-manifold-tour.out");
    let t = run().unwrap();
    // Each pair of distances is one printed line of the command-line program.
    for pair in [t.circle_near_d, t.circle_far_d, t.sphere_near_d, t.sphere_far_d, t.roll_near_d, t.roll_far_d, t.disk_far_d, t.disk_near_d] {
        let line = format!("{:?} {:?}", pair.straight, pair.along);
        assert!(golden.lines().any(|l| l == line), "{line} not printed");
    }
    let rims = format!("{:?} {:?}", t.torus_rim_lengths[0], t.torus_rim_lengths[1]);
    assert!(golden.lines().any(|l| l == rims), "{rims}");
    assert!(golden.contains(&format!("{:?}", t.worked_sum)));
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn near_pairs_nearly_agree_and_far_pairs_do_not() {
    let t = run().unwrap();
    for near in [t.circle_near_d, t.sphere_near_d, t.roll_near_d] {
        assert!(near.ratio() > 1.0 && near.ratio() < 1.005, "{near:?}");
    }
    assert!(t.circle_far_d.ratio() > 1.3 && t.sphere_far_d.ratio() > 1.6, "{:?} {:?}", t.circle_far_d, t.sphere_far_d);
    assert!(t.roll_far_d.ratio() > 8.0, "{:?}", t.roll_far_d);
    assert!(t.disk_far_d.ratio() > 11.0 && (t.disk_near_d.ratio() - 2.0).abs() < 0.03, "{:?} {:?}", t.disk_far_d, t.disk_near_d);
    assert!((t.torus_rim_lengths[0] / t.torus_rim_lengths[1] - 2.0).abs() < 1e-9);
}

#[test]
fn every_drawn_point_is_on_its_space() {
    let t = run().unwrap();
    for q in &t.circle {
        assert!(((q[0] * q[0] + q[1] * q[1]).sqrt() - 1.0).abs() < 1e-12);
    }
    for line in t.parallels.iter().chain(&t.meridians).chain([&t.sphere_near_arc, &t.sphere_far_arc]) {
        for p in line {
            assert!((len3(*p) - 1.0).abs() < 1e-12);
        }
    }
    let [big, small] = t.radii;
    for line in t.torus_rings.iter().chain(&t.torus_tubes) {
        for p in line {
            let rho = (p[0] * p[0] + p[1] * p[1]).sqrt();
            assert!((((rho - big).powi(2) + p[2] * p[2]).sqrt() - small).abs() < 1e-12);
        }
    }
    for (line, map) in t.roll_along.iter().zip(&t.roll_along_map) {
        assert_eq!(line.len(), 61);
        // Along the sheet grows with t; the height is the map's.
        assert!(map.windows(2).all(|w| w[1][0] > w[0][0]));
        assert!(line.iter().zip(map).all(|(p, m)| (p[1] - m[1]).abs() < 1e-12));
    }
    for g in t.geos.iter().chain([&t.disk_far_geo, &t.disk_near_geo]) {
        assert_eq!(g.len(), 25);
        assert!(g.iter().all(|q| q[0] * q[0] + q[1] * q[1] < 1.0));
    }
    assert_eq!(t.disk_far_geo[0], t.disk_far[0]);
    assert!((t.disk_far_geo[24][0] - t.disk_far[1][0]).abs() < 1e-12);
}

#[test]
fn the_dimensions_and_the_counts() {
    let t = run().unwrap();
    assert_eq!(t.dims, [[1.0, 2.0], [2.0, 3.0], [2.0, 3.0], [2.0, 3.0], [2.0, 2.0]]);
    assert_eq!(SPACES.len(), 5);
    assert_eq!((t.parallels.len(), t.meridians.len()), (5, 12));
    assert_eq!((t.torus_rings.len(), t.torus_tubes.len()), (6, 12));
    assert_eq!((t.roll_along.len(), t.roll_across.len()), (5, 13));
    assert_eq!((t.rim.len(), t.ruler.len(), t.circle.len()), (91, 6, 73));
    // The maps are in degrees: a full turn of longitude, latitude within a quarter turn.
    assert!((t.parallels_ll[0][48][0] - 360.0).abs() < 1e-9);
    assert!(t.meridians_ll.iter().flatten().all(|q| q[1].abs() <= 90.0 + 1e-9));
    assert!((t.torus_rings_uv[3][0][1] - 180.0).abs() < 1e-9);
    assert!(t.ruler_d.iter().zip([0.0, 1.0, 2.0, 3.0, 4.0, 5.0]).all(|(a, b)| (a - b).abs() < 1e-9));
}

// New eyes (X_eTaL-fine-tuning's docs/new-eyes.md, cited): the reader
// knows neither geometry nor machine learning nor X_eTaL, so nothing
// the page shows goes unexplained, and nothing is called better.

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = manifold_tour_web::text::all();
    assert_eq!(lesson::eli5::missing(&texts), Vec::<String>::new());
    let used = lesson::eli5::used(&texts);
    for id in ["manifold", "intrinsic-dimension", "ambient-dimension", "locally-flat", "geodesic", "manifold-hypothesis", "data-manifold", "polytope", "metric", "curvature", "measured-analytical", "distortion"] {
        assert!(used.contains(&id.to_string()), "{id} not opened anywhere; used: {used:?}");
    }
}

#[test]
fn no_verdict_word_in_the_prose() {
    for t in manifold_tour_web::text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t:.60}");
    }
}

#[test]
fn every_library_function_the_program_calls_is_found_in_its_source() {
    let src = program();
    let calls = lesson::explain::calls(&src);
    assert!(calls.iter().any(|(l, n)| l == "Manifold" && n == "s_phereArc"), "{calls:?}");
    for (lib, name) in &calls {
        assert!(lesson::explain::definition(lib, name).is_some(), "{lib}:{name} has no definition the shell can show");
    }
    assert!(site::own("Manifold") && !site::own("Trig"));
}
