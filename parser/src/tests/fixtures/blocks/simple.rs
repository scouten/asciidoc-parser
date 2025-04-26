use std::fmt;

use crate::{
    blocks::{IsBlock, SimpleBlock},
    span::HasSpan,
    tests::fixtures::{attributes::TAttrlist, TContent, TSpan},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TSimpleBlock {
    pub content: TContent,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub anchor: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TSimpleBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleBlock")
            .field("content", &self.content)
            .field("source", &self.source)
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
    if fixture.title.is_some() != observed.title().is_some() {
        return false;
    }

    if let Some(ref fixture_title) = fixture.title {
        if let Some(ref observed_title) = observed.title() {
            if fixture_title != observed_title {
                return false;
            }
        }
    }

    if fixture.anchor.is_some() != observed.anchor().is_some() {
        return false;
    }

    if let Some(ref fixture_anchor) = fixture.anchor {
        if let Some(ref observed_anchor) = observed.anchor() {
            if fixture_anchor != observed_anchor {
                return false;
            }
        }
    }

    if fixture.attrlist.is_some() != observed.attrlist().is_some() {
        return false;
    }

    if let Some(ref fixture_attrlist) = fixture.attrlist {
        if let Some(ref observed_attrlist) = observed.attrlist() {
            if &fixture_attrlist != observed_attrlist {
                return false;
            }
        }
    }

    &fixture.source == observed.span() && &fixture.content == observed.content()
}
