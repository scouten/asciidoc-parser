use std::slice::Iter;

use crate::{
    document::Attribute, primitives::trim_input_for_rem, span::ParseResult, HasSpan, Span,
};

/// An AsciiDoc document may begin with a document header. The document header
/// encapsulates the document title, author and revision information,
/// document-wide attributes, and other document metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header<'a> {
    title: Option<Span<'a>>,
    attributes: Vec<Attribute<'a>>,
    source: Span<'a>,
}

impl<'a> Header<'a> {
    pub(crate) fn parse(i: Span<'a>) -> Option<ParseResult<Self>> {
        let source = i.discard_empty_lines();

        // TEMPORARY: Titles are optional, but we're not prepared for that yet.
        let title = parse_title(source)?;

        let mut attributes: Vec<Attribute> = vec![];
        let mut rem = title.rem;

        while let Some(attr) = Attribute::parse(rem) {
            attributes.push(attr.t);
            rem = attr.rem;
        }

        let source = trim_input_for_rem(source, rem);

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
    pub fn title(&'a self) -> Option<Span<'a>> {
        self.title
    }

    /// Return an iterator over the attributes in this header.
    pub fn attributes(&'a self) -> Iter<'a, Attribute<'a>> {
        self.attributes.iter()
    }
}

impl<'a> HasSpan<'a> for Header<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

fn parse_title(i: Span<'_>) -> Option<ParseResult<Span>> {
    let line = i.take_non_empty_line()?;
    let equal = line.t.take_prefix("=")?;
    let ws = equal.rem.take_required_whitespace()?;

    Some(ParseResult {
        t: ws.rem,
        rem: line.rem,
    })
}
