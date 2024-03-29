use nom::IResult;

use super::{ContentModel, IsBlock};
use crate::{inlines::Inline, primitives::consume_empty_lines, strings::CowStr, HasSpan, Span};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'a>(Inline<'a>);

impl<'a> SimpleBlock<'a> {
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let (rem, inline) = Inline::parse_lines(source)?;
        Ok((consume_empty_lines(rem), Self(inline)))
    }

    /// Return the inline content of this block.
    pub fn inline(&self) -> &Inline<'a> {
        &self.0
    }
}

impl<'a> IsBlock<'a> for SimpleBlock<'a> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Simple
    }

    fn context(&self) -> CowStr<'a> {
        "paragraph".into()
    }
}

impl<'a> HasSpan<'a> for SimpleBlock<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        self.0.span()
    }
}
