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
    value: Span<'a>,
    source: Span<'a>,
}

impl<'a> ElementAttribute<'a> {
    pub(crate) fn parse(source: Span<'a>) -> Option<ParseResult<Self>> {
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

        Some(ParseResult {
            t: Self {
                name,
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
