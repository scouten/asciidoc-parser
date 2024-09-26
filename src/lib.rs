#![allow(clippy::module_inception)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(missing_docs)]
#![deny(warnings)]
#![doc = include_str!("../README.md")]

pub mod attributes;

pub mod blocks;

pub mod document;
pub use document::Document;

pub mod inlines;

mod span;
pub use span::{HasSpan, Span};

pub mod strings;

#[cfg(test)]
mod tests;

mod warnings;
