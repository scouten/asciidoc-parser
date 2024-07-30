use crate::{primitives::trim_input_for_rem, span::ParseResult, strings::CowStr, HasSpan, Span};

/// Document attributes are effectively document-scoped variables for the
/// AsciiDoc language. The AsciiDoc language defines a set of built-in
/// attributes, and also allows the author (or extensions) to define additional
/// document attributes, which may replace built-in attributes when permitted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute<'a> {
    name: Span<'a>,
    value: RawAttributeValue<'a>,
    source: Span<'a>,
}

impl<'a> Attribute<'a> {
    pub(crate) fn parse(i: Span<'a>) -> Option<ParseResult<Self>> {
        let attr_line = i.take_line_with_continuation()?;
        let colon = attr_line.t.take_prefix(":")?;

        let mut unset = false;
        let line = if colon.rem.starts_with('!') {
            unset = true;
            colon.rem.temp_slice_from(1..)
        } else {
            colon.rem
        };

        let name = line.take_ident()?;

        let line = if name.rem.starts_with('!') && !unset {
            unset = true;
            name.rem.temp_slice_from(1..)
        } else {
            name.rem
        };

        let line = line.take_prefix(":")?;

        let value = if unset {
            // Ensure line is now empty except for comment.
            RawAttributeValue::Unset
        } else if line.rem.is_empty() {
            RawAttributeValue::Set
        } else {
            let value = line.rem.take_whitespace();
            RawAttributeValue::Value(value.rem)
        };

        let source = trim_input_for_rem(i, attr_line.rem);
        Some(ParseResult {
            t: Self {
                name: name.t,
                value,
                source,
            },
            rem: attr_line.rem,
        })
    }

    /// Return a [`Span`] describing the attribute name.
    pub fn name(&'a self) -> &'a Span<'a> {
        &self.name
    }

    /// Return the attribute's raw value.
    pub fn raw_value(&'a self) -> &'a RawAttributeValue<'a> {
        &self.value
    }

    /// Return the attribute's interpolated value.
    pub fn value(&'a self) -> AttributeValue<'a> {
        self.value.as_attribute_value()
    }
}

impl<'a> HasSpan<'a> for Attribute<'a> {
    fn span(&'a self) -> &'a Span<'a> {
        &self.source
    }
}

/// The raw value of an [`Attribute`].
///
/// If the value contains a textual value, this value will
/// contain continuation markers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawAttributeValue<'a> {
    /// A custom value, described by its accompanying [`Span`].
    Value(Span<'a>),

    /// No explicit value. This is typically interpreted as either
    /// boolean `true` or a default value for a built-in attribute.
    Set,

    /// Explicitly unset. This is typically interpreted as boolean 'false'.
    Unset,
}

impl<'a> RawAttributeValue<'a> {
    /// Convert this to an [`AttributeValue`], resolving any interpolation
    /// necessary if the value contains a textual value.
    pub fn as_attribute_value(&self) -> AttributeValue<'a> {
        match self {
            Self::Value(span) => {
                let data = span.data();
                if data.contains('\n') {
                    let value: Vec<&str> = (0..)
                        .zip(data.lines())
                        .map(|(count, line)| {
                            let line = if count > 0 {
                                line.trim_start_matches(' ')
                            } else {
                                line
                            };

                            line.trim_start_matches('\r')
                                .trim_end_matches(' ')
                                .trim_end_matches('\\')
                                .trim_end_matches(' ')
                        })
                        .collect();

                    let value = value.join(" ");
                    AttributeValue::Value(CowStr::from(value))
                } else {
                    AttributeValue::Value(CowStr::Borrowed(data))
                }
            }

            Self::Set => AttributeValue::Set,
            Self::Unset => AttributeValue::Unset,
        }
    }
}

/// The interpreted value of an [`Attribute`].
///
/// If the value contains a textual value, this value will
/// have any continuation markers resolved, but will no longer
/// contain a reference to the [`Span`] that contains the value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeValue<'a> {
    /// A custom value with all necessary interpolations applied.
    Value(CowStr<'a>),

    /// No explicit value. This is typically interpreted as either
    /// boolean `true` or a default value for a built-in attribute.
    Set,

    /// Explicitly unset. This is typically interpreted as boolean 'false'.
    Unset,
}
