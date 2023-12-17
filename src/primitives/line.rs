use nom::{
    bytes::complete::{take_till, take_till1},
    error::{Error, ErrorKind},
    Err, IResult, Slice,
};

use crate::input::Input;

/// Return a single line from the source.
///
/// A line is terminated by end-of-input or a single `\n` character
/// or a single `\r\n` sequence. The end of line sequence is consumed
/// but not included in the returned line.
#[allow(dead_code)] // TEMPORARY
pub(crate) fn line<'a>(input: Input<'a>) -> IResult<Input, Input> {
    take_till(|c| c == '\n')(input)
        .map(|ri| trim_rem_start_matches((ri.0, ri.1), '\n'))
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
pub(crate) fn normalized_line<'a>(input: Input<'a>) -> IResult<Input, Input> {
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
pub(crate) fn non_empty_line<'a>(input: Input<'a>) -> IResult<Input, Input> {
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
fn trim_rem_start_matches<'a>(rem_inp: (Input<'a>, Input<'a>), c: char) -> (Input<'a>, Input<'a>) {
    if let Some(rem) = rem_inp.0.strip_prefix(c) {
        let prefix_len = rem_inp.0.len() - rem.len();
        let rem = rem_inp.0.slice(prefix_len..);
        (rem, rem_inp.1)
    } else {
        rem_inp
    }
}

#[allow(dead_code)] // TEMPORARY
fn trim_rem_end_matches<'a>(rem_inp: (Input<'a>, Input<'a>), c: char) -> (Input<'a>, Input<'a>) {
    if let Some(inp) = rem_inp.1.strip_suffix(c) {
        let inp = rem_inp.1.slice(0..inp.len());
        (rem_inp.0, inp)
    } else {
        rem_inp
    }
}

#[allow(dead_code)] // TEMPORARY
fn trim_trailing_spaces<'a>(rem_inp: (Input<'a>, Input<'a>)) -> (Input<'a>, Input<'a>) {
    let inp = rem_inp.1.trim_end_matches(' ');
    let inp = rem_inp.1.slice(0..inp.len());
    (rem_inp.0, inp)
}
