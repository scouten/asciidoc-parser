//! Describes the top-level document structure.

use std::slice::Iter;

use crate::{
    blocks::{parse_utils::parse_blocks_until, Block, ContentModel, IsBlock},
    document::Header,
    strings::CowStr,
    HasSpan, Span,
};

/// A document represents the top-level block element in AsciiDoc. It consists
/// of an optional document header and either a) one or more sections preceded
/// by an optional preamble or b) a sequence of top-level blocks only.
///
/// The document can be configured using a document header. The header is not a
/// block itself, but contributes metadata to the document, such as the document
/// title and document attributes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Document<'src> {
    header: Option<Header<'src>>,
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
}

impl<'src> Document<'src> {
    /// Parse a UTF-8 string as an AsciiDoc document.
    ///
    /// Note that the document references the underlying source string and
    /// necessarily has the same lifetime as the source.
    ///
    /// The `Document` data structure returned by this call and nearly all data
    /// structures contained within it are gated by the lifetime of the `source`
    /// text passed in to this function. For that reason all of those data
    /// structures are given the lifetime `'src`.
    ///
    /// **IMPORTANT:** The AsciiDoc language documentation states that UTF-16
    /// encoding is allowed if a byte-order-mark (BOM) is present at the
    /// start of a file. This format is not directly supported by the
    /// `asciidoc-parser` crate. Any UTF-16 content must be re-encoded as
    /// UTF-8 prior to parsing.
    ///
    /// TEMPORARY: Returns an `Option` which will be `None` if unable to parse.
    /// This will eventually be replaced with an annotation mechanism.
    pub fn parse(source: &'src str) -> Option<Self> {
        // TO DO: Add option for best-guess parsing?

        let source = Span::new(source);
        let i = source.discard_empty_lines();

        let (i, header) = if i.starts_with("= ") {
            let mi = Header::parse(i)?;
            (mi.after, Some(mi.item))
        } else {
            (i, None)
        };

        let maw_blocks = parse_blocks_until(i, |_| false);

        if !maw_blocks.warnings.is_empty() {
            todo!("Retain warnings");
        }

        let blocks = maw_blocks.item?;

        Some(Self {
            header,
            blocks: blocks.item,
            source,
        })
    }

    /// Return the document header if there is one.
    pub fn header(&'src self) -> Option<&'src Header<'src>> {
        self.header.as_ref()
    }
}

impl<'src> IsBlock<'src> for Document<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn context(&self) -> CowStr<'src> {
        "document".into()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.blocks.iter()
    }
}

impl<'src> HasSpan<'src> for Document<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
