use std::slice::Iter;

use crate::{document::Attribute, span::ParseResult, HasSpan, Span};

/// An AsciiDoc document may begin with a document header. The document header
/// encapsulates the document title, author and revision information,
/// document-wide attributes, and other document metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header<'src> {
    title: Option<Span<'src>>,
    attributes: Vec<Attribute<'src>>,
    source: Span<'src>,
}

impl<'src> Header<'src> {
    pub(crate) fn parse(source: Span<'src>) -> Option<ParseResult<'src, Self>> {
        let source = source.discard_empty_lines();

        // TEMPORARY: Titles are optional, but we're not prepared for that yet.
        let title = parse_title(source)?;

        let mut attributes: Vec<Attribute> = vec![];
        let mut rem = title.rem;

        while let Some(attr) = Attribute::parse(rem) {
            attributes.push(attr.t);
            rem = attr.rem;
        }

        let source = source.trim_remainder(rem);

        // Header must be followed by an empty line or EOF.
        let pr = rem.take_empty_line()?;

        Some(ParseResult {
            t: Self {
                title: Some(title.t),
                attributes,
                source,
            },
            rem: pr.rem.discard_empty_lines(),
        })
    }

    /// Return a [`Span`] describing the document title, if there was one.
    pub fn title(&'src self) -> Option<Span<'src>> {
        self.title
    }

    /// Return an iterator over the attributes in this header.
    pub fn attributes(&'src self) -> Iter<'src, Attribute<'src>> {
        self.attributes.iter()
    }
}

impl<'src> HasSpan<'src> for Header<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}

fn parse_title(source: Span<'_>) -> Option<ParseResult<Span>> {
    let line = source.take_non_empty_line()?;
    let equal = line.t.take_prefix("=")?;
    let ws = equal.rem.take_required_whitespace()?;

    Some(ParseResult {
        t: ws.rem,
        rem: line.rem,
    })
}
