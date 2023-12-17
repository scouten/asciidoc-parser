//! Contains various primitive parsing routines.
//! Not part of the public API surface.

mod line;
#[allow(unused_imports)]
pub(crate) use line::{line, non_empty_line, normalized_line};
