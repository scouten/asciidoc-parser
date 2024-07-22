use std::slice::Iter;

use nom::IResult;

use crate::{
    blocks::{ContentModel, IsBlock, MacroBlock, SectionBlock, SimpleBlock},
    primitives::normalized_line,
    strings::CowStr,
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
///
/// This enum represents all of the block types that are understood directly by
/// this parser and also implements the [`IsBlock`] trait.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)] // TEMPORARY: review later
#[non_exhaustive]
pub enum Block<'a> {
    /// A block that’s treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block).
    Simple(SimpleBlock<'a>),

    /// A block macro is a syntax for representing non-text elements or syntax
    /// that expands into text using the provided metadata.
    Macro(MacroBlock<'a>),

    /// A section helps to partition the document into a content hierarchy.
    /// May also be a part, chapter, or special section.
    Section(SectionBlock<'a>),
}

impl<'a> Block<'a> {
    /// Parse a block of any type and return a `Block` that describes it.
    ///
    /// Consumes any blank lines before and after the block.
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        let i = i.discard_empty_lines();

        // Try to discern the block type by scanning the first line.
        let line = normalized_line(i);
        if line.t.contains("::") {
            if let Ok((rem, macro_block)) = MacroBlock::parse(i) {
                return Ok((rem, Self::Macro(macro_block)));
            }

            // A line containing `::` might be some other kind of block, so we
            // don't automatically error out on a parse failure.
        } else if line.t.starts_with('=') {
            if let Ok((rem, section_block)) = SectionBlock::parse(i) {
                return Ok((rem, Self::Section(section_block)));
            }

            // A line starting with `=` might be some other kind of block, so we
            // don't automatically error out on a parse failure.
        }

        // If no other block kind matches, we can always use SimpleBlock.
        let (rem, simple_block) = SimpleBlock::parse(i)?;
        Ok((rem, Self::Simple(simple_block)))
    }
}

impl<'a> IsBlock<'a> for Block<'a> {
    fn content_model(&self) -> ContentModel {
        match self {
            Self::Simple(_) => ContentModel::Simple,
            Self::Macro(b) => b.content_model(),
            Self::Section(_) => ContentModel::Compound,
        }
    }

    fn context(&self) -> CowStr<'a> {
        match self {
            Self::Simple(b) => b.context(),
            Self::Macro(b) => b.context(),
            Self::Section(b) => b.context(),
        }
    }

    fn nested_blocks(&'a self) -> Iter<'a, Block<'a>> {
        match self {
            Self::Simple(b) => b.nested_blocks(),
            Self::Macro(b) => b.nested_blocks(),
            Self::Section(b) => b.nested_blocks(),
        }
    }
}

impl<'a> HasSpan<'a> for Block<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        match self {
            Self::Simple(b) => b.span(),
            Self::Macro(b) => b.span(),
            Self::Section(b) => b.span(),
        }
    }
}
