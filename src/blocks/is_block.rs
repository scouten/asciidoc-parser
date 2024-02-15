use std::fmt::Debug;

use crate::HasSpan;

/// Block elements form the main structure of an AsciiDoc document, starting
/// with the document itself.
///
/// A block element (aka block) is a discrete, line-oriented chunk of content in
/// an AsciiDoc document. Once parsed, that chunk of content becomes a block
/// element in the parsed document model. Certain blocks may contain other
/// blocks, so we say that blocks can be nested. The converter visits each block
/// in turn, in document order, converting it to a corresponding chunk of
/// output.
///
/// This trait implements many of the same core methods as the
/// [Block](crate::blocks::Block) enum but provides a mechanism for third-party
/// code to extend the behavior of blocks.
pub trait IsBlock<'a>: HasSpan<'a> + Clone + Debug + Eq + PartialEq {
    /// Returns the [ContentModel] for this block.
    fn content_model(&self) -> ContentModel;
}

/// The content model of a block determines what kind of content the block can
/// have (if any) and how that content is processed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)] // TO DO: Remove once all content models are referenced.
pub enum ContentModel {
    /// A block that may only contain other blocks (e.g., a section)
    Compound,

    /// A block that's treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block)
    Simple,

    /// A block that holds verbatim text (displayed "`as is`") (and subject to
    /// verbatim substitutions) (e.g., a listing block)
    Verbatim,

    /// A block that holds unprocessed content passed directly through to the
    /// output with no substitutions applied (e.g., a passthrough block)
    Raw,

    /// Ablock that has no content (e.g., an image block)
    Empty,

    /// A special content model reserved for tables that enforces a fixed
    /// structure
    Table,
}
