use crate::document::InterpretedValue;

/// Document attributes are used either to configure behavior in the processor
/// or to relay information about the document and its environment.
///
/// Attribute values can be established via the API using the
/// [`Parser::with_intrinsic_attribute()`] or
/// [`Parser::with_intrinsic_attribute_bool()`] functions.
///
/// [`Parser::with_intrinsic_attribute()`]: crate::Parser::with_intrinsic_attribute
/// [`Parser::with_intrinsic_attribute_bool()`]: crate::Parser::with_intrinsic_attribute_bool
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AttributeValue {
    /// Allowable values for the attribute.
    pub(crate) allowable_value: AllowableValue,

    /// Allowable contexts for modifying the attribute value.
    pub(crate) modification_context: ModificationContext,

    /// Current value of the attribute.
    pub(crate) value: InterpretedValue,
}

/// Allowable values for the attribute.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AllowableValue {
    /// Any value is accepted.
    Any,

    /// Indicates the attribute doesn't require an explicit value. The
    /// attribute is simply turned on by being set.
    Empty,

    /// In some cases, an empty value is interpreted by the processor as one of
    /// the allowable non-empty values. This effective value is prefixed with an
    /// equals sign and enclosed in square brackets (e.g., `[=auto]`). An
    /// attribute reference will resolve to an empty value rather than the
    /// effective value.
    Effective(InterpretedValue),

    /// Built-in attributes that are not set may have an implied value. The
    /// implied value is enclosed in parentheses (e.g., (attributes)). An
    /// implied value can't be resolved using an attribute reference.
    Implied(&'static str),
}

/// Allowable context(s) for modification of this attribute value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModificationContext {
    /// Value can only be configured via API.
    ApiOnly,

    /// The attribute must be set or unset by the end of the document header
    /// (i.e., set by the API or in the document header).
    ApiOrHeader,

    /// The attribute can be set anywhere in the document. However, changing an
    /// attribute only affects behavior for content that follows the assignment
    /// (in document order).
    Anywhere,
}
