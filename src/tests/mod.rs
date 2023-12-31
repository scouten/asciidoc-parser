// Tests are grouped under this module so as to avoid
// having the test code itself included in coverage numbers.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

mod asciidoc_lang;
mod blocks;
mod document;
mod error;
pub(crate) mod fixtures;
mod primitives;
mod strings;
