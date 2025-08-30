use std::collections::HashMap;

use crate::{
    Document, HasSpan,
    document::{Attribute, InterpretedValue},
    parser::{
        AllowableValue, AttributeValue, HtmlSubstitutionRenderer, InlineSubstitutionRenderer,
        ModificationContext, PathResolver,
    },
    warnings::{Warning, WarningType},
};

/// The [`Parser`] struct and its related structs allow a caller to configure
/// how AsciiDoc parsing occurs and then to initiate the parsing process.
#[derive(Clone, Debug)]
pub struct Parser<'p> {
    /// Attribute values at current state of parsing.
    pub(crate) attribute_values: HashMap<String, AttributeValue>,

    /// Default values for attributes if "set."
    default_attribute_values: HashMap<String, String>,

    /// Specifies how the basic raw text of a simple block will be converted to
    /// the format which will ultimately be presented in the final output.
    ///
    /// Typically this is an [`HtmlSubstitutionRenderer`] but clients may
    /// provide alternative implementations.
    pub(crate) renderer: &'p dyn InlineSubstitutionRenderer,

    /// Specifies how to generate clean and secure paths relative to the parsing
    /// context.
    pub path_resolver: PathResolver,
}

impl<'p> Parser<'p> {
    /// Parse a UTF-8 string as an AsciiDoc document.
    ///
    /// Note that the document references the underlying source string and
    /// necessarily has the same lifetime as the source.
    ///
    /// The [`Document`] data structure returned by this call and nearly all
    /// data structures contained within it are gated by the lifetime of the
    /// `source` text passed in to this function. For that reason all of
    /// those data structures are given the lifetime `'src`.
    ///
    /// **IMPORTANT:** The AsciiDoc language documentation states that UTF-16
    /// encoding is allowed if a byte-order-mark (BOM) is present at the
    /// start of a file. This format is not directly supported by the
    /// `asciidoc-parser` crate. Any UTF-16 content must be re-encoded as
    /// UTF-8 prior to parsing.
    ///
    /// **IMPORTANT:** The `Parser` struct will be updated with attributes and
    /// similar values discovered during parsing.
    ///
    /// # Warnings, not errors
    ///
    /// Any UTF-8 string is a valid AsciiDoc document, so this function does not
    /// return an [`Option`] or [`Result`] data type. There may be any number of
    /// character sequences that have ambiguous or potentially unintended
    /// meanings. For that reason, a caller is advised to review the warnings
    /// provided via the [`warnings()`] iterator.
    ///
    /// [`warnings()`]: Document::warnings
    pub fn parse<'src>(&mut self, source: &'src str) -> Document<'src> {
        Document::parse(source, self)
    }

    /// Retrieves the current interpreted value of a [document attribute].
    ///
    /// Each document holds a set of name-value pairs called document
    /// attributes. These attributes provide a means of configuring the AsciiDoc
    /// processor, declaring document metadata, and defining reusable content.
    /// This page introduces document attributes and answers some questions
    /// about the terminology used when referring to them.
    ///
    /// ## What are document attributes?
    ///
    /// Document attributes are effectively document-scoped variables for the
    /// AsciiDoc language. The AsciiDoc language defines a set of built-in
    /// attributes, and also allows the author (or extensions) to define
    /// additional document attributes, which may replace built-in attributes
    /// when permitted.
    ///
    /// Built-in attributes either provide access to read-only information about
    /// the document and its environment or allow the author to configure
    /// behavior of the AsciiDoc processor for a whole document or select
    /// regions. Built-in attributes are effectively unordered. User-defined
    /// attribute serve as a powerful text replacement tool. User-defined
    /// attributes are stored in the order in which they are defined.
    ///
    /// [document attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes/
    pub fn attribute_value<N: AsRef<str>>(&self, name: N) -> InterpretedValue {
        self.attribute_values
            .get(name.as_ref())
            .map(|av| av.value.clone())
            .map(|av| {
                if let InterpretedValue::Set = av
                    && let Some(default) = self.default_attribute_values.get(name.as_ref())
                {
                    InterpretedValue::Value(default.clone())
                } else {
                    av
                }
            })
            .unwrap_or(InterpretedValue::Unset)
    }

    /// Returns `true` if the parser has a [document attribute] by this name.
    ///
    /// [document attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes/
    pub fn has_attribute<N: AsRef<str>>(&self, name: N) -> bool {
        self.attribute_values.contains_key(name.as_ref())
    }

    /// Returns `true` if the parser has a [document attribute] which has been
    /// set by this name.
    ///
    /// [document attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes/
    pub fn is_attribute_set<N: AsRef<str>>(&self, name: N) -> bool {
        self.attribute_values
            .get(name.as_ref())
            .map(|a| a.value != InterpretedValue::Unset)
            .unwrap_or(false)
    }

    /// Sets the value of an [intrinsic attribute].
    ///
    /// Intrinsic attributes are set automatically by the processor. These
    /// attributes provide information about the document being processed (e.g.,
    /// `docfile`), the security mode under which the processor is running
    /// (e.g., `safe-mode-name`), and information about the user’s environment
    /// (e.g., `user-home`).
    ///
    /// The [`modification_context`](ModificationContext) establishes whether
    /// the value can be subsequently modified by the document header and/or in
    /// the document body.
    ///
    /// Subsequent calls to this function or [`with_intrinsic_attribute_bool()`]
    /// are always permitted. The last such call for any given attribute name
    /// takes precendence.
    ///
    /// [intrinsic attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes-ref/#intrinsic-attributes
    ///
    /// [`with_intrinsic_attribute_bool()`]: Self::with_intrinsic_attribute_bool
    pub fn with_intrinsic_attribute<N: AsRef<str>, V: AsRef<str>>(
        mut self,
        name: N,
        value: V,
        modification_context: ModificationContext,
    ) -> Self {
        let attribute_value = AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context,
            value: InterpretedValue::Value(value.as_ref().to_string()),
        };

        self.attribute_values
            .insert(name.as_ref().to_lowercase(), attribute_value);

        self
    }

    /* Comment out until we're prepared to use and test this.
        /// Sets the default value for an [intrinsic attribute].
        ///
        /// Default values for attributes are provided automatically by the
        /// processor. These values provide a falllback textual value for an
        /// attribute when it is merely "set" by the document via API, header, or
        /// document body.
        ///
        /// Calling this does not imply that the value is set automatically by
        /// default, nor does it establish any policy for where the value may be
        /// modified. For that, please use [`with_intrinsic_attribute`].
        ///
        /// [intrinsic attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes-ref/#intrinsic-attributes
        /// [`with_intrinsic_attribute`]: Self::with_intrinsic_attribute
        pub fn with_default_attribute_value<N: AsRef<str>, V: AsRef<str>>(
            mut self,
            name: N,
            value: V,
        ) -> Self {
            self.default_attribute_values
                .insert(name.as_ref().to_string(), value.as_ref().to_string());

            self
        }
    */

    /// Sets the value of an [intrinsic attribute] from a boolean flag.
    ///
    /// A boolean `true` is interpreted as "set." A boolean `false` is
    /// interpreted as "unset."
    ///
    /// Intrinsic attributes are set automatically by the processor. These
    /// attributes provide information about the document being processed (e.g.,
    /// `docfile`), the security mode under which the processor is running
    /// (e.g., `safe-mode-name`), and information about the user’s environment
    /// (e.g., `user-home`).
    ///
    /// The [`modification_context`](ModificationContext) establishes whether
    /// the value can be subsequently modified by the document header and/or in
    /// the document body.
    ///
    /// Subsequent calls to this function or [`with_intrinsic_attribute()`] are
    /// always permitted. The last such call for any given attribute name takes
    /// precendence.
    ///
    /// [intrinsic attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes-ref/#intrinsic-attributes
    ///
    /// [`with_intrinsic_attribute()`]: Self::with_intrinsic_attribute
    pub fn with_intrinsic_attribute_bool<N: AsRef<str>>(
        mut self,
        name: N,
        value: bool,
        modification_context: ModificationContext,
    ) -> Self {
        let attribute_value = AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context,
            value: if value {
                InterpretedValue::Set
            } else {
                InterpretedValue::Unset
            },
        };

        self.attribute_values
            .insert(name.as_ref().to_lowercase(), attribute_value);

        self
    }

    /// Called from [`Header::parse()`] to accept or reject an attribute value.
    pub(crate) fn set_attribute_from_header<'src>(
        &mut self,
        attr: &Attribute<'src>,
        warnings: &mut Vec<Warning<'src>>,
    ) {
        let attr_name = attr.name().data().to_lowercase();

        let existing_attr = self.attribute_values.get(&attr_name);

        // Verify that we have permission to overwrite any existing attribute value.
        if let Some(existing_attr) = existing_attr
            && existing_attr.modification_context == ModificationContext::ApiOnly
        {
            warnings.push(Warning {
                source: attr.span(),
                warning: WarningType::AttributeValueIsLocked(attr_name),
            });
            return;
        }

        let mut value = attr.value().clone();

        if let InterpretedValue::Set = value
            && let Some(default_value) = self.default_attribute_values.get(&attr_name)
        {
            value = InterpretedValue::Value(default_value.clone());
        }

        let attribute_value = AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::Anywhere,
            value,
        };

        self.attribute_values.insert(attr_name, attribute_value);
    }

    /// Called from [`Block::parse()`] to accept or reject an attribute value
    /// from a document (body) attribute.
    pub(crate) fn set_attribute_from_body<'src>(
        &mut self,
        attr: &Attribute<'src>,
        warnings: &mut Vec<Warning<'src>>,
    ) {
        let attr_name = attr.name().data().to_lowercase();

        // Verify that we have permission to overwrite any existing attribute value.
        if let Some(existing_attr) = self.attribute_values.get(&attr_name)
            && existing_attr.modification_context != ModificationContext::Anywhere
        {
            warnings.push(Warning {
                source: attr.span(),
                warning: WarningType::AttributeValueIsLocked(attr_name),
            });
            return;
        }

        let attribute_value = AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::Anywhere,
            value: attr.value().clone(),
        };

        self.attribute_values.insert(attr_name, attribute_value);
    }
}

const DEFAULT_RENDERER: &'static dyn InlineSubstitutionRenderer = &HtmlSubstitutionRenderer {};

impl Default for Parser<'_> {
    fn default() -> Self {
        Self {
            attribute_values: built_in_attrs(),
            default_attribute_values: built_in_default_values(),
            renderer: DEFAULT_RENDERER,
            path_resolver: PathResolver::default(),
        }
    }
}

fn built_in_attrs() -> HashMap<String, AttributeValue> {
    let mut attrs: HashMap<String, AttributeValue> = HashMap::new();

    attrs.insert(
        "sp".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::ApiOnly,
            value: InterpretedValue::Value(" ".into()),
        },
    );

    attrs.insert(
        "plus".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::ApiOnly,
            value: InterpretedValue::Value("+".into()),
        },
    );

    attrs.insert(
        "toc".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::ApiOrHeader,
            value: InterpretedValue::Unset,
        },
    );

    attrs.insert(
        "sectids".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Empty,
            modification_context: ModificationContext::Anywhere,
            value: InterpretedValue::Set,
        },
    );

    attrs.insert(
        "example-caption".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::Anywhere,
            value: InterpretedValue::Set,
        },
    );

    // TO DO: Replace ./images with value of imagesdir if that is non-default.
    attrs.insert(
        "iconsdir".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::Anywhere,
            value: InterpretedValue::Set,
        },
    );

    attrs
}

fn built_in_default_values() -> HashMap<String, String> {
    let mut defaults: HashMap<String, String> = HashMap::new();

    defaults.insert("example-caption".to_owned(), "Example".to_owned());
    defaults.insert("iconsdir".to_owned(), "./images/icons".to_owned());
    defaults.insert("sectnums".to_owned(), "all".to_owned());
    defaults.insert("toc".to_owned(), "auto".to_owned());

    defaults
}
