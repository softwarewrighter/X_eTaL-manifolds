//! Part 2's model natively: the page runs the second program's core,
//! what it reads back is what the command line prints, and the numbers
//! agree with each other where the page sets them side by side.

use optimization_explorer_web::training::{core, program, run, SOURCE};

#[test]
fn the_page_runs_the_second_programs_core() {
    assert!(SOURCE.contains(core()) && !core().contains("ch:t_est") && program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-optimization-training.out");
    let t = run().unwrap();
    // X_eTaL prints every float in full decimal; compared as numbers, to a part in 10^12.
    let parsed: Vec<Vec<f64>> = golden.lines().map(|l| l.split_whitespace().filter_map(|w| w.parse().ok()).collect()).collect();
    let printed = |v: &[f64]| parsed.iter().any(|l| l.len() == v.len() && l.iter().zip(v).all(|(a, b)| (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1e-300)));
    for r in t.bad.iter().chain(&t.good) {
        let c = r.record;
        assert!(printed(&[c.end_loss, c.end_accuracy, c.first_under, c.stays_under, c.mean_update, c.largest_update, c.numbers, c.multiplies]), "a record row");
    }
    for s in &t.sweep {
        assert!(printed(&[s.rate, s.end_loss, s.first_under, s.stays_under, s.largest_update]), "a sweep row");
    }
    for g in &t.invariance {
        assert!(printed(g), "an invariance row");
    }
    assert!(printed(&t.spent) && printed(&[t.condition_ratio]));
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_numbers_agree_with_each_other() {
    let t = run().unwrap();
    let pr = &t.preset;
    assert_eq!((t.x.len(), t.y.len(), t.held.len() + t.trained_on), (pr.k * pr.per, pr.k * pr.per, pr.k * pr.per));
    for r in t.bad.iter().chain(&t.good) {
        assert_eq!(r.log.len(), pr.steps + 1);
        let last = r.log[pr.steps];
        assert_eq!((last.loss, last.accuracy), (r.record.end_loss, r.record.end_accuracy));
        // The first step under the target is the first log row under it, or -1.
        let first = r.log.iter().position(|w| w.loss < pr.target).map(|k| k as f64).unwrap_or(-1.0);
        assert_eq!(first, r.record.first_under);
        // It stays under from a step at or after the first; every row from there is under.
        if r.record.stays_under >= 0.0 {
            assert!(r.record.stays_under >= r.record.first_under);
            assert!(r.log[r.record.stays_under as usize..].iter().all(|w| w.loss < pr.target));
        }
    }
    // Every run starts from all weights 0: every class 1/3, the loss ln 3, on both data.
    for r in t.bad.iter().chain(&t.good) {
        assert!((r.log[0].loss - 3f64.ln()).abs() < 1e-12);
    }
    // The metric undoes the scaling: the diagonal and natural runs take the same steps on both data.
    assert_eq!((t.bad[1].record.stays_under, t.bad[2].record.stays_under), (t.good[1].record.stays_under, t.good[2].record.stays_under));
    // The checks' gaps are small; the invariance table says what the page says.
    assert!(t.gaps[0] < 1e-8 && t.gaps[1] < 1e-12 && t.gaps[2] < 1e-6);
    assert!(t.invariance[2][0] < 1e-9 && t.invariance[2][1] < 1e-9 && t.invariance[1][0] < 1e-9 && t.invariance[1][1] > 1e-3 && t.invariance[0][0] > 1e-2);
    // The cost columns grow plain, diagonal, natural; the spent column is stays-under times per-step.
    for runs in [&t.bad, &t.good] {
        assert!(runs[0].record.multiplies < runs[1].record.multiplies && runs[1].record.multiplies < runs[2].record.multiplies);
    }
    for (i, r) in t.bad.iter().chain(&t.good).enumerate() {
        let want = if r.record.stays_under < 0.0 { -1.0 } else { r.record.stays_under * r.record.multiplies };
        assert_eq!(t.spent[i], want);
    }
    assert_eq!(t.sweep.len(), pr.sweep.len());
}
