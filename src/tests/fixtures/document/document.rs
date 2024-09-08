use std::{cmp::PartialEq, fmt};

use super::THeader;
use crate::{
    blocks::IsBlock,
    document::Document,
    tests::fixtures::{blocks::TBlock, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TDocument {
    pub header: Option<THeader>,
    pub blocks: Vec<TBlock>,
    pub source: TSpan,
}

impl fmt::Debug for TDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Document")
            .field("header", &self.header)
            .field("blocks", &self.blocks)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<Document<'src>> for TDocument {
    fn eq(&self, other: &Document<'src>) -> bool {
        tdocument_eq(self, other)
    }
}

impl<'src> PartialEq<TDocument> for Document<'src> {
    fn eq(&self, other: &TDocument) -> bool {
        tdocument_eq(other, self)
    }
}

impl<'src> PartialEq<TDocument> for &Document<'src> {
    fn eq(&self, other: &TDocument) -> bool {
        tdocument_eq(other, self)
    }
}

fn tdocument_eq(tdocument: &TDocument, document: &Document) -> bool {
    if &tdocument.source != document.span() {
        return false;
    }

    if tdocument.header.is_some() != document.header().is_some() {
        return false;
    } else if let Some(ref td_header) = tdocument.header {
        if let Some(d_header) = document.header() {
            if td_header != d_header {
                return false;
            }
        }
    }

    if tdocument.blocks.len() != document.nested_blocks().len() {
        return false;
    }

    for (td_block, block) in tdocument.blocks.iter().zip(document.nested_blocks()) {
        if td_block != block {
            return false;
        }
    }

    true
}
