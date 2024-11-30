use std::fmt;

use crate::{
    blocks::{ContentModel, IsBlock, RawDelimitedBlock},
    tests::fixtures::{attributes::TAttrlist, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TRawDelimitedBlock {
    pub lines: Vec<TSpan>,
    pub content_model: ContentModel,
    pub context: &'static str,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TRawDelimitedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawDelimitedBlock")
            .field("lines", &self.lines)
            .field("content_model", &self.content_model)
            .field("context", &self.context)
            .field("source", &self.source)
            .field("title", &self.title)
            .field("attrlist", &self.attrlist)
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

fn fixture_eq_observed(
    fixture: &TRawDelimitedBlock,
    observed: &RawDelimitedBlock,
) -> bool {
    if fixture.lines.len() != observed.lines().len() {
        return false;
    }

    for (td_line, line) in fixture.lines.iter().zip(observed.lines()) {
        if td_line != line {
            return false;
        }
    }

    if fixture.content_model != observed.content_model() {
        return false;
    }

    if fixture.context != observed.context().as_ref() {
        return false;
    }

    if fixture.title.is_some() != observed.title().is_some() {
        return false;
    }

    if let Some(ref trdb_title) = fixture.title {
        if let Some(ref rdb_title) = observed.title() {
            if trdb_title != rdb_title {
                return false;
            }
        }
    }

    if fixture.attrlist.is_some() != observed.attrlist().is_some() {
        return false;
    }

    if let Some(ref trdb_attrlist) = fixture.attrlist {
        if let Some(ref rdb_attrlist) = observed.attrlist() {
            if &trdb_attrlist != rdb_attrlist {
                return false;
            }
        }
    }

    &fixture.source == observed.span()
}
