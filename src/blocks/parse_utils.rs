use crate::{blocks::Block, span::ParseResult, Span};

/// Parse blocks until end of input or a pre-determined stop condition is
/// reached.
pub(crate) fn parse_blocks_until<'src, F>(
    mut source: Span<'src>,
    f: F,
) -> Option<ParseResult<'src, Vec<Block<'src>>>>
where
    F: Fn(&Span<'src>) -> bool,
{
    let mut blocks: Vec<Block<'src>> = vec![];
    source = source.discard_empty_lines();

    while !source.data().is_empty() {
        if f(&source) {
            break;
        }

        let pr = Block::parse(source)?;
        source = pr.rem;
        blocks.push(pr.t);
    }

    Some(ParseResult {
        t: blocks,
        rem: source,
    })
}
