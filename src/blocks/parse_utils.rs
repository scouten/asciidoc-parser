use nom::IResult;

use crate::{blocks::Block, primitives::consume_empty_lines, Span};

/// Parse blocks until end of input or a pre-determined stop condition is
/// reached.
#[allow(dead_code)] // TEMPORARY while building
pub(crate) fn parse_blocks_until<'a, F>(mut i: Span<'a>, f: F) -> IResult<Span, Vec<Block<'a>>>
where
    F: Fn(&Span<'a>) -> bool,
{
    let mut blocks: Vec<Block<'a>> = vec![];
    i = consume_empty_lines(i);

    while !i.data().is_empty() {
        let (i2, block) = Block::parse(i)?;
        i = i2;
        blocks.push(block);

        if f(&i) {
            break;
        }
    }

    Ok((i, blocks))
}
