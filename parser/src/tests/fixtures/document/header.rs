use std::{cmp::PartialEq, fmt};

use crate::{
    HasSpan,
    document::Header,
    tests::fixtures::{Span, document::Attribute},
};

#[derive(Eq, PartialEq)]
pub(crate) struct THeader {
    pub title_source: Option<Span>,
    pub title: Option<&'static str>,
    pub attributes: &'static [Attribute],
    pub source: Span,
}

impl fmt::Debug for THeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("title_source", &self.title_source)
            .field("title", &self.title)
            .field("attributes", &self.attributes)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<Header<'src>> for THeader {
    fn eq(&self, other: &Header<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<THeader> for Header<'_> {
    fn eq(&self, other: &THeader) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<THeader> for &Header<'_> {
    fn eq(&self, other: &THeader) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &THeader, observed: &Header) -> bool {
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

    true
}
