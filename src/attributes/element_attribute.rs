use crate::{span::MatchedItem, HasSpan, Span};

/// This struct represents a single element attribute.
///
/// Element attributes define the built-in and user-defined settings and
/// metadata that can be applied to an individual block element or inline
/// element in a document (including macros). Although the include directive is
/// not technically an element, element attributes can also be defined on an
/// include directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementAttribute<'src> {
    name: Option<Span<'src>>,
    shorthand_items: Vec<Span<'src>>,
    value: Span<'src>,
    source: Span<'src>,
}

impl<'src> ElementAttribute<'src> {
    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        Self::parse_internal(source, false)
    }

    pub(crate) fn parse_with_shorthand(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        Self::parse_internal(source, true)
    }

    fn parse_internal(
        source: Span<'src>,
        parse_shorthand: bool,
    ) -> Option<MatchedItem<'src, Self>> {
        let (name, after): (Option<Span>, Span) = match source.take_attr_name() {
            Some(name) => {
                let space = name.after.take_whitespace();
                match space.after.take_prefix("=") {
                    Some(equals) => {
                        let space = equals.after.take_whitespace();
                        if space.after.is_empty() || space.after.starts_with(',') {
                            (None, source)
                        } else {
                            (Some(name.item), space.after)
                        }
                    }
                    None => (None, source),
                }
            }
            None => (None, source),
        };

        let value = match after.data().chars().next() {
            Some('\'') | Some('"') => match after.take_quoted_string() {
                Some(v) => v,
                None => {
                    return None;
                }
            },
            _ => after.take_while(|c| c != ','),
        };

        if value.item.is_empty() {
            return None;
        }

        let source = source.trim_remainder(value.after);

        let shorthand_items = if name.is_none() && parse_shorthand {
            parse_shorthand_items(source)
        } else {
            vec![]
        };

        Some(MatchedItem {
            item: Self {
                name,
                shorthand_items,
                value: value.item,
                source,
            },
            after: value.after,
        })
    }

    /// Return a [`Span`] describing the attribute name.
    pub fn name(&'src self) -> &'src Option<Span<'src>> {
        &self.name
    }

    /// Return the shorthand items, if parsed via `parse_with_shorthand`.
    pub fn shorthand_items(&'src self) -> &'src Vec<Span<'src>> {
        &self.shorthand_items
    }

    /// Return the block style name from shorthand syntax.
    pub fn block_style(&'src self) -> Option<Span<'src>> {
        self.shorthand_items
            .first()
            .filter(|span| span.position(is_shorthand_delimiter).is_none())
            .copied()
    }

    /// Return the id attribute from shorthand syntax.
    ///
    /// If multiple id attributes were specified, only the first
    /// match is returned. (Multiple ids are not supported.)
    pub fn id(&'src self) -> Option<Span<'src>> {
        self.shorthand_items
            .iter()
            .find(|span| span.starts_with('#'))
            .map(|span| span.discard(1))
    }

    /// Return any role attributes that were found in shorthand syntax.
    pub fn roles(&'src self) -> Vec<Span<'src>> {
        self.shorthand_items
            .iter()
            .filter(|span| span.starts_with('.'))
            .map(|span| span.discard(1))
            .collect()
    }

    /// Return any option attributes that were found in shorthand syntax.
    pub fn options(&'src self) -> Vec<Span<'src>> {
        self.shorthand_items
            .iter()
            .filter(|span| span.starts_with('%'))
            .map(|span| span.discard(1))
            .collect()
    }

    /// Return the attribute's raw value.
    pub fn raw_value(&'src self) -> Span<'src> {
        self.value
    }

    //-/ Return the attribute's interpolated value.
    // pub fn value(&'src self) -> AttributeValue<'src> {
    //     self.value.as_attribute_value()
    // }
}

impl<'src> HasSpan<'src> for ElementAttribute<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}

fn parse_shorthand_items<'src>(span: Span<'src>) -> Vec<Span<'src>> {
    let mut span = span;
    let mut shorthand_items: Vec<Span<'src>> = vec![];

    // Look for block style selector.
    if let Some(block_style_pr) = span.split_at_match_non_empty(is_shorthand_delimiter) {
        shorthand_items.push(block_style_pr.item);
        span = block_style_pr.after;
    }

    while !span.is_empty() {
        // Assumption: First character is a delimiter.
        let after_delimiter = span.discard(1);
        match after_delimiter.position(is_shorthand_delimiter) {
            None => {
                if after_delimiter.is_empty() {
                    todo!("Flag warning for empty shorthand item (issue #120)");
                } else {
                    shorthand_items.push(span);
                    span = span.discard_all();
                }
            }
            Some(0) => {
                todo!("Flag warning for duplicate shorthand delimiter (issue #121)");
            }
            Some(index) => {
                let mi: MatchedItem<Span> = span.into_parse_result(index + 1);
                shorthand_items.push(mi.item);
                span = mi.after;
            }
        }
    }

    shorthand_items
}

fn is_shorthand_delimiter(c: char) -> bool {
    c == '#' || c == '%' || c == '.'
}
