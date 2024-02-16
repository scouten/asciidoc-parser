use std::fmt;

use crate::{blocks::SimpleBlock, tests::fixtures::inlines::TInline};

#[derive(Eq, PartialEq)]
pub(crate) struct TSimpleBlock(pub TInline);

impl fmt::Debug for TSimpleBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SimpleBlock").field(&self.0).finish()
    }
}

impl<'a> PartialEq<SimpleBlock<'a>> for TSimpleBlock {
    fn eq(&self, other: &SimpleBlock<'a>) -> bool {
        tsimple_block_eq(self, other)
    }
}

impl<'a> PartialEq<TSimpleBlock> for SimpleBlock<'a> {
    fn eq(&self, other: &TSimpleBlock) -> bool {
        tsimple_block_eq(other, self)
    }
}

fn tsimple_block_eq(tsimple_block: &TSimpleBlock, simple_block: &SimpleBlock) -> bool {
    &tsimple_block.0 == simple_block.inline()
}
