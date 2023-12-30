// I understand the purpose behind this warning, but
// this module/submodule layout feels preferable in this
// circumstance.
#[allow(clippy::module_inception)]
mod document;
pub(crate) use document::TDocument;

mod header;
pub(crate) use header::THeader;
