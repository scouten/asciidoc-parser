#![allow(clippy::module_inception)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(missing_docs)]
#![deny(warnings)]
#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

pub mod attributes;

pub mod blocks;

pub mod document;
pub use document::Document;

mod span;
pub use span::{HasSpan, Span};

pub mod strings;

#[cfg(test)]
mod tests;

mod warnings;
