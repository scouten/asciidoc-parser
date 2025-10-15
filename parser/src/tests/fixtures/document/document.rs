use std::{cmp::PartialEq, fmt};

use crate::{
    blocks::IsBlock,
    tests::fixtures::{Span, blocks::Block, document::Header, warnings::Warning},
};

#[derive(Eq, PartialEq)]
pub(crate) struct Document {
    pub header: Header,
    pub blocks: &'static [Block],
    pub source: Span,
    pub warnings: &'static [Warning],
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Document")
            .field("header", &self.header)
            .field("blocks", &self.blocks)
            .field("source", &self.source)
            .field("warnings", &self.warnings)
            .finish()
    }
}

impl<'src> PartialEq<crate::Document<'src>> for Document {
    fn eq(&self, other: &crate::Document<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Document> for crate::Document<'_> {
    fn eq(&self, other: &Document) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<Document> for &crate::Document<'_> {
    fn eq(&self, other: &Document) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Document, observed: &crate::Document) -> bool {
    if fixture.source != observed.span() {
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

    if observed.source_map().0.len() != 0 {
        false;
    }

    true
}
