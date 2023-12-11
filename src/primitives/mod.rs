//! Contains various primitive parsing routines.
//! Not part of the public API surface.

mod line;
#[allow(unused_imports)]
pub(crate) use line::{line, normalized_line};
