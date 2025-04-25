use std::slice::Iter;

use crate::{
    attributes::Attrlist,
    blocks::{parse_utils::parse_blocks_until, preamble::Preamble, Block, ContentModel, IsBlock},
    span::MatchedItem,
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
    HasSpan, Parser, Span,
};

/// A delimited block that can contain other blocks.
///
/// The following delimiters are recognized as compound delimited blocks:
///
/// | Delimiter | Content type |
/// |-----------|--------------|
/// | `====`    | Example      |
/// | `--`      | Open         |
/// | `****`    | Sidebar      |
/// | `____`    | Quote        |
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompoundDelimitedBlock<'src> {
    blocks: Vec<Block<'src>>,
    context: CowStr<'src>,
    source: Span<'src>,
    title: Option<Span<'src>>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> CompoundDelimitedBlock<'src> {
    pub(crate) fn is_valid_delimiter(line: &Span<'src>) -> bool {
        let data = line.data();

        if data == "--" {
            return true;
        }

        // TO DO (https://github.com/scouten/asciidoc-parser/issues/145):
        // Seek spec clarity: Do the characters after the fourth char
        // have to match the first four?

        if data.len() >= 4 {
            if data.starts_with("====") {
                data.split_at(4).1.chars().all(|c| c == '=')
            } else if data.starts_with("****") {
                data.split_at(4).1.chars().all(|c| c == '*')
            } else if data.starts_with("____") {
                data.split_at(4).1.chars().all(|c| c == '_')
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
        let maybe_delimiter_text = delimiter.item.data();

        // TO DO (https://github.com/scouten/asciidoc-parser/issues/146):
        // Seek spec clarity on whether three hyphens can be used to
        // delimit an open block. Assuming yes for now.
        let context = match maybe_delimiter_text
            .split_at(maybe_delimiter_text.len().min(4))
            .0
        {
            "====" => "example",
            "--" => "open",
            "****" => "sidebar",
            "____" => "quote",
            _ => return None,
        };

        if !Self::is_valid_delimiter(&delimiter.item) {
            return None;
        }

        let mut next = delimiter.after;
        let closing_delimiter = loop {
            if next.is_empty() {
                return Some(MatchAndWarnings {
                    item: None,
                    warnings: vec![Warning {
                        source: delimiter.item,
                        warning: WarningType::UnterminatedDelimitedBlock,
                    }],
                });
            }

            let line = next.take_normalized_line();
            if line.item.data() == delimiter.item.data() {
                break line;
            }
            next = line.after;
        };

        let inside_delimiters = delimiter.after.trim_remainder(closing_delimiter.item);

        let mut bogus_parser = Parser::default();
        let maw_blocks = parse_blocks_until(inside_delimiters, |_| false, &mut bogus_parser);

        let blocks = maw_blocks.item;
        let source = preamble.source.trim_remainder(closing_delimiter.after);

        Some(MatchAndWarnings {
            item: Some(MatchedItem {
                item: Self {
                    blocks: blocks.item,
                    context: context.into(),
                    source: source.trim_trailing_whitespace(),
                    title: preamble.title,
                    anchor: preamble.anchor,
                    attrlist: preamble.attrlist.clone(),
                },
                after: closing_delimiter.after,
            }),
            warnings: maw_blocks.warnings,
        })
    }
}

impl<'src> IsBlock<'src> for CompoundDelimitedBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn raw_context(&self) -> CowStr<'src> {
        self.context.clone()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.blocks.iter()
    }

    fn title(&'src self) -> Option<Span<'src>> {
        self.title
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        self.anchor
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for CompoundDelimitedBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
