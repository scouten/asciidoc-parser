use crate::{
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock},
    span::MatchedItem,
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
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
    pub(crate) fn parse(
        source: Span<'src>,
    ) -> MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>> {
        let line = source.take_normalized_line();

        // Line must end with `]`; otherwise, it's not a block macro.
        if !line.item.ends_with(']') {
            return MatchAndWarnings {
                item: None,
                warnings: vec![],
            };
        }

        let Some(name) = line.item.take_ident() else {
            return MatchAndWarnings {
                item: None,
                warnings: vec![Warning {
                    source: line.item,
                    warning: WarningType::InvalidMacroName,
                }],
            };
        };

        let Some(colons) = name.after.take_prefix("::") else {
            return MatchAndWarnings {
                item: None,
                warnings: vec![Warning {
                    source: name.after,
                    warning: WarningType::MacroMissingDoubleColon,
                }],
            };
        };

        let target = colons.after.take_while(|c| c != '[');

        let Some(open_brace) = target.after.take_prefix("[") else {
            return MatchAndWarnings {
                item: None,
                warnings: vec![Warning {
                    source: target.after,
                    warning: WarningType::MacroMissingAttributeList,
                }],
            };
        };

        let attrlist = open_brace.after.slice(0..open_brace.after.len() - 1);
        // Note that we already checked that this line ends with a close brace.

        let attrlist = Attrlist::parse(attrlist);

        MatchAndWarnings {
            item: Some(MatchedItem {
                item: Self {
                    name: name.item,
                    target: if target.item.is_empty() {
                        None
                    } else {
                        Some(target.item)
                    },
                    attrlist: attrlist.item.item,
                    source: line.item,
                },

                after: line.after.discard_empty_lines(),
            }),
            warnings: attrlist.warnings,
        }
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
