use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        Block, ContentModel, IsBlock, metadata::BlockMetadata, parse_utils::parse_blocks_until,
    },
    span::MatchedItem,
    strings::CowStr,
    warnings::MatchAndWarnings,
};

/// Sections partition the document into a content hierarchy. A section is an
/// implicit enclosure. Each section begins with a title and ends at the next
/// sibling section, ancestor section, or end of document. Nested section levels
/// must be sequential.
///
/// **WARNING:** This is a very preliminary implementation. There are many **TO
/// DO** items in this code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SectionBlock<'src> {
    level: usize,
    section_title: Span<'src>,
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
    title_source: Option<Span<'src>>,
    title: Option<String>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> SectionBlock<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
    ) -> Option<MatchAndWarnings<'src, MatchedItem<'src, Self>>> {
        let source = metadata.block_start.discard_empty_lines();
        let level = parse_title_line(source)?;

        let maw_blocks = parse_blocks_until(
            level.after,
            |i| peer_or_ancestor_section(*i, level.item.0),
            parser,
        );

        let blocks = maw_blocks.item;
        let source = metadata.source.trim_remainder(blocks.after);

        Some(MatchAndWarnings {
            item: MatchedItem {
                item: Self {
                    level: level.item.0,
                    section_title: level.item.1,
                    blocks: blocks.item,
                    source: source.trim_trailing_whitespace(),
                    title_source: metadata.title_source,
                    title: metadata.title.clone(),
                    anchor: metadata.anchor,
                    attrlist: metadata.attrlist.clone(),
                },
                after: blocks.after,
            },
            warnings: maw_blocks.warnings,
        })
    }

    /// Return the section's level.
    ///
    /// The section title must be prefixed with a section marker, which
    /// indicates the section level. The number of equal signs in the marker
    /// represents the section level using a 0-based index (e.g., two equal
    /// signs represents level 1). A section marker can range from two to six
    /// equal signs and must be followed by a space.
    ///
    /// This function will return an integer between 1 and 5.
    pub fn level(&self) -> usize {
        self.level
    }

    /// Return a [`Span`] containing the section title.
    pub fn section_title(&'src self) -> &'src Span<'src> {
        &self.section_title
    }
}

impl<'src> IsBlock<'src> for SectionBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn raw_context(&self) -> CowStr<'src> {
        "section".into()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.blocks.iter()
    }

    fn title_source(&'src self) -> Option<Span<'src>> {
        self.title_source
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        self.anchor
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for SectionBlock<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

fn parse_title_line(source: Span<'_>) -> Option<MatchedItem<'_, (usize, Span<'_>)>> {
    let mi = source.take_non_empty_line()?;
    let mut line = mi.item;

    // TO DO: Also support Markdown-style `#` markers.
    // TO DO: Enforce maximum of 6 `=` or `#` markers.
    // TO DO: Disallow empty title.

    let mut count = 0;

    while let Some(mi) = line.take_prefix("=") {
        count += 1;
        line = mi.after;
    }

    let title = line.take_required_whitespace()?;

    Some(MatchedItem {
        item: (count - 1, title.after),
        after: mi.after,
    })
}

fn peer_or_ancestor_section(source: Span<'_>, level: usize) -> bool {
    if let Some(mi) = parse_title_line(source) {
        mi.item.0 <= level
    } else {
        false
    }
}
