use std::fmt;

use crate::{
    blocks::{IsBlock, SectionBlock},
    tests::fixtures::{blocks::TBlock, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TSectionBlock {
    pub level: usize,
    pub title: TSpan,
    pub blocks: Vec<TBlock>,
    pub source: TSpan,
}

impl fmt::Debug for TSectionBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionBlock")
            .field("level", &self.level)
            .field("title", &self.title)
            .field("blocks", &self.blocks)
            .field("source", &self.source)
            .finish()
    }
}

impl<'a> PartialEq<SectionBlock<'a>> for TSectionBlock {
    fn eq(&self, other: &SectionBlock<'a>) -> bool {
        tsection_block_eq(self, other)
    }
}

impl<'a> PartialEq<TSectionBlock> for SectionBlock<'a> {
    fn eq(&self, other: &TSectionBlock) -> bool {
        tsection_block_eq(other, self)
    }
}

fn tsection_block_eq(tsection_block: &TSectionBlock, section_block: &SectionBlock) -> bool {
    if tsection_block.level != section_block.level() {
        return false;
    }

    if &tsection_block.title != section_block.title() {
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

    &tsection_block.source == section_block.span()
}
