use nom::{multi::many1, IResult};

use crate::{primitives::non_empty_line, Span};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'a> {
    /// Lines that were found.
    pub inlines: Vec<Span<'a>>,
}

impl<'a> SimpleBlock<'a> {
    #[allow(dead_code)] // TEMPORARY
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        let (i, inlines) = many1(non_empty_line)(i)?;
        Ok((i, Self { inlines }))
    }
}
