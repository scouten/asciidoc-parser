use crate::{
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock},
    span::ParseResult,
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
pub struct MacroBlock<'src> {
    name: Span<'src>,
    target: Option<Span<'src>>,
    attrlist: Attrlist<'src>,
    source: Span<'src>,
}

impl<'src> MacroBlock<'src> {
    pub(crate) fn parse(source: Span<'src>) -> Option<ParseResult<'src, Self>> {
        let line = source.take_normalized_line();

        // Line must end with `]`; otherwise, it's not a block macro.
        if !line.t.ends_with(']') {
            return None;
        }

        let line_wo_brace = line.t.slice(0..line.t.len() - 1);
        let name = line_wo_brace.take_ident()?;
        let colons = name.rem.take_prefix("::")?;
        let target = colons.rem.take_while(|c| c != '[');
        let open_brace = target.rem.take_prefix("[")?;
        let attrlist = Attrlist::parse(open_brace.rem)?;

        Some(ParseResult {
            t: Self {
                name: name.t,
                target: if target.t.is_empty() {
                    None
                } else {
                    Some(target.t)
                },
                attrlist: attrlist.t,
                source: line.t,
            },

            rem: line.rem.discard_empty_lines(),
        })
    }

    /// Return a [`Span`] describing the macro name.
    pub fn name(&'src self) -> &'src Span<'src> {
        &self.name
    }

    /// Return a [`Span`] describing the macro target.
    pub fn target(&'src self) -> Option<&'src Span<'src>> {
        self.target.as_ref()
    }

    /// Return the macro's attribute list.
    pub fn attrlist(&'src self) -> &'src Attrlist<'src> {
        &self.attrlist
    }
}

impl<'src> IsBlock<'src> for MacroBlock<'src> {
    fn content_model(&self) -> ContentModel {
        // TO DO: We'll probably want different macro types
        // to provide different content models. For now, just
        // default to "simple."
        ContentModel::Simple
    }

    fn context(&self) -> CowStr<'src> {
        // TO DO: We'll probably want different macro types to provide different
        // contexts. For now, just default to "paragraph."

        "paragraph".into()
    }
}

impl<'src> HasSpan<'src> for MacroBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
