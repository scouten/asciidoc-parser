pub(crate) mod attributes;
pub(crate) mod blocks;
pub(crate) mod content;
pub(crate) mod document;

pub(crate) mod inline_file_handler;
#[allow(unused)] // TEMPORARY while building
pub(crate) use inline_file_handler::InlineFileHandler;

mod span;
pub(crate) use span::Span;

pub(crate) mod warnings;
