//! Describes the top-level document structure.

use std::{marker::PhantomData, slice::Iter};

use self_cell::self_cell;

use crate::{
    Parser, Span,
    attributes::Attrlist,
    blocks::{Block, ContentModel, IsBlock, parse_utils::parse_blocks_until},
    document::Header,
    strings::CowStr,
    warnings::Warning,
};

/// A document represents the top-level block element in AsciiDoc. It consists
/// of an optional document header and either a) one or more sections preceded
/// by an optional preamble or b) a sequence of top-level blocks only.
///
/// The document can be configured using a document header. The header is not a
/// block itself, but contributes metadata to the document, such as the document
/// title and document attributes.
#[derive(Eq, PartialEq)]
pub struct Document<'src> {
    internal: Internal,
    _phantom: PhantomData<&'src ()>,
}

/// Internal dependent struct containing the actual data members that reference
/// the owned source.
#[derive(Debug, Eq, PartialEq)]
struct InternalDependent<'src> {
    header: Header<'src>,
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
    warnings: Vec<Warning<'src>>,
}

self_cell! {
    /// Internal implementation struct containing the actual data members.
    struct Internal {
        owner: String,
        #[covariant]
        dependent: InternalDependent,
    }
    impl {Debug, Eq, PartialEq}
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
    /// The `parser` reference is used during parsing to mutate parser state
    /// (such as setting document attributes) but no reference to the parser
    /// is retained in the returned `Document`. The parser borrow is released
    /// when this function returns.
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
    pub(crate) fn parse(source: &str, parser: &mut Parser) -> Self {
        let owned_source = source.to_string();

        let internal = Internal::new(owned_source, |owned_src| {
            let source = Span::new(owned_src);

            let mi = Header::parse(source, parser);
            let next = mi.item.after;

            let header = mi.item.item;
            let mut warnings = mi.warnings;

            let mut maw_blocks = parse_blocks_until(next, |_| false, parser);

            if !maw_blocks.warnings.is_empty() {
                warnings.append(&mut maw_blocks.warnings);
            }

            InternalDependent {
                header,
                blocks: maw_blocks.item.item,
                source: source.trim_trailing_whitespace(),
                warnings,
            }
        });

        Self {
            internal,
            _phantom: PhantomData,
        }
    }

    /// Return the document header.
    pub fn header(&self) -> &Header<'_> {
        &self.internal.borrow_dependent().header
    }

    /// Return an iterator over any warnings found during parsing.
    pub fn warnings(&self) -> Iter<'_, Warning<'_>> {
        self.internal.borrow_dependent().warnings.iter()
    }

    /// Return a [`Span`] describing the entire document source.
    pub fn span(&self) -> Span<'_> {
        self.internal.borrow_dependent().source
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
        self.internal.borrow_dependent().blocks.iter()
    }

    fn title_source(&'src self) -> Option<Span<'src>> {
        // Document title is reflected in the Header.
        None
    }

    fn title(&self) -> Option<&str> {
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

impl std::fmt::Debug for Document<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dependent = self.internal.borrow_dependent();
        f.debug_struct("Document")
            .field("header", &dependent.header)
            .field("blocks", &dependent.blocks)
            .field("source", &dependent.source)
            .field("warnings", &dependent.warnings)
            .finish()
    }
}
