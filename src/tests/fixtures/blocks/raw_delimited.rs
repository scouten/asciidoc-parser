use std::fmt;

use crate::{
    blocks::{ContentModel, IsBlock, RawDelimitedBlock},
    tests::fixtures::{attributes::TAttrlist, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TRawDelimitedBlock {
    pub lines: Vec<TSpan>,
    pub content_model: ContentModel,
    pub context: &'static str,
    pub source: TSpan,
    pub title: Option<TSpan>,
    pub attrlist: Option<TAttrlist>,
}

impl fmt::Debug for TRawDelimitedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawDelimitedBlock")
            .field("lines", &self.lines)
            .field("content_model", &self.content_model)
            .field("context", &self.context)
            .field("source", &self.source)
            .field("title", &self.title)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

impl<'src> PartialEq<RawDelimitedBlock<'src>> for TRawDelimitedBlock {
    fn eq(&self, other: &RawDelimitedBlock<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TRawDelimitedBlock> for RawDelimitedBlock<'_> {
    fn eq(&self, other: &TRawDelimitedBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(
    traw_delimited_block: &TRawDelimitedBlock,
    raw_delimited_block: &RawDelimitedBlock,
) -> bool {
    if traw_delimited_block.lines.len() != raw_delimited_block.lines().len() {
        return false;
    }

    for (td_line, line) in traw_delimited_block
        .lines
        .iter()
        .zip(raw_delimited_block.lines())
    {
        if td_line != line {
            return false;
        }
    }

    if traw_delimited_block.content_model != raw_delimited_block.content_model() {
        return false;
    }

    if traw_delimited_block.context != raw_delimited_block.context().as_ref() {
        return false;
    }

    if traw_delimited_block.title.is_some() != raw_delimited_block.title().is_some() {
        return false;
    }

    if let Some(ref trdb_title) = traw_delimited_block.title {
        if let Some(ref rdb_title) = raw_delimited_block.title() {
            if trdb_title != rdb_title {
                return false;
            }
        }
    }

    if traw_delimited_block.attrlist.is_some() != raw_delimited_block.attrlist().is_some() {
        return false;
    }

    if let Some(ref trdb_attrlist) = traw_delimited_block.attrlist {
        if let Some(ref rdb_attrlist) = raw_delimited_block.attrlist() {
            if &trdb_attrlist != rdb_attrlist {
                return false;
            }
        }
    }

    &traw_delimited_block.source == raw_delimited_block.span()
}
