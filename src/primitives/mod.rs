use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::recognize,
    multi::many0, sequence::pair, IResult, Parser, Slice,
};

mod line;
#[allow(unused_imports)]
pub(crate) use line::{
    consume_empty_lines, empty_line, line, line_with_continuation, non_empty_line, normalized_line,
};

/// Represents a subset of the overall input stream.
///
/// Annotated with 1-based line and column numbers relative to the
/// beginning of the overall input stream.
///
/// Called `Span` because its `data` member can be consumed
/// to yield another `Span` with annotations for the end of the
/// syntactic element in question.
pub type Span<'a> = nom_span::Spanned<&'a str>;

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
pub(crate) fn ident<'a>(i: Span<'a>) -> IResult<Span, Span> {
    recognize(pair(
        alt((alphanumeric1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"), tag("-")))),
    ))
    .parse(i)
}
