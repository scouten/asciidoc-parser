use std::borrow::Cow;

use pretty_assertions_sorted::assert_eq;
use regex::{Captures, Regex, RegexBuilder};

use crate::internal::{LookaheadReplacer, LookaheadResult, replace_with_lookahead};

#[test]
fn example() {
    let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();

    let hay = "
Greetings  1973
Wild\t1973
BornToRun\t\t\t\t1975
Darkness                    1978
TheRiver 1980
";

    let new = replace_with_lookahead(&re, hay, "$2 $1");

    assert_eq!(
        new,
        "
1973 Greetings
1973 Wild
1975 BornToRun
1978 Darkness
1980 TheRiver
"
    );
}

#[test]
fn no_match_optimization() {
    let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();

    let hay = "blah_blah_blah\nno_match";

    let new = replace_with_lookahead(&re, hay, "$2 $1");

    assert_eq!(new, Cow::Borrowed(hay));
}

impl LookaheadReplacer for &str {
    fn replace_append(
        &mut self,
        caps: &Captures<'_>,
        dst: &mut String,
        _after: &str,
    ) -> LookaheadResult {
        caps.expand(self, dst);
        LookaheadResult::Continue
    }
}

#[test]
fn match_after_quote() {
    let re = RegexBuilder::new(r#"(^|[^\w&;:"'`}])(?:\[([^\[\]]+)\])?`(\S|\S.*?\S)`\b{end-half}"#)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let hay = "Olaf had been with the company since the `'00s.\nHis desk overflowed with heaps of paper, apple cores and squeaky toys.\nWe couldn't find Olaf's keyboard.\nThe state of his desk was replicated, in triplicate, across all of\nthe werewolves`' desks.";

    let new = replace_with_lookahead(&re, hay, Rejectifier {});

    assert_eq!(
        new,
        "Olaf had been with the company since the `'HELLO00s.\nHis desk overflowed with heaps of paper, apple cores and squeaky toys.\nWe couldn't find Olaf's keyboard.\nThe state of his desk was replicated, in triplicate, across all of\nthe werewolves`' desks."
    );
}

struct Rejectifier {}

impl LookaheadReplacer for Rejectifier {
    fn replace_append(
        &mut self,
        caps: &Captures<'_>,
        dst: &mut String,
        after: &str,
    ) -> LookaheadResult {
        assert_eq!(after, "' desks.");
        dst.push_str(&caps[0][0..3]);
        dst.push_str("HELLO");
        LookaheadResult::SkipAheadAndRetry(3)
    }
}
