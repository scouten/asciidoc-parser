use nom::Slice;

use super::{ParseResult, Span};

impl<'a> Span<'a> {
    /// Split the span, consuming an attribute name if found.
    ///
    /// An attribute name consists of a word character (letter or numeral)
    /// followed by any number of word or `-` characters (e.g., `see-also`).
    #[allow(dead_code)]
    pub(crate) fn take_attr_name(self) -> Option<ParseResult<'a, Self>> {
        let mut chars = self.data.char_indices();

        if let Some((_, c)) = chars.next() {
            if !is_alphanum(c) {
                return None;
            }
        } else {
            return None;
        }

        while let Some((index, c)) = chars.next() {
            if !is_alphanum(c) && (c != '-') {
                return Some(self.into_parse_result(index));
            }
        }

        Some(self.into_parse_result(self.len()))
    }

    /// Split the span, consuming one quoted string if found.
    ///
    /// A string is defined as a single quote or double quote character,
    /// followed by any number of other characters, and terminated by a matching
    /// single or double quote character.
    ///
    /// The single or double quote may be included in the string by preceding it
    /// with a backslash. No other backslash escape sequences are recognized.
    ///
    /// IMPORTANT: The [`Span`] that is returned does not include the start or
    /// ending quote, but _does_ include (without transformation) any escaped
    /// quotes.
    pub(crate) fn take_quoted_string(self) -> Option<ParseResult<'a, Self>> {
        let mut chars = self.data.char_indices();

        let delimiter = match chars.next() {
            Some((_, '\'')) => '\'',
            Some((_, '"')) => '"',
            _ => {
                return None;
            }
        };

        let mut prev_was_backslash = false;

        for (index, c) in chars {
            if c == delimiter && !prev_was_backslash {
                return Some(ParseResult {
                    t: self.slice(1..index),
                    rem: self.slice(index + 1..),
                });
            }
            prev_was_backslash = c == '\\';
        }

        // Didn't find closing delimiter.
        None
    }
}

#[inline]
fn is_alpha(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

#[inline]
fn is_dec_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[inline]
fn is_alphanum(c: char) -> bool {
    is_alpha(c) || is_dec_digit(c)
}
