use super::{ContentModel, IsBlock};
use crate::{inlines::Inline, span::ParseResult, strings::CowStr, HasSpan, Span};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'src>(Inline<'src>);

impl<'src> SimpleBlock<'src> {
    pub(crate) fn parse(source: Span<'src>) -> Option<ParseResult<'src, Self>> {
        let inline = Inline::parse_lines(source)?;
        Some(ParseResult {
            t: Self(inline.t),
            rem: inline.rem.discard_empty_lines(),
        })
    }

    /// Return the inline content of this block.
    pub fn inline(&self) -> &Inline<'src> {
        &self.0
    }
}

impl<'src> IsBlock<'src> for SimpleBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Simple
    }

    fn context(&self) -> CowStr<'src> {
        "paragraph".into()
    }
}

impl<'src> HasSpan<'src> for SimpleBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        self.0.span()
    }
}
