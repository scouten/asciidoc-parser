use std::fmt;

use crate::{
    blocks::{ContentModel, IsBlock, RawDelimitedBlock},
    tests::fixtures::TSpan,
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TRawDelimitedBlock {
    pub lines: Vec<TSpan>,
    pub content_model: ContentModel,
    pub context: &'static str,
    pub source: TSpan,
    pub title: Option<TSpan>,
}

impl fmt::Debug for TRawDelimitedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawDelimitedBlock")
            .field("lines", &self.lines)
            .field("content_model", &self.content_model)
            .field("context", &self.context)
            .field("source", &self.source)
            .field("title", &self.title)
            .finish()
    }
}

impl<'src> PartialEq<RawDelimitedBlock<'src>> for TRawDelimitedBlock {
    fn eq(&self, other: &RawDelimitedBlock<'src>) -> bool {
        traw_delimited_block_eq(self, other)
    }
}

impl<'src> PartialEq<TRawDelimitedBlock> for RawDelimitedBlock<'src> {
    fn eq(&self, other: &TRawDelimitedBlock) -> bool {
        traw_delimited_block_eq(other, self)
    }
}

fn traw_delimited_block_eq(
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

    &traw_delimited_block.source == raw_delimited_block.span()
}
