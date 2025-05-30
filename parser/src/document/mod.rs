//! Describes the top-level document structure.

mod attribute;
pub use attribute::{Attribute, InterpretedValue, RawAttributeValue};

mod document;
pub use document::Document;

mod header;
pub use header::Header;
