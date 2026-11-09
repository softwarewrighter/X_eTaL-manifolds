//! The page's model natively: the page runs the command line's core,
//! what it reads back is what the command line prints, and the two
//! facts hold on the numbers; and the new-eyes checks.

use sphere_hello_web::micro::{core, program, run, PAIRS, SOURCE};

#[test]
fn the_page_runs_the_command_lines_core() {
    assert!(SOURCE.contains(core()));
    assert!(!core().contains("ch:t_est"));
    assert!(program().starts_with(core()));
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-sphere-hello.out");
    let h = run().unwrap();
    let line = |v: &[f64]| v.iter().map(|x| format!("{x:?}")).collect::<Vec<_>>().join(" ");
    assert!(golden.lines().any(|l| l.trim() == line(&h.plain)), "the plain mean");
    assert!(golden.lines().any(|l| l.trim() == format!("{:?}", h.plain_length)), "its length");
    assert!(golden.lines().any(|l| l.trim() == format!("{:?}", h.apart)), "the means apart");
    assert_eq!(golden.lines().filter(|l| l.starts_with("FAIL")).count(), 0);
}

#[test]
fn the_two_facts_hold_on_the_numbers() {
    let h = run().unwrap();
    for (i, j) in PAIRS {
        let (chord, arc) = h.distances(i, j);
        assert!(arc >= chord && (chord - 2.0 * (arc / 2.0).sin()).abs() < 1e-9, "{i} {j}");
    }
    assert_eq!(h.same_orders(), 6);
    assert!(h.plain_length < 1.0 && h.apart > 0.05);
    let len = |p: [f64; 3]| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    assert!((len(h.on_sphere) - 1.0).abs() < 1e-12);
    for (k, arc) in h.arcs.iter().enumerate() {
        assert_eq!(arc.len(), 13);
        let (i, j) = PAIRS[k];
        assert!(arc.iter().all(|q| (len(*q) - 1.0).abs() < 1e-9));
        assert!(arc[0].iter().zip(&h.p[i - 1]).all(|(a, b)| (a - b).abs() < 1e-9) && arc[12].iter().zip(&h.p[j - 1]).all(|(a, b)| (a - b).abs() < 1e-9));
    }
    assert_eq!((h.parallels.len(), h.meridians.len(), h.map.len()), (5, 12, 6));
}

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = sphere_hello_web::text::all();
    assert_eq!(lesson::eli5::missing(&texts), Vec::<String>::new());
    assert!(lesson::eli5::used(&texts).contains(&"frechet-mean".to_string()));
}

#[test]
fn no_verdict_word_in_the_prose() {
    for t in sphere_hello_web::text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t:.60}");
    }
}

#[test]
fn every_library_function_the_program_calls_is_found_in_its_source() {
    for (lib, name) in lesson::explain::calls(&program()) {
        assert!(lesson::explain::definition(&lib, &name).is_some(), "{lib}:{name}");
    }
}
