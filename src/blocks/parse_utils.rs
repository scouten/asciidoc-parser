use crate::{
    blocks::Block,
    span::MatchedItem,
    warnings::{MatchAndWarnings, Warning},
    Span,
};

/// Parse blocks until end of input or a pre-determined stop condition is
/// reached.
pub(crate) fn parse_blocks_until<'src, F>(
    mut source: Span<'src>,
    f: F,
) -> MatchAndWarnings<'src, MatchedItem<'src, Vec<Block<'src>>>>
where
    F: Fn(&Span<'src>) -> bool,
{
    let mut blocks: Vec<Block<'src>> = vec![];
    let mut warnings: Vec<Warning<'src>> = vec![];

    source = source.discard_empty_lines();

    while !source.data().is_empty() {
        if f(&source) {
            break;
        }

        let mut maw = Block::parse(source);

        if !maw.warnings.is_empty() {
            warnings.append(&mut maw.warnings);
        }

        if let Some(mi) = maw.item {
            source = mi.after;
            blocks.push(mi.item);
        }
    }

    MatchAndWarnings {
        item: MatchedItem {
            item: blocks,
            after: source,
        },
        warnings,
    }
}
