use nom::{
    bytes::complete::{tag, take_until1},
    error::{Error, ErrorKind},
    Err, IResult,
};

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    primitives::{consume_empty_lines, ident, normalized_line, trim_input_for_rem},
    HasSpan, Span,
};

/// Sections partition the document into a content hierarchy. A section is an implicit enclosure. Each section begins with a title and ends at the next sibling section, ancestor section, or end of document. Nested section levels must be sequential.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SectionBlock<'a> {
    level: u8,
    title: Span<'a>,
    blocks: Vec<Block<'a>>,
    source: Span<'a>,
}

impl<'a> SectionBlock<'a> {
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let i = consume_empty_lines(source);

        let (i, header) = if i.starts_with("= ") {
            let (i, header) = Header::parse(i)?;
            (i, Some(header))
        } else {
            (i, None)
        };

        let (_rem, blocks) = parse_blocks(i)?;

        Ok(Self {
            header,
            blocks,
            source,
        })

    }

    /// Return the section's level.
    /// 
    /// The section title must be prefixed with a section marker, which indicates the section level. The number of equal signs in the marker represents the section level using a 0-based index (e.g., two equal signs represents level 1). A section marker can range from two to six equal signs and must be followed by a space.
    /// 
    /// This function will return an integer between 1 and 5.
    pub fn level(&self) -> u8 {
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
}

impl<'a> HasSpan<'a> for SectionBlock<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
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
