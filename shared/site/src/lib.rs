//! X_eTaL-manifolds' site, for its pages: whose page it is
//! (`microscope::chrome::Site`, which microscope's header and footer
//! take), a footer that adds this repository's own pin, and the plain
//! sections of a page's arc (the question, the background, the
//! rationale, the purpose, the summary, what is next), each marked
//! `data-frame` so the browser check can see the page is framed. The
//! teaching shell of the sibling lessons is not here (ask GM1,
//! docs/xetal-ML-asks.md): when it lands, these sections are the ones
//! it re-frames. Presentation only: nothing here computes a number.

use microscope::chrome::{header_of, provenance, Site};
use yew::prelude::*;

pub const REPO: &str = "https://github.com/softwarewrighter/X_eTaL-manifolds";

/// This repository's pages.
pub const SITE: Site = Site {
    name: "X_eTaL-manifolds",
    url: REPO,
    commit: env!("SITE_SHA"),
    catalog: ("../", "All demos"),
    links: &[("All demos", "../"), ("Cross-reference", "../doc/")],
};

/// The X_eTaL-libraries commit this repository pins (its own pin; the
/// X_eTaL-ML pin and the X_eTaL commit it pins are microscope's).
pub const LIBRARIES_COMMIT: &str = env!("LIBRARIES_SHA");

/// The logo (back to the catalog), the title and a lede.
pub fn header(title: &str, lede: &str) -> Html {
    header_of(&SITE, title, lede)
}

fn sep() -> Html {
    html! { <span class="sep">{ "\u{00b7}" }</span> }
}

/// The footer every page of this repository shows: copyright, license,
/// the repository, the catalog and the cross-reference, then the pins
/// (X_eTaL-ML, X_eTaL-libraries, X_eTaL) and the build.
pub fn footer() -> Html {
    let mut marks = provenance(&SITE);
    marks.insert(1, format!("X_eTaL-libraries {LIBRARIES_COMMIT}"));
    let last = marks.len() - 1;
    let marks = marks.into_iter().enumerate().map(|(i, p)| html! { <><span>{ p }</span>if i < last { { sep() } }</> });
    let links = SITE.links.iter().map(|(text, href)| html! { <><a href={*href}>{ *text }</a>{ sep() }</> });
    html! {
        <footer>
            <span>{ "Copyright (c) 2026 Michael A Wright" }</span>{ sep() }
            <span>{ "MIT License" }</span>{ sep() }
            <a href={SITE.url} target="_blank">{ "Repository" }</a>{ sep() }
            { for links }
            { for marks }
        </footer>
    }
}

/// The parts of a page's arc that frame its subject: before it, the
/// question in a newcomer's words, what to know first, the idea and
/// what the page is and is not; after it, the summary against the
/// question and what is next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frame {
    Question,
    Background,
    Rationale,
    Purpose,
    Summary,
    Next,
}

impl Frame {
    /// The mark the browser check looks for (`data-frame`).
    pub fn name(self) -> &'static str {
        match self {
            Frame::Question => "question",
            Frame::Background => "background",
            Frame::Rationale => "rationale",
            Frame::Purpose => "purpose",
            Frame::Summary => "summary",
            Frame::Next => "next",
        }
    }
}

/// One framing section: a heading and its prose.
pub fn frame(which: Frame, title: &str, body: Html) -> Html {
    html! {
        <section class={classes!("panel", "frame", which.name())} data-frame={which.name()}>
            <h2>{ title }</h2>
            { body }
        </section>
    }
}

/// A paragraph of plain prose.
pub fn p(text: &str) -> Html {
    html! { <p class="prose">{ text }</p> }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_site_names_this_repository() {
        assert_eq!(SITE.name, "X_eTaL-manifolds");
        assert!(SITE.url.ends_with("/X_eTaL-manifolds"));
        assert!(!SITE.commit.is_empty() && !LIBRARIES_COMMIT.is_empty());
    }

    #[test]
    fn the_footer_names_every_pin() {
        let marks = provenance(&SITE);
        assert!(marks[0].starts_with("X_eTaL-ML "), "{marks:?}");
        assert!(marks[1].starts_with("X_eTaL "), "{marks:?}");
        assert!(marks[2].starts_with("build ("), "{marks:?}");
    }

    #[test]
    fn every_frame_has_the_name_the_browser_check_wants() {
        let names: Vec<&str> = [Frame::Question, Frame::Background, Frame::Rationale, Frame::Purpose, Frame::Summary, Frame::Next].iter().map(|f| f.name()).collect();
        assert_eq!(names, ["question", "background", "rationale", "purpose", "summary", "next"]);
    }
}
