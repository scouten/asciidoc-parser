use std::fmt;

use crate::{
    blocks::{CompoundDelimitedBlock, IsBlock},
    tests::fixtures::{attributes::TAttrlist, blocks::TBlock, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TCompoundDelimitedBlock {
    pub blocks: Vec<TBlock>,
    pub context: &'static str,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TCompoundDelimitedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompoundDelimitedBlock")
            .field("blocks", &self.blocks)
            .field("context", &self.context)
            .field("source", &self.source)
            .field("title", &self.title)
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

    for (td_block, block) in fixture
        .blocks
        .iter()
        .zip(observed.nested_blocks())
    {
        if td_block != block {
            return false;
        }
    }

    if fixture.context != observed.context().as_ref() {
        return false;
    }

    if fixture.title.is_some() != observed.title().is_some() {
        return false;
    }

    if let Some(ref tcdb_title) = fixture.title {
        if let Some(ref cdb_title) = observed.title() {
            if tcdb_title != cdb_title {
                return false;
            }
        }
    }

    if fixture.attrlist.is_some() != observed.attrlist().is_some() {
        return false;
    }

    if let Some(ref tcdb_attrlist) = fixture.attrlist {
        if let Some(ref cdb_attrlist) = observed.attrlist() {
            if &tcdb_attrlist != cdb_attrlist {
                return false;
            }
        }
    }

    &fixture.source == observed.span()
}
