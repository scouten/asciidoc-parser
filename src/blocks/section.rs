use std::slice::Iter;

use nom::{bytes::complete::tag, character::complete::space1, multi::many1_count, IResult};

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    primitives::{consume_empty_lines, non_empty_line},
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
    #[allow(dead_code)]
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let source = consume_empty_lines(source);

        let (rem, (level, title)) = parse_title_line(source)?;

        let (rem, blocks) = parse_blocks(rem)?;

        Ok((
            rem,
            Self {
                level,
                title,
                blocks,
                source,
            },
        ))
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

    fn nested_blocks(&'a self) -> Iter<'a, Block<'a>> {
        self.blocks.iter()
    }
}

impl<'a> HasSpan<'a> for SectionBlock<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

fn parse_title_line<'a>(source: Span<'a>) -> IResult<Span<'a>, (usize, Span<'a>)> {
    let (rem, line) = non_empty_line(source)?;

    // TO DO: Also support Markdown-style `#` markers.
    // TO DO: Enforce maximum of 6 `=` or `#` markers.
    // TO DO: Disallow empty title.
    let (space_title, count) = many1_count(tag("="))(line)?;
    let (title, _) = space1(space_title)?;

    Ok((rem, (count - 1, title)))
}

fn parse_blocks<'a>(mut i: Span<'a>) -> IResult<Span, Vec<Block<'a>>> {
    // TO DO: See if we can share code with Document's parse_blocks fn.
    // TO DO: Stop when we encounter a sibling or ancestor section marker.

    let mut blocks: Vec<Block<'a>> = vec![];
    i = consume_empty_lines(i);

    while !i.data().is_empty() {
        let (i2, block) = Block::parse(i)?;
        i = i2;
        blocks.push(block);
    }

    Ok((i, blocks))
}
