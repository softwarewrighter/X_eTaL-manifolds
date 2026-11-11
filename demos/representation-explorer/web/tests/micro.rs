//! The page's model natively: the page runs the command line's core,
//! what it reads back is what the command line prints, the numbers
//! are the comparison's and agree with the pictures' counts; and the
//! new-eyes checks.

use representation_explorer_web::micro::{core, program, run, SOURCE};

#[test]
fn the_page_runs_the_command_lines_core() {
    assert!(SOURCE.contains(core()) && !core().contains("ch:t_est") && program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-representation-explorer.out");
    let e = run().unwrap();
    for r in &e.records {
        let line = format!("{:?} {:?} {:?} {:?}", r.accuracy, r.held, r.trained, r.operations);
        assert!(golden.lines().any(|l| l.split_whitespace().collect::<Vec<_>>().join(" ") == line), "{line}");
    }
    for d in &e.distortion {
        let line = format!("{:?} {:?} {:?}", d.scale, d.mean, d.worst);
        assert!(golden.lines().any(|l| l.split_whitespace().collect::<Vec<_>>().join(" ") == line), "{line}");
    }
    assert!(golden.contains(&format!("{} {}", e.worst_pair.0, e.worst_pair.1)));
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_numbers_are_the_comparisons() {
    let e = run().unwrap();
    assert_eq!(e.records.len(), 8);
    // The baseline's rows come out the same: flat 0.9, the directions 0.75 flat and 0.775 on the sphere.
    assert!((e.records[0].accuracy - 0.9).abs() < 1e-12 && (e.records[2].accuracy - 0.75).abs() < 1e-12 && (e.records[3].accuracy - 0.775).abs() < 1e-12);
    // Written before the run: the sphere costs on the flat clusters, under both methods.
    assert!(e.records[1].accuracy < e.records[0].accuracy && e.records[5].accuracy < e.records[4].accuracy);
    // Nearest neighbors trains nothing; the network trains the same count under both representations.
    assert!(e.records[..4].iter().all(|r| r.trained == 0.0));
    assert_eq!((e.records[4].trained, e.records[5].trained, e.records[6].trained, e.records[7].trained), (51.0, 51.0, 68.0, 68.0));
    // The accuracies are the shares the pictures count.
    assert!((e.records[0].accuracy - (30 - e.flat.wrong()) as f64 / 30.0).abs() < 1e-12);
    assert!((e.records[1].accuracy - (30 - e.flat_unit.wrong()) as f64 / 30.0).abs() < 1e-12);
    assert!((e.records[2].accuracy - (40 - e.directions.wrong()) as f64 / 40.0).abs() < 1e-12);
    assert!((e.records[3].accuracy - (40 - e.unit.wrong()) as f64 / 40.0).abs() < 1e-12);
    assert!((e.records[4].accuracy - (30 - e.flat.net_wrong()) as f64 / 30.0).abs() < 1e-12);
    assert!((e.records[7].accuracy - (40 - e.unit.net_wrong()) as f64 / 40.0).abs() < 1e-12);
    // Every loss fell; the convergence rows are the curves' ends.
    for (l, c) in e.losses.iter().zip(&e.convergence) {
        assert_eq!(l.len(), 301);
        assert_eq!((l[0], l[300]), (c.start, c.end));
        assert!(c.end < c.start);
    }
    // The matching representations keep their set's distances; the crossed ones lose.
    assert_eq!((e.distortion[0].mean, e.distortion[0].worst, e.distortion[3].mean, e.distortion[3].worst), (0.0, 0.0, 0.0, 0.0));
    assert!(e.distortion[1].mean > 0.1 && e.distortion[2].mean > 0.1);
}

#[test]
fn the_pictures_are_on_the_sphere_and_the_grids_are_full() {
    let e = run().unwrap();
    let on = |q: &[f64; 3]| ((q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt() - 1.0).abs() < 1e-9;
    assert!(e.unit.points.iter().all(on) && e.arc.iter().all(on) && e.sphere_mean.iter().all(on));
    assert!(e.parallels.iter().flatten().all(on) && e.meridians.iter().flatten().all(on));
    assert_eq!((e.parallels.len(), e.meridians.len()), (5, 12));
    assert!(e.unit_mean.iter().all(|q| !on(q)));
    assert_eq!((e.grid.len(), e.flat_boundary.len(), e.angle_boundary.len()), (441, 441, 441));
    assert_eq!((e.map_grid.len(), e.map_angle.len(), e.map_half.len(), e.map_one.len(), e.map_two.len()), (288, 288, 288, 288, 288));
    assert!(e.flat_boundary.iter().all(|c| (1..=3).contains(c)) && e.map_two.iter().all(|c| (1..=4).contains(c)));
    assert_eq!(e.map.len(), 160);
    // The worked pair is the table's second row, and the arc is the angle long, the chord shorter.
    assert!((e.distances[1][0] - e.worked_straight).abs() < 1e-12 && (e.distances[1][2] - e.worked_angle).abs() < 1e-9);
    assert!(e.chord_length < e.arc_length && (e.arc_length - e.distances[1][2]).abs() < 1e-12);
}

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = representation_explorer_web::text::all();
    assert_eq!(lesson::eli5::missing(&texts), Vec::<String>::new());
}

#[test]
fn no_verdict_word_in_the_prose() {
    for t in representation_explorer_web::text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t:.60}");
    }
}

#[test]
fn every_library_function_the_program_calls_is_found_in_its_source() {
    for (lib, name) in lesson::explain::calls(&program()) {
        assert!(lesson::explain::definition(&lib, &name).is_some(), "{lib}:{name}");
    }
}
