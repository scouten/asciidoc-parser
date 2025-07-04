use crate::{
    span::{content::SubstitutionGroup, MatchedItem},
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
    Content, HasSpan, Parser, Span,
};

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
    shorthand_items: Vec<&'src str>,
    value: CowStr<'src>,
    source: Span<'src>,
}

impl<'src> ElementAttribute<'src> {
    pub(crate) fn parse(
        source: Span<'src>,
        parser: &Parser,
    ) -> MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>> {
        Self::parse_internal(source, parser, false)
    }

    pub(crate) fn parse_with_shorthand(
        source: Span<'src>,
        parser: &Parser,
    ) -> MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>> {
        Self::parse_internal(source, parser, true)
    }

    fn parse_internal(
        source: Span<'src>,
        parser: &Parser,
        parse_shorthand: bool,
    ) -> MatchAndWarnings<'src, Option<MatchedItem<'src, Self>>> {
        let mut warnings: Vec<Warning<'src>> = vec![];

        let (name, after): (Option<Span>, Span) = match source.take_attr_name() {
            Some(name) => {
                let space = name.after.take_whitespace();
                match space.after.take_prefix("=") {
                    Some(equals) => {
                        let space = equals.after.take_whitespace();
                        if space.after.is_empty() || space.after.starts_with(',') {
                            // TO DO: Is this a warning? Possible spec ambiguity.
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
                    warnings.push(Warning {
                        source: after,
                        warning: WarningType::AttributeValueMissingTerminatingQuote,
                    });

                    return MatchAndWarnings {
                        item: None,
                        warnings,
                    };
                }
            },
            _ => after.take_while(|c| c != ','),
        };

        if value.item.is_empty() {
            return MatchAndWarnings {
                item: None,
                warnings,
            };
        }

        let after = value.after;
        let source = source.trim_remainder(after);

        let value: CowStr<'_> = if value.item.data().contains(['<', '>', '&', '{']) {
            let mut content = Content::from(value.item);
            SubstitutionGroup::AttributeEntryValue.apply(&mut content, parser, None);
            CowStr::from(content.rendered().to_string())
        } else {
            CowStr::from(value.item.data())
        };

        let shorthand_items = if name.is_none() && parse_shorthand {
            parse_shorthand_items(source, &mut warnings)
        } else {
            vec![]
        };

        MatchAndWarnings {
            item: Some(MatchedItem {
                item: Self {
                    name,
                    shorthand_items,
                    value,
                    source,
                },
                after,
            }),
            warnings,
        }
    }

    /// Return a [`Span`] describing the attribute name.
    pub fn name(&'src self) -> Option<Span<'src>> {
        self.name
    }

    /// Return the shorthand items, if applicable.
    ///
    /// Shorthand items are only parsed for certain element attributes. If this
    /// attribute is not of the appropriate kind, this will return an empty
    /// list.
    pub fn shorthand_items(&'src self) -> Vec<&'src str> {
        self.shorthand_items.iter().map(|i| i.as_ref()).collect()
    }

    /// Return the block style name from shorthand syntax.
    pub fn block_style(&'src self) -> Option<&'src str> {
        self.shorthand_items
            .first()
            .filter(|v| !v.chars().any(is_shorthand_delimiter))
            .map(|v| v.as_ref())
    }

    /// Return the ID attribute from shorthand syntax.
    ///
    /// If multiple ID attributes were specified, only the first
    /// match is returned. (Multiple IDs are not supported.)
    ///
    /// You can assign an ID to a block using the shorthand syntax, the longhand
    /// syntax, or a legacy block anchor.
    ///
    /// In the shorthand syntax, you prefix the name with a hash (`#`) in the
    /// first position attribute:
    ///
    /// ```asciidoc
    /// [#goals]
    /// * Goal 1
    /// * Goal 2
    /// ```
    ///
    /// In the longhand syntax, you use a standard named attribute:
    ///
    /// ```asciidoc
    /// [id=goals]
    /// * Goal 1
    /// * Goal 2
    /// ```
    ///
    /// In the legacy block anchor syntax, you surround the name with double
    /// square brackets:
    ///
    /// ```asciidoc
    /// [[goals]]
    /// * Goal 1
    /// * Goal 2
    /// ```
    pub fn id(&'src self) -> Option<&'src str> {
        self.shorthand_items
            .iter()
            .find(|v| v.starts_with('#'))
            .map(|v| &v[1..])
    }

    /// Return any role attributes that were found in shorthand syntax.
    ///     
    /// You can assign one or more roles to blocks and most inline elements
    /// using the `role` attribute. The `role` attribute is a [named attribute].
    /// Even though the attribute name is singular, it may contain multiple
    /// (space-separated) roles. Roles may also be defined using a shorthand
    /// (dot-prefixed) syntax.
    ///
    /// A role:
    /// 1. adds additional semantics to an element
    /// 2. can be used to apply additional styling to a group of elements (e.g.,
    ///    via a CSS class selector)
    /// 3. may activate additional behavior if recognized by the converter
    ///
    /// **TIP:** The `role` attribute in AsciiDoc always get mapped to the
    /// `class` attribute in the HTML output. In other words, role names are
    /// synonymous with HTML class names, thus allowing output elements to be
    /// identified and styled in CSS using class selectors (e.g.,
    /// `sidebarblock.role1`).
    ///
    /// [named attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/positional-and-named-attributes/#named
    pub fn roles(&'src self) -> Vec<&'src str> {
        self.shorthand_items
            .iter()
            .filter(|span| span.starts_with('.'))
            .map(|span| &span[1..])
            .collect()
    }

    /// Return any option attributes that were found in shorthand syntax.
    ///     
    /// The `options` attribute (often abbreviated as `opts`) is a versatile
    /// [named attribute] that can be assigned one or more values. It can be
    /// defined globally as document attribute as well as a block attribute on
    /// an individual block.
    ///
    /// There is no strict schema for options. Any options which are not
    /// recognized are ignored.
    ///
    /// You can assign one or more options to a block using the shorthand or
    /// formal syntax for the options attribute.
    ///
    /// # Shorthand options syntax for blocks
    ///
    /// To assign an option to a block, prefix the value with a percent sign
    /// (`%`) in an attribute list. The percent sign implicitly sets the
    /// `options` attribute.
    ///
    /// ## Example 1: Sidebar block with an option assigned using the shorthand dot
    ///
    /// ```asciidoc
    /// [%option]
    /// ****
    /// This is a sidebar with an option assigned to it, named option.
    /// ****
    /// ```
    ///
    /// You can assign multiple options to a block by prest
    /// fixing each value with
    /// a percent sign (`%`).
    ///
    /// ## Example 2: Sidebar with two options assigned using the shorthand dot
    /// ```asciidoc
    /// [%option1%option2]
    /// ****
    /// This is a sidebar with two options assigned to it, named option1 and option2.
    /// ****
    /// ```
    ///
    /// # Formal options syntax for blocks
    ///
    /// Explicitly set `options` or `opts`, followed by the equals sign (`=`),
    /// and then the value in an attribute list.
    ///
    /// ## Example 3. Sidebar block with an option assigned using the formal syntax
    /// ```asciidoc
    /// [opts=option]
    /// ****
    /// This is a sidebar with an option assigned to it, named option.
    /// ****
    /// ```
    ///
    /// Separate multiple option values with commas (`,`).
    ///
    /// ## Example 4. Sidebar with three options assigned using the formal syntax
    /// ```asciidoc
    /// [opts="option1,option2"]
    /// ****
    /// This is a sidebar with two options assigned to it, option1 and option2.
    /// ****
    /// ```
    ///
    /// [named attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/positional-and-named-attributes/#named
    pub fn options(&'src self) -> Vec<&'src str> {
        self.shorthand_items
            .iter()
            .filter(|v| v.starts_with('%'))
            .map(|v| &v[1..])
            .collect()
    }

    /// Return the attribute's value.
    ///
    /// Note that this value will have had special characters and attribute
    /// value replacements applied to it.
    pub fn value(&'src self) -> &'src str {
        self.value.as_ref()
    }
}

impl<'src> HasSpan<'src> for ElementAttribute<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}

fn parse_shorthand_items<'src>(
    mut span: Span<'src>,
    warnings: &mut Vec<Warning<'src>>,
) -> Vec<&'src str> {
    let mut shorthand_items: Vec<Span<'src>> = vec![];

    // Look for block style selector.
    if let Some(block_style_pr) = span.split_at_match_non_empty(is_shorthand_delimiter) {
        shorthand_items.push(
            block_style_pr
                .item
                .discard_whitespace()
                .trim_trailing_whitespace(),
        );

        span = block_style_pr.after;
    }

    while !span.is_empty() {
        // Assumption: First character is a delimiter.
        let after_delimiter = span.discard(1);
        match after_delimiter.position(is_shorthand_delimiter) {
            None => {
                if after_delimiter.is_empty() {
                    warnings.push(Warning {
                        source: span,
                        warning: WarningType::EmptyShorthandItem,
                    });
                    span = after_delimiter;
                } else {
                    shorthand_items.push(span.trim_trailing_whitespace());
                    span = span.discard_all();
                }
            }
            Some(0) => {
                warnings.push(Warning {
                    source: span.trim_remainder(after_delimiter),
                    warning: WarningType::EmptyShorthandItem,
                });
                span = after_delimiter;
            }
            Some(index) => {
                let mi: MatchedItem<Span> = span.into_parse_result(index + 1);
                shorthand_items.push(mi.item.trim_trailing_whitespace());
                span = mi.after;
            }
        }
    }

    shorthand_items.iter().map(|span| span.data()).collect()
}

fn is_shorthand_delimiter(c: char) -> bool {
    c == '#' || c == '%' || c == '.'
}
