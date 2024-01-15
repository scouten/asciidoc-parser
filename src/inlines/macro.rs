use nom::{
    bytes::complete::{tag, take_until1},
    IResult,
};

use crate::{
    primitives::{ident, trim_input_for_rem},
    HasSpan, Span,
};

/// An inline macro can be used in an inline context to create new inline
/// content.
///
/// This struct is returned when the inline form of a *named macro* is detected.
///
/// ```ignore
/// <name>:<target>?[<attrlist>?].
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineMacro<'a> {
    name: Span<'a>,
    target: Option<Span<'a>>,
    attrlist: Option<Span<'a>>,
    source: Span<'a>,
}

impl<'a> InlineMacro<'a> {
    #[allow(dead_code)]
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let (i, name) = ident(source)?;
        let (i, _) = tag(":")(i)?;

        let (i, target) = if i.starts_with('[') {
            (i, None)
        } else {
            let (i, target) = take_until1("[")(i)?;
            (i, Some(target))
        };

        let (i, _) = tag("[")(i)?;

        let (i, attrlist) = if i.starts_with(']') {
            (i, None)
        } else {
            let (i, attrlist) = take_until1("]")(i)?;
            (i, Some(attrlist))
        };

        let (i, _) = tag("]")(i)?;

        let source = trim_input_for_rem(source, i);
        Ok((
            i,
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

impl<'a> HasSpan<'a> for InlineMacro<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}
