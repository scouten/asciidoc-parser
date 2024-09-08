use crate::Span;

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
