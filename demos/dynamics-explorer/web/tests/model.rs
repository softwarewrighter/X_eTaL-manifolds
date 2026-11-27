//! The page's model natively: the page runs the command line's core,
//! what it reads back is what the command line prints, and the
//! comparisons hold on the numbers; and the new-eyes checks.

use dynamics_explorer_web::model::{core, program, run, FIELDS, RULES, SOURCE};

#[test]
fn the_page_runs_the_command_lines_core() {
    assert!(SOURCE.contains(core()));
    assert!(!core().contains("ch:t_est"));
    assert!(program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-dynamics-explorer.out");
    let d = run().unwrap();
    // X_eTaL prints every float in full decimal; the numbers are compared as numbers, to a part in 10^12.
    let parsed: Vec<Vec<f64>> = golden.lines().map(|l| l.split_whitespace().filter_map(|t| t.parse().ok()).collect()).collect();
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1e-300);
    let printed = |v: &[f64]| parsed.iter().any(|l| l.len() == v.len() && l.iter().zip(v).all(|(a, b)| near(*a, *b)));
    for f in &d.fields {
        let m = f.measures;
        assert!(printed(&[m.div, m.curl, m.div_mean, m.curl_mean, m.div_gap, m.curl_gap]), "measures");
        let l = f.loop_;
        assert!(printed(&[l.sum, l.out, l.curl_area, l.div_area]), "a loop");
        let s = f.stability;
        assert!(printed(&[s.trace, s.det, s.re[0], s.re[1], s.im, s.largest, s.smallest]), "stability");
    }
    assert!(printed(&d.fields.iter().map(|f| f.path_error).collect::<Vec<_>>()), "path errors");
    assert!(printed(&d.same_work), "the same work");
    assert!(printed(&d.rules.iter().map(|r| r.factor).collect::<Vec<_>>()), "the game's factors");
    assert!(printed(&d.sides_rotation) && printed(&d.height_rotation), "the square");
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
    assert!(golden.lines().filter(|l| l.starts_with("ok: ")).count() == 16);
}

#[test]
fn the_comparisons_hold_on_the_numbers() {
    let d = run().unwrap();
    let pr = &d.preset;
    assert_eq!((d.fields.len(), FIELDS.len(), d.rules.len(), RULES.len()), (5, 5, 4, 4));
    assert_eq!(d.grid.len(), pr.side * pr.side);
    for (k, f) in d.fields.iter().enumerate() {
        assert_eq!(f.arrows.len(), d.grid.len());
        assert_eq!((f.rk.len(), f.exact.len()), (4, 4));
        for j in 0..4 {
            assert_eq!((f.rk[j].len(), f.exact[j].len()), (pr.path_steps + 1, pr.path_steps + 1));
            assert_eq!((f.rk[j][0], f.exact[j][0]), (pr.starts[j], pr.starts[j]));
        }
        // The paths against the exact ones, the measures against the matrix, the loop against Green's theorem.
        assert!(f.path_error < pr.tol, "{k}");
        assert!((f.measures.div - (f.a[0] + f.a[3])).abs() < 1e-12 && (f.measures.curl - (f.a[2] - f.a[1])).abs() < 1e-12);
        assert!(f.measures.div_gap < 1e-6 && f.measures.curl_gap < 1e-6);
        assert!((f.loop_.sum - f.loop_.curl_area).abs() < 1e-9 && (f.loop_.out - f.loop_.div_area).abs() < 1e-9);
        assert_eq!(f.symmetric, f.a[1] == f.a[2]);
        // A symmetric field: the loop sum is zero; a turning field: it is not.
        assert_eq!(f.loop_.sum.abs() < 1e-9, f.symmetric);
        // The arrow at a grid point is A times the point.
        let (q, v) = (d.grid[7], f.arrows[7]);
        assert!((v[0] - (f.a[0] * q[0] + f.a[1] * q[1])).abs() < 1e-12 && (v[1] - (f.a[2] * q[0] + f.a[3] * q[1])).abs() < 1e-12);
        // What the eigenvalues say agrees with what the paths did.
        let s = f.stability;
        if s.re[0] < 0.0 { assert!(s.largest < 1.0) }
        if s.re[0] > 0.0 { assert!(s.largest > 1.0) }
        if s.re[0] == 0.0 { assert!((s.largest - 1.0).abs() < 1e-3 && (s.smallest - 1.0).abs() < 1e-3) }
    }
    assert_eq!(d.sides_rotation, [2.0, 2.0, 2.0, 2.0]);
    assert_eq!(d.height_rotation, [-2.0, -4.0, -6.0, -8.0]);
    assert!(d.sides_attractor.iter().all(|x| x.abs() < 1e-12));
    assert!(d.gradient_gaps[0] < 1e-6 && d.gradient_gaps[1] < 1e-6);
    assert_eq!((d.circle.len(), d.square.len(), d.square_work.len()), (pr.loop_n + 1, 5, 4));
    // The rotation: Euler's distance from the circle grows at every size, as its factor says; Runge-Kutta's stays small.
    assert_eq!(d.rotations.len(), 4);
    for r in &d.rotations {
        assert_eq!((r.euler.len(), r.rk.len(), r.euler_off.len(), r.rk_off.len()), (pr.rot_steps + 1, pr.rot_steps + 1, pr.rot_steps + 1, pr.rot_steps + 1));
        assert_eq!((r.euler[0], r.rk[0]), (pr.r0, pr.r0));
        assert!(r.euler_off.windows(2).all(|w| w[1] > w[0]));
        assert!((r.euler_factor - (1.0 + r.size * r.size).sqrt()).abs() < 1e-12);
        assert!(r.rk_factor < 1.0 && r.euler_factor > 1.0);
        assert!((r.measured[0] - r.predicted[0]).abs() <= 1e-9 * (1.0 + r.predicted[0].abs()));
        assert_eq!((r.measured[0], r.measured[1]), (r.euler_off[pr.rot_steps], r.rk_off[pr.rot_steps]));
        assert!(r.rk_off[pr.rot_steps].abs() < r.euler_off[pr.rot_steps].abs());
    }
    let sw = d.same_work;
    assert!(sw[0] == sw[1] && sw[2] == sw[3] && sw[5].abs() < sw[4].abs());
    // The game: descent and the look-ahead come in, simultaneous steps go out, turns stay bounded and keep E.
    let n = pr.game_steps;
    for r in &d.rules {
        assert_eq!((r.path.len(), r.distance.len()), (n + 1, n + 1));
        assert_eq!(r.path[0], pr.game0);
    }
    assert!(d.rules[0].distance[n] < 1e-3 && d.rules[1].distance[n] > 2.0 && d.rules[3].distance[n] < 0.5);
    assert!(d.rules[2].distance.iter().all(|x| *x < 2.0));
    assert!(d.rules[1].distance.windows(2).all(|w| w[1] > w[0]));
    assert!(d.turns_energy.iter().all(|e| (e - d.turns_energy[0]).abs() < 1e-9));
    assert!(d.game_field_gap < 1e-6);
}

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = dynamics_explorer_web::text::all();
    assert_eq!(lesson::eli5::missing(&texts), Vec::<String>::new());
    let used = lesson::eli5::used(&texts);
    for id in ["vector-field", "gradient-field", "divergence-curl", "flow-integrator", "rest-point", "loop-sum", "two-player-game", "extragradient", "eigenvector"] {
        assert!(used.contains(&id.to_string()), "{id}");
    }
}

#[test]
fn no_verdict_word_in_the_prose() {
    for t in dynamics_explorer_web::text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t:.60}");
    }
}

#[test]
fn every_library_function_the_program_calls_is_found_in_its_source() {
    for (lib, name) in lesson::explain::calls(&program()) {
        assert!(lesson::explain::definition(&lib, &name).is_some(), "{lib}:{name}");
    }
}
