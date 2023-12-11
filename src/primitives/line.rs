use nom::{bytes::complete::take_till, IResult};

#[allow(dead_code)] // TEMPORARY
pub(crate) fn line(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == '\n')(input)
        .map(|ri| trim_rem_start_matches(ri, '\n'))
        .map(|ri| trim_rem_end_matches(ri, '\r'))
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
