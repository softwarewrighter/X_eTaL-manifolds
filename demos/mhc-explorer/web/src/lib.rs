//! mHC explorer in the browser: the demo's own programs run once, part 1
//! (mhc-explorer.xtl) at once and part 2 (mhc-depth.xtl) after the first
//! paint; part 1: residual streams and the matrices that mix them, a point
//! on the path from the identity to uniform mixing picked by the
//! reader; part 2: a stack of layers under four kinds of mixing, at the
//! start and trained, a depth picked by the reader; on the shared
//! teaching shell. `micro` and `depth` are the models (tested natively);
//! `text` the words; `stack` part 2's views; `app` the page.

pub mod app;
pub mod depth;
pub mod micro;
pub mod stack;
pub mod text;
