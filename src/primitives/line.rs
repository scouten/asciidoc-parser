use nom::{
    bytes::complete::{take_till, take_till1},
    IResult,
};

/// Return a single line from the source.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
#[allow(dead_code)] // TEMPORARY
pub(crate) fn line(input: &str) -> IResult<&str, &str> {
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
#[allow(dead_code)] // TEMPORARY
pub(crate) fn normalized_line(input: &str) -> IResult<&str, &str> {
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
#[allow(dead_code)] // TEMPORARY
pub(crate) fn non_empty_line(input: &str) -> IResult<&str, &str> {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };

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

#[allow(dead_code)] // TEMPORARY
fn trim_rem_start_matches<'a>(rem_inp: (&'a str, &'a str), c: char) -> (&'a str, &'a str) {
    if let Some(rem) = rem_inp.0.strip_prefix(c) {
        (rem, rem_inp.1)
    } else {
        rem_inp
    }
}

#[allow(dead_code)] // TEMPORARY
fn trim_rem_end_matches<'a>(rem_inp: (&'a str, &'a str), c: char) -> (&'a str, &'a str) {
    if let Some(inp) = rem_inp.1.strip_suffix(c) {
        (rem_inp.0, inp)
    } else {
        rem_inp
    }
}

#[allow(dead_code)] // TEMPORARY
fn trim_trailing_spaces<'a>(rem_inp: (&'a str, &'a str)) -> (&'a str, &'a str) {
    (rem_inp.0, rem_inp.1.trim_end_matches(' '))
}