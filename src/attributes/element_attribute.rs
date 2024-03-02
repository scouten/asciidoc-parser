use nom::{
    bytes::complete::{is_not, tag},
    character::complete::space0,
    IResult,
};

use crate::{
    primitives::{attr_name, quoted_string, trim_input_for_rem},
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
        let i = source;

        let (rem, name): (Span<'a>, Option<Span<'a>>) = if let Ok((rem, name)) = attr_name(i) {
            let (rem, _) = space0(rem)?;
            if let Ok((rem, _)) = tag::<&str, Span<'a>, nom::error::Error<Span<'a>>>("=")(rem) {
                let (rem, _) = space0(rem)?;
                if rem.len() == 0 || rem.starts_with(',') {
                    (i, None)
                } else {
                    (rem, Some(name))
                }
            } else {
                (i, None)
            }
        } else {
            (i, None)
        };

        let (rem, value) = parse_value(rem)?;
        let source = trim_input_for_rem(source, rem);

        Ok((
            rem,
            Self {
                name,
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

fn parse_value(source: Span<'_>) -> IResult<Span<'_>, Span<'_>> {
    if source.starts_with('\'') || source.starts_with('"') {
        quoted_string(source)
    } else {
        is_not(",")(source)
    }
}
