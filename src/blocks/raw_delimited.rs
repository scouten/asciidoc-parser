#![allow(dead_code)] // TEMPORARY

use super::{ContentModel, IsBlock};
use crate::{strings::CowStr, HasSpan, Span};

/// A delimited block that contains verbatim, raw, or comment text. The content
/// between the matching delimiters is not parsed for block syntax.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawDelimitedBlock<'src> {
    lines: Vec<Span<'src>>,
    content_model: ContentModel,
    context: CowStr<'src>,
    source: Span<'src>,
}

impl<'src> RawDelimitedBlock<'src> {
    pub(crate) fn is_valid_delimiter(line: &Span<'src>) -> bool {
        let data = line.data();

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

    // pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
    //     let inline = Inline::parse_lines(source)?;
    //     Some(MatchedItem {
    //         item: Self(inline.item),
    //         after: inline.after.discard_empty_lines(),
    //     })
    // }

    // /// Return the inline content of this block.
    // pub fn inline(&self) -> &Inline<'src> {
    //     &self.0
    // }
}

impl<'src> IsBlock<'src> for RawDelimitedBlock<'src> {
    fn content_model(&self) -> ContentModel {
        self.content_model
    }

    fn context(&self) -> CowStr<'src> {
        self.context.clone()
    }
}

impl<'src> HasSpan<'src> for RawDelimitedBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
