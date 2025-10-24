/// The structs and enums in this module are approximate mocks
/// of the correspondingly-named concepts in the public API surface.
///
/// The primary differences here are:
///
/// (1) The data members are public, which allows expected values
///     to be declared inline in unit tests.
///
/// (2) The `Debug` implementations use the name of the corresponding
///     concept in the public API surface so that debug output is
///     easier to comprehend.
mod attribute;
pub(crate) use attribute::Attribute;

mod attribute_value;
pub(crate) use attribute_value::InterpretedValue;

mod author;
pub(crate) use author::Author;

mod author_line;
pub(crate) use author_line::AuthorLine;

mod catalog;
pub(crate) use catalog::Catalog;

mod document;
pub(crate) use document::Document;

mod header;
pub(crate) use header::Header;

mod ref_entry;
pub(crate) use ref_entry::RefEntry;

mod revision_line;
pub(crate) use revision_line::RevisionLine;
