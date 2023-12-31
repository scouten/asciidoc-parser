use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0},
    combinator::recognize,
    multi::many0,
    sequence::pair,
    IResult, Parser, Slice,
};

use crate::{
    primitives::{line_with_continuation, trim_input_for_rem},
    HasSpan, Span,
};

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
    pub(crate) fn parse(source: Span<'a>) -> IResult<Span, Self> {
        let (rem, line) = line_with_continuation(source)?;

        let mut unset = false;
        let (mut line, _) = tag(":")(line)?;

        if line.starts_with('!') {
            unset = true;
            line = line.slice(1..);
        }

        let (mut line, name) = recognize(pair(
            alt((alphanumeric1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag("-")))),
        ))
        .parse(line)?;

        if line.starts_with('!') && !unset {
            unset = true;
            line = line.slice(1..);
        }

        let (line, _) = tag(":")(line)?;

        let value = if unset {
            // Ensure line is now empty except for comment.
            RawAttributeValue::Unset
        } else if line.is_empty() {
            RawAttributeValue::Set
        } else {
            let (value, _) = space0(line)?;
            RawAttributeValue::Value(value)
        };

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
    pub fn name(&'a self) -> &'a Span<'a> {
        &self.name
    }

    /// Return the attribute's value.
    pub fn value(&'a self) -> &'a RawAttributeValue<'a> {
        &self.value
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
