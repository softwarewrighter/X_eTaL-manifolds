//! The page's model natively: the page runs the command line's core,
//! what it reads back is what the command line prints, and the
//! properties the page states hold on the numbers; and the new-eyes
//! checks.

use mhc_explorer_web::micro::{core, program, run, SOURCE};

#[test]
fn the_page_runs_the_command_lines_core() {
    assert!(SOURCE.contains(core()) && !core().contains("ch:t_est") && program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-mhc-explorer.out");
    let m = run().unwrap();
    let parsed: Vec<Vec<f64>> = golden.lines().map(|l| l.split_whitespace().filter_map(|w| w.parse().ok()).collect()).collect();
    let printed = |v: &[f64]| parsed.iter().any(|l| l.len() == v.len() && l.iter().zip(v).all(|(a, b)| (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1e-300)));
    assert!(printed(&m.counts) && printed(&m.one) && printed(&m.corner) && printed(&m.middle));
    for r in &m.copies {
        assert!(printed(r));
    }
    for e in &m.errors {
        assert!(printed(e));
    }
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_properties_hold_on_the_numbers() {
    let m = run().unwrap();
    // Sums and non-negativity along the path; singular values at most 1, the first 1 and the others 1 - t.
    assert_eq!(m.path.len(), 11);
    for pt in &m.path {
        assert!(pt.row_sums.iter().chain(&pt.col_sums).all(|s| (s - 1.0).abs() < 1e-15));
        assert!(pt.smallest >= 0.0 && pt.off < 1e-15);
        assert!(pt.singular.iter().all(|s| *s <= 1.0 + 1e-15));
        assert!((pt.singular[0] - 1.0).abs() < 1e-12 && (pt.singular[1] - (1.0 - pt.t)).abs() < 1e-12);
    }
    // Sinkhorn's row error falls and reaches the tolerance; the columns sum to 1 after every sweep.
    assert_eq!(m.errors.len(), m.sweeps + 1);
    assert!(m.errors.windows(2).all(|w| w[1][0] <= w[0][0]));
    assert!(m.errors[1..].iter().all(|e| e[1] < 1e-15));
    assert!(m.sweeps_to_tolerance > 0.0 && m.errors[m.sweeps_to_tolerance as usize][0] < m.tolerance);
    // The doubly stochastic matrix keeps the mean at every count and shrinks the spread; the orthogonal keeps the length.
    let (ds, or) = (&m.kinds[0], &m.kinds[3]);
    assert!(ds.signal.iter().all(|r| (r[0] - ds.signal[0][0]).abs() < 1e-9));
    assert!(ds.signal.windows(2).all(|w| w[1][1] < w[0][1]));
    assert!(or.signal.iter().all(|r| (r[2] - or.signal[0][2]).abs() < 1e-9));
    assert!(ds.singular[0] <= 1.0 + 1e-9 && or.singular.iter().all(|s| (s - 1.0).abs() < 1e-9));
    // The free matrix's powers grow, the scaled one's die away.
    assert!(m.kinds[1].signal[3][2] > 1e6 && m.kinds[2].signal[3][2] < 1e-6);
    // The polytope: six corners on the set, the size-2 set a segment from the swap to the identity, the corner and the middle.
    assert_eq!(m.perms.len(), 6);
    assert!(m.perm_checks.iter().all(|c| *c == 0.0));
    assert_eq!((m.two_by_two[0], m.two_by_two[10]), ([0.0, 1.0, 1.0, 0.0], [1.0, 0.0, 0.0, 1.0]));
    assert!(m.corner[0] < 0.0 && m.corner[2] >= 0.0 && m.middle.iter().all(|v| *v > 0.0));
    assert_eq!(m.counts, [9.0, 5.0, 4.0, 4.0]);
    assert_eq!(m.zeros, [6.0, 0.0]);
    assert!(m.rates[1] < m.kinds[1].singular[0] && m.rates[1] > 1.0 && m.rates[2] < 1.0);
    assert!(m.row_sums_after.iter().any(|s| (s - 1.0).abs() > 1e-6));
}

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = mhc_explorer_web::text::all();
    assert_eq!(lesson::eli5::missing(&texts), Vec::<String>::new());
    let used = lesson::eli5::used(&texts);
    for id in ["doubly-stochastic", "sinkhorn", "polytope", "singular-value", "mixing-gain", "residual-stream", "hyper-connections", "mhc", "orthogonal-group"] {
        assert!(used.contains(&id.to_string()), "{id}");
    }
}

#[test]
fn no_verdict_word_in_the_prose() {
    for t in mhc_explorer_web::text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t:.60}");
    }
}

#[test]
fn every_library_function_the_program_calls_is_found_in_its_source() {
    for (lib, name) in lesson::explain::calls(&program()) {
        assert!(lesson::explain::definition(&lib, &name).is_some(), "{lib}:{name}");
    }
}
