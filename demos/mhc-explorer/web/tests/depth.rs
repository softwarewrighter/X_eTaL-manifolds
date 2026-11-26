//! Part 2's model natively: the page runs the second program's core
//! with the offline records, what it reads back is what the command
//! line prints, and the numbers agree with each other where the page
//! sets them side by side.

use std::sync::OnceLock;

use mhc_explorer_web::depth::{core, program, run, Depth, SOURCE};

/// The program runs once for every test here.
fn model() -> &'static Depth {
    static M: OnceLock<Depth> = OnceLock::new();
    M.get_or_init(|| run().unwrap())
}

#[test]
fn the_page_runs_the_second_programs_core() {
    assert!(SOURCE.contains(core()) && !core().contains("ch:t_est") && program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-mhc-depth.out");
    let t = model();
    let parsed: Vec<Vec<f64>> = golden.lines().map(|l| l.split_whitespace().filter_map(|w| w.parse().ok()).collect()).collect();
    let printed = |v: &[f64]| parsed.iter().any(|l| l.len() == v.len() && l.iter().zip(v).all(|(a, b)| (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1e-300)));
    assert!(printed(&t.gradient_gaps) && printed(&[t.page_gap]));
    for row in &t.cells {
        for c in row {
            let s = &c.start;
            assert!(printed(&[s.up[0], s.up[1], s.up[2], s.down[0], s.down[1], s.down[2], s.spread[0], s.spread[1], s.spread[2], s.gain[0], s.gain[1], s.off[0], s.off[1]]), "a start row");
        }
    }
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_numbers_agree_with_each_other() {
    let t = model();
    let pr = &t.preset;
    assert_eq!(t.trained_on + t.held_out, pr.k * pr.per);
    assert_eq!(t.per_layer, pr.n * pr.n + pr.d * (pr.d + 1) + 2 * pr.n);
    for row in &t.cells {
        for (di, c) in row.iter().enumerate() {
            let l = pr.depths[di];
            assert_eq!((c.start.sizes.len(), c.start.gradients.len(), c.start.spreads.len()), (l + 1, l + 1, l + 1));
            assert_eq!(c.records.len(), pr.seeds);
            assert_eq!(c.after.finite, c.records.iter().filter(|r| r.finite).count());
            for r in &c.records {
                assert_eq!((r.losses.len(), r.gradients.len()), (pr.steps + 1, pr.steps));
            }
        }
    }
    // The page's run is the first steps of the first seed's record at the smallest depth.
    for (k, run) in t.page_losses.iter().enumerate() {
        assert_eq!(run.len(), pr.page_steps + 1);
        assert!(run.iter().zip(&t.cells[k][0].records[0].losses).all(|(a, b)| (a - b).abs() < 1e-12));
    }
    // Every kind's stack starts from the same numbers: the first state's size is the same for all four.
    for di in 0..4 {
        let s0 = t.cells[0][di].start.sizes[0];
        assert!(t.cells.iter().all(|row| (row[di].start.sizes[0] - s0).abs() < 1e-12));
    }
    assert!(t.gradient_gaps.iter().all(|g| *g < 1e-7) && t.page_gap < 1e-12);
}

#[test]
fn what_the_page_says_holds_on_the_numbers() {
    let t = model();
    let (plain, free, ds, orth) = (&t.cells[0], &t.cells[1], &t.cells[2], &t.cells[3]);
    // The free mixing's composite gain grows with depth and passes 10 at the deepest; the constrained stay within 0.5 to 2.
    assert!(free.windows(2).all(|w| w[1].start.gain[0] > w[0].start.gain[0]) && free[3].start.gain[0] > 10.0);
    assert!(ds.iter().chain(orth.iter()).all(|e| (0.5..=2.0).contains(&e.start.gain[0])));
    // At the deepest no free seed trained; every constrained and plain seed did; the cost within twice the free one's.
    assert_eq!((free[3].after.trained, free[3].after.finite), (0, 0));
    assert!([plain, ds, orth].iter().all(|k| k.iter().all(|e| e.after.trained == t.preset.seeds)));
    assert!(ds[3].cost < 2.0 * free[3].cost && orth[3].cost < 2.0 * free[3].cost);
    // The plain residual grows too: signal up and gradient down, more with depth.
    assert!(plain.windows(2).all(|w| w[1].start.up[0] > w[0].start.up[0] && w[1].start.down[0] > w[0].start.down[0]));
    // The doubly stochastic stack shrinks the spread with depth, at the start and after training; the orthogonal does not.
    assert!(ds.windows(2).all(|w| w[1].start.spread[0] < w[0].start.spread[0]) && ds[3].after.spread < 0.2);
    assert!(orth.iter().all(|e| e.start.spread[0] > 1.0 && e.after.spread > 1.0));
    // At the deepest, the accuracies of the orthogonal and doubly stochastic stacks lie within the plain residual's range or overlap it.
    let (p, o, d) = (plain[3].after.held, orth[3].after.held, ds[3].after.held);
    assert!(o[1] <= p[2] && d[0] >= p[1] && d[0] <= p[2]);
}

#[test]
fn the_analysis_reads_the_numbers_it_states() {
    let t = model();
    let (plain, free, ds, orth) = (&t.cells[0], &t.cells[1], &t.cells[2], &t.cells[3]);
    // Free at 16 layers: one of three finite; at 32 none.
    assert_eq!((free[2].after.finite, free[3].after.finite), (1, 0));
    // At 8 layers the orthogonal accuracy lies above the plain residual's whole range; elsewhere the ranges overlap.
    assert!(orth[1].after.held[1] > plain[1].after.held[2]);
    for di in [0, 2, 3] {
        assert!(orth[di].after.held[1] <= plain[di].after.held[2] && plain[di].after.held[1] <= orth[di].after.held[2]);
    }
    // At 32 layers the doubly stochastic stack has the highest mean end loss of the finite stacks.
    assert!(ds[3].after.loss[0] > plain[3].after.loss[0] && ds[3].after.loss[0] > orth[3].after.loss[0]);
    // The plain residual's held-out accuracy at 32 layers is below its accuracy at 4.
    assert!(plain[3].after.held[0] < plain[0].after.held[0]);
    // The doubly stochastic stack and the plain residual: the same gradient growth at the start, the same first loss.
    for di in 0..4 {
        assert!(ds[di].start.down.iter().zip(&plain[di].start.down).all(|(a, b)| (a - b).abs() <= 1e-12 * a.abs()));
    }
    assert!((t.page_losses[0][0] - t.page_losses[2][0]).abs() < 1e-12);
    // The orthogonal gain stays near 1.8, inside 0.5 to 2.
    assert!(orth.iter().all(|e| e.start.gain[0] > 1.5 && e.start.gain[0] < 2.0));
}
