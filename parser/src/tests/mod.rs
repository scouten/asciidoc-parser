// Tests are grouped under this module so as to avoid
// having the test code itself included in coverage numbers.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

mod asciidoc_lang;
mod asciidoctor_rb;
mod blocks;
mod content;
mod document;
pub(crate) mod fixtures;
mod internal;
mod parser;
pub(crate) mod prelude;
pub(crate) mod sdd;
mod span;
mod strings;
mod warnings;
