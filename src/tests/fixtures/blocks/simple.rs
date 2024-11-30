use std::fmt;

use crate::{
    blocks::{IsBlock, SimpleBlock},
    span::HasSpan,
    tests::fixtures::{attributes::TAttrlist, inlines::TInline, TSpan},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TSimpleBlock {
    pub inline: TInline,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TSimpleBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleBlock")
            .field("inline", &self.inline)
            .field("source", &self.source)
            .field("title", &self.title)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

impl<'src> PartialEq<SimpleBlock<'src>> for TSimpleBlock {
    fn eq(&self, other: &SimpleBlock<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TSimpleBlock> for SimpleBlock<'_> {
    fn eq(&self, other: &TSimpleBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(tsimple_block: &TSimpleBlock, simple_block: &SimpleBlock) -> bool {
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

    if tsimple_block.attrlist.is_some() != simple_block.attrlist().is_some() {
        return false;
    }

    if let Some(ref tsb_attrlist) = tsimple_block.attrlist {
        if let Some(ref sb_attrlist) = simple_block.attrlist() {
            if &tsb_attrlist != sb_attrlist {
                return false;
            }
        }
    }

    &tsimple_block.source == simple_block.span() && &tsimple_block.inline == simple_block.inline()
}
