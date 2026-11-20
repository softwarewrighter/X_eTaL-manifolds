//! Optimization explorer in the browser: the demo's own program
//! (optimization-explorer.xtl) run once; one fixed loss, its contours
//! and gradient field, descent under three metrics with the control's
//! anisotropy picked by the reader, the steepest direction under two
//! metrics worked, and a step on the sphere against a flat step
//! repaired; on the shared teaching shell and turning view. `micro` is
//! the model (tested natively); `text` the words; `app` the page.

pub mod app;
pub mod micro;
pub mod model;
pub mod text;
pub mod training;
