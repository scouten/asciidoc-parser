use nom::IResult;

use crate::{primitives::normalized_line, HasSpan, Span};

/// An inline element is a phrase (i.e., span of content) within a block element
/// or one of its attributes in an AsciiDoc document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Inline<'a> {
    /// Uninterpreted text (i.e., plain text) is text (character data) for which
    /// all inline grammar rules fail to match.
    Uninterpreted(Span<'a>),
}

impl<'a> Inline<'a> {
    /// Parse a span (typically a line) of any type and return an `Inline` that
    /// describes it.
    #[allow(dead_code)]
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        // TEMPORARY: Naive approach ... everything is a plain span.
        // Assuming for now that it's a line.

        let (rem, span) = normalized_line(i)?;
        Ok((rem, Self::Uninterpreted(span)))
    }
}

impl<'a> HasSpan<'a> for Inline<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        match self {
            Self::Uninterpreted(i) => i,
        }
    }
}
