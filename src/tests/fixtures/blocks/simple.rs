use std::fmt;

use crate::{
    blocks::{IsBlock, SimpleBlock},
    span::HasSpan,
    tests::fixtures::{attributes::TAttrlist, inlines::TInline, TSpan},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TSimpleBlock {
    pub inline: TInline,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TSimpleBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleBlock")
            .field("inline", &self.inline)
            .field("source", &self.source)
            .field("title", &self.title)
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

    if let Some(ref tsb_title) = fixture.title {
        if let Some(ref sb_title) = observed.title() {
            if tsb_title != sb_title {
                return false;
            }
        }
    }

    if fixture.attrlist.is_some() != observed.attrlist().is_some() {
        return false;
    }

    if let Some(ref tsb_attrlist) = fixture.attrlist {
        if let Some(ref sb_attrlist) = observed.attrlist() {
            if &tsb_attrlist != sb_attrlist {
                return false;
            }
        }
    }

    &fixture.source == observed.span() && &fixture.inline == observed.inline()
}
