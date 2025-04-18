use std::slice::Iter;

use crate::{
    attributes::Attrlist,
    blocks::{preamble::Preamble, ContentModel, IsBlock},
    span::MatchedItem,
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
    HasSpan, Span,
};

/// A delimited block that contains verbatim, raw, or comment text. The content
/// between the matching delimiters is not parsed for block syntax.
///
/// The following delimiters are recognized as raw delimited blocks:
///
/// | Delimiter | Content type |
/// |-----------|--------------|
/// | `////`    | Comment      |
/// | `----`    | Listing      |
/// | `....`    | Literal      |
/// | `++++`    | Passthrough  |
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawDelimitedBlock<'src> {
    lines: Vec<Span<'src>>,
    content_model: ContentModel,
    context: CowStr<'src>,
    source: Span<'src>,
    title: Option<Span<'src>>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> RawDelimitedBlock<'src> {
    pub(crate) fn is_valid_delimiter(line: &Span<'src>) -> bool {
        let data = line.data();

        // TO DO (https://github.com/scouten/asciidoc-parser/issues/145):
        // Seek spec clarity: Do the characters after the fourth char
        // have to match the first four?

        if data.len() >= 4 {
            if data.starts_with("////") {
                data.split_at(4).1.chars().all(|c| c == '/')
            } else if data.starts_with("----") {
                data.split_at(4).1.chars().all(|c| c == '-')
            } else if data.starts_with("....") {
                data.split_at(4).1.chars().all(|c| c == '.')
            } else if data.starts_with("++++") {
                data.split_at(4).1.chars().all(|c| c == '+')
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(crate) fn parse(
        preamble: &Preamble<'src>,
    ) -> Option<MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>>> {
        let delimiter = preamble.block_start.take_normalized_line();

        if delimiter.item.len() < 4 {
            return None;
        }

        let (content_model, context) = match delimiter.item.data().split_at(4).0 {
            "////" => (ContentModel::Raw, "comment"),
            "----" => (ContentModel::Verbatim, "listing"),
            "...." => (ContentModel::Verbatim, "literal"),
            "++++" => (ContentModel::Raw, "pass"),
            _ => return None,
        };

        if !Self::is_valid_delimiter(&delimiter.item) {
            return None;
        }

        let mut lines: Vec<Span<'src>> = vec![];
        let mut empty_lines: Vec<Span<'src>> = vec![];
        let mut next = delimiter.after.discard_empty_lines();

        while !next.is_empty() {
            // TO DO: Should we retain trailing white space when in Raw content model?
            let line = next.take_normalized_line();
            if line.item.data() == delimiter.item.data() {
                return Some(MatchAndWarnings {
                    item: Some(MatchedItem {
                        item: Self {
                            lines,
                            content_model,
                            context: context.into(),
                            source: preamble
                                .source
                                .trim_remainder(line.after)
                                .trim_trailing_whitespace(),
                            title: preamble.title,
                            anchor: preamble.anchor,
                            attrlist: preamble.attrlist.clone(),
                        },
                        after: line.after,
                    }),
                    warnings: vec![],
                });
            }

            if line.item.is_empty() {
                empty_lines.push(line.item);
            } else {
                lines.append(&mut empty_lines);
                lines.push(line.item);
            }

            next = line.after;
        }

        Some(MatchAndWarnings {
            item: None,
            warnings: vec![Warning {
                source: delimiter.item,
                warning: WarningType::UnterminatedDelimitedBlock,
            }],
        })
    }

    /// Return an iterator over the lines in this delimited block.
    pub fn lines(&'src self) -> Iter<'src, Span<'src>> {
        self.lines.iter()
    }
}

impl<'src> IsBlock<'src> for RawDelimitedBlock<'src> {
    fn content_model(&self) -> ContentModel {
        self.content_model
    }

    fn raw_context(&self) -> CowStr<'src> {
        self.context.clone()
    }

    fn title(&self) -> Option<Span<'src>> {
        self.title
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        self.anchor
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for RawDelimitedBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
