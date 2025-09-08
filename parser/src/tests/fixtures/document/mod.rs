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

mod document;
pub(crate) use document::TDocument;

mod header;
pub(crate) use header::THeader;
