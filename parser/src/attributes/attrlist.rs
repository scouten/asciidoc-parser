use std::slice::Iter;

use crate::{
    attributes::{element_attribute::ParseShorthand, ElementAttribute},
    span::{content::SubstitutionStep, MatchedItem},
    strings::CowStr,
    warnings::{MatchAndWarnings, Warning, WarningType},
    Content, HasSpan, Parser, Span,
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
    pub(crate) fn parse(
        source: Span<'src>,
        parser: &Parser,
    ) -> MatchAndWarnings<'src, MatchedItem<'src, Self>> {
        let mut attributes: Vec<ElementAttribute> = vec![];
        let mut parse_shorthand_items = true;
        let mut warnings: Vec<Warning<'src>> = vec![];

        // Apply attribute value substitutions before parsing attrlist content.
        let source_cow = if source.contains('{') && source.contains('}') {
            let mut content = Content::from(source);
            SubstitutionStep::AttributeReferences.apply(&mut content, parser, None);

            if let CowStr::Boxed(value) = content.rendered {
                CowStr::Boxed(value)
            } else {
                CowStr::from(source.data())
            }
        } else {
            CowStr::from(source.data())
        };

        if source_cow.starts_with('[') && source_cow.ends_with(']') {
            todo!("Parse block anchor syntax (issue #122)");
        }

        let mut index = 0;

        let after_index = loop {
            let (maybe_attr, warning_types) = ElementAttribute::parse(
                &source_cow,
                index,
                parser,
                ParseShorthand(parse_shorthand_items),
            );

            if !warnings.is_empty() {
                // Because we do attribute value substitution early on in parsing, we can't
                // pinpoint the exact location of warnings in an attribute list. For that
                // reason, individual attribute parsing only returns the warning type and we
                // then map it back to the entire attrlist source.
                for warning_type in warning_types {
                    warnings.push(Warning {
                        source,
                        warning: warning_type,
                    });
                }
            }

            let Some((attr, new_index)) = maybe_attr else {
                break index;
            };

            if attr.name().is_none() {
                parse_shorthand_items = false;
            }

            attributes.push(attr);

            let mut after = Span::new(source_cow.as_ref()).discard(new_index);
            after = after.take_whitespace().after;

            match after.take_prefix(",") {
                Some(comma) => {
                    after = comma.after.take_whitespace().after;

                    if after.starts_with(',') {
                        warnings.push(Warning {
                            source,
                            warning: WarningType::EmptyAttributeValue,
                        });
                        after = after.discard(1);
                        index = after.byte_offset();
                        continue;
                    }

                    index = after.byte_offset();
                }
                None => {
                    break after.byte_offset();
                }
            }
        };

        if after_index < source_cow.len() {
            warnings.push(Warning {
                source,
                warning: WarningType::MissingCommaAfterQuotedAttributeValue,
            });
        }

        MatchAndWarnings {
            item: MatchedItem {
                item: Self { attributes, source },
                after: source.discard_all(),
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
                attr_name == name
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
    pub fn id(&'src self) -> Option<&'src str> {
        self.nth_attribute(1)
            .and_then(|attr1| attr1.id())
            .or_else(|| self.named_attribute("id").map(|attr| attr.value()))
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
    pub fn roles(&'src self) -> Vec<&'src str> {
        let mut roles = self
            .nth_attribute(1)
            .map(|attr1| attr1.roles())
            .unwrap_or_default();

        if let Some(role_attr) = self.named_attribute("role") {
            let mut role_span = Span::new(role_attr.value());
            let mut formal_roles: Vec<&'src str> = vec![];
            role_span = role_span.take_while(|c| c == ' ').after;

            while !role_span.is_empty() {
                let mi = role_span.take_while(|c| c != ' ');
                if !mi.item.is_empty() {
                    formal_roles.push(mi.item.data());
                }
                role_span = mi.after.take_while(|c| c == ' ').after;
            }

            roles.append(&mut formal_roles);
        }

        roles
    }

    /// Returns any option attributes that were found.
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
    /// You can assign multiple options to a block by prefixing each value with
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
        let mut options = self
            .nth_attribute(1)
            .map(|attr1| attr1.options())
            .unwrap_or_default();

        if let Some(option_attr) = self.named_attribute("opts") {
            let mut option_span = Span::new(option_attr.value());
            let mut formal_options: Vec<&'src str> = vec![];
            option_span = option_span.take_while(|c| c == ',').after;

            while !option_span.is_empty() {
                let mi = option_span.take_while(|c| c != ',');
                if !mi.item.is_empty() {
                    formal_options.push(mi.item.data());
                }
                option_span = mi.after.take_while(|c| c == ',').after;
            }

            options.append(&mut formal_options);
        }

        if let Some(option_attr) = self.named_attribute("options") {
            let mut option_span = Span::new(option_attr.value());
            let mut formal_options: Vec<&'_ str> = vec![];
            option_span = option_span.take_while(|c| c == ',').after;

            while !option_span.is_empty() {
                let mi = option_span.take_while(|c| c != ',');
                if !mi.item.is_empty() {
                    formal_options.push(mi.item.data());
                }
                option_span = mi.after.take_while(|c| c == ',').after;
            }

            options.append(&mut formal_options);
        }

        options
    }

    /// Returns `true` if this attribute list has the named option.
    ///
    /// See [`options()`] for a description of option syntax.
    ///
    /// [`options()`]: Self::options
    pub fn has_option<N: AsRef<str>>(&'src self, name: N) -> bool {
        // PERF: Might help to optimize away the construction of the options Vec.
        let options = self.options();
        let name = name.as_ref();
        options.contains(&name)
    }
}

impl<'src> HasSpan<'src> for Attrlist<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
