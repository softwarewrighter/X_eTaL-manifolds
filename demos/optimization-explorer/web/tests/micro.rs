//! The page's model natively: the page runs the command line's core,
//! what it reads back is what the command line prints, and the
//! comparisons hold on the numbers; and the new-eyes checks.

use optimization_explorer_web::micro::{core, program, run, SOURCE, WAYS};

#[test]
fn the_page_runs_the_command_lines_core() {
    assert!(SOURCE.contains(core()));
    assert!(!core().contains("ch:t_est"));
    assert!(program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-optimization-explorer.out");
    let o = run().unwrap();
    // X_eTaL prints every float in full decimal; the numbers are compared as numbers, to a part in 10^12.
    let parsed: Vec<Vec<f64>> = golden.lines().map(|l| l.split_whitespace().filter_map(|t| t.parse().ok()).collect()).collect();
    let printed = |v: &[f64]| parsed.iter().any(|l| l.len() == v.len() && l.iter().zip(v).all(|(a, b)| (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1e-300)));
    let f = &o.plain_first;
    assert!(printed(&[f.d[0], f.d[1], f.eta, f.to[0], f.to[1], f.dot, f.den]), "the plain first step");
    let r = o.plain_record;
    assert!(printed(&[r.steps_to_target, r.after_one, r.after_five, r.end, r.numbers, r.multiplies]), "the plain record");
    for c in &o.control {
        assert!(printed(&[c.a, c.kappa, c.steps_to_target, c.after_five, c.end]), "control {}", c.a);
    }
    assert!(printed(&o.newton_dir), "Newton's direction");
    for c in &o.climbs {
        assert!(printed(&c.record), "a climb's record");
    }
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_comparisons_hold_on_the_numbers() {
    let o = run().unwrap();
    let n = o.preset.steps + 1;
    assert_eq!((o.plain.path.len(), o.plain.loss.len(), o.newton.path.len(), o.newton.loss.len()), (n, n, n, n));
    assert_eq!((o.control.len(), o.control_paths.len(), o.control_losses.len()), (o.preset.anis.len(), o.preset.anis.len(), o.preset.anis.len()));
    assert_eq!((o.contours.len(), o.field.len(), o.check_at.len()), (o.preset.levels.len(), 35, 3));
    // The paths start at the start; the records' end losses are the logs' last.
    assert_eq!((o.plain.path[0], o.newton.path[0]), (o.preset.start, o.preset.start));
    assert_eq!((o.plain_record.end, o.newton_record.end), (o.plain.loss[n - 1], o.newton.loss[n - 1]));
    for (k, c) in o.control.iter().enumerate() {
        assert_eq!(c.end, o.control_losses[k][n - 1]);
        assert_eq!(o.control_paths[k][0], o.preset.start);
        let first = o.control_losses[k].iter().position(|&l| l < o.preset.target).map(|k| k as f64).unwrap_or(-1.0);
        assert_eq!(first, c.steps_to_target);
    }
    // a = 1 is the plain path, a = 1/3 is Newton's.
    let at = |a: f64| o.control_at(a).unwrap();
    assert_eq!(o.control_paths[at(1.0)], o.plain.path);
    assert_eq!(o.control[at(1.0 / 3.0)].steps_to_target, o.newton_record.steps_to_target);
    assert!(o.newton_record.steps_to_target == 1.0 && o.plain_record.steps_to_target > 1.0);
    // The steepest directions have length 1 under their rulers and differ.
    assert!((o.plain_dir[0].powi(2) + o.plain_dir[1].powi(2) - 1.0).abs() < 1e-9);
    assert!((o.newton_dir[0].powi(2) + 3.0 * o.newton_dir[1].powi(2) - 1.0).abs() < 1e-9);
    assert!((o.plain_dir[0] - o.newton_dir[0]).abs() > 0.1);
    assert!(o.gradient_gap < 1e-8 && o.quotient_gap < 1e-8);
    assert!((o.plain_first.eta - o.plain_first.dot / o.plain_first.den).abs() < 1e-12 && o.start_parts[0] + o.start_parts[1] == 2.0 * o.plain.loss[0]);
    assert!(o.contraction[0] > o.contraction[1] && o.contraction[1] > 0.0 && o.contraction[0] < 1.0);
    // The climbs: every point on the sphere, all three within the tolerance at the end, the flat step off the sphere.
    let on = |q: &[f64; 3]| ((q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt() - 1.0).abs() < 1e-9;
    assert_eq!(WAYS.len(), o.climbs.len());
    for c in &o.climbs {
        assert_eq!((c.path.len(), c.log.len()), (o.preset.sphere_steps + 1, o.preset.sphere_steps + 1));
        assert!(c.path.iter().all(on));
        assert!(c.record[1] < o.preset.tolerance && c.record[2] > 0.0);
        let first = c.log.iter().position(|r| r[1] < o.preset.tolerance).map(|k| k as f64).unwrap_or(-1.0);
        assert_eq!(first, c.record[2]);
    }
    assert!(o.climbs[0].record[3] < 1e-9 && o.climbs[2].record[3] > 0.1);
    assert!(on(&o.top) && o.parallels.iter().flatten().all(on) && o.meridians.iter().flatten().all(on));
}

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = optimization_explorer_web::text::all();
    assert_eq!(lesson::eli5::missing(&texts), Vec::<String>::new());
    let used = lesson::eli5::used(&texts);
    for id in ["metric", "preconditioner", "riemannian-gradient", "gradient-field", "exponential-map", "condition-number", "line-search", "contour", "eigenvector"] {
        assert!(used.contains(&id.to_string()), "{id}");
    }
}

#[test]
fn no_verdict_word_in_the_prose() {
    for t in optimization_explorer_web::text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t:.60}");
    }
}

#[test]
fn every_library_function_the_program_calls_is_found_in_its_source() {
    for (lib, name) in lesson::explain::calls(&program()) {
        assert!(lesson::explain::definition(&lib, &name).is_some(), "{lib}:{name}");
    }
}
