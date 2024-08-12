use std::slice::Iter;

use crate::{
    blocks::{parse_utils::parse_blocks_until, Block, ContentModel, IsBlock},
    primitives::trim_input_for_rem,
    span::ParseResult,
    strings::CowStr,
    HasSpan, Span,
};

/// Sections partition the document into a content hierarchy. A section is an
/// implicit enclosure. Each section begins with a title and ends at the next
/// sibling section, ancestor section, or end of document. Nested section levels
/// must be sequential.
///
/// WARNING: This is a very preliminary implementation. There are many TO DO
/// items in this code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SectionBlock<'a> {
    level: usize,
    title: Span<'a>,
    blocks: Vec<Block<'a>>,
    source: Span<'a>,
}

impl<'a> SectionBlock<'a> {
    pub(crate) fn parse(source: Span<'a>) -> Option<ParseResult<Self>> {
        let source = source.discard_empty_lines();
        let level = parse_title_line(source)?;
        let blocks = parse_blocks_until(level.rem, |i| peer_or_ancestor_section(*i, level.t.0))?;
        let source = trim_input_for_rem(source, blocks.rem);

        Some(ParseResult {
            t: Self {
                level: level.t.0,
                title: level.t.1,
                blocks: blocks.t,
                source,
            },
            rem: blocks.rem,
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

    /// Return a [`Span`] describing the section title.
    pub fn title(&'a self) -> &'a Span<'a> {
        &self.title
    }
}

impl<'a> IsBlock<'a> for SectionBlock<'a> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn context(&self) -> CowStr<'a> {
        "section".into()
    }

    fn nested_blocks(&'a self) -> Iter<'a, Block<'a>> {
        self.blocks.iter()
    }
}

impl<'a> HasSpan<'a> for SectionBlock<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

fn parse_title_line(source: Span<'_>) -> Option<ParseResult<(usize, Span)>> {
    let pr = source.take_non_empty_line()?;
    let mut line = pr.t;

    // TO DO: Also support Markdown-style `#` markers.
    // TO DO: Enforce maximum of 6 `=` or `#` markers.
    // TO DO: Disallow empty title.

    let mut count = 0;

    while let Some(pr) = line.take_prefix("=") {
        count += 1;
        line = pr.rem;
    }

    let title = line.take_required_whitespace()?;

    Some(ParseResult {
        t: (count - 1, title.rem),
        rem: pr.rem,
    })
}

fn peer_or_ancestor_section(i: Span<'_>, level: usize) -> bool {
    if let Some(pr) = parse_title_line(i) {
        pr.t.0 <= level
    } else {
        false
    }
}
