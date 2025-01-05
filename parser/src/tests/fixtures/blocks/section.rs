use std::fmt;

use crate::{
    blocks::{IsBlock, SectionBlock},
    tests::fixtures::{attributes::TAttrlist, blocks::TBlock, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TSectionBlock {
    pub level: usize,
    pub section_title: TSpan,
    pub blocks: Vec<TBlock>,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TSectionBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionBlock")
            .field("level", &self.level)
            .field("section_title", &self.section_title)
            .field("blocks", &self.blocks)
            .field("source", &self.source)
            .field("title", &self.title)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

impl<'src> PartialEq<SectionBlock<'src>> for TSectionBlock {
    fn eq(&self, other: &SectionBlock<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TSectionBlock> for SectionBlock<'_> {
    fn eq(&self, other: &TSectionBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TSectionBlock, observed: &SectionBlock) -> bool {
    if fixture.level != observed.level() {
        return false;
    }

    if &fixture.section_title != observed.section_title() {
        return false;
    }

    if fixture.blocks.len() != observed.nested_blocks().len() {
        return false;
    }

    for (fixture_block, observed_block) in fixture.blocks.iter().zip(observed.nested_blocks()) {
        if fixture_block != observed_block {
            return false;
        }
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
}
