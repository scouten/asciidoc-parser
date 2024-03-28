use std::{ops::Deref, slice::Iter};

use nom::{
    bytes::complete::tag, character::complete::space0, combinator::eof, multi::separated_list0,
    sequence::pair, IResult,
};

use crate::{attributes::ElementAttribute, HasSpan, Span};

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
    #[allow(dead_code)]
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let i = source;

        let (rem, attributes) =
            separated_list0(pair(tag(","), space0), ElementAttribute::parse)(i)?;

        let (rem, _) = eof(rem)?;

        Ok((rem, Self { attributes, source }))
    }

    /// Returns an iterator over the attributes contained within
    /// this attrlist.
    #[allow(dead_code)]
    pub fn attributes(&'a self) -> Iter<'a, ElementAttribute<'a>> {
        self.attributes.iter()
    }

    /// Returns the first attribute with the given name.
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn named_or_positional_attribute(
        &'a self,
        name: &str,
        index: usize,
    ) -> Option<&'a ElementAttribute<'a>> {
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
    /// ```ignore
    /// [#goals]
    /// * Goal 1
    /// * Goal 2
    /// ```
    ///
    /// In the longhand syntax, you use a standard named attribute:
    ///
    /// ```ignore
    /// [id=goals]
    /// * Goal 1
    /// * Goal 2
    /// ```
    ///
    /// In the legacy block anchor syntax, you surround the name with double
    /// square brackets:
    ///
    /// ```ignore
    /// [[goals]]
    /// * Goal 1
    /// * Goal 2
    /// ```
    pub fn id(&'a self) -> Option<&'a Span<'a>> {
        None
    }
}

impl<'a> HasSpan<'a> for Attrlist<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}
