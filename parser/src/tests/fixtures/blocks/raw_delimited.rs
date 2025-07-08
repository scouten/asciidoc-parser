use std::fmt;

use crate::{
    HasSpan,
    blocks::{ContentModel, IsBlock, RawDelimitedBlock},
    span::content::SubstitutionGroup,
    tests::fixtures::{TSpan, attributes::TAttrlist, content::TContent},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TRawDelimitedBlock {
    pub content: TContent,
    pub content_model: ContentModel,
    pub context: &'static str,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub anchor: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
    pub substitution_group: SubstitutionGroup,
}

impl fmt::Debug for TRawDelimitedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawDelimitedBlock")
            .field("content", &self.content)
            .field("content_model", &self.content_model)
            .field("context", &self.context)
            .field("source", &self.source)
            .field("title", &self.title)
            .field("anchor", &self.anchor)
            .field("attrlist", &self.attrlist)
            .field("substitution_group", &self.substitution_group)
            .finish()
    }
}

impl<'src> PartialEq<RawDelimitedBlock<'src>> for TRawDelimitedBlock {
    fn eq(&self, other: &RawDelimitedBlock<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TRawDelimitedBlock> for RawDelimitedBlock<'_> {
    fn eq(&self, other: &TRawDelimitedBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TRawDelimitedBlock, observed: &RawDelimitedBlock) -> bool {
    if &fixture.content != observed.content() {
        return false;
    }

    if fixture.content_model != observed.content_model() {
        return false;
    }

    if fixture.context != observed.raw_context().as_ref() {
        return false;
    }

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

    &fixture.source == observed.span()
        && fixture.substitution_group == observed.substitution_group()
}
