use crate::{primitives::trim_input_for_rem, span::ParseResult, HasSpan, Span};

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
    pub(crate) fn parse(source: Span<'a>) -> Option<ParseResult<Self>> {
        let name = source.take_ident()?;
        let colon = name.rem.take_prefix(":")?;
        let target = colon.rem.take_while(|c| c != '[');
        let open_brace = target.rem.take_prefix("[")?;
        let attrlist = open_brace.rem.take_while(|c| c != ']');
        let close_brace = attrlist.rem.take_prefix("]")?;
        let source = trim_input_for_rem(source, close_brace.rem);

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
