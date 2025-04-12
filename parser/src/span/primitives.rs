use super::{MatchedItem, Span};

impl<'src> Span<'src> {
    /// Split the span, consuming an identifier if found.
    ///
    /// IMPORTANT: This function, is not quite deprecated yet, but its use is
    /// strongly discouraged. The concept of "identifier" is not crisply defined
    /// in the Asciidoc documentation, so – for now – we're borrowing the
    /// definition from Rust, which is a single alphabetic character or
    /// underscore, followed by any number of alphanumeric characters or
    /// underscores.
    ///
    /// Please use more specific functions, such as `take_attr_name` or
    /// `take_user_attr_name`, when possible.
    pub(crate) fn take_ident(self) -> Option<MatchedItem<'src, Self>> {
        let mut chars = self.data.char_indices();

        let (_, c) = chars.next()?;
        if !c.is_ascii_alphabetic() && c != '_' {
            return None;
        }

        for (index, c) in chars {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return Some(self.into_parse_result(index));
            }
        }

        Some(self.into_parse_result(self.len()))
    }

    /// Split the span, consuming an attribute name if found.
    ///
    /// An [attribute name] consists of a word character (letter or numeral)
    /// followed by any number of word or `-` characters (e.g., `see-also`).
    ///
    /// [attribute name]: https://docs.asciidoctor.org/asciidoc/latest/attributes/positional-and-named-attributes/#attribute-list-parsing
    pub(crate) fn take_attr_name(self) -> Option<MatchedItem<'src, Self>> {
        let mut chars = self.data.char_indices();

        let (_, c) = chars.next()?;
        if !c.is_ascii_alphanumeric() {
            return None;
        }

        for (index, c) in chars {
            if !c.is_ascii_alphanumeric() && c != '-' {
                return Some(self.into_parse_result(index));
            }
        }

        Some(self.into_parse_result(self.len()))
    }

    /// Split the span, consuming a user-defined attribute name if found.
    ///
    /// [User-defined attribute names] must:
    ///
    /// * be at least one character long,
    /// * begin with a word character (`a`-`z`, `0`-`9`, or `_`), and
    /// * only contain word characters and hyphens (`-`).
    ///
    /// A user-defined attribute name cannot contain dots (`.`) or spaces.
    /// Although uppercase characters are permitted in an attribute name, the
    /// name is converted to lowercase before being stored. For example,
    /// `URL-REPO` and `URL-Repo` are treated as `url-repo` when a document is
    /// loaded or converted. A best practice is to only use lowercase letters in
    /// the name and avoid starting the name with a number.
    ///
    /// IMPORTANT: This function does _not_ perform the lower-case normalization
    /// defined above.
    ///
    /// [User-defined attribute names]: https://docs.asciidoctor.org/asciidoc/latest/attributes/names-and-values/#user-defined
    pub(crate) fn take_user_attr_name(self) -> Option<MatchedItem<'src, Self>> {
        let mut chars = self.data.char_indices();

        let (_, c) = chars.next()?;
        if !c.is_ascii_alphabetic() && c != '_' {
            return None;
        }

        for (index, c) in chars {
            if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
                return Some(self.into_parse_result(index));
            }
        }

        Some(self.into_parse_result(self.len()))
    }

    /// Returns [`true`] if the span properly forms an [XML Name].
    ///
    /// [XML Name]: https://www.w3.org/TR/REC-xml/#NT-Name
    pub(crate) fn is_xml_name(self) -> bool {
        let mut chars = self.data.chars();

        if let Some(c) = chars.next() {
            match c {
                ':' | '_' => (),
                '\u{037e}' => {
                    return false;
                }
                _ => {
                    if !c.is_alphabetic() {
                        return false;
                    }
                }
            }
        } else {
            return false;
        }

        chars.all(|c| match c {
            ':' | '_' | '.' | '-' | '\u{00b7}' => true,
            '\u{037e}' => false,
            _ => c.is_alphanumeric(),
        })
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
    /// **IMPORTANT:** The [`Span`] that is returned does not include the start
    /// or ending quote, but _does_ include (without transformation) any
    /// escaped quotes.
    pub(crate) fn take_quoted_string(self) -> Option<MatchedItem<'src, Self>> {
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
                return Some(MatchedItem {
                    item: self.slice(1..index),
                    after: self.slice_from(index + 1..),
                });
            }
            prev_was_backslash = c == '\\';
        }

        // Didn't find closing delimiter.
        None
    }

    /// Given a second [`Span`], which must be a trailing remainder of `self`,
    /// return the portion of `self` that excludes the second (remainder).
    ///
    /// Note that the trailing remainder condition is not enforced.
    pub(crate) fn trim_remainder(self, after: Span<'src>) -> Span<'src> {
        let offset = (self.offset + self.len()).min(after.offset);

        if offset <= self.offset {
            // Invalid input: We'll respond with an empty slice.
            self.slice(0..0)
        } else {
            let trim_len = offset - self.offset;
            self.slice(0..trim_len)
        }
    }
}
