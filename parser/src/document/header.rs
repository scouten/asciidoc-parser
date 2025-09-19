use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    content::{Content, SubstitutionGroup},
    document::Attribute,
    span::MatchedItem,
    warnings::{MatchAndWarnings, Warning, WarningType},
};

/// An AsciiDoc document may begin with a document header. The document header
/// encapsulates the document title, author and revision information,
/// document-wide attributes, and other document metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header<'src> {
    title_source: Option<Span<'src>>,
    title: Option<String>,
    attributes: Vec<Attribute<'src>>,
    comments: Vec<Span<'src>>,
    source: Span<'src>,
}

impl<'src> Header<'src> {
    pub(crate) fn parse(
        mut source: Span<'src>,
        parser: &mut Parser,
    ) -> MatchAndWarnings<'src, MatchedItem<'src, Self>> {
        let original_src = source;

        let mut attributes: Vec<Attribute> = vec![];
        let mut comments: Vec<Span<'src>> = vec![];
        let mut warnings: Vec<Warning<'src>> = vec![];

        // Look for empty lines and/or comments before header.
        while !source.is_empty() {
            let line_mi = source.take_normalized_line();
            let line = line_mi.item;

            if line.is_empty() {
                source = line_mi.after;
            } else if line.starts_with("//") && !line.starts_with("///") {
                comments.push(line);
                source = line_mi.after;
            } else {
                break;
            }
        }

        let (title_source, mut after) = if let Some(mi) = parse_title(source) {
            // TO DO: Look for author/revision lines.
            (Some(mi.item), mi.after)
        } else {
            (None, source)
        };

        let title = title_source.map(|ref span| {
            let mut content = Content::from(*span);
            SubstitutionGroup::Header.apply(&mut content, parser, None);
            content.rendered.into_string()
        });

        while let Some(attr) = Attribute::parse(after, parser) {
            parser.set_attribute_from_header(&attr.item, &mut warnings);
            attributes.push(attr.item);
            after = attr.after;
        }

        let source = source.trim_remainder(after);

        // Nothing resembling a header so far? Don't look for empty line.
        if title_source.is_none() && attributes.is_empty() {
            return MatchAndWarnings {
                item: MatchedItem {
                    item: Self {
                        title_source: None,
                        title: None,
                        attributes: vec![],
                        comments: vec![],
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
                    title_source,
                    title,
                    attributes,
                    comments,
                    source: source.trim_trailing_whitespace(),
                },
                after,
            },
            warnings,
        }
    }

    /// Return a [`Span`] describing the raw document title, if there was one.
    pub fn title_source(&'src self) -> Option<Span<'src>> {
        self.title_source
    }

    /// Return the document's title, if there was one, having applied header
    /// substitutions.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Return an iterator over the attributes in this header.
    pub fn attributes(&'src self) -> Iter<'src, Attribute<'src>> {
        self.attributes.iter()
    }

    /// Return an iterator over the comments in this header.
    pub fn comments(&'src self) -> Iter<'src, Span<'src>> {
        self.comments.iter()
    }
}

impl<'src> HasSpan<'src> for Header<'src> {
    fn span(&self) -> Span<'src> {
        self.source
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
