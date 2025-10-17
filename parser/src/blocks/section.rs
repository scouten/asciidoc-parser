use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        Block, ContentModel, IsBlock, metadata::BlockMetadata, parse_utils::parse_blocks_until,
    },
    content::{Content, SubstitutionGroup},
    span::MatchedItem,
    strings::CowStr,
    warnings::{Warning, WarningType},
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
    section_title: Content<'src>,
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
        warnings: &mut Vec<Warning<'src>>,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.discard_empty_lines();
        let level_and_title = parse_title_line(source, 0, warnings)?;

        let mut maw_blocks = parse_blocks_until(
            level_and_title.after,
            |i| peer_or_ancestor_section(*i, level_and_title.item.0, warnings),
            parser,
        );

        let blocks = maw_blocks.item;
        let source = metadata.source.trim_remainder(blocks.after);

        let mut section_title = Content::from(level_and_title.item.1);
        SubstitutionGroup::Title.apply(&mut section_title, parser, metadata.attrlist.as_ref());

        warnings.append(&mut maw_blocks.warnings);

        Some(MatchedItem {
            item: Self {
                level: level_and_title.item.0,
                section_title,
                blocks: blocks.item,
                source: source.trim_trailing_whitespace(),
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                attrlist: metadata.attrlist.clone(),
            },
            after: blocks.after,
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

    /// Return a [`Span`] containing the section title source.
    pub fn section_title_source(&self) -> Span<'src> {
        self.section_title.original()
    }

    /// Return the processed section title after substitutions have been
    /// applied.
    pub fn section_title(&'src self) -> &'src str {
        self.section_title.rendered()
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

fn parse_title_line<'src>(
    source: Span<'src>,
    parent_level: usize,
    warnings: &mut Vec<Warning<'src>>,
) -> Option<MatchedItem<'src, (usize, Span<'src>)>> {
    let mi = source.take_non_empty_line()?;
    let mut line = mi.item;

    // TO DO: Also support Markdown-style `#` markers.
    // TO DO: Disallow empty title.

    let mut count = 0;

    while let Some(mi) = line.take_prefix("=") {
        count += 1;
        line = mi.after;
    }

    if count == 1 {
        warnings.push(Warning {
            source: source.take_normalized_line().item,
            warning: WarningType::Level0SectionHeadingNotSupported,
        });

        return None;
    }

    if count > 6 {
        warnings.push(Warning {
            source: source.take_normalized_line().item,
            warning: WarningType::SectionHeadingLevelExceedsMaximum(count - 1),
        });

        return None;
    }

    let title = line.take_required_whitespace()?;

    if count > parent_level + 2 {
        warnings.push(Warning {
            source: source.take_normalized_line().item,
            warning: WarningType::SectionHeadingLevelSkipped(parent_level + 1, count - 1),
        });
    }

    Some(MatchedItem {
        item: (count - 1, title.after),
        after: mi.after,
    })
}

fn peer_or_ancestor_section<'src>(
    source: Span<'src>,
    level: usize,
    warnings: &mut Vec<Warning<'src>>,
) -> bool {
    if let Some(mi) = parse_title_line(source, level, warnings) {
        mi.item.0 <= level
    } else {
        false
    }
}
