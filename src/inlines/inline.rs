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
        let mut inlines: Vec<Self> = vec![];

        loop {
            let (span2, uninterp) = parse_uninterpreted(span)?;

            if !uninterp.is_empty() {
                inlines.push(Self::Uninterpreted(uninterp));
            }

            span = span2;
            if span.is_empty() {
                break;
            }

            let (span2, interp) = parse_interpreted(span)?;
            inlines.push(interp);

            span = span2;
        }

        if inlines.len() == 1 {
            if let Some(first) = inlines.into_iter().next() {
                Ok((rem, first))
            } else {
                panic!("I'm confused. len() == 1, but next() returned None");
            }
        } else {
            Ok((rem, Self::Sequence(inlines, trim_input_for_rem(i, rem))))
        }
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
fn parse_uninterpreted<'a>(i: Span<'a>) -> IResult<Span, Span> {
    let mut rem = i.clone();
    let mut at_word_boundary = true;

    loop {
        let mut iter = rem.iter_elements();
        let Some(c) = iter.next() else {
            break;
        };

        if at_word_boundary {
            if InlineMacro::parse(rem).is_ok() {
                break;
            }
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
fn parse_interpreted<'a>(i: Span<'a>) -> IResult<Span, Inline<'a>> {
    InlineMacro::parse(i).map(|(rem, x)| (rem, Inline::Macro(x)))
}
