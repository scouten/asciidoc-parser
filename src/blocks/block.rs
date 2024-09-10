use std::slice::Iter;

use crate::{
    blocks::{ContentModel, IsBlock, MacroBlock, SectionBlock, SimpleBlock},
    span::MatchedItem,
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
pub enum Block<'src> {
    /// A block that’s treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block).
    Simple(SimpleBlock<'src>),

    /// A block macro is a syntax for representing non-text elements or syntax
    /// that expands into text using the provided metadata.
    Macro(MacroBlock<'src>),

    /// A section helps to partition the document into a content hierarchy.
    /// May also be a part, chapter, or special section.
    Section(SectionBlock<'src>),
}

impl<'src> Block<'src> {
    /// Parse a block of any type and return a `Block` that describes it.
    ///
    /// Consumes any blank lines before and after the block.
    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let source = source.discard_empty_lines();

        // Try to discern the block type by scanning the first line.
        let line = source.take_normalized_line();
        if line.item.contains("::") {
            if let Some(macro_block) = MacroBlock::parse(source) {
                return Some(MatchedItem {
                    item: Self::Macro(macro_block.item),
                    after: macro_block.after,
                });
            }

            // A line containing `::` might be some other kind of block, so we
            // don't automatically error out on a parse failure.
        } else if line.item.starts_with('=') {
            if let Some(section_block) = SectionBlock::parse(source) {
                return Some(MatchedItem {
                    item: Self::Section(section_block.item),
                    after: section_block.after,
                });
            }

            // A line starting with `=` might be some other kind of block, so we
            // don't automatically error out on a parse failure.
        }

        // If no other block kind matches, we can always use SimpleBlock.
        SimpleBlock::parse(source).map(|mi| MatchedItem {
            item: Self::Simple(mi.item),
            after: mi.after,
        })
    }
}

impl<'src> IsBlock<'src> for Block<'src> {
    fn content_model(&self) -> ContentModel {
        match self {
            Self::Simple(_) => ContentModel::Simple,
            Self::Macro(b) => b.content_model(),
            Self::Section(_) => ContentModel::Compound,
        }
    }

    fn context(&self) -> CowStr<'src> {
        match self {
            Self::Simple(b) => b.context(),
            Self::Macro(b) => b.context(),
            Self::Section(b) => b.context(),
        }
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        match self {
            Self::Simple(b) => b.nested_blocks(),
            Self::Macro(b) => b.nested_blocks(),
            Self::Section(b) => b.nested_blocks(),
        }
    }
}

impl<'src> HasSpan<'src> for Block<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        match self {
            Self::Simple(b) => b.span(),
            Self::Macro(b) => b.span(),
            Self::Section(b) => b.span(),
        }
    }
}
