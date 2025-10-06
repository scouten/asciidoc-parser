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
        let authors: Vec<Author> = source
            .data()
            .split("; ")
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
