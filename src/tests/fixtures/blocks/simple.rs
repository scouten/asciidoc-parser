use crate::{blocks::SimpleBlock, tests::fixtures::TSpan};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TSimpleBlock {
    pub inlines: Vec<TSpan>,
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
    if tsimple_block.inlines.len() != simple_block.inlines.len() {
        return false;
    }

    for (tsb_line, sb_line) in tsimple_block
        .inlines
        .iter()
        .zip(simple_block.inlines.iter())
    {
        if &tsb_line != &sb_line {
            return false;
        }
    }

    true
}
