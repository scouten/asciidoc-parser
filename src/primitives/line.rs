use nom::{
    bytes::complete::{take_till, take_till1},
    combinator::recognize,
    error::{Error, ErrorKind},
    multi::many0,
    sequence::pair,
    Err, IResult, Parser, Slice,
};

use crate::{primitives::ParseResult, Span};

/// Return a single line from the source.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
pub(crate) fn line(input: Span<'_>) -> ParseResult<Span> {
    let line = if let Some(index) = input.find('\n') {
        ParseResult {
            rem: input.slice(index..),
            t: input.slice(0..index),
        }
    } else {
        // No `\n` found; the entire input is the line.
        ParseResult {
            rem: input.slice(input.len()..),
            t: input,
        }
    };

    let line = trim_rem_start_matches(line, '\n');
    trim_rem_end_matches(line, '\r')
}

/// Return a single _normalized_ line from the source.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
///
/// All trailing spaces are removed from the line.
pub(crate) fn normalized_line(i: Span<'_>) -> ParseResult<Span> {
    trim_trailing_spaces(line(i))
}

/// Returns a single _normalized, non-empty_ line from the source
/// if one exists.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
///
/// All trailing spaces are removed from the line.
///
/// Returns `None` if the line becomes empty after trailing spaces have been
/// removed.
pub(crate) fn non_empty_line(input: Span<'_>) -> Option<ParseResult<Span>> {
    // Result<(Spanned<&str>, Spanned<&str>),
    // nom::Err<nom::error::Error<Spanned<&str>>>>
    take_till1::<_, Span, nom::error::Error<Span>>(|c| c == '\n')(input)
        .ok()
        .map(|line| {
            trim_rem_start_matches(
                ParseResult {
                    rem: line.0,
                    t: line.1,
                },
                '\n',
            )
        })
        .map(|line| trim_rem_end_matches(line, '\r'))
        .map(trim_trailing_spaces)
        .and_then(|line| if line.t.is_empty() { None } else { Some(line) })
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
/// Returns `None` if the line becomes empty after trailing spaces have been
/// removed.
pub(crate) fn line_with_continuation(i: Span<'_>) -> Option<ParseResult<Span>> {
    recognize(pair(
        many0(one_line_with_continuation),
        take_till(|c| c == '\n'),
    ))
    .parse(i)
    .map(|line| {
        trim_rem_start_matches(
            ParseResult {
                rem: line.0,
                t: line.1,
            },
            '\n',
        )
    })
    .map(|line| trim_rem_end_matches(line, '\r'))
    .map(trim_trailing_spaces)
    .ok()
    .filter(|line| !line.t.is_empty())
}

fn one_line_with_continuation(input: Span<'_>) -> IResult<Span, Span> {
    let line = normalized_line(input);
    if line.t.ends_with('\\') {
        Ok((line.rem, line.t))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::NonEmpty)))
    }
}

/// Consumes an empty line.
///
/// An empty line may contain any number of white space characters.
///
/// Returns `None` if the line contains any non-white-space characters.
pub(crate) fn empty_line(i: Span<'_>) -> Option<ParseResult<Span>> {
    let l = line(i);

    if l.t.data().bytes().all(nom::character::is_space) {
        Some(l)
    } else {
        None
    }
}

/// Consumes zero or more empty lines.
///
/// Returns the original input if any error occurs or no empty lines are found.
pub(crate) fn consume_empty_lines(mut i: Span<'_>) -> Span {
    while !i.data().is_empty() {
        match empty_line(i) {
            Some(line) => i = line.rem,
            None => break,
        }
    }

    i
}

fn trim_rem_start_matches<'a>(i: ParseResult<'a, Span<'a>>, c: char) -> ParseResult<Span<'a>> {
    if let Some(rem) = i.rem.strip_prefix(c) {
        let prefix_len = i.rem.len() - rem.len();
        let rem = i.rem.slice(prefix_len..);
        ParseResult { rem, t: i.t }
    } else {
        i
    }
}

fn trim_rem_end_matches<'a>(i: ParseResult<'a, Span<'a>>, c: char) -> ParseResult<Span<'a>> {
    if let Some(inp) = i.t.strip_suffix(c) {
        let inp = i.t.slice(0..inp.len());
        ParseResult { rem: i.rem, t: inp }
    } else {
        i
    }
}

fn trim_trailing_spaces<'a>(i: ParseResult<'a, Span<'a>>) -> ParseResult<Span<'a>> {
    let inp = i.t.trim_end_matches(' ');
    let inp = i.t.slice(0..inp.len());
    ParseResult { rem: i.rem, t: inp }
}
