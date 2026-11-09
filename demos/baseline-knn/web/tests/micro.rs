//! The page's model natively: the page runs the command line's core,
//! what it reads back is what the command line prints, and the numbers
//! are the baseline's; and the new-eyes checks.

use baseline_knn_web::micro::{core, program, run, SOURCE};

#[test]
fn the_page_runs_the_command_lines_core() {
    assert!(SOURCE.contains(core()) && !core().contains("ch:t_est") && program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-baseline-knn.out");
    let b = run().unwrap();
    for r in &b.records {
        let line = format!("{:?} {:?} {:?} {:?}", r.accuracy, r.held, r.trained, r.operations);
        assert!(golden.lines().any(|l| l.split_whitespace().collect::<Vec<_>>().join(" ") == line), "{line}");
    }
    assert!(golden.contains(&format!("{:?} {:?}", b.distortion.0, b.distortion.1)));
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_numbers_are_the_baselines() {
    let b = run().unwrap();
    assert_eq!(b.records.len(), 5);
    assert!(b.records[0].accuracy > 0.8 && b.records[1].accuracy > 0.8 && b.records[2].accuracy > 0.6);
    assert!(b.records[3].accuracy >= b.records[2].accuracy);
    assert!(b.records.iter().all(|r| r.trained == 0.0 && r.held > 0.0));
    // The accuracy is the share of held-out points guessed right, as the picture counts them.
    assert!((b.records[0].accuracy - (30 - b.flat.wrong()) as f64 / 30.0).abs() < 1e-12);
    assert!((b.records[2].accuracy - (40 - b.directions.wrong()) as f64 / 40.0).abs() < 1e-12);
    assert!((b.records[4].accuracy - (7 - b.tree.wrong()) as f64 / 7.0).abs() < 1e-12);
    assert_eq!((b.flat.points.len(), b.directions.points.len(), b.tree.points.len(), b.edges.len()), (120, 160, 31, 30));
    assert!(b.unit.points.iter().all(|q| ((q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt() - 1.0).abs() < 1e-9));
    assert!(b.distortion.0 > 0.15 && b.distortion.1 > 0.75);
}

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = baseline_knn_web::text::all();
    assert_eq!(lesson::eli5::missing(&texts), Vec::<String>::new());
}

#[test]
fn no_verdict_word_in_the_prose() {
    for t in baseline_knn_web::text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t:.60}");
    }
}

#[test]
fn every_library_function_the_program_calls_is_found_in_its_source() {
    for (lib, name) in lesson::explain::calls(&program()) {
        assert!(lesson::explain::definition(&lib, &name).is_some(), "{lib}:{name}");
    }
}
