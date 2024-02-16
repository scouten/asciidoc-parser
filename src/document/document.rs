//! Describes the top-level document structure.

use std::slice::Iter;

use nom::IResult;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    document::Header,
    primitives::consume_empty_lines,
    Error, HasSpan, Span,
};

/// A document represents the top-level block element in AsciiDoc. It consists
/// of an optional document header and either a) one or more sections preceded
/// by an optional preamble or b) a sequence of top-level blocks only.
///
/// The document can be configured using a document header. The header is not a
/// block itself, but contributes metadata to the document, such as the document
/// title and document attributes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Document<'a> {
    header: Option<Header<'a>>,
    blocks: Vec<Block<'a>>,
    source: Span<'a>,
}

impl<'a> Document<'a> {
    /// Parse a UTF-8 string as an AsciiDoc document.
    ///
    /// Note that the document references the underlying source string and
    /// necessarily has the same lifetime as the source.
    ///
    /// **IMPORTANT:** The AsciiDoc language documentation states that UTF-16
    /// encoding is allowed if a byte-order-mark (BOM) is present at the
    /// start of a file. This format is not directly supported by the
    /// `asciidoc-parser` crate. Any UTF-16 content must be re-encoded as
    /// UTF-8 prior to parsing.
    pub fn parse(source: &'a str) -> Result<Self, Error> {
        // TO DO: Add option for best-guess parsing?

        let source = Span::new(source, true);
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

    /// Return the document header if there is one.
    pub fn header(&'a self) -> Option<&'a Header<'a>> {
        self.header.as_ref()
    }
}

impl<'a> IsBlock<'a> for Document<'a> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn nested_blocks(&'a self) -> Iter<'a, Block<'a>> {
        self.blocks.iter()
    }
}

impl<'a> HasSpan<'a> for Document<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

fn parse_blocks<'a>(mut i: Span<'a>) -> IResult<Span, Vec<Block<'a>>> {
    // TO DO: See if we can share code with Section's parse_blocks fn.
    let mut blocks: Vec<Block<'a>> = vec![];
    i = consume_empty_lines(i);

    while !i.data().is_empty() {
        let (i2, block) = Block::parse(i)?;
        i = i2;
        blocks.push(block);
    }

    Ok((i, blocks))
}
