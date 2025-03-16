use std::{ops::Deref, slice::Iter};

use crate::{
    attributes::ElementAttribute,
    span::MatchedItem,
    warnings::{MatchAndWarnings, Warning, WarningType},
    HasSpan, Span,
};

/// The source text that’s used to define attributes for an element is referred
/// to as an **attrlist.** An attrlist is always enclosed in a pair of square
/// brackets. This applies for block attributes as well as attributes on a block
/// or inline macro. The processor splits the attrlist into individual attribute
/// entries, determines whether each entry is a positional or named attribute,
/// parses the entry accordingly, and assigns the result as an attribute on the
/// node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attrlist<'src> {
    attributes: Vec<ElementAttribute<'src>>,
    source: Span<'src>,
}

impl<'src> Attrlist<'src> {
    /// **IMPORTANT:** This `source` span passed to this function should NOT
    /// include the opening or closing square brackets for the attrlist.
    /// This is because the rules for closing brackets differ when parsing
    /// inline, macro, and block elements.
    pub(crate) fn parse(source: Span<'src>) -> MatchAndWarnings<'src, MatchedItem<'src, Self>> {
        let mut after = source;
        let mut attributes: Vec<ElementAttribute> = vec![];
        let mut parse_shorthand_items = true;
        let mut warnings: Vec<Warning<'src>> = vec![];

        if source.starts_with('[') && source.ends_with(']') {
            todo!("Parse block anchor syntax (issue #122)");
        }

        loop {
            let mut maybe_attr_and_warnings = if parse_shorthand_items {
                ElementAttribute::parse_with_shorthand(after)
            } else {
                ElementAttribute::parse(after)
            };

            if !maybe_attr_and_warnings.warnings.is_empty() {
                warnings.append(&mut maybe_attr_and_warnings.warnings);
            }

            let maybe_attr = maybe_attr_and_warnings.item;
            let Some(attr) = maybe_attr else {
                break;
            };

            if attr.item.name().is_none() {
                parse_shorthand_items = false;
            }

            attributes.push(attr.item);

            after = attr.after.take_whitespace().after;
            match after.take_prefix(",") {
                Some(comma) => {
                    after = comma.after.take_whitespace().after;
                    if after.starts_with(',') {
                        warnings.push(Warning {
                            source: comma.item,
                            warning: WarningType::EmptyAttributeValue,
                        });
                        after = after.discard(1);
                        continue;
                    }
                }
                None => {
                    break;
                }
            }
        }

        if !after.is_empty() {
            warnings.push(Warning {
                source: after,
                warning: WarningType::MissingCommaAfterQuotedAttributeValue,
            });

            after = after.discard_all();
        }

        MatchAndWarnings {
            item: MatchedItem {
                item: Self { attributes, source },
                after,
            },
            warnings,
        }
    }

    /// Returns an iterator over the attributes contained within
    /// this attrlist.
    pub fn attributes(&'src self) -> Iter<'src, ElementAttribute<'src>> {
        self.attributes.iter()
    }

    /// Returns the first attribute with the given name.
    pub fn named_attribute(&'src self, name: &str) -> Option<&'src ElementAttribute<'src>> {
        self.attributes.iter().find(|attr| {
            if let Some(attr_name) = attr.name() {
                attr_name.deref() == &name
            } else {
                false
            }
        })
    }

    /// Returns the given (1-based) positional attribute.
    ///
    /// **IMPORTANT:** Named attributes with names are disregarded when
    /// counting.
    pub fn nth_attribute(&'src self, n: usize) -> Option<&'src ElementAttribute<'src>> {
        if n == 0 {
            None
        } else {
            self.attributes
                .iter()
                .filter(|attr| attr.name().is_none())
                .nth(n - 1)
        }
    }

    /// Returns the first attribute with the given name or index.
    ///
    /// Some block and macro types provide implicit mappings between attribute
    /// names and positions to permit a shorthand syntax.
    ///
    /// This method will search by name first, and fall back to positional
    /// indexing if the name doesn't yield a match.
    pub fn named_or_positional_attribute(
        &'src self,
        name: &str,
        index: usize,
    ) -> Option<&'src ElementAttribute<'src>> {
        self.named_attribute(name)
            .or_else(|| self.nth_attribute(index))
    }

    /// Returns the ID attribute (if any).
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
    pub fn id(&'src self) -> Option<Span<'src>> {
        self.nth_attribute(1)
            .and_then(|attr1| attr1.id())
            .or_else(|| self.named_attribute("id").map(|attr| attr.raw_value()))
    }

    /// Returns any role attributes that were found.
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
    pub fn roles(&'src self) -> Vec<Span<'src>> {
        let mut roles = self
            .nth_attribute(1)
            .map(|attr1| attr1.roles())
            .unwrap_or_default();

        if let Some(role_attr) = self.named_attribute("role") {
            let mut role_span = role_attr.raw_value();
            let mut formal_roles: Vec<Span<'_>> = vec![];
            role_span = role_span.take_while(|c| c == ' ').after;

            while !role_span.is_empty() {
                let mi = role_span.take_while(|c| c != ' ');
                if !mi.item.is_empty() {
                    formal_roles.push(mi.item);
                }
                role_span = mi.after.take_while(|c| c == ' ').after;
            }

            roles.append(&mut formal_roles);
        }

        roles
    }
}

impl<'src> HasSpan<'src> for Attrlist<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
