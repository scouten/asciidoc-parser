use std::{ops::Deref, slice::Iter};

use crate::{attributes::ElementAttribute, span::ParseResult, HasSpan, Span};

/// The source text that’s used to define attributes for an element is referred
/// to as an attrlist. An attrlist is always enclosed in a pair of square
/// brackets. This applies for block attributes as well as attributes on a block
/// or inline macro. The processor splits the attrlist into individual attribute
/// entries, determines whether each entry is a positional or named attribute,
/// parses the entry accordingly, and assigns the result as an attribute on the
/// node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attrlist<'a> {
    attributes: Vec<ElementAttribute<'a>>,
    source: Span<'a>,
}

impl<'a> Attrlist<'a> {
    /// IMPORTANT: This `source` span passed to this function should NOT include
    /// the opening or closing square brackets for the attrlist. This is because
    /// the rules for closing brackets differ when parsing inline, macro, and
    /// block elements.
    pub(crate) fn parse(source: Span<'a>) -> Option<ParseResult<Self>> {
        let mut rem = source;
        let mut attributes: Vec<ElementAttribute> = vec![];

        loop {
            let Some(attr) = ElementAttribute::parse(rem) else {
                break;
            };
            attributes.push(attr.t);

            rem = attr.rem.take_whitespace().rem;
            match rem.take_prefix(",") {
                Some(comma) => {
                    rem = comma.rem.take_whitespace().rem;
                }
                None => {
                    break;
                }
            }
        }

        if !rem.is_empty() {
            return None;
        }

        Some(ParseResult {
            t: Self { attributes, source },
            rem,
        })
    }

    /// Returns an iterator over the attributes contained within
    /// this attrlist.
    pub fn attributes(&'a self) -> Iter<'a, ElementAttribute<'a>> {
        self.attributes.iter()
    }

    /// Returns the first attribute with the given name.
    pub fn named_attribute(&'a self, name: &str) -> Option<&'a ElementAttribute<'a>> {
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
    /// IMPORTANT: Named attributes with names are disregarded when counting.
    pub fn nth_attribute(&'a self, n: usize) -> Option<&'a ElementAttribute<'a>> {
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
        &'a self,
        name: &str,
        index: usize,
    ) -> Option<&'a ElementAttribute<'a>> {
        self.named_attribute(name)
            .or_else(|| self.nth_attribute(index))
    }
}

impl<'a> HasSpan<'a> for Attrlist<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}
