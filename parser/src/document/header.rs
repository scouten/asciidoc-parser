use std::slice::Iter;

use crate::{
    document::Attribute,
    span::MatchedItem,
    warnings::{MatchAndWarnings, Warning, WarningType},
    HasSpan, Parser, Span,
};

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
    pub(crate) fn parse(
        source: Span<'src>,
        _parser: &mut Parser,
    ) -> MatchAndWarnings<'src, MatchedItem<'src, Self>> {
        let original_src = source;

        let mut attributes: Vec<Attribute> = vec![];
        let mut warnings: Vec<Warning<'src>> = vec![];

        let source = source.discard_empty_lines();

        let (title, mut after) = if let Some(mi) = parse_title(source) {
            (Some(mi.item), mi.after)
        } else {
            (None, source)
        };

        while let Some(attr) = Attribute::parse(after) {
            attributes.push(attr.item);
            after = attr.after;
        }

        let source = source.trim_remainder(after);

        // Nothing resembling a header so far? Don't look for empty line.
        if title.is_none() && attributes.is_empty() {
            return MatchAndWarnings {
                item: MatchedItem {
                    item: Self {
                        title: None,
                        attributes,
                        source: original_src.into_parse_result(0).item,
                    },
                    after,
                },
                warnings,
            };
        }

        // Header is valid so far. Warn if not followed by empty line or EOF.
        after = match after.take_empty_line() {
            Some(mi) => mi.after.discard_empty_lines(),
            None => {
                warnings.push(Warning {
                    source: after.take_line().item,
                    warning: WarningType::DocumentHeaderNotTerminated,
                });
                after
            }
        };

        MatchAndWarnings {
            item: MatchedItem {
                item: Self {
                    title,
                    attributes,
                    source: source.trim_trailing_whitespace(),
                },
                after,
            },
            warnings,
        }
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

fn parse_title(source: Span<'_>) -> Option<MatchedItem<'_, Span<'_>>> {
    let line = source.take_non_empty_line()?;
    let equal = line.item.take_prefix("=")?;
    let ws = equal.after.take_required_whitespace()?;

    Some(MatchedItem {
        item: ws.after,
        after: line.after,
    })
}
