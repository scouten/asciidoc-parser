use crate::document::InterpretedValue;

/// Document attributes are used either to configure behavior in the processor
/// or to relay information about the document and its environment.
///
/// Unless otherwise marked, these attributes can be modified (set or unset)
/// from the API using the `:attributes` option, from the CLI using the `-a`
/// option, or in the document (often in the document header) using an attribute
/// entry.
///
/// TO DO: Update above statement to reflect the API defined for Rust crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AttributeValue<'src> {
    /// Allowable values for the attribute.
    pub(crate) allowable_value: AllowableValue<'src>,

    /// Allowable contexts for modifying the attribute value.
    pub(crate) modification_context: ModificationContext,

    /// Current value of the attribute.
    pub(crate) value: InterpretedValue<'src>,
}

/// Allowable values for the attribute.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AllowableValue<'src> {
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
    Effective(InterpretedValue<'src>),

    /// Built-in attributes that are not set may have an implied value. The
    /// implied value is enclosed in parentheses (e.g., (attributes)). An
    /// implied value can't be resolved using an attribute reference.
    Implied(&'static str),
}

/// Allowable context(s) for modification of this attribute value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ModificationContext {
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
