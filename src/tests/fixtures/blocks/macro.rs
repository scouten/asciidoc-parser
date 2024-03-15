use std::fmt;

use crate::{
    blocks::MacroBlock,
    tests::fixtures::{attributes::TAttrlist, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TMacroBlock {
    pub name: TSpan,
    pub target: Option<TSpan>,
    pub attrlist: TAttrlist,
    pub source: TSpan,
}

impl fmt::Debug for TMacroBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MacroBlock")
            .field("name", &self.name)
            .field("target", &self.target)
            .field("attrlist", &self.attrlist)
            .field("source", &self.source)
            .finish()
    }
}

impl<'a> PartialEq<MacroBlock<'a>> for TMacroBlock {
    fn eq(&self, other: &MacroBlock<'a>) -> bool {
        tmacro_block_eq(self, other)
    }
}

impl<'a> PartialEq<TMacroBlock> for MacroBlock<'a> {
    fn eq(&self, other: &TMacroBlock) -> bool {
        tmacro_block_eq(other, self)
    }
}

fn tmacro_block_eq(tmacro_block: &TMacroBlock, macro_block: &MacroBlock) -> bool {
    if &tmacro_block.name != macro_block.name() {
        return false;
    }

    if tmacro_block.target.is_some() != macro_block.target().is_some() {
        return false;
    }

    if let Some(ref th_target) = tmacro_block.target {
        if let Some(ref h_target) = macro_block.target() {
            if &th_target != h_target {
                return false;
            }
        }
    }

    if tmacro_block.attrlist != *macro_block.attrlist() {
        return false;
    }

    &tmacro_block.source == macro_block.span()
}
