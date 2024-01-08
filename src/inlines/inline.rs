use nom::{multi::many1, IResult};

use crate::{
    primitives::{non_empty_line, trim_input_for_rem},
    HasSpan, Span,
};

/// An inline element is a phrase (i.e., span of content) within a block element
/// or one of its attributes in an AsciiDoc document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Inline<'a> {
    /// Uninterpreted text (i.e., plain text) is text (character data) for which
    /// all inline grammar rules fail to match.
    Uninterpreted(Span<'a>),

    /// A sequence of other inline blocks.
    Sequence(Vec<Self>, Span<'a>),
}

impl<'a> Inline<'a> {
    /// Parse a span (typically a line) of any type and return an `Inline` that
    /// describes it.
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        // TEMPORARY: Naive approach ... everything is a plain span.
        // Assuming for now that it's a line.

        let (rem, span) = non_empty_line(i)?;
        Ok((rem, Self::Uninterpreted(span)))
    }

    /// Parse a sequence of non-empty lines as a single `Inline` that
    /// describes it.
    pub(crate) fn parse_lines(i: Span<'a>) -> IResult<Span, Self> {
        let (rem, first_line) = Self::parse(i)?;

        if let Ok((rem2, mut more_inlines)) = many1(Self::parse)(rem) {
            more_inlines.insert(0, first_line);

            let source = trim_input_for_rem(i, rem2);
            Ok((rem2, Self::Sequence(more_inlines, source)))
        } else {
            Ok((rem, first_line))
        }
    }
}

impl<'a> HasSpan<'a> for Inline<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        match self {
            Self::Uninterpreted(i) => i,
            Self::Sequence(_, i) => i,
        }
    }
}
