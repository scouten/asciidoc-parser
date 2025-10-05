use std::slice::Iter;

use crate::{HasSpan, Parser, Span, document::Author};

/// The author line is directly after the document title line in the document
/// header. When the content on this line is structured correctly, the processor
/// assigns the content to the built-in author and email attributes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorLine<'src> {
    authors: Vec<Author>,
    source: Span<'src>,
}

impl<'src> AuthorLine<'src> {
    pub(crate) fn parse(source: Span<'src>, parser: &mut Parser) -> Self {
        let authors: Vec<Author> = split_authors_on_semicolon_whitespace(source.data())
            .filter_map(|raw_author| Author::parse(raw_author, parser))
            .collect();

        for (index, author) in authors.iter().enumerate() {
            set_nth_attribute(parser, "author", index, author.name());
            set_nth_attribute(parser, "authorinitials", index, author.initials());
            set_nth_attribute(parser, "firstname", index, author.firstname());

            if let Some(middlename) = author.middlename() {
                set_nth_attribute(parser, "middlename", index, middlename);
            }

            if let Some(lastname) = author.lastname() {
                set_nth_attribute(parser, "lastname", index, lastname);
            }

            if let Some(email) = author.email() {
                set_nth_attribute(parser, "email", index, email);
            }
        }

        Self { authors, source }
    }

    /// Return an iterator over the authors in this author line.
    pub fn authors(&'src self) -> Iter<'src, Author> {
        self.authors.iter()
    }
}

/// Split author line on semicolon followed by whitespace. This preserves
/// semicolons that are part of character references (e.g., `&#174;`) or
/// attribute references that escape character references (e.g.,
/// `&#174;{empty}`).
fn split_authors_on_semicolon_whitespace(input: &str) -> impl Iterator<Item = &str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ';' {
            // Check if the semicolon is followed by whitespace.
            if i + 1 < chars.len() && chars[i + 1].is_whitespace() {
                // Split here: Take everything from start to i (excluding the semicolon).
                let part = &input[start..find_byte_index(input, start, i)];
                if !part.trim().is_empty() {
                    parts.push(part.trim());
                }

                // Skip the semicolon and any following whitespace.
                i += 1;
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                }
                start = find_byte_index(input, 0, i);
                continue;
            }
        }
        i += 1;
    }

    // Add the remaining part if any.
    if start < input.len() {
        let part = &input[start..];
        if !part.trim().is_empty() {
            parts.push(part.trim());
        }
    }

    parts.into_iter()
}

/// Find the byte index in the original string corresponding to a character
/// index.
fn find_byte_index(input: &str, start_byte: usize, char_index: usize) -> usize {
    let substring = &input[start_byte..];
    let mut byte_offset = 0;

    for (current_char_index, ch) in substring.chars().enumerate() {
        if current_char_index >= char_index {
            break;
        }
        byte_offset += ch.len_utf8();
    }

    start_byte + byte_offset
}

fn set_nth_attribute<V: AsRef<str>>(parser: &mut Parser, name: &str, index: usize, value: V) {
    let name = if index == 0 {
        name.to_string()
    } else {
        format!("{name}_{count}", count = index + 1)
    };

    parser.set_attribute_by_value_from_header(name, value);
}

impl<'src> HasSpan<'src> for AuthorLine<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}
