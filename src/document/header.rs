use std::slice::Iter;

use nom::{bytes::complete::tag, character::complete::space1, multi::many0, IResult};

use crate::{
    document::Attribute,
    primitives::{consume_empty_lines, empty_line, non_empty_line, trim_input_for_rem},
    HasSpan, Span,
};

/// An AsciiDoc document may begin with a document header. The document header
/// encapsulates the document title, author and revision information,
/// document-wide attributes, and other document metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header<'a> {
    title: Option<Span<'a>>,
    attributes: Vec<Attribute<'a>>,
    source: Span<'a>,
}

impl<'a> Header<'a> {
    pub(crate) fn parse(i: Span<'a>) -> IResult<Span, Self> {
        let source = consume_empty_lines(i);

        // TEMPORARY: Titles are optional, but we're not prepared for that yet.
        let (rem, title) = parse_title(source)?;
        let (rem, attributes) = many0(Attribute::parse)(rem)?;

        // Header must be followed by an empty line.
        let (_, _) = empty_line(rem)?;

        let source = trim_input_for_rem(source, rem);
        Ok((
            consume_empty_lines(rem),
            Self {
                title: Some(title),
                attributes,
                source,
            },
        ))
    }

    /// Return a [`Span`] describing the document title, if there was one.
    pub fn title(&'a self) -> Option<Span<'a>> {
        self.title
    }

    /// Return an iterator over the attributes in this header.
    pub fn attributes(&'a self) -> Iter<'a, Attribute<'a>> {
        self.attributes.iter()
    }
}

impl<'a> HasSpan<'a> for Header<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

fn parse_title(i: Span<'_>) -> IResult<Span, Span<'_>> {
    let line = non_empty_line(i).ok_or(nom::Err::Error(nom::error::Error::new(
        i,
        nom::error::ErrorKind::TakeTill1,
    )))?;

    let (title, _) = tag("=")(line.t)?;
    let (title, _) = space1(title)?;

    Ok((line.rem, title))
}
