use std::sync::LazyLock;

use regex::Regex;

use crate::{HasSpan, Span, content::Content, span::MatchedItem};

/// A list item is signaled by one of several designated marker sequences.
#[derive(Clone, Eq, PartialEq)]
pub enum ListItemMarker<'src> {
    /// Unordered list (hyphen).
    Hyphen(Span<'src>),

    /// Unordered list (asterisks).
    Asterisks(Span<'src>),

    /// Unordered list (Unicode bullet).
    Bullet(Span<'src>),

    /// Ordered list (dots).
    Dots(Span<'src>),

    /// Uppercase letter followed by dot (alpha list).
    AlphaListCapital(Span<'src>),

    /// Lowercase letter followed by dot (alpha list).
    AlphaListLower(Span<'src>),

    /// Lowercase Roman numeral followed by closing paren.
    RomanNumeralLower(Span<'src>),

    /// Explicit Arabic numeral followed by dot (e.g., "7.").
    ArabicNumeral(Span<'src>),

    /// A term to be defined.
    DefinedTerm {
        /// The name of the term being defined.
        term: Content<'src>,

        /// The marker (`::`, etc.) used to call out the definition.
        marker: Span<'src>,

        /// The source span for the entire term assembly.
        source: Span<'src>,
    },
}

impl<'src> ListItemMarker<'src> {
    pub(crate) fn starts_with_marker(source: Span<'src>) -> bool {
        LIST_ITEM_MARKER.is_match(source.data())
    }

    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let source = source.discard_whitespace();

        if let Some(captures) = LIST_ITEM_MARKER.captures(source.data()) {
            let marker = source.slice(0..captures[1].len());
            let marker_str = marker.data();
            let after = source.slice_from(captures[1].len()..).discard_whitespace();

            let first_char = captures[1].chars().next();

            let item = if marker_str == "-" {
                Self::Hyphen(marker)
            } else if marker_str.starts_with('*') {
                Self::Asterisks(marker)
            } else if marker_str == "•" {
                Self::Bullet(marker)
            } else if marker_str.starts_with('.') {
                Self::Dots(marker)
            } else if let Some(first_char) = first_char
                && first_char.is_ascii_uppercase()
                && marker_str.ends_with('.')
            {
                Self::AlphaListCapital(marker)
            } else if let Some(first_char) = first_char
                && first_char.is_ascii_lowercase()
                && marker_str.ends_with('.')
            {
                Self::AlphaListLower(marker)
            } else if marker_str.ends_with(')')
                && marker_str
                    .chars()
                    .take(marker_str.len() - 1)
                    .all(|c| "ivxlcdm".contains(c))
            {
                Self::RomanNumeralLower(marker)
            } else if marker_str.ends_with('.')
                && marker_str
                    .chars()
                    .take(marker_str.len() - 1)
                    .all(|c| c.is_ascii_digit())
            {
                Self::ArabicNumeral(marker)
            } else {
                todo!("Not handled yet: {}", &captures[1]);
            };

            Some(MatchedItem { item, after })
        } else {
            let captures = DESCRIPTION_LIST_MARKER.captures(source.data())?;

            let after = source.slice_from(captures[0].len()..).discard_whitespace();

            let source = source
                .slice_to(..captures[0].len())
                .trim_trailing_whitespace();

            let term_len = captures[1].len();
            let term = source.slice(0..term_len);
            let term: Content<'src> = term.into();

            let marker = source.slice_from(term_len..);

            Some(MatchedItem {
                item: Self::DefinedTerm {
                    term,
                    marker,
                    source,
                },
                after,
            })
        }
    }

    /// Test for equality, disregarding span offsets.
    pub(crate) fn is_match_for(&self, other: &Self) -> bool {
        match self {
            Self::Hyphen(self_span) => match other {
                Self::Hyphen(other_span) => self_span.data() == other_span.data(),
                _ => false,
            },

            Self::Asterisks(self_span) => match other {
                Self::Asterisks(other_span) => self_span.data() == other_span.data(),
                _ => false,
            },

            Self::Bullet(self_span) => match other {
                Self::Bullet(other_span) => self_span.data() == other_span.data(),
                _ => false,
            },

            Self::Dots(self_span) => match other {
                Self::Dots(other_span) => self_span.data() == other_span.data(),
                _ => false,
            },

            Self::AlphaListCapital(_self_span) => {
                matches!(other, Self::AlphaListCapital(_other_span))
            }

            Self::AlphaListLower(_self_span) => {
                matches!(other, Self::AlphaListLower(_other_span))
            }

            Self::RomanNumeralLower(_self_span) => {
                matches!(other, Self::RomanNumeralLower(_other_span))
            }

            Self::ArabicNumeral(_self_span) => {
                matches!(other, Self::ArabicNumeral(_other_span))
            }

            Self::DefinedTerm {
                term: _,
                marker: self_marker,
                source: _,
            } => match other {
                Self::DefinedTerm {
                    term: _,
                    marker: other_marker,
                    source: _,
                } => self_marker.data() == other_marker.data(),
                _ => false,
            },
        }
    }

    /// Returns the ordinal value for explicit markers, or `None` for implicit
    /// markers.
    ///
    /// Explicit markers like `x.`, `7.`, or `iv)` have specific sequence
    /// values. Implicit markers like `.` or `*` don't have ordinal values.
    pub(crate) fn ordinal_value(&self) -> Option<u32> {
        match self {
            Self::AlphaListLower(span) => {
                // "x." -> 24 (1-indexed: a=1, b=2, ..., x=24)
                let ch = span.data().chars().next()?;
                Some((ch as u32) - ('a' as u32) + 1)
            }

            Self::AlphaListCapital(span) => {
                // "X." -> 24 (1-indexed: A=1, B=2, ..., X=24)
                let ch = span.data().chars().next()?;
                Some((ch as u32) - ('A' as u32) + 1)
            }

            Self::ArabicNumeral(span) => {
                // "7." -> 7
                span.data().trim_end_matches('.').parse().ok()
            }

            Self::RomanNumeralLower(span) => {
                // "xvii)" -> 17
                parse_roman_numeral(span.data().trim_end_matches(')'))
            }

            // Implicit markers (dots, asterisks, etc.) don't have ordinal values.
            _ => None,
        }
    }

    /// Converts an ordinal value back to the display form for this marker type.
    ///
    /// Used to generate warning messages about expected vs. actual sequence
    /// values.
    pub(crate) fn ordinal_to_marker_text(&self, ordinal: u32) -> Option<String> {
        match self {
            Self::AlphaListLower(_) => {
                // 24 -> "x"
                char::from_u32('a' as u32 + ordinal - 1).map(|c| c.to_string())
            }

            Self::AlphaListCapital(_) => {
                // 24 -> "X"
                char::from_u32('A' as u32 + ordinal - 1).map(|c| c.to_string())
            }

            Self::ArabicNumeral(_) => {
                // 7 -> "7"
                Some(ordinal.to_string())
            }

            Self::RomanNumeralLower(_) => {
                // 17 -> "xvii"
                Some(to_roman_numeral_lower(ordinal))
            }

            // Implicit markers don't have ordinal display forms.
            _ => None,
        }
    }
}

impl<'src> HasSpan<'src> for ListItemMarker<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Self::Hyphen(x) => *x,
            Self::Asterisks(x) => *x,
            Self::Bullet(x) => *x,
            Self::Dots(x) => *x,
            Self::AlphaListCapital(x) => *x,
            Self::AlphaListLower(x) => *x,
            Self::RomanNumeralLower(x) => *x,
            Self::ArabicNumeral(x) => *x,

            Self::DefinedTerm {
                term: _,
                marker: _,
                source,
            } => *source,
        }
    }
}

impl std::fmt::Debug for ListItemMarker<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hyphen(x) => f.debug_tuple("ListItemMarker::Hyphen").field(x).finish(),
            Self::Asterisks(x) => f.debug_tuple("ListItemMarker::Asterisks").field(x).finish(),
            Self::Bullet(x) => f.debug_tuple("ListItemMarker::Bullet").field(x).finish(),
            Self::Dots(x) => f.debug_tuple("ListItemMarker::Dots").field(x).finish(),

            Self::AlphaListCapital(x) => f
                .debug_tuple("ListItemMarker::AlphaListCapital")
                .field(x)
                .finish(),

            Self::AlphaListLower(x) => f
                .debug_tuple("ListItemMarker::AlphaListLower")
                .field(x)
                .finish(),

            Self::RomanNumeralLower(x) => f
                .debug_tuple("ListItemMarker::RomanNumeralLower")
                .field(x)
                .finish(),

            Self::ArabicNumeral(x) => f
                .debug_tuple("ListItemMarker::ArabicNumeral")
                .field(x)
                .finish(),

            Self::DefinedTerm {
                term,
                marker,
                source,
            } => f
                .debug_struct("ListItemMarker::DefinedTerm")
                .field("term", term)
                .field("marker", marker)
                .field("source", source)
                .finish(),
        }
    }
}

static LIST_ITEM_MARKER: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)    
            ^(                      # Capture group for list marker
                -                       # Hyphen (unordered list)
                |\*+                    # One or more asterisks (unordered list, up to 5 levels)
                |\.+                    # One or more dots (ordered list, up to 5 levels)
                |\u{2022}               # Bullet character • (unordered list)
                |\d+\.                  # Digits followed by dot (numbered list)
                |[a-zA-Z]\.             # Letter followed by dot (alpha list)
                |[IVXivx]+\)            # Roman numerals followed by ) (Roman list)
            )
            [\ \t]                  # Required whitespace after marker
        "#,
    )
    .unwrap()
});

static DESCRIPTION_LIST_MARKER: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)
            ^                       # Start of line
            (                       # Capture group 1: Term being defined
                [^\ \t]                 # At least one non-whitespace character (start of term)
                .*?                     # Any characters (rest of term, non-greedy)
            )
            (?::::?:?|;;)           # Delimiter: ::, :::, ::::, or ;;
            (?:$|[\ \t])            # End of line or whitespace after marker
        "#,
    )
    .unwrap()
});

/// Parses a lowercase Roman numeral string into its numeric value.
fn parse_roman_numeral(s: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut prev_value: u32 = 0;

    for ch in s.chars().rev() {
        let value = match ch {
            'i' | 'I' => 1,
            'v' | 'V' => 5,
            'x' | 'X' => 10,
            'l' | 'L' => 50,
            'c' | 'C' => 100,
            'd' | 'D' => 500,
            'm' | 'M' => 1000,
            _ => return None,
        };

        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }
        prev_value = value;
    }

    if result > 0 { Some(result) } else { None }
}

/// Converts a numeric value to a lowercase Roman numeral string.
fn to_roman_numeral_lower(mut n: u32) -> String {
    const NUMERALS: &[(u32, &str)] = &[
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ];

    let mut result = String::new();
    for &(value, numeral) in NUMERALS {
        while n >= value {
            result.push_str(numeral);
            n -= value;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]
    #![allow(clippy::unwrap_used)]

    use pretty_assertions_sorted::assert_eq;

    use crate::{HasSpan, span::MatchedItem, tests::prelude::*};

    fn lim_parse<'a>(
        source: &'a str,
    ) -> Option<MatchedItem<'a, crate::blocks::ListItemMarker<'a>>> {
        crate::blocks::ListItemMarker::parse(crate::Span::new(source))
    }

    #[test]
    fn hyphen() {
        assert!(lim_parse("-").is_none());
        assert!(lim_parse("-- x").is_none());

        let lim = lim_parse("- blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Hyphen(Span {
                data: "-",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "-",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Hyphen(\n    Span {\n        data: \"-\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );
    }

    #[test]
    fn asterisks() {
        assert!(lim_parse("*").is_none());
        assert!(lim_parse("*- x").is_none());

        let lim = lim_parse("* blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Asterisks(Span {
                data: "*",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "*",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Asterisks(\n    Span {\n        data: \"*\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );

        let lim = lim_parse("***** blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Asterisks(Span {
                data: "*****",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 7,
                offset: 6,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "*****",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Asterisks(\n    Span {\n        data: \"*****\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );
    }

    #[test]
    fn dots() {
        assert!(lim_parse(".").is_none());
        assert!(lim_parse(".- x").is_none());

        let lim = lim_parse(". blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Dots(Span {
                data: ".",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: ".",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Dots(\n    Span {\n        data: \".\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );

        let lim = lim_parse("..... blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Dots(Span {
                data: ".....",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 7,
                offset: 6,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: ".....",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Dots(\n    Span {\n        data: \".....\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );
    }

    #[test]
    fn roman_numeral_lower() {
        assert!(lim_parse("i").is_none());
        assert!(lim_parse("i.").is_none());

        let lim = lim_parse("i) blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::RomanNumeralLower(Span {
                data: "i)",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 4,
                offset: 3,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "i)",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::RomanNumeralLower(\n    Span {\n        data: \"i)\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );

        let lim = lim_parse("xvii) blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::RomanNumeralLower(Span {
                data: "xvii)",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 7,
                offset: 6,
            }
        );
    }

    #[test]
    fn alpha_list_lower() {
        assert!(lim_parse("a").is_none());
        assert!(lim_parse("a)").is_none());

        let lim = lim_parse("a. blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::AlphaListLower(Span {
                data: "a.",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 4,
                offset: 3,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "a.",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::AlphaListLower(\n    Span {\n        data: \"a.\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );

        let lim = lim_parse("x. blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::AlphaListLower(Span {
                data: "x.",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }
}
