use crate::{
    HasSpan, Parser, Span,
    content::{Content, SubstitutionGroup},
    span::MatchedItem,
    strings::CowStr,
};

/// Document attributes are effectively document-scoped variables for the
/// AsciiDoc language. The AsciiDoc language defines a set of built-in
/// attributes, and also allows the author (or extensions) to define additional
/// document attributes, which may replace built-in attributes when permitted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute<'src> {
    name: Span<'src>,
    value_source: Option<Span<'src>>,
    value: InterpretedValue,
    source: Span<'src>,
}

impl<'src> Attribute<'src> {
    pub(crate) fn parse(
        source: Span<'src>,
        parser: &Parser<'_>,
    ) -> Option<MatchedItem<'src, Self>> {
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

        let (value, value_source) = if unset {
            // Ensure line is now empty except for comment.
            (InterpretedValue::Unset, None)
        } else if line.after.is_empty() {
            (InterpretedValue::Set, None)
        } else {
            let raw_value = line.after.take_whitespace();
            (
                InterpretedValue::from_raw_value(&raw_value.after, parser),
                Some(raw_value.after),
            )
        };

        let source = source.trim_remainder(attr_line.after);
        Some(MatchedItem {
            item: Self {
                name: name.item,
                value_source,
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

    /// Return a [`Span`] containing the attribute's raw value (if present).
    pub fn raw_value(&'src self) -> Option<Span<'src>> {
        self.value_source
    }

    /// Return the attribute's interpolated value.
    pub fn value(&'src self) -> &'src InterpretedValue {
        &self.value
    }
}

impl<'src> HasSpan<'src> for Attribute<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

/// The interpreted value of an [`Attribute`].
///
/// If the value contains a textual value, this value will
/// have any continuation markers resolved, but will no longer
/// contain a reference to the [`Span`] that contains the value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterpretedValue {
    /// A custom value with all necessary interpolations applied.
    Value(String),

    /// No explicit value. This is typically interpreted as either
    /// boolean `true` or a default value for a built-in attribute.
    Set,

    /// Explicitly unset. This is typically interpreted as boolean `false`.
    Unset,
}

impl InterpretedValue {
    fn from_raw_value(raw_value: &Span<'_>, parser: &Parser) -> Self {
        let data = raw_value.data();
        let mut content = Content::from(*raw_value);

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

            content.rendered = CowStr::Boxed(value.join("").into_boxed_str());
        }

        SubstitutionGroup::Header.apply(&mut content, parser, None);

        InterpretedValue::Value(content.rendered.into_string())
    }

    pub(crate) fn as_maybe_str(&self) -> Option<&str> {
        match self {
            InterpretedValue::Value(value) => Some(value.as_ref()),
            _ => None,
        }
    }
}
