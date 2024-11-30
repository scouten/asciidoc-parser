use std::slice::Iter;

use crate::{
    attributes::Attrlist,
    blocks::{
        preamble::Preamble, CompoundDelimitedBlock, ContentModel, IsBlock, MacroBlock,
        RawDelimitedBlock, SectionBlock, SimpleBlock,
    },
    span::MatchedItem,
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
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

    /// A delimited block that contains verbatim, raw, or comment text. The
    /// content between the matching delimiters is not parsed for block
    /// syntax.
    RawDelimited(RawDelimitedBlock<'src>),

    /// A delimited block that can contain other blocks.
    CompoundDelimited(CompoundDelimitedBlock<'src>),
}

impl<'src> Block<'src> {
    /// Parse a block of any type and return a `Block` that describes it.
    ///
    /// Consumes any blank lines before and after the block.
    pub(crate) fn parse(
        source: Span<'src>,
    ) -> MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>> {
        // Optimization: If the first line doesn't match any of the early indications
        // for delimited blocks, titles, or attrlists, we can skip directly to treating
        // this as a simple block. That saves quite a bit of parsing time.

        // If it does contain any of those markers, we fall through to the more costly
        // tests below which can more accurately classify the upcoming block.
        if let Some(first_char) = source.chars().next() {
            if !matches!(
                first_char,
                '.' | '#' | '=' | '/' | '-' | '+' | '*' | '_' | '['
            ) {
                let first_line = source.take_line();
                if !first_line.item.contains("::") {
                    if let Some(MatchedItem {
                        item: simple_block,
                        after,
                    }) = SimpleBlock::parse_fast(source)
                    {
                        return MatchAndWarnings {
                            item: Some(MatchedItem {
                                item: Self::Simple(simple_block),
                                after,
                            }),
                            warnings: vec![],
                        };
                    }
                }
            }
        }

        // Optimization not possible; start by looking for a preamble (title and/or
        // attrlist).
        let MatchAndWarnings {
            item: mut preamble,
            mut warnings,
        } = Preamble::parse(source);

        if let Some(mut rdb_maw) = RawDelimitedBlock::parse(&preamble) {
            // If we found an initial delimiter without its matching
            // closing delimiter, we will issue an unmatched delimiter warning
            // and attempt to parse this as some other kind of block.
            if !rdb_maw.warnings.is_empty() {
                warnings.append(&mut rdb_maw.warnings);
            }

            if let Some(rdb) = rdb_maw.item {
                return MatchAndWarnings {
                    item: Some(MatchedItem {
                        item: Self::RawDelimited(rdb.item),
                        after: rdb.after,
                    }),
                    warnings,
                };
            }
        }

        if let Some(mut cdb_maw) = CompoundDelimitedBlock::parse(&preamble) {
            // If we found an initial delimiter without its matching
            // closing delimiter, we will issue an unmatched delimiter warning
            // and attempt to parse this as some other kind of block.
            if !cdb_maw.warnings.is_empty() {
                warnings.append(&mut cdb_maw.warnings);
            }

            if let Some(cdb) = cdb_maw.item {
                return MatchAndWarnings {
                    item: Some(MatchedItem {
                        item: Self::CompoundDelimited(cdb.item),
                        after: cdb.after,
                    }),
                    warnings,
                };
            }
        }

        // Try to discern the block type by scanning the first line.
        let line = preamble.block_start.take_normalized_line();

        if line.item.contains("::") {
            let mut macro_block_maw = MacroBlock::parse(&preamble);

            if let Some(macro_block) = macro_block_maw.item {
                // Only propagate warnings from macro block parsing if we think this
                // *is* a macro block. Otherwise, there would likely be too many false
                // positives.
                if !macro_block_maw.warnings.is_empty() {
                    warnings.append(&mut macro_block_maw.warnings);
                }

                return MatchAndWarnings {
                    item: Some(MatchedItem {
                        item: Self::Macro(macro_block.item),
                        after: macro_block.after,
                    }),
                    warnings,
                };
            }

            // A line containing `::` might be some other kind of block, so we
            // don't automatically error out on a parse failure.
        }

        if line.item.starts_with('=') {
            if let Some(mut maw_section_block) = SectionBlock::parse(&preamble) {
                if !maw_section_block.warnings.is_empty() {
                    warnings.append(&mut maw_section_block.warnings);
                }

                return MatchAndWarnings {
                    item: Some(MatchedItem {
                        item: Self::Section(maw_section_block.item.item),
                        after: maw_section_block.item.after,
                    }),
                    warnings,
                };
            }

            // A line starting with `=` might be some other kind of block, so we
            // don't automatically error out on a parse failure.
        }

        // First, let's look for a fun edge case. Perhaps the text contains a preamble
        // but no block immediately following. If we're not careful, we could spin in a
        // loop (for example, `parse_blocks_until`) thinking there will be another
        // block, but there isn't.

        // The following check disables that spin loop.
        let simple_block_mi = SimpleBlock::parse(&preamble);

        if simple_block_mi.is_none() && !preamble.is_empty() {
            // We have a preamble with no block. Treat it as a simple block but issue a
            // warning.

            warnings.push(Warning {
                source: preamble.source,
                warning: WarningType::MissingBlockAfterTitleOrAttributeList,
            });

            // Remove the preamble content so that SimpleBlock will read the title/attrlist
            // line(s) as regular content.
            preamble.title = None;
            preamble.attrlist = None;
            preamble.block_start = preamble.source;
        }

        // If no other block kind matches, we can always use SimpleBlock.
        MatchAndWarnings {
            item: SimpleBlock::parse(&preamble).map(|mi| MatchedItem {
                item: Self::Simple(mi.item),
                after: mi.after,
            }),
            warnings,
        }
    }
}

impl<'src> IsBlock<'src> for Block<'src> {
    fn content_model(&self) -> ContentModel {
        match self {
            Self::Simple(_) => ContentModel::Simple,
            Self::Macro(b) => b.content_model(),
            Self::Section(_) => ContentModel::Compound,
            Self::RawDelimited(b) => b.content_model(),
            Self::CompoundDelimited(b) => b.content_model(),
        }
    }

    fn context(&self) -> CowStr<'src> {
        match self {
            Self::Simple(b) => b.context(),
            Self::Macro(b) => b.context(),
            Self::Section(b) => b.context(),
            Self::RawDelimited(b) => b.context(),
            Self::CompoundDelimited(b) => b.context(),
        }
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        match self {
            Self::Simple(b) => b.nested_blocks(),
            Self::Macro(b) => b.nested_blocks(),
            Self::Section(b) => b.nested_blocks(),
            Self::RawDelimited(b) => b.nested_blocks(),
            Self::CompoundDelimited(b) => b.nested_blocks(),
        }
    }

    fn title(&'src self) -> Option<Span<'src>> {
        match self {
            Self::Simple(b) => b.title(),
            Self::Macro(b) => b.title(),
            Self::Section(b) => b.title(),
            Self::RawDelimited(b) => b.title(),
            Self::CompoundDelimited(b) => b.title(),
        }
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        match self {
            Self::Simple(b) => b.attrlist(),
            Self::Macro(b) => b.attrlist(),
            Self::Section(b) => b.attrlist(),
            Self::RawDelimited(b) => b.attrlist(),
            Self::CompoundDelimited(b) => b.attrlist(),
        }
    }
}

impl<'src> HasSpan<'src> for Block<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        match self {
            Self::Simple(b) => b.span(),
            Self::Macro(b) => b.span(),
            Self::Section(b) => b.span(),
            Self::RawDelimited(b) => b.span(),
            Self::CompoundDelimited(b) => b.span(),
        }
    }
}
