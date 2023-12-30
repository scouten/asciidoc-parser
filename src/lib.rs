#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(missing_docs)]
#![deny(warnings)]
#![doc = include_str!("../README.md")]

pub mod blocks;

pub mod document;

mod error;
pub use error::{Error, ParseResult};

mod has_span;
pub use has_span::HasSpan;

pub(crate) mod primitives;
pub use primitives::Span;
pub mod strings;

#[cfg(test)]
mod tests;
