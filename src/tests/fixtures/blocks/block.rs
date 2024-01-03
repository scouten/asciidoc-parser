use crate::{
    blocks::Block,
    tests::fixtures::blocks::{TMacroBlock, TSimpleBlock},
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TBlock {
    Simple(TSimpleBlock),
    Macro(TMacroBlock),
}

impl<'a> PartialEq<Block<'a>> for TBlock {
    fn eq(&self, other: &Block<'a>) -> bool {
        tblock_eq(self, other)
    }
}

impl<'a> PartialEq<TBlock> for Block<'a> {
    fn eq(&self, other: &TBlock) -> bool {
        tblock_eq(other, self)
    }
}

fn tblock_eq(tblock: &TBlock, block: &Block) -> bool {
    match tblock {
        TBlock::Simple(ref tsimple_block) => match block {
            Block::Simple(ref simple_block) => tsimple_block == simple_block,
            _ => false,
        },

        TBlock::Macro(ref tmacro_block) => match block {
            Block::Macro(ref macro_block) => tmacro_block == macro_block,
            _ => false,
        },
    }
}
