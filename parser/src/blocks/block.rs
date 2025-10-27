use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        CompoundDelimitedBlock, ContentModel, IsBlock, MediaBlock, RawDelimitedBlock, SectionBlock,
        SimpleBlock, metadata::BlockMetadata,
    },
    content::SubstitutionGroup,
    document::{Attribute, RefType},
    span::MatchedItem,
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
};

/// **Block elements** form the main structure of an AsciiDoc document, starting
/// with the document itself.
///
/// A block element (aka **block**) is a discrete, line-oriented chunk of
/// content in an AsciiDoc document. Once parsed, that chunk of content becomes
/// a block element in the parsed document model. Certain blocks may contain
/// other blocks, so we say that blocks can be nested. The converter visits each
/// block in turn, in document order, converting it to a corresponding chunk of
/// output.
///
/// This enum represents all of the block types that are understood directly by
/// this parser and also implements the [`IsBlock`] trait.
#[derive(Clone, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)] // TEMPORARY: review later
#[non_exhaustive]
pub enum Block<'src> {
    /// A block that’s treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block).
    Simple(SimpleBlock<'src>),

    /// A media block is used to represent an image, video, or audio block
    /// macro.
    Media(MediaBlock<'src>),

    /// A section helps to partition the document into a content hierarchy.
    /// May also be a part, chapter, or special section.
    Section(SectionBlock<'src>),

    /// A delimited block that contains verbatim, raw, or comment text. The
    /// content between the matching delimiters is not parsed for block
    /// syntax.
    RawDelimited(RawDelimitedBlock<'src>),

    /// A delimited block that can contain other blocks.
    CompoundDelimited(CompoundDelimitedBlock<'src>),

    /// When an attribute is defined in the document body using an attribute
    /// entry, that’s simply referred to as a document attribute.
    DocumentAttribute(Attribute<'src>),
}

impl<'src> std::fmt::Debug for Block<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Simple(block) => f.debug_tuple("Block::Simple").field(block).finish(),
            Block::Media(block) => f.debug_tuple("Block::Media").field(block).finish(),
            Block::Section(block) => f.debug_tuple("Block::Section").field(block).finish(),

            Block::RawDelimited(block) => {
                f.debug_tuple("Block::RawDelimited").field(block).finish()
            }

            Block::CompoundDelimited(block) => f
                .debug_tuple("Block::CompoundDelimited")
                .field(block)
                .finish(),

            Block::DocumentAttribute(block) => f
                .debug_tuple("Block::DocumentAttribute")
                .field(block)
                .finish(),
        }
    }
}

impl<'src> Block<'src> {
    /// Parse a block of any type and return a `Block` that describes it.
    ///
    /// Consumes any blank lines before and after the block.
    pub(crate) fn parse(
        source: Span<'src>,
        parser: &mut Parser,
    ) -> MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>> {
        // Optimization: If the first line doesn't match any of the early indications
        // for delimited blocks, titles, or attrlists, we can skip directly to treating
        // this as a simple block. That saves quite a bit of parsing time.
        let first_line = source.take_line();

        // If it does contain any of those markers, we fall through to the more costly
        // tests below which can more accurately classify the upcoming block.
        if let Some(first_char) = source.chars().next()
            && !matches!(
                first_char,
                '.' | '#' | '=' | '/' | '-' | '+' | '*' | '_' | '[' | ':'
            )
            && !first_line.item.contains("::")
            && let Some(MatchedItem {
                item: simple_block,
                after,
            }) = SimpleBlock::parse_fast(source, parser)
        {
            let mut warnings = vec![];
            let block = Self::Simple(simple_block);

            Self::register_block_id(
                block.id(),
                block.title(),
                block.span(),
                parser,
                &mut warnings,
            );

            return MatchAndWarnings {
                item: Some(MatchedItem { item: block, after }),
                warnings,
            };
        }

        // Look for document attributes first since these don't support block metadata.
        if first_line.item.starts_with(':')
            && (first_line.item.ends_with(':') || first_line.item.contains(": "))
            && let Some(attr) = Attribute::parse(source, parser)
        {
            let mut warnings: Vec<Warning<'src>> = vec![];
            parser.set_attribute_from_body(&attr.item, &mut warnings);

            return MatchAndWarnings {
                item: Some(MatchedItem {
                    item: Self::DocumentAttribute(attr.item),
                    after: attr.after,
                }),
                warnings,
            };
        }

        // Optimization not possible; start by looking for block metadata (title,
        // attrlist, etc.).
        let MatchAndWarnings {
            item: mut metadata,
            mut warnings,
        } = BlockMetadata::parse(source, parser);

        if let Some(mut rdb_maw) = RawDelimitedBlock::parse(&metadata, parser)
            && let Some(rdb) = rdb_maw.item
        {
            if !rdb_maw.warnings.is_empty() {
                warnings.append(&mut rdb_maw.warnings);
            }

            let block = Self::RawDelimited(rdb.item);

            Self::register_block_id(
                block.id(),
                block.title(),
                block.span(),
                parser,
                &mut warnings,
            );

            return MatchAndWarnings {
                item: Some(MatchedItem {
                    item: block,
                    after: rdb.after,
                }),
                warnings,
            };
        }

        if let Some(mut cdb_maw) = CompoundDelimitedBlock::parse(&metadata, parser)
            && let Some(cdb) = cdb_maw.item
        {
            if !cdb_maw.warnings.is_empty() {
                warnings.append(&mut cdb_maw.warnings);
            }

            let block = Self::CompoundDelimited(cdb.item);

            Self::register_block_id(
                block.id(),
                block.title(),
                block.span(),
                parser,
                &mut warnings,
            );

            return MatchAndWarnings {
                item: Some(MatchedItem {
                    item: block,
                    after: cdb.after,
                }),
                warnings,
            };
        }

        // Try to discern the block type by scanning the first line.
        let line = metadata.block_start.take_normalized_line();

        if line.item.starts_with("image::")
            || line.item.starts_with("video::")
            || line.item.starts_with("video::")
        {
            let mut media_block_maw = MediaBlock::parse(&metadata, parser);

            if let Some(media_block) = media_block_maw.item {
                // Only propagate warnings from media block parsing if we think this
                // *is* a media block. Otherwise, there would likely be too many false
                // positives.
                if !media_block_maw.warnings.is_empty() {
                    warnings.append(&mut media_block_maw.warnings);
                }

                let block = Self::Media(media_block.item);

                Self::register_block_id(
                    block.id(),
                    block.title(),
                    block.span(),
                    parser,
                    &mut warnings,
                );

                return MatchAndWarnings {
                    item: Some(MatchedItem {
                        item: block,
                        after: media_block.after,
                    }),
                    warnings,
                };
            }

            // This might be some other kind of block, so we don't automatically
            // error out on a parse failure.
        }

        if (line.item.starts_with('=') || line.item.starts_with('#'))
            && let Some(mi_section_block) = SectionBlock::parse(&metadata, parser, &mut warnings)
        {
            // A line starting with `=` or `#` might be some other kind of block, so we
            // continue quietly if `SectionBlock` parser rejects this block.

            return MatchAndWarnings {
                item: Some(MatchedItem {
                    item: Self::Section(mi_section_block.item),
                    after: mi_section_block.after,
                }),
                warnings,
            };
        }

        // First, let's look for a fun edge case. Perhaps the text contains block
        // metadata but no block immediately following. If we're not careful, we could
        // spin in a loop (for example, `parse_blocks_until`) thinking there will be
        // another block, but there isn't.

        // The following check disables that spin loop.
        let simple_block_mi = SimpleBlock::parse(&metadata, parser);

        if simple_block_mi.is_none() && !metadata.is_empty() {
            // We have a metadata with no block. Treat it as a simple block but issue a
            // warning.

            warnings.push(Warning {
                source: metadata.source,
                warning: WarningType::MissingBlockAfterTitleOrAttributeList,
            });

            // Remove the metadata content so that SimpleBlock will read the title/attrlist
            // line(s) as regular content.
            metadata.title_source = None;
            metadata.title = None;
            metadata.anchor = None;
            metadata.attrlist = None;
            metadata.block_start = metadata.source;
        }

        // If no other block kind matches, we can always use SimpleBlock.
        let mut result = MatchAndWarnings {
            item: SimpleBlock::parse(&metadata, parser).map(|mi| MatchedItem {
                item: Self::Simple(mi.item),
                after: mi.after,
            }),
            warnings,
        };

        if let Some(ref matched_item) = result.item {
            Self::register_block_id(
                matched_item.item.id(),
                matched_item.item.title(),
                matched_item.item.span(),
                parser,
                &mut result.warnings,
            );
        }

        result
    }

    /// Register a block's ID with the catalog if the block has an ID.
    ///
    /// This should be called for all block types except `SectionBlock`,
    /// which handles its own catalog registration.
    fn register_block_id(
        id: Option<&str>,
        title: Option<&str>,
        span: Span<'src>,
        parser: &mut Parser,
        warnings: &mut Vec<Warning<'src>>,
    ) {
        if let Some(id) = id
            && let Some(catalog) = parser.catalog_mut()
            && let Err(_duplicate_error) = catalog.register_ref(
                id,
                title, // Use block title as reftext if available
                RefType::Anchor,
            )
        {
            // If registration fails due to duplicate ID, issue a warning.
            warnings.push(Warning {
                source: span,
                warning: WarningType::DuplicateId(id.to_string()),
            });
        }
    }
}

impl<'src> IsBlock<'src> for Block<'src> {
    fn content_model(&self) -> ContentModel {
        match self {
            Self::Simple(_) => ContentModel::Simple,
            Self::Media(b) => b.content_model(),
            Self::Section(_) => ContentModel::Compound,
            Self::RawDelimited(b) => b.content_model(),
            Self::CompoundDelimited(b) => b.content_model(),
            Self::DocumentAttribute(b) => b.content_model(),
        }
    }

    fn raw_context(&self) -> CowStr<'src> {
        match self {
            Self::Simple(b) => b.raw_context(),
            Self::Media(b) => b.raw_context(),
            Self::Section(b) => b.raw_context(),
            Self::RawDelimited(b) => b.raw_context(),
            Self::CompoundDelimited(b) => b.raw_context(),
            Self::DocumentAttribute(b) => b.raw_context(),
        }
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        match self {
            Self::Simple(b) => b.nested_blocks(),
            Self::Media(b) => b.nested_blocks(),
            Self::Section(b) => b.nested_blocks(),
            Self::RawDelimited(b) => b.nested_blocks(),
            Self::CompoundDelimited(b) => b.nested_blocks(),
            Self::DocumentAttribute(b) => b.nested_blocks(),
        }
    }

    fn title_source(&'src self) -> Option<Span<'src>> {
        match self {
            Self::Simple(b) => b.title_source(),
            Self::Media(b) => b.title_source(),
            Self::Section(b) => b.title_source(),
            Self::RawDelimited(b) => b.title_source(),
            Self::CompoundDelimited(b) => b.title_source(),
            Self::DocumentAttribute(b) => b.title_source(),
        }
    }

    fn title(&self) -> Option<&str> {
        match self {
            Self::Simple(b) => b.title(),
            Self::Media(b) => b.title(),
            Self::Section(b) => b.title(),
            Self::RawDelimited(b) => b.title(),
            Self::CompoundDelimited(b) => b.title(),
            Self::DocumentAttribute(b) => b.title(),
        }
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        match self {
            Self::Simple(b) => b.anchor(),
            Self::Media(b) => b.anchor(),
            Self::Section(b) => b.anchor(),
            Self::RawDelimited(b) => b.anchor(),
            Self::CompoundDelimited(b) => b.anchor(),
            Self::DocumentAttribute(b) => b.anchor(),
        }
    }

    fn anchor_reftext(&'src self) -> Option<Span<'src>> {
        match self {
            Self::Simple(b) => b.anchor_reftext(),
            Self::Media(b) => b.anchor_reftext(),
            Self::Section(b) => b.anchor_reftext(),
            Self::RawDelimited(b) => b.anchor_reftext(),
            Self::CompoundDelimited(b) => b.anchor_reftext(),
            Self::DocumentAttribute(b) => b.anchor_reftext(),
        }
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        match self {
            Self::Simple(b) => b.attrlist(),
            Self::Media(b) => b.attrlist(),
            Self::Section(b) => b.attrlist(),
            Self::RawDelimited(b) => b.attrlist(),
            Self::CompoundDelimited(b) => b.attrlist(),
            Self::DocumentAttribute(b) => b.attrlist(),
        }
    }

    fn substitution_group(&self) -> SubstitutionGroup {
        match self {
            Self::Simple(b) => b.substitution_group(),
            Self::Media(b) => b.substitution_group(),
            Self::Section(b) => b.substitution_group(),
            Self::RawDelimited(b) => b.substitution_group(),
            Self::CompoundDelimited(b) => b.substitution_group(),
            Self::DocumentAttribute(b) => b.substitution_group(),
        }
    }
}

impl<'src> HasSpan<'src> for Block<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Self::Simple(b) => b.span(),
            Self::Media(b) => b.span(),
            Self::Section(b) => b.span(),
            Self::RawDelimited(b) => b.span(),
            Self::CompoundDelimited(b) => b.span(),
            Self::DocumentAttribute(b) => b.span(),
        }
    }
}
