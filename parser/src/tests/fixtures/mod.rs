pub(crate) mod attributes;
pub(crate) mod blocks;
pub(crate) mod content;
pub(crate) mod document;
pub(crate) mod inline_file_handler;

mod span;
pub(crate) use span::Span;

pub(crate) mod warnings;
