use nom::{multi::many1, IResult, InputIter, InputTake};

use crate::{
    inlines::InlineMacro,
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

    /// An inline macro.
    Macro(InlineMacro<'a>),
}

impl<'a> Inline<'a> {
    /// Parse a span (typically a line) of any type and return an `Inline` that
    /// describes it.
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        let (rem, mut span) = non_empty_line(i)?;

        // Special-case optimization: If the entire span is one
        // uninterpreted block, just return that without the allocation
        // overhead of the Vec of inlines.

        let (mut span2, mut uninterp) = parse_uninterpreted(span)?;

        if span2.is_empty() {
            return Ok((rem, Self::Uninterpreted(uninterp)));
        }

        let mut inlines: Vec<Self> = vec![];

        loop {
            if !uninterp.is_empty() {
                inlines.push(Self::Uninterpreted(uninterp));
            }

            span = span2;
            if span.is_empty() {
                break;
            }

            let (span3, interp) = parse_interpreted(span)?;

            if span3.is_empty() && inlines.is_empty() {
                return Ok((rem, interp));
            }

            inlines.push(interp);

            span = span3;

            (span2, uninterp) = parse_uninterpreted(span)?;
        }

        Ok((rem, Self::Sequence(inlines, trim_input_for_rem(i, rem))))
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
            Self::Macro(m) => m.span(),
        }
    }
}

// Parse the largest possible block of "uninterpreted" text.
// Remainder is either empty span or first span that requires
// special interpretation.
fn parse_uninterpreted(i: Span<'_>) -> IResult<Span, Span> {
    let mut rem = i;
    let mut at_word_boundary = true;

    loop {
        let mut iter = rem.iter_elements();
        let Some(c) = iter.next() else {
            break;
        };

        if at_word_boundary && InlineMacro::parse(rem).is_ok() {
            break;
        }

        at_word_boundary = matches!(c, ' ' | '\t');
        let (rem2, c) = rem.take_split(1);
        if let Some(c) = c.data().chars().next() {
            at_word_boundary = matches!(c, ' ' | '\t');
        }

        rem = rem2;
    }

    Ok((rem, trim_input_for_rem(i, rem)))
}

// Parse the block as a special "interpreted" inline sequence or error out.
fn parse_interpreted(i: Span<'_>) -> IResult<Span, Inline<'_>> {
    InlineMacro::parse(i).map(|(rem, x)| (rem, Inline::Macro(x)))
}
