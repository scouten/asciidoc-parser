use crate::{primitives::trim_source_for_rem, span::ParseResult, HasSpan, Span};

/// An inline macro can be used in an inline context to create new inline
/// content.
///
/// This struct is returned when the inline form of a *named macro* is detected.
///
/// ```ignore
/// <name>:<target>?[<attrlist>?].
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineMacro<'src> {
    name: Span<'src>,
    target: Option<Span<'src>>,
    attrlist: Option<Span<'src>>,
    source: Span<'src>,
}

impl<'src> InlineMacro<'src> {
    pub(crate) fn parse(source: Span<'src>) -> Option<ParseResult<'src, Self>> {
        let name = source.take_ident()?;
        let colon = name.rem.take_prefix(":")?;
        let target = colon.rem.take_while(|c| c != '[');
        let open_brace = target.rem.take_prefix("[")?;
        let attrlist = open_brace.rem.take_while(|c| c != ']');
        let close_brace = attrlist.rem.take_prefix("]")?;
        let source = trim_source_for_rem(source, close_brace.rem);

        Some(ParseResult {
            t: Self {
                name: name.t,
                target: if target.t.is_empty() {
                    None
                } else {
                    Some(target.t)
                },
                attrlist: if attrlist.t.is_empty() {
                    None
                } else {
                    Some(attrlist.t)
                },
                source,
            },
            rem: close_brace.rem,
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

    /// Return a [`Span`] describing the macro's attribute list.
    pub fn attrlist(&'src self) -> Option<&'src Span<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for InlineMacro<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
