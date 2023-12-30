use std::{cmp::PartialEq, fmt};

use super::THeader;
use crate::{
    document::Document,
    tests::fixtures::{blocks::TBlock, TSpan},
    HasSpan,
};

// Approximate mock of Document type that we can use
// to declare expected values for easier test writing.
//
// Primary difference is that the data members are public
// so we can declare them inline.
#[derive(Eq, PartialEq)]
pub(crate) struct TDocument {
    pub header: Option<THeader>,
    pub blocks: Vec<TBlock>,
    pub source: TSpan,
}

impl fmt::Debug for TDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Intentionally mimic the output of nom_span::Spanned
        // so diffs point the unit test author to the important
        // differences.
        f.debug_struct("Document")
            .field("header", &self.header)
            .field("blocks", &self.blocks)
            .field("source", &self.source)
            .finish()
    }
}

impl<'a> PartialEq<Document<'a>> for TDocument {
    fn eq(&self, other: &Document<'a>) -> bool {
        tdocument_eq(self, other)
    }
}

impl<'a> PartialEq<TDocument> for Document<'a> {
    fn eq(&self, other: &TDocument) -> bool {
        tdocument_eq(other, self)
    }
}

impl<'a> PartialEq<TDocument> for &Document<'a> {
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

    if tdocument.blocks.len() != document.blocks().len() {
        return false;
    }

    for (td_block, block) in tdocument.blocks.iter().zip(document.blocks()) {
        if td_block != block {
            return false;
        }
    }

    true
}
