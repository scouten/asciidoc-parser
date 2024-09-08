use crate::{blocks::Block, span::ParseResult, Span};

/// Parse blocks until end of input or a pre-determined stop condition is
/// reached.
pub(crate) fn parse_blocks_until<'src, F>(
    mut i: Span<'src>,
    f: F,
) -> Option<ParseResult<'src, Vec<Block<'src>>>>
where
    F: Fn(&Span<'src>) -> bool,
{
    let mut blocks: Vec<Block<'src>> = vec![];
    i = i.discard_empty_lines();

    while !i.data().is_empty() {
        if f(&i) {
            break;
        }

        let pr = Block::parse(i)?;
        i = pr.rem;
        blocks.push(pr.t);
    }

    Some(ParseResult { t: blocks, rem: i })
}
