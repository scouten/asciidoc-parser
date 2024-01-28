use nom::{
    bytes::complete::{take_till, take_till1},
    combinator::recognize,
    error::{Error, ErrorKind},
    multi::many0,
    sequence::pair,
    Err, IResult, Parser, Slice,
};

use crate::Span;

/// Return a single line from the source.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
pub(crate) fn line(input: Span<'_>) -> IResult<Span, Span> {
    take_till(|c| c == '\n')(input)
        .map(|ri| trim_rem_start_matches(ri, '\n'))
        .map(|ri| trim_rem_end_matches(ri, '\r'))
}

/// Return a single _normalized_ line from the source.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
///
/// All trailing spaces are removed from the line.
pub(crate) fn normalized_line(input: Span<'_>) -> IResult<Span, Span> {
    take_till(|c| c == '\n')(input)
        .map(|ri| trim_rem_start_matches(ri, '\n'))
        .map(|ri| trim_rem_end_matches(ri, '\r'))
        .map(trim_trailing_spaces)
}

/// Return a single _normalized, non-empty_ line from the source.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
///
/// All trailing spaces are removed from the line.
///
/// Returns an error if the line becomes empty after trailing spaces have been
/// removed.
pub(crate) fn non_empty_line(input: Span<'_>) -> IResult<Span, Span> {
    if !input.contains('\n') && !(input.ends_with(' ') || input.ends_with('\t') || input.is_empty())
    {
        let rem = input.slice(input.len()..);
        return Ok((rem, input));
    }

    take_till1(|c| c == '\n')(input)
        .map(|ri| trim_rem_start_matches(ri, '\n'))
        .map(|ri| trim_rem_end_matches(ri, '\r'))
        .map(trim_trailing_spaces)
        .and_then(|(rem, inp)| {
            if inp.is_empty() {
                Err(Err::Error(Error::new(input, ErrorKind::TakeTill1)))
            } else {
                Ok((rem, inp))
            }
        })
}

/// Return a normalized, non-empty line that may be continued onto subsequent
/// lines with an explicit continuation marker.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
///
/// A line is _not_ terminated by `\n` when preceded by a `+` character.
///
/// SPEC QUESTION: Is it allowed to have a `+` character followed by
/// white space? For now, I'm saying yes.
///
/// Trailing white spaces are removed from the final line and are not
/// removed from any lines with continuations.
///
/// Returns an error if the line becomes empty after trailing spaces have been
/// removed.
pub(crate) fn line_with_continuation(input: Span<'_>) -> IResult<Span, Span> {
    recognize(pair(
        many0(one_line_with_continuation),
        take_till(|c| c == '\n'),
    ))
    .parse(input)
    .map(|ri| trim_rem_start_matches(ri, '\n'))
    .map(|ri| trim_rem_end_matches(ri, '\r'))
    .map(trim_trailing_spaces)
    .and_then(|(rem, inp)| {
        if inp.is_empty() {
            Err(Err::Error(Error::new(input, ErrorKind::TakeTill1)))
        } else {
            Ok((rem, inp))
        }
    })
}

fn one_line_with_continuation(input: Span<'_>) -> IResult<Span, Span> {
    let (rem, line) = normalized_line(input)?;
    if line.ends_with('\\') {
        Ok((rem, line))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::NonEmpty)))
    }
}

/// Consumes an empty line.
///
/// An empty line may contain any number of white space characters.
///
/// Returns an error if the line contains any non-white-space characters.
pub(crate) fn empty_line(input: Span<'_>) -> IResult<Span, Span> {
    let (i, line) = line(input)?;

    if line.data().bytes().all(nom::character::is_space) {
        Ok((i, line))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::NonEmpty)))
    }
}

/// Consumes zero or more empty lines.
///
/// Returns the original input if any error occurs or no empty lines are found.
pub(crate) fn consume_empty_lines(mut input: Span<'_>) -> Span {
    while !input.data().is_empty() {
        match empty_line(input) {
            Ok((rem, _)) => input = rem,
            Err(_) => break,
        }
    }

    input
}

fn trim_rem_start_matches<'a>(rem_inp: (Span<'a>, Span<'a>), c: char) -> (Span<'a>, Span<'a>) {
    if let Some(rem) = rem_inp.0.strip_prefix(c) {
        let prefix_len = rem_inp.0.len() - rem.len();
        let rem = rem_inp.0.slice(prefix_len..);
        (rem, rem_inp.1)
    } else {
        rem_inp
    }
}

fn trim_rem_end_matches<'a>(rem_inp: (Span<'a>, Span<'a>), c: char) -> (Span<'a>, Span<'a>) {
    if let Some(inp) = rem_inp.1.strip_suffix(c) {
        let inp = rem_inp.1.slice(0..inp.len());
        (rem_inp.0, inp)
    } else {
        rem_inp
    }
}

fn trim_trailing_spaces<'a>(rem_inp: (Span<'a>, Span<'a>)) -> (Span<'a>, Span<'a>) {
    let inp = rem_inp.1.trim_end_matches(' ');
    let inp = rem_inp.1.slice(0..inp.len());
    (rem_inp.0, inp)
}
