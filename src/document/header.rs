use nom::{bytes::complete::tag, character::complete::space0, IResult};

use crate::{
    primitives::{consume_empty_lines, non_empty_line, trim_input_for_rem},
    HasSpan, Span,
};

/// An AsciiDoc document may begin with a document header. The document header
/// encapsulates the document title, author and revision information,
/// document-wide attributes, and other document metadata.
#[allow(dead_code)] // TEMPORARY while building
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header<'a> {
    title: Option<Span<'a>>,
    source: Span<'a>,
}

impl<'a> Header<'a> {
    #[allow(dead_code)] // TEMPORARY
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        let source = consume_empty_lines(i);

        // TEMPORARY: Titles are optional, but we're not prepared for that yet.
        let (rem, title) = parse_title(source)?;

        let source = trim_input_for_rem(source, rem);
        Ok((
            rem,
            Self {
                title: Some(title),
                source,
            },
        ))
    }

    /// Return a [`Span`] describing the document title, if there was one.
    pub fn title(&'a self) -> Option<Span<'a>> {
        self.title
    }
}

impl<'a> HasSpan<'a> for Header<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

fn parse_title(i: Span<'_>) -> IResult<Span, Span<'_>> {
    let (rem, line) = non_empty_line(i)?;

    let (title, _) = tag("= ")(line)?;
    let (title, _) = space0(title)?;

    Ok((rem, title))
}
