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

fn fixture_eq_observed(fixture: &TBlock, block: &Block) -> bool {
    match fixture {
        TBlock::Simple(ref tsimple_block) => match block {
            Block::Simple(ref simple_block) => tsimple_block == simple_block,
            _ => false,
        },

        TBlock::Macro(ref tmacro_block) => match block {
            Block::Macro(ref macro_block) => tmacro_block == macro_block,
            _ => false,
        },

        TBlock::Section(ref tsection_block) => match block {
            Block::Section(ref section_block) => tsection_block == section_block,
            _ => false,
        },

        TBlock::RawDelimited(ref traw_delimited_block) => match block {
            Block::RawDelimited(ref raw_delimited_block) => {
                traw_delimited_block == raw_delimited_block
            }
            _ => false,
        },

        TBlock::CompoundDelimited(ref tcompound_delimited_block) => match block {
            Block::CompoundDelimited(ref compound_delimited_block) => {
                tcompound_delimited_block == compound_delimited_block
            }
            _ => false,
        },
    }
}
