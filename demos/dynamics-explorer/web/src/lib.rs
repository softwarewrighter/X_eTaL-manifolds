//! Dynamics explorer in the browser: the demo's own program
//! (dynamics-explorer.xtl) run once; five plane fields with their
//! arrows, paths, divergence, curl and rest points, the field picked by
//! the reader; Euler's step against Runge-Kutta's on the rotation, the
//! step size and the steps shown picked by the reader; a loop worked in
//! full; gradient descent beside a two-player game and two repairs; on
//! the shared teaching shell. `model` is the model (tested natively);
//! `text` the words; `app` the page.

pub mod app;
pub mod model;
pub mod text;
