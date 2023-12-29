use crate::{blocks::Block, tests::fixtures::blocks::TSimpleBlock};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TBlock {
    /// A block that’s treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block).
    Simple(TSimpleBlock),
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
        TBlock::Simple(ref tsimple_block) => {
            match block {
                Block::Simple(ref simple_block) => tsimple_block == simple_block,
            }
            // _ => false ...
        }
    }
}
