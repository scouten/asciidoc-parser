use nom::IResult;

use super::SimpleBlock;
use crate::{primitives::consume_empty_lines, Span};

/// Block elements form the main structure of an AsciiDoc document, starting
/// with the document itself.
///
/// A block element (aka block) is a discrete, line-oriented chunk of content in
/// an AsciiDoc document. Once parsed, that chunk of content becomes a block
/// element in the parsed document model. Certain blocks may contain other
/// blocks, so we say that blocks can be nested. The converter visits each block
/// in turn, in document order, converting it to a corresponding chunk of
/// output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block<'a> {
    /// A block that’s treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block).
    Simple(SimpleBlock<'a>),
}

impl<'a> Block<'a> {
    #[allow(dead_code)]
    /// Parse a block of any type and return a `Block` that describes it.
    ///
    /// Consumes any blank lines before and after the block.
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        let i = consume_empty_lines(i);

        // TEMPORARY: So far, we only know SimpleBlock.
        // Later we'll start to try to discern other block types.
        let (rem, simple_block) = SimpleBlock::parse(i)?;
        Ok((rem, Self::Simple(simple_block)))
    }
}
