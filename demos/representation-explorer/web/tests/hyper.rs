//! Part 2's model natively: the page runs the second program's core,
//! what it reads back is what the command line prints, and the numbers
//! agree with the pictures' counts.

use representation_explorer_web::hyper::{core, program, run, SOURCE};

#[test]
fn the_page_runs_the_second_programs_core() {
    assert!(SOURCE.contains(core()) && !core().contains("ch:t_est") && program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-representation-hyperbolic.out");
    let h = run().unwrap();
    for f in &h.fits {
        let r = f.record;
        let line = format!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", r.mean, r.worst, r.steps_to_target, r.forgiven, r.trained, r.operations, r.off, r.knn, r.logistic);
        assert!(golden.lines().any(|l| l.split_whitespace().collect::<Vec<_>>().join(" ") == line), "{line}");
        assert!(golden.contains(&format!("{} {}", f.worst_pair.0, f.worst_pair.1)));
    }
    for c in &h.curves {
        let line = format!("{:?} {:?} {:?} {:?} {:?}", c.k, c.mean, c.worst, c.steps_to_target, c.off);
        assert!(golden.lines().any(|l| l.split_whitespace().collect::<Vec<_>>().join(" ") == line), "{line}");
    }
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_numbers_are_the_comparisons() {
    let h = run().unwrap();
    let n = h.n();
    assert_eq!((n, h.edges.len(), h.held.len() + h.train.len()), (31, 30, 31));
    assert_eq!(h.fits.len(), 3);
    for f in &h.fits {
        assert_eq!((f.points.len(), f.log.len(), f.edge_distortion.len()), (n, h.preset.steps + 1, n - 1));
        // The record's end row is the log's last; the distortion fell.
        let last = f.log[h.preset.steps];
        assert_eq!((last.mean, last.worst), (f.record.mean, f.record.worst));
        assert!(last.mean < f.log[0].mean);
        // The accuracies are the shares the pictures count.
        assert!((f.record.knn - (7 - h.wrong(&f.knn_guess)) as f64 / 7.0).abs() < 1e-12);
        assert!((f.record.logistic - (7 - h.wrong(&f.logistic_guess)) as f64 / 7.0).abs() < 1e-12);
        // The steps to the target is the first log row under it, or -1.
        let first = f.log.iter().position(|r| r.mean < h.preset.target).map(|k| k as f64).unwrap_or(-1.0);
        assert_eq!(first, f.record.steps_to_target);
    }
    // The control's rows at +1, 0 and -1 are the three fits read at the control's step.
    let at = h.preset.control_steps;
    for (c, f) in [(0, 0), (3, 1), (6, 2)] {
        assert_eq!(h.curves[c].k, h.fits[f].curvature);
        assert_eq!(h.curves[c].mean, h.fits[f].log[at].mean);
    }
    assert_eq!(h.curves.len(), h.preset.curvatures.len());
    // The sphere embedding is on the sphere; the disk's inside the disk; the plane's third coordinate 0.
    let on = |q: &[f64; 3]| ((q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt() - 1.0).abs() < 1e-9;
    assert!(h.fits[0].points.iter().all(on));
    assert!(h.fits[2].points.iter().all(|q| q[0] * q[0] + q[1] * q[1] < 1.0 && q[2] == 0.0));
    assert!(h.fits[1].points.iter().all(|q| q[2] == 0.0));
    assert!(h.parallels.iter().flatten().all(on) && h.meridians.iter().flatten().all(on));
    assert_eq!(h.sphere_map.len(), n);
    // The worked distance is the disk's and more than the plain one; the geodesic is the shorter path.
    assert!((h.worked_disk - h.worked_acosh).abs() < 1e-9 && h.worked_disk > h.worked_plain);
    assert!(h.path_lengths[0] < h.path_lengths[1] && h.geodesic.len() == 13);
    // The room table: nodes double, the disk's spacing grows.
    assert!(h.room.windows(2).all(|w| w[1][0] == 2.0 * w[0][0] && w[1][4] > w[0][4]));
    // Node 2's nearest in the disk are three nodes; the root and the last leaf are depth apart, the first and last leaves 2 depth.
    assert_eq!(h.neighbors.len(), 3);
    assert_eq!((h.tree_distance(1, 31), h.tree_distance(16, 31)), (h.preset.depth as f64, 2.0 * h.preset.depth as f64));
}
