//! X_eTaL-manifolds' own parts for its pages, on top of the shared
//! shells of X_eTaL-ML (`lesson`, the teaching shell of ask GM1, and
//! `microscope`): the six parts this repository adds to the page arc
//! of its plan's principle 6 (as `lesson::chrome::Frame::Other`), the
//! mark measured or analytical every figure carries (G9), the
//! comparison table of choices against measures (plan G5, principle
//! 1; asked of the shell as GM8, this thin one meanwhile), and the
//! aside for a function of this repository's own libraries (the
//! shell's `explain::function` credits a dependent's libraries to
//! X_eTaL-libraries, ask GM7; this one says where they live). The site,
//! the header and footer with the pins, the six framing kinds, the
//! glossary and the code links are the shell's. Presentation only:
//! nothing here computes a number a page shows.

use lesson::chrome::{Frame, SITE};
use lesson::code::Linker;
use lesson::explain::{calls, definition, imports, Definition};
use yew::prelude::*;

include!(concat!(env!("OUT_DIR"), "/own.rs"));

/// Whether a library is one of this repository's own (`libs/<Name>/`).
pub fn own(library: &str) -> bool {
    OWN_LIBRARIES.contains(&library)
}

/// Where a function's source is on GitHub: this repository at the
/// build's commit for its own libraries, the shell's answer for the
/// rest.
pub fn source_url(d: &Definition) -> String {
    match own(&d.library) {
        true => format!("{}/blob/{}/libs/{}/src/{}.xtl#L{}", SITE.url, lesson::chrome::COMMIT, d.library, d.library, d.line),
        false => d.source_url.clone(),
    }
}

/// The repository a library lives in, for a sentence.
pub fn home(library: &str) -> &'static str {
    match own(library) {
        true => "this repository",
        false => lesson::explain::home(library),
    }
}

/// Every library function a program calls, one line each, as the
/// shell's `explain::table` draws it, with this repository's own
/// libraries credited to it (GM7).
pub fn functions(program: &str) -> Html {
    let ims = imports(program);
    let linker = Linker::of(program);
    let rows = calls(program).into_iter().filter_map(|(lib, name)| {
        let alias = ims.iter().find(|i| i.library == lib).map(|i| i.alias.clone())?;
        let d = definition(&lib, &name)?;
        Some(html! {
            <tr>
                <td>{ linker.code(&format!("{alias}{name}{}", if d.source.starts_with("m:") { "<" } else { "" })) }</td>
                <td>{ d.doc.clone() }</td>
                <td class="links"><a href={source_url(&d)} target="_blank">{"source"}</a>{" \u{00b7} "}<a href={d.xref.clone()}>{"cross-reference"}</a></td>
            </tr>
        })
    });
    html! { <table class="functions"><tr><th>{"Function"}</th><th>{"What the library says it does"}</th><th>{"Read it"}</th></tr>{ for rows }</table> }
}

/// The parts of the arc this repository adds to the shell's six (the
/// question, the background, the rationale, the purpose, the summary,
/// what is next): the intuition with a geometric picture, the
/// mathematics with every variable explained, the experiment, the ML
/// run (or the statement that nothing is trained), the analysis and
/// comparison, and the research extension. Every page has all twelve
/// (scripts/browser-check.py requires them).
pub const INTUITION: Frame = Frame::Other("intuition");
pub const MATHEMATICS: Frame = Frame::Other("mathematics");
pub const EXPERIMENT: Frame = Frame::Other("experiment");
pub const RUN: Frame = Frame::Other("run");
pub const ANALYSIS: Frame = Frame::Other("analysis");
pub const EXTENSION: Frame = Frame::Other("extension");

/// The added parts, in the order a page shows them (between the
/// purpose and the summary).
pub const ADDED: [Frame; 6] = [INTUITION, MATHEMATICS, EXPERIMENT, RUN, ANALYSIS, EXTENSION];

/// Every frame a page of this repository has, in order.
pub fn all_frames() -> Vec<&'static str> {
    let mut v: Vec<&str> = lesson::chrome::FRAMES[..4].iter().map(|f| f.name()).collect();
    v.extend(ADDED.iter().map(|f| f.name()));
    v.extend(lesson::chrome::FRAMES[4..].iter().map(|f| f.name()));
    v
}

/// A figure's mark: measured (from the X_eTaL run) or analytical (a
/// formula drawn, which the run also computed point by point).
pub fn mark(measured: bool) -> Html {
    let (class, text, tip) = match measured {
        true => ("measured", "measured", "these numbers were computed by the X_eTaL program on this page, in your browser"),
        false => ("analytical", "analytical", "a shape drawn from its formula; every point of it was still computed by the X_eTaL program, not by the page"),
    };
    html! { <span class={classes!("mark", class)} title={tip}>{ text }</span> }
}

/// One cell of a comparison: a number as the run printed it, marked.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub text: String,
    pub measured: bool,
}

impl Cell {
    pub fn measured(text: impl Into<String>) -> Cell {
        Cell { text: text.into(), measured: true }
    }
    pub fn analytical(text: impl Into<String>) -> Cell {
        Cell { text: text.into(), measured: false }
    }
}

/// The comparison table of a page (plan G5): the choices compared as
/// rows, the measures as columns, every cell a number from the X_eTaL
/// run marked measured or analytical, and under it the shell's
/// sentences for what changed (`lesson::narrate`: comparisons, never a
/// verdict; `sentences` are those, already made). Presentation only.
pub fn comparison(measures: &[&str], rows: &[(&str, Vec<Cell>)], sentences: &[String]) -> Html {
    let head = measures.iter().map(|m| html! { <th>{ m.to_string() }</th> });
    let body = rows.iter().map(|(choice, cells)| html! {
        <tr>
            <th scope="row">{ choice.to_string() }</th>
            { for cells.iter().map(|c| html! { <td>{ c.text.clone() }{ mark(c.measured) }</td> }) }
        </tr>
    });
    html! { <>
        <table class="numbers comparison">
            <thead><tr><th>{ "choice" }</th>{ for head }</tr></thead>
            <tbody>{ for body }</tbody>
        </table>
        { for sentences.iter().map(|s| html! { <p class="narrated">{ s.clone() }</p> }) }
    </> }
}

/// The aside for a function of this repository's own libraries
/// (`libs/`), with the shell's definition (its doc, its source, its
/// line, its cross-reference entry) and the right home and source link
/// (the shell's own aside credits them to X_eTaL-libraries, ask GM7:
/// this goes when that lands). `purpose` and `reading` are the page's
/// words, as in `lesson::explain::function`.
pub fn own_function(alias: &str, d: &Definition, purpose: Html, reading: Html) -> Html {
    let import = format!("\"{alias}\" u_se< \"{}\"", d.library);
    let source = microscope::libs::LIBRARIES.iter().find(|(f, _)| *f == format!("{}.xtl", d.library)).map(|(_, s)| *s).unwrap_or("");
    let within = Linker::within(&d.library, source);
    let outside = Linker::of(&import);
    let source_url = source_url(d);
    html! {
        <details class="aside" data-aside={format!("{alias}{}", d.name)}>
            <summary>{ format!("See the code behind this: the function {} of the {} library, where it comes from, what it does, how it is written", d.name.replace('_', ""), d.library) }</summary>
            <div class="aside-body">
                <h3>{"The code, and what it is for"}</h3>
                {purpose}
                <h3>{"Where it comes from"}</h3>
                <p>{ format!("It is a function of the {} library, which is part of {} ({}: written here, for what is particular to manifolds and the geometry of ML). A program opens the library with the one line below, giving it a short name of its choice, here {}. After that, a name drawn with a small raised {} before it is one of that library's functions.", d.library, home(&d.library), SITE.name, alias.trim_end_matches(':'), alias.trim_end_matches(':')) }</p>
                <p>{ outside.code(&import) }</p>
                <h3>{"How it is written"}</h3>
                <p>{ format!("This is the function exactly as the library has it (line {} of its file), not a copy made for this page:", d.line) }</p>
                { within.block(&d.source) }
                {reading}
                <h3>{"The library's own description (written for programmers)"}</h3>
                <p class="quote">{ d.doc.clone() }</p>
                <p class="links">
                    <a href={source_url} target="_blank">{"Its source on GitHub"}</a>{" \u{00b7} "}
                    <a href={d.xref.clone()}>{"In the cross-reference"}</a>
                </p>
            </div>
        </details>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shell_knows_this_repository() {
        assert_eq!(SITE.name, "X_eTaL-manifolds");
        assert!(SITE.url.ends_with("/X_eTaL-manifolds"));
        let pins: Vec<&str> = lesson::chrome::pins().iter().map(|p| p.0).collect();
        assert_eq!(pins, ["X_eTaL-ML", "X_eTaL-libraries", "X_eTaL"]);
    }

    #[test]
    fn the_twelve_frames_in_order() {
        assert_eq!(all_frames(), ["question", "background", "rationale", "purpose", "intuition", "mathematics", "experiment", "run", "analysis", "extension", "summary", "next"]);
    }

    #[test]
    fn this_repositorys_glossary_is_read_and_its_libraries_are_found() {
        assert!(lesson::eli5::concept("manifold").is_some(), "the glossary rows of docs/glossary.md");
        assert!(lesson::eli5::concept("table").is_some(), "the shell's base rows");
        let d = lesson::explain::definition("Manifold", "s_phereDistance").expect("Manifold embedded");
        assert!(d.source.starts_with("l:s_phereDistance := "));
        assert!(lesson::explain::definition("Kit", "f_lat").is_some());
        assert_eq!(OWN_LIBRARIES, ["Kit", "Manifold"]);
        assert!(source_url(&d).contains("/X_eTaL-manifolds/blob/") && source_url(&d).ends_with("#L104"), "{}", source_url(&d));
        assert_eq!(home("Manifold"), "this repository");
        assert_eq!(home("Trig"), "X_eTaL-libraries");
    }
}
