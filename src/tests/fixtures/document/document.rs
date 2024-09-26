use std::{cmp::PartialEq, fmt};

use super::THeader;
use crate::{
    blocks::IsBlock,
    document::Document,
    tests::fixtures::{blocks::TBlock, warnings::TWarning, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TDocument {
    pub header: THeader,
    pub blocks: Vec<TBlock>,
    pub source: TSpan,
    pub warnings: Vec<TWarning>,
}

impl fmt::Debug for TDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Document")
            .field("header", &self.header)
            .field("blocks", &self.blocks)
            .field("source", &self.source)
            .field("warnings", &self.warnings)
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

    if &tdocument.header != document.header() {
        return false;
    }

    if tdocument.blocks.len() != document.nested_blocks().len() {
        return false;
    }

    for (td_block, block) in tdocument.blocks.iter().zip(document.nested_blocks()) {
        if td_block != block {
            return false;
        }
    }

    if tdocument.warnings.len() != document.warnings().len() {
        return false;
    }

    for (td_warning, warning) in tdocument.warnings.iter().zip(document.warnings()) {
        if td_warning != warning {
            return false;
        }
    }

    true
}
