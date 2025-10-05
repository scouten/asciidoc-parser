//! Describes the top-level document structure.

mod attribute;
pub use attribute::{Attribute, InterpretedValue};

/// Author parsing and representation.
pub mod author;
pub use author::Author;

mod author_line;
pub use author_line::AuthorLine;

mod document;
pub use document::Document;

mod header;
pub use header::Header;

mod revision_line;
pub use revision_line::RevisionLine;
