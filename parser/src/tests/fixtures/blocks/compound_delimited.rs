use std::fmt;

use crate::{
    HasSpan,
    blocks::{CompoundDelimitedBlock, IsBlock},
    tests::fixtures::{TSpan, attributes::TAttrlist, blocks::TBlock},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TCompoundDelimitedBlock {
    pub blocks: &'static [TBlock],
    pub context: &'static str,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub anchor: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TCompoundDelimitedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompoundDelimitedBlock")
            .field("blocks", &self.blocks)
            .field("context", &self.context)
            .field("source", &self.source)
            .field("title", &self.title)
            .field("anchor", &self.anchor)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

impl<'src> PartialEq<CompoundDelimitedBlock<'src>> for TCompoundDelimitedBlock {
    fn eq(&self, other: &CompoundDelimitedBlock<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TCompoundDelimitedBlock> for CompoundDelimitedBlock<'_> {
    fn eq(&self, other: &TCompoundDelimitedBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(
    fixture: &TCompoundDelimitedBlock,
    observed: &CompoundDelimitedBlock,
) -> bool {
    if fixture.blocks.len() != observed.nested_blocks().len() {
        return false;
    }

    for (fixture_block, observed_block) in fixture.blocks.iter().zip(observed.nested_blocks()) {
        if fixture_block != observed_block {
            return false;
        }
    }

    if fixture.context != observed.raw_context().as_ref() {
        return false;
    }

    if fixture.title.is_some() != observed.title().is_some() {
        return false;
    }

    if let Some(ref tcdb_title) = fixture.title
        && let Some(ref cdb_title) = observed.title()
        && tcdb_title != cdb_title
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

    if let Some(ref tcdb_attrlist) = fixture.attrlist
        && let Some(ref cdb_attrlist) = observed.attrlist()
        && &tcdb_attrlist != cdb_attrlist
    {
        return false;
    }

    fixture.source == observed.span()
}
