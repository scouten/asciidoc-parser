use std::slice::Iter;

use nom::IResult;

use crate::{blocks::Block, primitives::consume_empty_lines, Error, Span};

/// A document represents the top-level block element in AsciiDoc. It consists
/// of an optional document header and either a) one or more sections preceded
/// by an optional preamble or b) a sequence of top-level blocks only.
///
/// The document can be configured using a document header. The header is not a
/// block itself, but contributes metadata to the document, such as the document
/// title and document attributes.
#[allow(dead_code)] // TEMPORARY while building
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Document<'a> {
    blocks: Vec<Block<'a>>,
    source: Span<'a>,
}

impl<'a> Document<'a> {
    /// Parse a UTF-8 string as an AsciiDoc document.
    ///
    /// Note that the document references the underlying source string and
    /// necessarily has the same lifetime as the source.
    pub fn parse(source: &'a str) -> Result<Self, Error> {
        let source = Span::new(source, true);
        let i = source;

        // TO DO: Look for document header.
        // TO DO: Add option for best-guess parsing?

        let (_rem, blocks) = parse_blocks(i)?;

        // let blocks: Vec<Block<'a>> = vec![]; // TEMPORARY
        Ok(Self { source, blocks })
    }

    /// Return a [`Span`] describing the entire document as parsed.
    pub fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }

    /// Return an iterator over the blocks in this document.
    pub fn blocks(&'a self) -> Iter<'a, Block<'a>> {
        self.blocks.iter()
    }
}

fn parse_blocks<'a>(mut i: Span<'a>) -> IResult<Span, Vec<Block<'a>>> {
    let mut blocks: Vec<Block<'a>> = vec![];
    i = consume_empty_lines(i);

    while !i.data().is_empty() {
        // TO DO: Handle other kinds of blocks.
        let (i2, block) = Block::parse(i)?;
        i = i2;
        blocks.push(block);
    }

    Ok((i, blocks))
}
