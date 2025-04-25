//! Describes the top-level document structure.

use std::slice::Iter;

use crate::{
    attributes::Attrlist,
    blocks::{parse_utils::parse_blocks_until, Block, ContentModel, IsBlock},
    document::Header,
    strings::CowStr,
    warnings::Warning,
    HasSpan, Parser, Span,
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
    header: Header<'src>,
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
    warnings: Vec<Warning<'src>>,
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
    /// # Warnings, not errors
    ///
    /// Any UTF-8 string is a valid AsciiDoc document, so this function does not
    /// return an [`Option`] or [`Result`] data type. There may be any number of
    /// character sequences that have ambiguous or potentially unintended
    /// meanings. For that reason, a caller is advised to review the warnings
    /// provided via the [`warnings()`] iterator.
    ///
    /// [`warnings()`]: Self::warnings
    pub(crate) fn parse(source: &'src str, parser: &mut Parser) -> Self {
        let source = Span::new(source);
        let i = source.discard_empty_lines();
        let i = if i.is_empty() { source } else { i };

        let mi = Header::parse(i, parser);
        let i = mi.item.after;

        let header = mi.item.item;
        let mut warnings = mi.warnings;

        let mut maw_blocks = parse_blocks_until(i, |_| false, parser);

        if !maw_blocks.warnings.is_empty() {
            warnings.append(&mut maw_blocks.warnings);
        }

        Self {
            header,
            blocks: maw_blocks.item.item,
            source: source.trim_trailing_whitespace(),
            warnings,
        }
    }

    /// Return the document header.
    pub fn header(&'src self) -> &'src Header<'src> {
        &self.header
    }

    /// Return an iterator over any warnings found during parsing.
    pub fn warnings(&'src self) -> Iter<'src, Warning<'src>> {
        self.warnings.iter()
    }
}

impl<'src> IsBlock<'src> for Document<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn raw_context(&self) -> CowStr<'src> {
        "document".into()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.blocks.iter()
    }

    fn title(&'src self) -> Option<Span<'src>> {
        // Document title is reflected in the Header.
        None
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        None
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        // Document attributes are reflected in the Header.
        None
    }
}

impl<'src> HasSpan<'src> for Document<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
