use crate::{
    blocks::Block,
    tests::fixtures::blocks::{
        TCompoundDelimitedBlock, TMacroBlock, TRawDelimitedBlock, TSectionBlock, TSimpleBlock,
    },
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TBlock {
    Simple(TSimpleBlock),
    Macro(TMacroBlock),
    Section(TSectionBlock),
    RawDelimited(TRawDelimitedBlock),
    CompoundDelimited(TCompoundDelimitedBlock),
}

impl<'src> PartialEq<Block<'src>> for TBlock {
    fn eq(&self, other: &Block<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TBlock> for Block<'_> {
    fn eq(&self, other: &TBlock) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TBlock, observed: &Block) -> bool {
    match fixture {
        TBlock::Simple(simple_fixture) => match observed {
            Block::Simple(simple_observed) => simple_fixture == simple_observed,
            _ => false,
        },

        TBlock::Macro(macro_fixture) => match observed {
            Block::Macro(macro_observed) => macro_fixture == macro_observed,
            _ => false,
        },

        TBlock::Section(section_fixture) => match observed {
            Block::Section(section_observed) => section_fixture == section_observed,
            _ => false,
        },

        TBlock::RawDelimited(rdb_fixture) => match observed {
            Block::RawDelimited(rdb_observed) => rdb_fixture == rdb_observed,
            _ => false,
        },

        TBlock::CompoundDelimited(cdb_fixture) => match observed {
            Block::CompoundDelimited(cdb_observed) => cdb_fixture == cdb_observed,
            _ => false,
        },
    }
}
