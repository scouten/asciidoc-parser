use crate::{span::MatchedItem, strings::CowStr, HasSpan, Span};

/// Document attributes are effectively document-scoped variables for the
/// AsciiDoc language. The AsciiDoc language defines a set of built-in
/// attributes, and also allows the author (or extensions) to define additional
/// document attributes, which may replace built-in attributes when permitted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute<'src> {
    name: Span<'src>,
    value: RawAttributeValue<'src>,
    source: Span<'src>,
}

impl<'src> Attribute<'src> {
    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let attr_line = source.take_line_with_continuation()?;
        let colon = attr_line.item.take_prefix(":")?;

        let mut unset = false;
        let line = if colon.after.starts_with('!') {
            unset = true;
            colon.after.slice_from(1..)
        } else {
            colon.after
        };

        let name = line.take_user_attr_name()?;

        let line = if name.after.starts_with('!') && !unset {
            unset = true;
            name.after.slice_from(1..)
        } else {
            name.after
        };

        let line = line.take_prefix(":")?;

        let value = if unset {
            // Ensure line is now empty except for comment.
            RawAttributeValue::Unset
        } else if line.after.is_empty() {
            RawAttributeValue::Set
        } else {
            let value = line.after.take_whitespace();
            RawAttributeValue::Value(value.after)
        };

        let source = source.trim_remainder(attr_line.after);
        Some(MatchedItem {
            item: Self {
                name: name.item,
                value,
                source: source.trim_trailing_whitespace(),
            },
            after: attr_line.after,
        })
    }

    /// Return a [`Span`] describing the attribute name.
    pub fn name(&'src self) -> &'src Span<'src> {
        &self.name
    }

    /// Return the attribute's raw value.
    pub fn raw_value(&'src self) -> &'src RawAttributeValue<'src> {
        &self.value
    }

    /// Return the attribute's interpolated value.
    pub fn value(&'src self) -> InterpretedValue<'src> {
        self.value.as_interpreted_value()
    }
}

impl<'src> HasSpan<'src> for Attribute<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}

/// The raw value of an [`Attribute`].
///
/// If the value contains a textual value, this value will
/// contain continuation markers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawAttributeValue<'src> {
    /// A custom value, described by its accompanying [`Span`].
    Value(Span<'src>),

    /// No explicit value. This is typically interpreted as either
    /// boolean `true` or a default value for a built-in attribute.
    Set,

    /// Explicitly unset. This is typically interpreted as boolean `false`.
    Unset,
}

impl<'src> RawAttributeValue<'src> {
    /// Convert this to an [`InterpretedValue`], resolving any interpolation
    /// necessary if the value contains a textual value.
    pub fn as_interpreted_value(&self) -> InterpretedValue<'src> {
        match self {
            Self::Value(span) => {
                let data = span.data();
                if data.contains('\n') {
                    let lines: Vec<&str> = data.lines().collect();
                    let last_count = lines.len() - 1;

                    let value: Vec<String> = lines
                        .iter()
                        .enumerate()
                        .map(|(count, line)| {
                            let line = if count > 0 {
                                line.trim_start_matches(' ')
                            } else {
                                line
                            };

                            let line = line
                                .trim_start_matches('\r')
                                .trim_end_matches(' ')
                                .trim_end_matches('\\')
                                .trim_end_matches(' ');

                            if line.ends_with('+') {
                                format!("{}\n", line.trim_end_matches('+').trim_end_matches(' '))
                            } else if count < last_count {
                                format!("{line} ")
                            } else {
                                line.to_string()
                            }
                        })
                        .collect();

                    let value = value.join("");
                    InterpretedValue::Value(CowStr::from(value))
                } else {
                    InterpretedValue::Value(CowStr::Borrowed(data))
                }
            }

            Self::Set => InterpretedValue::Set,
            Self::Unset => InterpretedValue::Unset,
        }
    }
}

/// The interpreted value of an [`Attribute`].
///
/// If the value contains a textual value, this value will
/// have any continuation markers resolved, but will no longer
/// contain a reference to the [`Span`] that contains the value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterpretedValue<'src> {
    /// A custom value with all necessary interpolations applied.
    Value(CowStr<'src>),

    /// No explicit value. This is typically interpreted as either
    /// boolean `true` or a default value for a built-in attribute.
    Set,

    /// Explicitly unset. This is typically interpreted as boolean `false`.
    Unset,
}
