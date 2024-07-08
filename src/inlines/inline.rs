use nom::InputTake;

use crate::{
    inlines::InlineMacro,
    primitives::{non_empty_line, trim_input_for_rem, ParseResult},
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
    ///
    /// Returns `None` if input doesn't start with a non-empty line.
    pub(crate) fn parse(i: Span<'a>) -> Option<ParseResult<Self>> {
        let (rem, mut span) = non_empty_line(i)?;

        // Special-case optimization: If the entire span is one
        // uninterpreted block, just return that without the allocation
        // overhead of the Vec of inlines.

        let (mut span2, mut uninterp) = parse_uninterpreted(span);

        if span2.is_empty() {
            return Some(ParseResult {
                t: Self::Uninterpreted(uninterp),
                rem,
            });
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
                return Some(ParseResult { t: interp, rem });
            }

            inlines.push(interp);

            span = span3;

            (span2, uninterp) = parse_uninterpreted(span);
        }

        Some(ParseResult {
            t: Self::Sequence(inlines, trim_input_for_rem(i, rem)),
            rem,
        })
    }

    /// Parse a sequence of non-empty lines as a single `Inline` that
    /// describes it.
    ///
    /// Returns `None` if there is not at least one non-empty line at
    /// beginning of input.
    pub(crate) fn parse_lines(i: Span<'a>) -> Option<ParseResult<Self>> {
        let mut inlines: Vec<Inline<'a>> = vec![];
        let mut next = i;

        while let Some(inline) = Self::parse(next) {
            next = inline.rem;
            inlines.push(inline.t);
        }

        if inlines.len() < 2 {
            inlines.pop().map(|inline| ParseResult {
                t: inline,
                rem: next,
            })
        } else {
            let source = trim_input_for_rem(i, next);
            Some(ParseResult {
                t: Self::Sequence(inlines, source),
                rem: next,
            })
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
fn parse_uninterpreted(i: Span<'_>) -> (Span, Span) {
    let mut rem = i;
    let mut at_word_boundary = true;

    // Optimization: If line doesn't contain special markup chars,
    // then it's all uninterpreted.

    if !rem.contains(':') {
        return (trim_input_for_rem(i, rem), rem);
    }

    loop {
        if (at_word_boundary && InlineMacro::parse(rem).is_ok()) || rem.is_empty() {
            break;
        }

        let (rem2, c) = rem.take_split(1);
        at_word_boundary = matches!(c.data().chars().next(), Some(' ') | Some('\t'));

        rem = rem2;
    }

    (rem, trim_input_for_rem(i, rem))
}

// Parse the block as a special "interpreted" inline sequence or error out.
fn parse_interpreted(i: Span<'_>) -> Option<(Span, Inline<'_>)> {
    InlineMacro::parse(i)
        .map(|(rem, x)| (rem, Inline::Macro(x)))
        .ok()
}
