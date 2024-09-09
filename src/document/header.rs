use std::slice::Iter;

use crate::{document::Attribute, span::MatchedItem, HasSpan, Span};

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
    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let source = source.discard_empty_lines();

        // TEMPORARY: Titles are optional, but we're not prepared for that yet.
        let title = parse_title(source)?;

        let mut attributes: Vec<Attribute> = vec![];
        let mut after = title.after;

        while let Some(attr) = Attribute::parse(after) {
            attributes.push(attr.item);
            after = attr.after;
        }

        let source = source.trim_remainder(after);

        // Header must be followed by an empty line or EOF.
        let mi = after.take_empty_line()?;

        Some(MatchedItem {
            item: Self {
                title: Some(title.item),
                attributes,
                source,
            },
            after: mi.after.discard_empty_lines(),
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

fn parse_title(source: Span<'_>) -> Option<MatchedItem<Span>> {
    let line = source.take_non_empty_line()?;
    let equal = line.item.take_prefix("=")?;
    let ws = equal.after.take_required_whitespace()?;

    Some(MatchedItem {
        item: ws.after,
        after: line.after,
    })
}
