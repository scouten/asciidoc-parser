use std::{cmp::PartialEq, fmt};

use crate::{
    document::Header,
    tests::fixtures::{document::TAttribute, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct THeader {
    pub title: Option<TSpan>,
    pub attributes: Vec<TAttribute>,
    pub source: TSpan,
}

impl fmt::Debug for THeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
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

fn fixture_eq_observed(fixture: &THeader, header: &Header) -> bool {
    if &fixture.source != header.span() {
        return false;
    }

    if fixture.title.is_some() != header.title().is_some() {
        return false;
    }

    if let Some(ref th_title) = fixture.title {
        if let Some(ref h_title) = header.title() {
            if th_title != h_title {
                return false;
            }
        }
    }

    if fixture.attributes.len() != header.attributes().len() {
        return false;
    }

    for (th_attribute, attribute) in fixture.attributes.iter().zip(header.attributes()) {
        if th_attribute != attribute {
            return false;
        }
    }

    true
}
