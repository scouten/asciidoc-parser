use std::{fmt::Debug, slice::Iter};

use crate::{blocks::Block, strings::CowStr, HasSpan, Span};

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
/// This trait implements many of the same core methods as the [Block] enum but
/// provides a mechanism for third-party code to extend the behavior of blocks.
pub trait IsBlock<'src>: HasSpan<'src> + Clone + Debug + Eq + PartialEq {
    /// Returns the [`ContentModel`] for this block.
    fn content_model(&self) -> ContentModel;

    /// Returns the context for this block.
    ///
    /// A block’s context is also sometimes referred to as a name, such as an
    /// example block, a sidebar block, an admonition block, or a section.
    ///
    /// Every block has a context. The context is often implied by the syntax,
    /// but can be declared explicitly in certain cases. The context is what
    /// distinguishes one kind of block from another. You can think of the
    /// context as the block’s type.
    ///
    /// For that reason, the context is not defined as an enumeration, but
    /// rather as a string type that is optimized for the case where predefined
    /// constants are viable.
    fn context(&self) -> CowStr<'src>;

    /// Returns an iterator over the nested blocks contained within
    /// this block.
    ///
    /// Many block types do not have nested blocks so the default implementation
    /// returns an empty iterator.
    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        const NO_BLOCKS: &[Block<'static>] = &[];
        NO_BLOCKS.iter()
    }

    /// Returns the title for this block, if present.
    fn title(&'src self) -> Option<Span<'src>>;
}

/// The content model of a block determines what kind of content the block can
/// have (if any) and how that content is processed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
