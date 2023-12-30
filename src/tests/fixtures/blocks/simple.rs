use std::fmt;

use crate::{blocks::SimpleBlock, tests::fixtures::TSpan, HasSpan};

#[derive(Eq, PartialEq)]
pub(crate) struct TSimpleBlock {
    pub inlines: Vec<TSpan>,
    pub source: TSpan,
}

impl fmt::Debug for TSimpleBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Intentionally mimic the output of nom_span::Spanned
        // so diffs point the unit test author to the important
        // differences.
        f.debug_struct("SimpleBlock")
            .field("inlines", &self.inlines)
            .field("source", &self.source)
            .finish()
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
    if tsimple_block.inlines.len() != simple_block.inlines.len() {
        return false;
    }

    if &tsimple_block.source != simple_block.span() {
        return false;
    }

    for (tsb_line, sb_line) in tsimple_block
        .inlines
        .iter()
        .zip(simple_block.inlines.iter())
    {
        if tsb_line != sb_line {
            return false;
        }
    }

    true
}
