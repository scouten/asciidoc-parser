use crate::{inlines::InlineMacro, span::MatchedItem, HasSpan, Span};

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
    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let line = source.take_non_empty_line()?;
        let mut span = line.item;

        // Special-case optimization: If the entire span is one
        // uninterpreted block, just return that without the allocation
        // overhead of the Vec of inlines.

        let mut uninterp = parse_uninterpreted(span);
        if uninterp.after.is_empty() {
            return Some(MatchedItem {
                item: Self::Uninterpreted(uninterp.item),
                after: line.after,
            });
        }

        let mut inlines: Vec<Self> = vec![];

        loop {
            if !uninterp.item.is_empty() {
                inlines.push(Self::Uninterpreted(uninterp.item));
            }

            span = uninterp.after;
            if span.is_empty() {
                break;
            }

            let interp = parse_interpreted(span)?;
            if interp.after.is_empty() && inlines.is_empty() {
                return Some(interp);
            }

            inlines.push(interp.item);
            span = interp.after;

            uninterp = parse_uninterpreted(span);
        }

        Some(MatchedItem {
            item: Self::Sequence(inlines, source.trim_remainder(line.after)),
            after: line.after,
        })
    }

    /// Parse a sequence of non-empty lines as a single `Inline` that
    /// describes it.
    ///
    /// Returns `None` if there is not at least one non-empty line at
    /// beginning of input.
    pub(crate) fn parse_lines(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let mut inlines: Vec<Inline<'src>> = vec![];
        let mut next = source;

        while let Some(inline) = Self::parse(next) {
            next = inline.after;
            inlines.push(inline.item);
        }

        if inlines.len() < 2 {
            inlines.pop().map(|inline| MatchedItem {
                item: inline,
                after: next,
            })
        } else {
            let source = source.trim_remainder(next);
            Some(MatchedItem {
                item: Self::Sequence(inlines, source),
                after: next,
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
fn parse_uninterpreted(source: Span<'_>) -> MatchedItem<Span> {
    // Optimization: If line doesn't contain special markup chars,
    // then it's all uninterpreted.

    if !source.contains(':') {
        return source.into_parse_result(source.len());
    }

    let mut after = source;

    while !after.is_empty() {
        if InlineMacro::parse(after).is_some() {
            break;
        }

        let word = after.take_while(|c| c != ' ' && c != '\t');
        let spaces = word.after.take_whitespace();
        after = spaces.after;
    }

    MatchedItem {
        item: source.trim_remainder(after),
        after: after,
    }
}

// Parse the block as a special "interpreted" inline sequence or error out.
fn parse_interpreted(source: Span<'_>) -> Option<MatchedItem<Inline<'_>>> {
    InlineMacro::parse(source).map(|inline| MatchedItem {
        item: Inline::Macro(inline.item),
        after: inline.after,
    })
}
