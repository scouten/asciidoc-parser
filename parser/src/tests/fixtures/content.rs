use std::cmp::PartialEq;

use crate::{content::Content, tests::fixtures::TSpan};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TContent {
    pub original: TSpan,
    pub rendered: &'static str,
}

impl<'src> PartialEq<Content<'src>> for TContent {
    fn eq(&self, other: &Content<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TContent> for Content<'_> {
    fn eq(&self, other: &TContent) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TContent> for &Content<'_> {
    fn eq(&self, other: &TContent) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TContent, observed: &Content) -> bool {
    fixture.original == observed.original() && fixture.rendered == observed.rendered()
}
