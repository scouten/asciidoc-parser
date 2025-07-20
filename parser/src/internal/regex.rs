use std::borrow::Cow;

use regex::{Captures, Regex};

/// Similar to `[regex::Regex::replace_all]` but takes a `[LookaheadReplacer]`
/// which can look ahead after the match and potentially abort the replacement.
pub fn replace_with_lookahead<'h, LR: LookaheadReplacer>(
    regex: &Regex,
    mut haystack: &'h str,
    mut rep: LR,
) -> Cow<'h, str> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;

    'retry: loop {
        // Optimization: If we don't have any matches, we can continue to borrow the
        // source.
        let mut it = regex.captures_iter(haystack).enumerate().peekable();

        if new.is_empty() && it.peek().is_none() {
            return Cow::Borrowed(haystack);
        }

        for (i, cap) in it {
            // unwrap on 0 is OK because captures only reports matches
            #[allow(clippy::unwrap_used)]
            let m = cap.get(0).unwrap();
            new.push_str(&haystack[last_match..m.start()]);

            let after = &haystack[m.end()..];

            match rep.replace_append(&cap, &mut new, after) {
                LookaheadResult::Continue => {
                    last_match = m.end();
                }

                LookaheadResult::SkipAheadAndRetry(n) => {
                    assert!(n > 0);
                    haystack = &haystack[m.start() + n..];
                    continue 'retry;
                }
            }
        }

        break;
    }

    new.push_str(&haystack[last_match..]);
    Cow::Owned(new)
}

/// Result of a call to `[LookaheadReplacer::replace_append`].
pub(crate) enum LookaheadResult {
    /// Replacement was successful. Continue as normal.
    Continue,

    /// Look-ahead was unsuccessful. Skip _n_ characters and restart the search.
    SkipAheadAndRetry(usize),
}

/// Alternative to
pub(crate) trait LookaheadReplacer {
    fn replace_append(
        &mut self,
        caps: &Captures<'_>,
        dst: &mut String,
        after_match: &str,
    ) -> LookaheadResult;
}
