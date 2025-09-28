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
            .split(';')
            .filter_map(|raw_author| Author::parse(raw_author, parser))
            .collect();

        Self { authors, source }
    }

    /// Return an iterator over the authors in this author line.
    pub fn authors(&'src self) -> Iter<'src, Author> {
        self.authors.iter()
    }
}

impl<'src> HasSpan<'src> for AuthorLine<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}
