use nom::{bytes::complete::tag, character::complete::space0, IResult, Slice};

use crate::{primitives::trim_input_for_rem, HasSpan, Span};

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
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let i = source;

        let (rem, name): (Span<'a>, Option<Span<'a>>) = if let Some(pr) = i.take_attr_name() {
            let (rem, _) = space0(pr.rem)?;
            if let Ok((rem, _)) = tag::<&str, Span<'a>, nom::error::Error<Span<'a>>>("=")(rem) {
                let (rem, _) = space0(rem)?;
                if rem.len() == 0 || rem.starts_with(',') {
                    (i, None)
                } else {
                    (rem, Some(pr.t))
                }
            } else {
                (i, None)
            }
        } else {
            (i, None)
        };

        let value = match rem.data().chars().next() {
            Some('\'') | Some('"') => match rem.take_quoted_string() {
                Some(v) => v,
                None => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        rem.slice(rem.len()..),
                        nom::error::ErrorKind::Char,
                    )));
                }
            },
            _ => rem.take_while(|c| c != ','),
        };

        if value.t.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                Span::new(""),
                nom::error::ErrorKind::IsNot,
            )));
        }

        let source = trim_input_for_rem(source, value.rem);

        Ok((
            value.rem,
            Self {
                name,
                value: value.t,
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
