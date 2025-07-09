use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock, preamble::Preamble},
    span::MatchedItem,
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
};

/// A macro block can be used in a block context to create a new block element.
///
/// This struct is returned when the block form of a *named macro* is detected.
///
/// ```asciidoc
/// <name>::<target>?[<attrlist>?].
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MacroBlock<'src> {
    name: Span<'src>,
    target: Option<Span<'src>>,
    macro_attrlist: Attrlist<'src>,
    source: Span<'src>,
    title: Option<Span<'src>>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> MacroBlock<'src> {
    pub(crate) fn parse(
        preamble: &Preamble<'src>,
        parser: &mut Parser,
    ) -> MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>> {
        let line = preamble.block_start.take_normalized_line();

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

        let macro_attrlist = Attrlist::parse(attrlist, parser);

        let source: Span = preamble.source.trim_remainder(line.after);
        let source = source.slice(0..source.trim().len());

        MatchAndWarnings {
            item: Some(MatchedItem {
                item: Self {
                    name: name.item,
                    target: if target.item.is_empty() {
                        None
                    } else {
                        Some(target.item)
                    },
                    macro_attrlist: macro_attrlist.item.item,
                    source,
                    title: preamble.title,
                    anchor: preamble.anchor,
                    attrlist: preamble.attrlist.clone(),
                },

                after: line.after.discard_empty_lines(),
            }),
            warnings: macro_attrlist.warnings,
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
    ///
    /// **IMPORTANT:** This is the list of attributes _within_ the macro block
    /// definition itself.
    ///
    /// See also [`attrlist()`] for attributes that can be defined before the
    /// macro invocation.
    ///
    /// [`attrlist()`]: Self::attrlist()
    pub fn macro_attrlist(&'src self) -> &'src Attrlist<'src> {
        &self.macro_attrlist
    }
}

impl<'src> IsBlock<'src> for MacroBlock<'src> {
    fn content_model(&self) -> ContentModel {
        // TO DO: We'll probably want different macro types to provide different content
        // models. For now, just default to "simple."
        ContentModel::Simple
    }

    fn raw_context(&self) -> CowStr<'src> {
        // TO DO: We'll probably want different macro types to provide different
        // contexts. For now, just default to "paragraph."

        "paragraph".into()
    }

    fn title(&'src self) -> Option<Span<'src>> {
        self.title
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        self.anchor
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for MacroBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
