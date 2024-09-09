use crate::Span;

/// Given two [`Span`]s, the second of which must be a trailing remainder
/// of the first, return the first input trimmed to exclude the second.
///
/// Note that the trailing remainder condition is not enforced.
pub(crate) fn trim_source_for_rem<'src>(source: Span<'src>, rem: Span<'src>) -> Span<'src> {
    // Sanity check: If rem is longer than source, we can't trim.
    let rlen = rem.len();
    let slen = source.len();

    if rlen >= slen {
        source.slice(0..0)
    } else {
        let trim_len = slen - rlen;
        source.slice(0..trim_len)
    }
}
