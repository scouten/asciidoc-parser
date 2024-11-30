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
        tsection_block_eq(self, other)
    }
}

impl PartialEq<TSectionBlock> for SectionBlock<'_> {
    fn eq(&self, other: &TSectionBlock) -> bool {
        tsection_block_eq(other, self)
    }
}

fn tsection_block_eq(tsection_block: &TSectionBlock, section_block: &SectionBlock) -> bool {
    if tsection_block.level != section_block.level() {
        return false;
    }

    if &tsection_block.section_title != section_block.section_title() {
        return false;
    }

    if tsection_block.blocks.len() != section_block.nested_blocks().len() {
        return false;
    }

    for (td_block, block) in tsection_block
        .blocks
        .iter()
        .zip(section_block.nested_blocks())
    {
        if td_block != block {
            return false;
        }
    }

    if tsection_block.title.is_some() != section_block.title().is_some() {
        return false;
    }

    if let Some(ref tsb_title) = tsection_block.title {
        if let Some(ref sb_title) = section_block.title() {
            if tsb_title != sb_title {
                return false;
            }
        }
    }

    if tsection_block.attrlist.is_some() != section_block.attrlist().is_some() {
        return false;
    }

    if let Some(ref tsb_attrlist) = tsection_block.attrlist {
        if let Some(ref sb_attrlist) = section_block.attrlist() {
            if &tsb_attrlist != sb_attrlist {
                return false;
            }
        }
    }

    &tsection_block.source == section_block.span()
}
