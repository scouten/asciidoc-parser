use std::fmt;

use crate::{
    HasSpan,
    blocks::{IsBlock, MacroBlock},
    tests::fixtures::{TSpan, attributes::TAttrlist},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TMacroBlock {
    pub name: TSpan,
    pub target: Option<TSpan>,
    pub macro_attrlist: TAttrlist,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub anchor: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TMacroBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MacroBlock")
            .field("name", &self.name)
            .field("target", &self.target)
            .field("macro_attrlist", &self.macro_attrlist)
            .field("source", &self.source)
            .field("anchor", &self.anchor)
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

fn fixture_eq_observed(fixture: &TMacroBlock, observed: &MacroBlock) -> bool {
    if &fixture.name != observed.name() {
        return false;
    }

    if fixture.target.is_some() != observed.target().is_some() {
        return false;
    }

    if let Some(ref fixture_target) = fixture.target
        && let Some(ref observed_target) = observed.target()
        && &fixture_target != observed_target
    {
        return false;
    }

    if fixture.macro_attrlist != *observed.macro_attrlist() {
        return false;
    }

    if fixture.title.is_some() != observed.title().is_some() {
        return false;
    }

    if let Some(ref fixture_title) = fixture.title
        && let Some(ref observed_title) = observed.title()
        && fixture_title != observed_title
    {
        return false;
    }

    if fixture.anchor.is_some() != observed.anchor().is_some() {
        return false;
    }

    if let Some(ref tcdb_anchor) = fixture.anchor
        && let Some(ref cdb_anchor) = observed.anchor()
        && tcdb_anchor != cdb_anchor
    {
        return false;
    }

    if fixture.attrlist.is_some() != observed.attrlist().is_some() {
        return false;
    }

    if let Some(ref fixture_attrlist) = fixture.attrlist
        && let Some(ref observed_attrlist) = observed.attrlist()
        && &fixture_attrlist != observed_attrlist
    {
        return false;
    }

    &fixture.source == observed.span()
}
