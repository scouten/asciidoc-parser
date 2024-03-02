// use nom::{bytes::complete::tag, character::complete::space0, IResult, Slice};
use nom::{bytes::complete::is_not, IResult};

use crate::{
    primitives::{quoted_string, trim_input_for_rem},
    HasSpan, Span,
};

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
    value: Span<'a>,
    source: Span<'a>,
}

impl<'a> ElementAttribute<'a> {
    #[allow(dead_code)]
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let i = source.clone();

        let (rem, value) = parse_value(i)?;
        let source = trim_input_for_rem(source, rem);

        Ok((
            rem,
            Self {
                name: None,
                value,
                source,
            },
        ))
    }

    /// Return a [`Span`] describing the attribute name.
    pub fn name(&'a self) -> &'a Option<Span<'a>> {
        &self.name
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

fn parse_value<'a>(source: Span<'a>) -> IResult<Span<'a>, Span<'a>> {
    if source.starts_with('\'') || source.starts_with('"') {
        quoted_string(source)
    } else {
        is_not(",")(source)
    }
}
