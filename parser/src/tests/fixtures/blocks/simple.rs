use std::fmt;

use crate::{
    blocks::{IsBlock, SimpleBlock},
    span::HasSpan,
    tests::fixtures::{TSpan, attributes::TAttrlist, content::TContent},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TSimpleBlock {
    pub content: TContent,
    pub source: TSpan,
    pub title_source: Option<TSpan>,
    pub title: Option<&'static str>,
    pub anchor: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TSimpleBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleBlock")
            .field("content", &self.content)
            .field("source", &self.source)
            .field("title_source", &self.title_source)
            .field("title", &self.title)
            .field("anchor", &self.anchor)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

impl<'src> PartialEq<SimpleBlock<'src>> for TSimpleBlock {
    fn eq(&self, other: &SimpleBlock<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TSimpleBlock> for SimpleBlock<'_> {
    fn eq(&self, other: &TSimpleBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TSimpleBlock, observed: &SimpleBlock) -> bool {
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

    if fixture.anchor.is_some() != observed.anchor().is_some() {
        return false;
    }

    if let Some(ref fixture_anchor) = fixture.anchor
        && let Some(ref observed_anchor) = observed.anchor()
        && fixture_anchor != observed_anchor
    {
        return false;
    }

    if fixture.attrlist.is_some() != observed.attrlist().is_some() {
        return false;
    }

    if let Some(ref fixture_attrlist) = fixture.attrlist
        && let Some(ref observed_attrlist) = observed.attrlist()
        && &fixture_attrlist != observed_attrlist
    {
        return false;
    }

    fixture.source == observed.span() && &fixture.content == observed.content()
}
