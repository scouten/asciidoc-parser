use std::fmt;

use crate::{
    blocks::{IsBlock, MacroBlock},
    tests::fixtures::{attributes::TAttrlist, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TMacroBlock {
    pub name: TSpan,
    pub target: Option<TSpan>,
    pub macro_attrlist: TAttrlist,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TMacroBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MacroBlock")
            .field("name", &self.name)
            .field("target", &self.target)
            .field("macro_attrlist", &self.macro_attrlist)
            .field("source", &self.source)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

impl<'src> PartialEq<MacroBlock<'src>> for TMacroBlock {
    fn eq(&self, other: &MacroBlock<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TMacroBlock> for MacroBlock<'_> {
    fn eq(&self, other: &TMacroBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TMacroBlock, macro_block: &MacroBlock) -> bool {
    if &fixture.name != macro_block.name() {
        return false;
    }

    if fixture.target.is_some() != macro_block.target().is_some() {
        return false;
    }

    if let Some(ref th_target) = fixture.target {
        if let Some(ref h_target) = macro_block.target() {
            if &th_target != h_target {
                return false;
            }
        }
    }

    if fixture.macro_attrlist != *macro_block.macro_attrlist() {
        return false;
    }

    if fixture.title.is_some() != macro_block.title().is_some() {
        return false;
    }

    if let Some(ref tm_title) = fixture.title {
        if let Some(ref m_title) = macro_block.title() {
            if tm_title != m_title {
                return false;
            }
        }
    }

    if fixture.attrlist.is_some() != macro_block.attrlist().is_some() {
        return false;
    }

    if let Some(ref tm_attrlist) = fixture.attrlist {
        if let Some(ref m_attrlist) = macro_block.attrlist() {
            if &tm_attrlist != m_attrlist {
                return false;
            }
        }
    }

    &fixture.source == macro_block.span()
}
