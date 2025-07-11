use crate::{
    Content, HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock, preamble::Preamble},
    span::{MatchedItem, content::SubstitutionGroup},
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
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
    content: Content<'src>,
    content_model: ContentModel,
    context: CowStr<'src>,
    source: Span<'src>,
    title: Option<Span<'src>>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
    substitution_group: SubstitutionGroup,
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
        parser: &mut Parser,
    ) -> Option<MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>>> {
        let delimiter = preamble.block_start.take_normalized_line();

        if delimiter.item.len() < 4 {
            return None;
        }

        let (content_model, context, substitution_group) = match delimiter.item.data().split_at(4).0
        {
            "////" => (ContentModel::Raw, "comment", SubstitutionGroup::None),
            "----" => (
                ContentModel::Verbatim,
                "listing",
                SubstitutionGroup::Verbatim,
            ),
            "...." => (
                ContentModel::Verbatim,
                "literal",
                SubstitutionGroup::Verbatim,
            ),
            "++++" => (ContentModel::Raw, "pass", SubstitutionGroup::Pass),
            _ => return None,
        };

        if !Self::is_valid_delimiter(&delimiter.item) {
            return None;
        }

        let content_start = delimiter.after;
        let mut next = content_start;

        while !next.is_empty() {
            let line = next.take_normalized_line();
            if line.item.data() == delimiter.item.data() {
                let content = content_start.trim_remainder(next).trim_trailing_line_end();

                let mut content: Content<'src> = content.into();
                substitution_group.apply(&mut content, parser, preamble.attrlist.as_ref());

                return Some(MatchAndWarnings {
                    item: Some(MatchedItem {
                        item: Self {
                            content,
                            content_model,
                            context: context.into(),
                            source: preamble
                                .source
                                .trim_remainder(line.after)
                                .trim_trailing_line_end(),
                            title: preamble.title,
                            anchor: preamble.anchor,
                            attrlist: preamble.attrlist.clone(),
                            substitution_group,
                        },
                        after: line.after,
                    }),
                    warnings: vec![],
                });
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

    /// Return the interpreted content of this block.
    pub fn content(&self) -> &Content<'src> {
        &self.content
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

    fn substitution_group(&'src self) -> SubstitutionGroup {
        self.substitution_group.clone()
    }
}

impl<'src> HasSpan<'src> for RawDelimitedBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
