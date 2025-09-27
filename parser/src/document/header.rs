use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    content::{Content, SubstitutionGroup},
    document::{Attribute, AuthorLine},
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
    author_line: Option<AuthorLine<'src>>,
    comments: Vec<Span<'src>>,
    source: Span<'src>,
}

impl<'src> Header<'src> {
    pub(crate) fn parse(
        mut source: Span<'src>,
        parser: &mut Parser,
    ) -> MatchAndWarnings<'src, MatchedItem<'src, Self>> {
        let original_source = source.discard_empty_lines();

        let mut title_source: Option<Span<'src>> = None;
        let mut title: Option<String> = None;
        let mut attributes: Vec<Attribute> = vec![];
        let mut author_line: Option<AuthorLine<'src>> = None;
        let mut comments: Vec<Span<'src>> = vec![];
        let mut warnings: Vec<Warning<'src>> = vec![];

        // Aside from the title line, items can appear in almost any order.
        while !source.is_empty() {
            let line_mi = source.take_normalized_line();
            let line = line_mi.item;

            // A blank line after the title ends the header.
            if line.is_empty() {
                if title.is_some() {
                    break;
                }
                source = line_mi.after;
            } else if line.starts_with("//") && !line.starts_with("///") {
                comments.push(line);
                source = line_mi.after;
            } else if line.starts_with(':')
                && let Some(attr) = Attribute::parse(source, parser)
            {
                parser.set_attribute_from_header(&attr.item, &mut warnings);
                attributes.push(attr.item);
                source = attr.after;
            } else if title.is_none() && line.starts_with("= ") {
                let title_span = line.discard(2).discard_whitespace();
                title = Some(apply_header_subs(title_span.data(), parser));
                title_source = Some(title_span);
                source = line_mi.after;
            } else if title.is_some() && author_line.is_none() {
                author_line = Some(AuthorLine::parse(line, parser));
                source = line_mi.after;
            // else if title.is_some() && author_line.is_some() {
            // parse revision line
            } else {
                if title.is_some() {
                    warnings.push(Warning {
                        source: line,
                        warning: WarningType::DocumentHeaderNotTerminated,
                    });
                }
                break;
            }
        }

        let after = source.discard_empty_lines();
        let source = original_source.trim_remainder(source);

        MatchAndWarnings {
            item: MatchedItem {
                item: Self {
                    title_source,
                    title,
                    attributes,
                    author_line,
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

    /// Returns the author line, if found.
    pub fn author_line(&self) -> Option<&AuthorLine<'src>> {
        self.author_line.as_ref()
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

fn apply_header_subs(source: &str, parser: &Parser) -> String {
    let span = Span::new(source);

    let mut content = Content::from(span);
    SubstitutionGroup::Header.apply(&mut content, parser, None);

    content.rendered().to_string()
}
