//! Describes the top-level document structure.

// I understand the purpose behind this warning, but
// this module/submodule layout feels preferable in this
// circumstance.
#[allow(clippy::module_inception)]
mod document;
pub use document::Document;
