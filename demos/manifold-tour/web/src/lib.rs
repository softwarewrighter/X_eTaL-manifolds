//! Manifold tour in the browser: the demo's own program
//! (manifold-tour.xtl) run once; its coordinates drawn turning, flat,
//! and as numbers. `micro` is the model (tested natively); `turn` the
//! page's projection of three dimensions onto two (presentation only,
//! the workaround for ask GM3); `text` the words; `app` the page.

pub mod app;
pub mod micro;
pub mod text;
pub mod turn;
