use crate::{
    inlines::InlineMacro, primitives::trim_source_for_rem, span::ParseResult, HasSpan, Span,
};

/// An inline element is a phrase (i.e., span of content) within a block element
/// or one of its attributes in an AsciiDoc document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Inline<'src> {
    /// Uninterpreted text (i.e., plain text) is text (character data) for which
    /// all inline grammar rules fail to match.
    Uninterpreted(Span<'src>),

    /// A sequence of other inline blocks.
    Sequence(Vec<Self>, Span<'src>),

    /// An inline macro.
    Macro(InlineMacro<'src>),
}

impl<'src> Inline<'src> {
    /// Parse a span (typically a line) of any type and return an `Inline` that
    /// describes it.
    ///
    /// Returns `None` if input doesn't start with a non-empty line.
    pub(crate) fn parse(source: Span<'src>) -> Option<ParseResult<'src, Self>> {
        let line = source.take_non_empty_line()?;
        let mut span = line.t;

        // Special-case optimization: If the entire span is one
        // uninterpreted block, just return that without the allocation
        // overhead of the Vec of inlines.

        let mut uninterp = parse_uninterpreted(span);
        if uninterp.rem.is_empty() {
            return Some(ParseResult {
                t: Self::Uninterpreted(uninterp.t),
                rem: line.rem,
            });
        }

        let mut inlines: Vec<Self> = vec![];

        loop {
            if !uninterp.t.is_empty() {
                inlines.push(Self::Uninterpreted(uninterp.t));
            }

            span = uninterp.rem;
            if span.is_empty() {
                break;
            }

            let interp = parse_interpreted(span)?;
            if interp.rem.is_empty() && inlines.is_empty() {
                return Some(interp);
            }

            inlines.push(interp.t);
            span = interp.rem;

            uninterp = parse_uninterpreted(span);
        }

        Some(ParseResult {
            t: Self::Sequence(inlines, trim_source_for_rem(source, line.rem)),
            rem: line.rem,
        })
    }

    /// Parse a sequence of non-empty lines as a single `Inline` that
    /// describes it.
    ///
    /// Returns `None` if there is not at least one non-empty line at
    /// beginning of input.
    pub(crate) fn parse_lines(source: Span<'src>) -> Option<ParseResult<'src, Self>> {
        let mut inlines: Vec<Inline<'src>> = vec![];
        let mut next = source;

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
            let source = trim_source_for_rem(source, next);
            Some(ParseResult {
                t: Self::Sequence(inlines, source),
                rem: next,
            })
        }
    }
}

impl<'src> HasSpan<'src> for Inline<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        match self {
            Self::Uninterpreted(source) => source,
            Self::Sequence(_, source) => source,
            Self::Macro(m) => m.span(),
        }
    }
}

// Parse the largest possible block of "uninterpreted" text.
// Remainder is either empty span or first span that requires
// special interpretation.
fn parse_uninterpreted(source: Span<'_>) -> ParseResult<Span> {
    // Optimization: If line doesn't contain special markup chars,
    // then it's all uninterpreted.

    if !source.contains(':') {
        return source.into_parse_result(source.len());
    }

    let mut rem = source;

    while !rem.is_empty() {
        if InlineMacro::parse(rem).is_some() {
            break;
        }

        let word = rem.take_while(|c| c != ' ' && c != '\t');
        let spaces = word.rem.take_whitespace();
        rem = spaces.rem;
    }

    ParseResult {
        t: trim_source_for_rem(source, rem),
        rem,
    }
}

// Parse the block as a special "interpreted" inline sequence or error out.
fn parse_interpreted(source: Span<'_>) -> Option<ParseResult<Inline<'_>>> {
    InlineMacro::parse(source).map(|inline| ParseResult {
        t: Inline::Macro(inline.t),
        rem: inline.rem,
    })
}
