use crate::{primitives::trim_input_for_rem, span::ParseResult, HasSpan, Span};

/// This struct represents a single element attribute.
///
/// Element attributes define the built-in and user-defined settings and
/// metadata that can be applied to an individual block element or inline
/// element in a document (including macros). Although the include directive is
/// not technically an element, element attributes can also be defined on an
/// include directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementAttribute<'a> {
    name: Option<Span<'a>>,
    shorthand_items: Vec<Span<'a>>,
    value: Span<'a>,
    source: Span<'a>,
}

impl<'a> ElementAttribute<'a> {
    pub(crate) fn parse(source: Span<'a>) -> Option<ParseResult<Self>> {
        Self::parse_internal(source, false)
    }

    pub(crate) fn parse_with_shorthand(source: Span<'a>) -> Option<ParseResult<Self>> {
        Self::parse_internal(source, true)
    }

    fn parse_internal(source: Span<'a>, parse_shorthand: bool) -> Option<ParseResult<Self>> {
        let (name, rem): (Option<Span>, Span) = match source.take_attr_name() {
            Some(name) => {
                let space = name.rem.take_whitespace();
                match space.rem.take_prefix("=") {
                    Some(equals) => {
                        let space = equals.rem.take_whitespace();
                        if space.rem.is_empty() || space.rem.starts_with(',') {
                            (None, source)
                        } else {
                            (Some(name.t), space.rem)
                        }
                    }
                    None => (None, source),
                }
            }
            None => (None, source),
        };

        let value = match rem.data().chars().next() {
            Some('\'') | Some('"') => match rem.take_quoted_string() {
                Some(v) => v,
                None => {
                    return None;
                }
            },
            _ => rem.take_while(|c| c != ','),
        };

        if value.t.is_empty() {
            return None;
        }

        let source = trim_input_for_rem(source, value.rem);

        let shorthand_items = if name.is_none() && parse_shorthand {
            parse_shorthand_items(source)
        } else {
            vec![]
        };

        Some(ParseResult {
            t: Self {
                name,
                shorthand_items,
                value: value.t,
                source,
            },
            rem: value.rem,
        })
    }

    /// Return a [`Span`] describing the attribute name.
    pub fn name(&'a self) -> &'a Option<Span<'a>> {
        &self.name
    }

    /// Return the shorthand items, if this is the first positional attribute.
    pub fn shorthand_items(&'a self) -> &'a Vec<Span<'a>> {
        &self.shorthand_items
    }

    /// Return the block style name from shorthand syntax.
    pub fn block_style(&'a self) -> Option<Span<'a>> {
        self.shorthand_items
            .first()
            .filter(|span| span.position(is_shorthand_delimiter).is_none())
            .copied()
    }

    /// Return the id attribute from shorthand syntax.
    ///
    /// If multiple id attributes were specified, only the first
    /// match is returned. (Multiple ids are not supported.)
    pub fn id(&'a self) -> Option<Span<'a>> {
        self.shorthand_items
            .iter()
            .find(|span| span.starts_with('#'))
            .map(|span| span.discard(1))
    }

    /// Return any role attributes that were found in shorthand syntax.
    pub fn roles(&'a self) -> Vec<Span<'a>> {
        self.shorthand_items
            .iter()
            .filter(|span| span.starts_with('.'))
            .map(|span| span.discard(1))
            .collect()
    }

    /// Return the attribute's raw value.
    pub fn raw_value(&'a self) -> &'a Span<'a> {
        &self.value
    }

    //-/ Return the attribute's interpolated value.
    // pub fn value(&'a self) -> AttributeValue<'a> {
    //     self.value.as_attribute_value()
    // }
}

impl<'a> HasSpan<'a> for ElementAttribute<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

fn parse_shorthand_items<'a>(span: Span<'a>) -> Vec<Span<'a>> {
    let mut span = span;
    let mut shorthand_items: Vec<Span<'a>> = vec![];

    // Look for block style selector.
    if let Some(block_style_pr) = span.split_at_match_non_empty(is_shorthand_delimiter) {
        shorthand_items.push(block_style_pr.t);
        span = block_style_pr.rem;
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
                let pr: ParseResult<Span> = span.into_parse_result(index + 1);
                shorthand_items.push(pr.t);
                span = pr.rem;
            }
        }
    }

    shorthand_items
}

fn is_shorthand_delimiter(c: char) -> bool {
    c == '#' || c == '%' || c == '.'
}
