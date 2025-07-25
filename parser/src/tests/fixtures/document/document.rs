use std::{cmp::PartialEq, fmt};

use crate::{
    HasSpan,
    blocks::IsBlock,
    document::Document,
    tests::fixtures::{TSpan, blocks::TBlock, document::THeader, warnings::TWarning},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TDocument {
    pub header: THeader,
    pub blocks: Vec<TBlock>,
    pub source: TSpan,
    pub warnings: &'static [TWarning],
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
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TDocument> for Document<'_> {
    fn eq(&self, other: &TDocument) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TDocument> for &Document<'_> {
    fn eq(&self, other: &TDocument) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TDocument, observed: &Document) -> bool {
    if &fixture.source != observed.span() {
        return false;
    }

    if &fixture.header != observed.header() {
        return false;
    }

    if fixture.blocks.len() != observed.nested_blocks().len() {
        return false;
    }

    for (fixture_block, observed_block) in fixture.blocks.iter().zip(observed.nested_blocks()) {
        if fixture_block != observed_block {
            return false;
        }
    }

    if fixture.warnings.len() != observed.warnings().len() {
        return false;
    }

    for (fixture_warning, observed_warning) in fixture.warnings.iter().zip(observed.warnings()) {
        if fixture_warning != observed_warning {
            return false;
        }
    }

    true
}
