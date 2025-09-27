use std::{cmp::PartialEq, fmt};

use crate::{
    HasSpan,
    tests::fixtures::{
        Span,
        document::{Attribute, AuthorLine},
    },
};

#[derive(Eq, PartialEq)]
pub(crate) struct Header {
    pub title_source: Option<Span>,
    pub title: Option<&'static str>,
    pub attributes: &'static [Attribute],
    pub author_line: Option<AuthorLine>,
    pub comments: &'static [Span],
    pub source: Span,
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("title_source", &self.title_source)
            .field("title", &self.title)
            .field("attributes", &self.attributes)
            .field("author_line", &self.author_line)
            .field("comments", &self.comments)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<crate::document::Header<'src>> for Header {
    fn eq(&self, other: &crate::document::Header<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Header> for crate::document::Header<'_> {
    fn eq(&self, other: &Header) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<Header> for &crate::document::Header<'_> {
    fn eq(&self, other: &Header) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Header, observed: &crate::document::Header) -> bool {
    if fixture.source != observed.span() {
        return false;
    }

    if fixture.title_source.is_some() != observed.title_source().is_some() {
        return false;
    }

    if let Some(ref fixture_title_source) = fixture.title_source
        && let Some(ref observed_title_source) = observed.title_source()
        && fixture_title_source != observed_title_source
    {
        return false;
    }

    if fixture.title.is_some() != observed.title().is_some() {
        return false;
    }

    if let Some(ref fixture_title) = fixture.title
        && let Some(ref observed_title) = observed.title()
        && fixture_title != observed_title
    {
        return false;
    }

    if fixture.attributes.len() != observed.attributes().len() {
        return false;
    }

    for (fixture_attribute, observed_attribute) in
        fixture.attributes.iter().zip(observed.attributes())
    {
        if fixture_attribute != observed_attribute {
            return false;
        }
    }

    if fixture.author_line.is_some() != observed.author_line().is_some() {
        return false;
    }

    if let Some(fixture_author_line) = &fixture.author_line
        && let Some(observed_author_line) = observed.author_line()
        && fixture_author_line != observed_author_line
    {
        return false;
    }

    if fixture.comments.len() != observed.comments().len() {
        return false;
    }

    for (fixture_comment, observed_comment) in fixture.comments.iter().zip(observed.comments()) {
        if fixture_comment != observed_comment {
            return false;
        }
    }

    true
}
