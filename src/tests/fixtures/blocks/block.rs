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
        TBlock::Simple(ref simple_fixture) => match observed {
            Block::Simple(ref simple_observed) => simple_fixture == simple_observed,
            _ => false,
        },

        TBlock::Macro(ref macro_fixture) => match observed {
            Block::Macro(ref macro_observed) => macro_fixture == macro_observed,
            _ => false,
        },

        TBlock::Section(ref section_fixture) => match observed {
            Block::Section(ref section_observed) => section_fixture == section_observed,
            _ => false,
        },

        TBlock::RawDelimited(ref rdb_fixture) => match observed {
            Block::RawDelimited(ref rdb_observed) => rdb_fixture == rdb_observed,
            _ => false,
        },

        TBlock::CompoundDelimited(ref cdb_fixture) => match observed {
            Block::CompoundDelimited(ref cdb_observed) => cdb_fixture == cdb_observed,
            _ => false,
        },
    }
}
