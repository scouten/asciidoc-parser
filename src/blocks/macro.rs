use nom::{
    bytes::complete::{tag, take_until1},
    error::{Error, ErrorKind},
    Err, IResult,
};

use crate::{
    primitives::{consume_empty_lines, normalized_line, trim_input_for_rem},
    HasSpan, Span,
};

/// A macro block can be used in a block context to create a new block element.
///
/// This struct is returned when the block form of a *named macro* is detected.
///
/// ```ignore
/// <name>::<target>?[<attrlist>?].
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MacroBlock<'a> {
    name: Span<'a>,
    target: Option<Span<'a>>,
    attrlist: Option<Span<'a>>,
    source: Span<'a>,
}

impl<'a> MacroBlock<'a> {
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let (rem, line) = normalized_line(source)?;

        let (line, name) = take_until1("::")(line)?;
        let (line, _) = tag("::")(line)?;

        let (line, target) = if line.starts_with('[') {
            (line, None)
        } else {
            let (line, target) = take_until1("[")(line)?;
            (line, Some(target))
        };

        let (line, _) = tag("[")(line)?;

        let (line, attrlist) = if line.starts_with(']') {
            (line, None)
        } else {
            let (line, attrlist) = take_until1("]")(line)?;
            (line, Some(attrlist))
        };

        let (line, _) = tag("]")(line)?;
        if !line.is_empty() {
            return Err(Err::Error(Error::new(line, ErrorKind::NonEmpty)));
        }

        let source = trim_input_for_rem(source, rem);
        Ok((
            consume_empty_lines(rem),
            Self {
                name,
                target,
                attrlist,
                source,
            },
        ))
    }

    /// Return a [`Span`] describing the macro name.
    pub fn name(&'a self) -> &'a Span<'a> {
        &self.name
    }

    /// Return a [`Span`] describing the macro target.
    pub fn target(&'a self) -> Option<&Span<'a>> {
        self.target.as_ref()
    }

    /// Return a [`Span`] describing the macro's attribute list.
    pub fn attrlist(&'a self) -> Option<&Span<'a>> {
        self.attrlist.as_ref()
    }
}

impl<'a> HasSpan<'a> for MacroBlock<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}
