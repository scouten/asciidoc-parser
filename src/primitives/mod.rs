use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, char},
    combinator::recognize,
    multi::many0,
    sequence::{delimited, pair},
    IResult, Parser, Slice,
};

use crate::Span;

mod line;
#[allow(unused_imports)]
pub(crate) use line::{
    consume_empty_lines, empty_line, line, line_with_continuation, non_empty_line, normalized_line,
};

/// Represents a successful parse result and subsequent remainder of the input
/// stream.
pub(crate) struct ParseResult<'a, T> {
    pub(crate) t: T,
    pub(crate) rem: Span<'a>,
}

/// Given two [`Span`]s, the second of which must be a trailing remainder
/// of the first, return the first input trimmed to exclude the second.
///
/// Note that the trailing remainder condition is not enforced.
pub(crate) fn trim_input_for_rem<'a>(inp: Span<'a>, rem: Span<'a>) -> Span<'a> {
    // Sanity check: If rem is longer than inp, we can't trim.
    let rlen = rem.len();
    let ilen = inp.len();

    if rlen >= ilen {
        inp.slice(0..0)
    } else {
        let trim_len = ilen - rlen;
        inp.slice(0..trim_len)
    }
}

/// Recognize an identifier at the beginning of the current source.
///
/// NOTE: The concept of "identifier" is not crisply defined in the Asciidoc
/// documentation, so – for now – we're borrowing the definition from Rust.
pub(crate) fn ident(i: Span<'_>) -> IResult<Span, Span> {
    recognize(pair(
        alt((alphanumeric1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"), tag("-")))),
    ))
    .parse(i)
}

/// Recognize an attribute name at the beginning of the current source.
///
/// An attribute name consists of a word character (letter or numeral) followed
/// by any number of word or `-` characters (e.g., `see-also`).
#[allow(dead_code)]
pub(crate) fn attr_name(i: Span<'_>) -> IResult<Span, Span> {
    recognize(pair(alphanumeric1, many0(alt((alphanumeric1, tag("-")))))).parse(i)
}

/// Recognize a quoted string. The string is read until the next unescaped
/// matching quote.
///
/// IMPORTANT: The [`Span`] that is returned does not include the start or
/// ending quote, but does include (without transformation) any escaped quotes.
#[allow(dead_code)]
pub(crate) fn quoted_string(i: Span<'_>) -> IResult<Span, Span> {
    let (rem, _) = alt((
        delimited(
            char('\''),
            many0(alt((tag("\\\'"), is_not("\\\'")))),
            char('\''),
        ),
        delimited(
            char('"'),
            many0(alt((tag("\\\""), is_not("\\\"")))),
            char('"'),
        ),
    ))
    .parse(i)?;

    let content = trim_input_for_rem(i, rem);
    let content_len = content.len();
    let content = content.slice(1..content_len - 1);

    Ok((rem, content))
}
