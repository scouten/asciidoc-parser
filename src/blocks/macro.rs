use nom::{
    bytes::complete::{tag, take_until},
    error::{Error, ErrorKind},
    sequence::tuple,
    Err, IResult, Slice,
};

use crate::{
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock},
    primitives::ident,
    strings::CowStr,
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
    attrlist: Attrlist<'a>,
    source: Span<'a>,
}

impl<'a> MacroBlock<'a> {
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let line = source.take_normalized_line();

        // Line must end with `]`; otherwise, it's not a block macro.
        if !line.t.ends_with(']') {
            return Err(Err::Error(Error::new(line.t, ErrorKind::Tag)));
        }

        let line_wo_brace = line.t.slice(0..line.t.len() - 1);

        let (attrlist, (name, _colons, target, _braces)) =
            tuple((ident, tag("::"), take_until("["), tag("[")))(line_wo_brace)?;

        let (_, attrlist) = Attrlist::parse(attrlist)?;

        Ok((
            line.rem.discard_empty_lines(),
            Self {
                name,
                target: if target.is_empty() {
                    None
                } else {
                    Some(target)
                },
                attrlist,
                source: line.t,
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

    /// Return the macro's attribute list.
    pub fn attrlist(&'a self) -> &'a Attrlist<'a> {
        &self.attrlist
    }
}

impl<'a> IsBlock<'a> for MacroBlock<'a> {
    fn content_model(&self) -> ContentModel {
        // TO DO: We'll probably want different macro types
        // to provide different content models. For now, just
        // default to "simple."
        ContentModel::Simple
    }

    fn context(&self) -> CowStr<'a> {
        // TO DO: We'll probably want different macro types to provide different
        // contexts. For now, just default to "paragraph."

        "paragraph".into()
    }
}

impl<'a> HasSpan<'a> for MacroBlock<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}
