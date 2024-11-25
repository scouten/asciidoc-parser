use std::fmt;

use crate::{
    blocks::{IsBlock, SimpleBlock},
    tests::fixtures::{inlines::TInline, TSpan},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TSimpleBlock {
    pub inline: TInline,
    pub title: Option<TSpan>,
}

impl fmt::Debug for TSimpleBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleBlock")
            .field("inline", &self.inline)
            .field("title", &self.title)
            .finish()
    }
}

impl<'src> PartialEq<SimpleBlock<'src>> for TSimpleBlock {
    fn eq(&self, other: &SimpleBlock<'src>) -> bool {
        tsimple_block_eq(self, other)
    }
}

impl<'src> PartialEq<TSimpleBlock> for SimpleBlock<'src> {
    fn eq(&self, other: &TSimpleBlock) -> bool {
        tsimple_block_eq(other, self)
    }
}

fn tsimple_block_eq(tsimple_block: &TSimpleBlock, simple_block: &SimpleBlock) -> bool {
    if tsimple_block.title.is_some() != simple_block.title().is_some() {
        return false;
    }

    if let Some(ref tsb_title) = tsimple_block.title {
        if let Some(ref sb_title) = simple_block.title() {
            if tsb_title != sb_title {
                return false;
            }
        }
    }

    &tsimple_block.inline == simple_block.inline()
}
