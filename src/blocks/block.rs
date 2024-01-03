use nom::IResult;

use crate::{
    blocks::{MacroBlock, SimpleBlock},
    primitives::{consume_empty_lines, normalized_line},
    HasSpan, Span,
};

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
#[non_exhaustive]
pub enum Block<'a> {
    /// A block that’s treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block).
    Simple(SimpleBlock<'a>),

    /// A block macro is a syntax for representing non-text elements or syntax
    /// that expands into text using the provided metadata.
    Macro(MacroBlock<'a>),
}

impl<'a> Block<'a> {
    /// Parse a block of any type and return a `Block` that describes it.
    ///
    /// Consumes any blank lines before and after the block.
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        let i = consume_empty_lines(i);

        // Try to discern the block type by scanning the first line.
        let (_, line) = normalized_line(i)?;
        if line.contains("::") {
            if let Ok((rem, macro_block)) = MacroBlock::parse(i) {
                return Ok((rem, Self::Macro(macro_block)));
            }

            // A line containing `::` might be some other kind of block, so we
            // don't automatically error out on a parse failure.
        }

        // If no other block kind matches, we can always use SimpleBlock.
        let (rem, simple_block) = SimpleBlock::parse(i)?;
        Ok((rem, Self::Simple(simple_block)))
    }
}

impl<'a> HasSpan<'a> for Block<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        match self {
            Self::Simple(b) => b.span(),
            Self::Macro(b) => b.span(),
        }
    }
}
