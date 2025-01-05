use crate::{span::MatchedItem, HasSpan, Span};

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
    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let name = source.take_ident()?;
        let colon = name.after.take_prefix(":")?;
        let target = colon.after.take_while(|c| c != '[');
        let open_brace = target.after.take_prefix("[")?;
        let attrlist = open_brace.after.take_while(|c| c != ']');
        let close_brace = attrlist.after.take_prefix("]")?;
        let source = source.trim_remainder(close_brace.after);

        Some(MatchedItem {
            item: Self {
                name: name.item,
                target: if target.item.is_empty() {
                    None
                } else {
                    Some(target.item)
                },
                attrlist: if attrlist.item.is_empty() {
                    None
                } else {
                    Some(attrlist.item)
                },
                source,
            },
            after: close_brace.after,
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
