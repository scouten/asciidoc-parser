use crate::{blocks::Block, span::ParseResult, Span};

/// Parse blocks until end of input or a pre-determined stop condition is
/// reached.
pub(crate) fn parse_blocks_until<'a, F>(
    mut i: Span<'a>,
    f: F,
) -> Option<ParseResult<Vec<Block<'a>>>>
where
    F: Fn(&Span<'a>) -> bool,
{
    let mut blocks: Vec<Block<'a>> = vec![];
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
