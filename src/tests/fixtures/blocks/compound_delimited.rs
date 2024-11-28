use std::fmt;

use crate::{
    blocks::{CompoundDelimitedBlock, IsBlock},
    tests::fixtures::{blocks::TBlock, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TCompoundDelimitedBlock {
    pub blocks: Vec<TBlock>,
    pub context: &'static str,
    pub source: TSpan,
}

impl fmt::Debug for TCompoundDelimitedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompoundDelimitedBlock")
            .field("blocks", &self.blocks)
            .field("context", &self.context)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<CompoundDelimitedBlock<'src>> for TCompoundDelimitedBlock {
    fn eq(&self, other: &CompoundDelimitedBlock<'src>) -> bool {
        tcompound_delimited_block_eq(self, other)
    }
}

impl PartialEq<TCompoundDelimitedBlock> for CompoundDelimitedBlock<'_> {
    fn eq(&self, other: &TCompoundDelimitedBlock) -> bool {
        tcompound_delimited_block_eq(other, self)
    }
}

fn tcompound_delimited_block_eq(
    tcompound_delimited_block: &TCompoundDelimitedBlock,
    compound_delimited_block: &CompoundDelimitedBlock,
) -> bool {
    if tcompound_delimited_block.blocks.len() != compound_delimited_block.nested_blocks().len() {
        return false;
    }

    for (td_block, block) in tcompound_delimited_block
        .blocks
        .iter()
        .zip(compound_delimited_block.nested_blocks())
    {
        if td_block != block {
            return false;
        }
    }

    if tcompound_delimited_block.context != compound_delimited_block.context().as_ref() {
        return false;
    }

    &tcompound_delimited_block.source == compound_delimited_block.span()
}
