use nom::{multi::many1, IResult};

use crate::{
    primitives::{consume_empty_lines, non_empty_line, trim_input_for_rem},
    HasSpan, Span,
};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'a> {
    /// Lines that were found.
    /// TO DO: Make private
    pub inlines: Vec<Span<'a>>,

    source: Span<'a>,
}

impl<'a> SimpleBlock<'a> {
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let (rem, inlines) = many1(non_empty_line)(source)?;
        let source = trim_input_for_rem(source, rem);
        Ok((consume_empty_lines(rem), Self { inlines, source }))
    }
}

impl<'a> HasSpan<'a> for SimpleBlock<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}
