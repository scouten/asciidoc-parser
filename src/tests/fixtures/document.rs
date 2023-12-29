use std::cmp::PartialEq;

use crate::{
    tests::fixtures::{blocks::TBlock, TSpan},
    Document,
};

// Approximate mock of Document type that we can use
// to declare expected values for easier test writing.
//
// Primary difference is that the data members are public
// so we can declare them inline.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TDocument {
    pub blocks: Vec<TBlock>,
    pub source: TSpan,
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
