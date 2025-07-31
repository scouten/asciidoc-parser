use crate::{
    Parser, Span,
    attributes::Attrlist,
    content::{Content, SubstitutionGroup},
    span::MatchedItem,
    warnings::{MatchAndWarnings, Warning, WarningType},
};

/// `BlockMetadata` represents the common elements that can precede any block
/// type (such as title and attribute list). It is used internally to track
/// those values before the specific block type is fully formed.
#[derive(Debug)]
pub(crate) struct BlockMetadata<'src> {
    /// The block's raw title, if any.
    pub(crate) title_source: Option<Span<'src>>,

    /// The block's rendered title, if any.
    pub(crate) title: Option<String>,

    /// The block's anchor, if any. The span does not include the opening or
    /// closing square brace pair.
    pub(crate) anchor: Option<Span<'src>>,

    /// The block's attribute list, if any.
    pub(crate) attrlist: Option<Attrlist<'src>>,

    /// The source span as understood when the block metadata was first
    /// encountered. Does not necessarily end at the end of the block.
    pub(crate) source: Span<'src>,

    /// The source span after reading the optional title and attribute list.
    /// This is the beginning of content for the specific block type.
    pub(crate) block_start: Span<'src>,
}

impl<'src> BlockMetadata<'src> {
    /// (For testing only) Parse the block metadata from a raw text constant.
    #[cfg(test)]
    pub(crate) fn new(data: &'src str) -> Self {
        let mut temp_parser = Parser::default();
        Self::parse(Span::new(data), &mut temp_parser).item
    }

    /// Parse the title and attribute list for a block, if any.
    pub(crate) fn parse(source: Span<'src>, parser: &mut Parser) -> MatchAndWarnings<'src, Self> {
        let mut warnings: Vec<Warning<'src>> = vec![];
        let source = source.discard_empty_lines();

        // TO DO (https://github.com/scouten/asciidoc-parser/issues/203):
        // Figure out if these items have to appear in specific order.

        // Does this block have a title?
        let maybe_title = source.take_normalized_line();
        let (title_source, block_start) =
            if maybe_title.item.starts_with('.') && !maybe_title.item.starts_with("..") {
                let title = maybe_title.item.discard(1);
                if title.take_whitespace().item.is_empty() {
                    (Some(title), maybe_title.after)
                } else {
                    (None, source)
                }
            } else {
                (None, source)
            };

        let title = title_source.map(|ref span| {
            let mut content = Content::from(*span);
            SubstitutionGroup::Normal.apply(&mut content, parser, None);
            content.rendered.into_string()
        });

        // Does this block have a block anchor?
        let (anchor, block_start) = if let Some(MatchAndWarnings {
            item:
                MatchedItem {
                    item: anchor,
                    after: block_start,
                },
            warnings: mut attrlist_warnings,
        }) = parse_maybe_block_anchor(block_start)
        {
            if !attrlist_warnings.is_empty() {
                warnings.append(&mut attrlist_warnings);
            }

            (Some(anchor), block_start)
        } else {
            (None, block_start)
        };

        // Does this block have an attribute list?
        let (attrlist, block_start) = if let Some(MatchAndWarnings {
            item:
                MatchedItem {
                    item: attrlist,
                    after: block_start,
                },
            warnings: mut attrlist_warnings,
        }) = parse_maybe_attrlist_line(block_start, parser)
        {
            if !attrlist_warnings.is_empty() {
                warnings.append(&mut attrlist_warnings);
            }

            (Some(attrlist), block_start)
        } else {
            (None, block_start)
        };

        MatchAndWarnings {
            item: Self {
                title_source,
                title,
                anchor,
                attrlist,
                source,
                block_start,
            },
            warnings,
        }
    }

    /// Return `true` if both `title` and `attrlist` are empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.title.is_none() && self.attrlist.is_none()
    }
}

fn parse_maybe_block_anchor(
    source: Span<'_>,
) -> Option<MatchAndWarnings<'_, MatchedItem<'_, Span<'_>>>> {
    if !source.starts_with("[[") {
        return None;
    }

    let MatchedItem {
        item: line,
        after: block_start,
    } = source.take_normalized_line();

    if !line.ends_with("]]") {
        return None;
    }

    // Drop opening and closing brace pairs now that we know they are there.
    let anchor_src = line.slice(2..line.len() - 2);
    if anchor_src.is_empty() {
        return Some(MatchAndWarnings {
            item: MatchedItem {
                item: anchor_src,
                after: block_start,
            },
            warnings: vec![Warning {
                source: anchor_src,
                warning: WarningType::EmptyBlockAnchorName,
            }],
        });
    }

    // Warn if anchor name doesn't match the XML "Name" pattern.
    let warnings = if anchor_src.is_xml_name() {
        vec![]
    } else {
        vec![Warning {
            source: anchor_src,
            warning: WarningType::InvalidBlockAnchorName,
        }]
    };

    Some(MatchAndWarnings {
        item: MatchedItem {
            item: anchor_src,
            after: block_start,
        },
        warnings,
    })
}

fn parse_maybe_attrlist_line<'src>(
    source: Span<'src>,
    parser: &Parser,
) -> Option<MatchAndWarnings<'src, MatchedItem<'src, Attrlist<'src>>>> {
    let first_char = source.chars().next()?;
    if first_char != '[' {
        return None;
    }

    let MatchedItem {
        item: line,
        after: block_start,
    } = source.take_normalized_line();

    if !line.ends_with(']') {
        return None;
    }

    // Drop opening and closing braces now that we know they are there.
    let attrlist_src = line.slice(1..line.len() - 1);

    let MatchAndWarnings {
        item: MatchedItem {
            item: attrlist,
            after: _,
        },
        warnings,
    } = Attrlist::parse(attrlist_src, parser);

    Some(MatchAndWarnings {
        item: MatchedItem {
            item: attrlist,
            after: block_start,
        },
        warnings,
    })
}
