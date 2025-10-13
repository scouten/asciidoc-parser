use std::{collections::HashMap, rc::Rc};

use crate::{
    Document, HasSpan,
    document::{Attribute, InterpretedValue},
    parser::{
        AllowableValue, AttributeValue, HtmlSubstitutionRenderer, IncludeFileHandler,
        InlineSubstitutionRenderer, ModificationContext, PathResolver, preprocessor::preprocess,
    },
    warnings::{Warning, WarningType},
};

/// The [`Parser`] struct and its related structs allow a caller to configure
/// how AsciiDoc parsing occurs and then to initiate the parsing process.
#[derive(Clone, Debug)]
pub struct Parser {
    /// Attribute values at current state of parsing.
    pub(crate) attribute_values: HashMap<String, AttributeValue>,

    /// Default values for attributes if "set."
    default_attribute_values: HashMap<String, String>,

    /// Specifies how the basic raw text of a simple block will be converted to
    /// the format which will ultimately be presented in the final output.
    ///
    /// Typically this is an [`HtmlSubstitutionRenderer`] but clients may
    /// provide alternative implementations.
    pub(crate) renderer: Rc<dyn InlineSubstitutionRenderer>,

    /// Specifies the name of the primary file to be parsed.
    pub(crate) primary_file_name: Option<String>,

    /// Specifies how to generate clean and secure paths relative to the parsing
    /// context.
    pub path_resolver: PathResolver,

    /// Handler for resolving include:: directives.
    pub(crate) include_file_handler: Option<Rc<dyn IncludeFileHandler>>,
}

impl Parser {
    /// Parse a UTF-8 string as an AsciiDoc document.
    ///
    /// The [`Document`] data structure returned by this call has a '`static`
    /// lifetime; this is an implementation detail. It retains a copy of the
    /// `source` string that was passed in, but it is not tied to the lifetime
    /// of that string.
    ///
    /// Nearly all of the data structures contained within the [`Document`]
    /// structure are tied to the lifetime of the document and have a `'src`
    /// lifetime to signal their dependency on the source document.
    ///
    /// **IMPORTANT:** The AsciiDoc language documentation states that UTF-16
    /// encoding is allowed if a byte-order-mark (BOM) is present at the
    /// start of a file. This format is not directly supported by the
    /// `asciidoc-parser` crate. Any UTF-16 content must be re-encoded as
    /// UTF-8 prior to parsing.
    ///
    /// The `Parser` struct will be updated with document attribute values
    /// discovered during parsing. These values may be inspected using
    /// [`attribute_value()`].
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
    /// [`attribute_value()`]: Self::attribute_value
    pub fn parse(&mut self, source: &str) -> Document<'static> {
        let (preprocessed_source, _source_map) = preprocess(source, self);
        Document::parse(&preprocessed_source, self)
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

    /// Returns `true` if the parser has a [document attribute] by this name
    /// which has been set (i.e. is present and not [unset]).
    ///
    /// [document attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes/
    /// [unset]: https://docs.asciidoctor.org/asciidoc/latest/attributes/unset-attributes/
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

    /// Replace the default [`InlineSubstitutionRenderer`] for this parser.
    ///
    /// The default implementation of [`InlineSubstitutionRenderer`] that is
    /// provided is suitable for HTML5 rendering. If you are targeting a
    /// different back-end rendering, you will need to provide your own
    /// implementation and set it using this call before parsing.
    pub fn with_inline_substitution_renderer<ISR: InlineSubstitutionRenderer + 'static>(
        mut self,
        renderer: ISR,
    ) -> Self {
        self.renderer = Rc::new(renderer);

        self
    }

    /// Sets the name of the primary file to be parsed when [`parse()`] is
    /// called.
    ///
    /// This name will be used for any error messages detected in this file and
    /// also will be passed to [`IncludeFileHandler::resolve_target()`] as the
    /// `source` argument for any `include::` file resolution requests from this
    /// file.
    ///
    /// [`parse()`]: Self::parse
    /// [`IncludeFileHandler::resolve_target()`]: crate::parser::IncludeFileHandler::resolve_target
    pub fn with_primary_file_name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.primary_file_name = Some(name.as_ref().to_owned());

        self
    }

    /// Sets the [`IncludeFileHandler`] for this parser.
    ///
    /// The include file handler is responsible for resolving `include::`
    /// directives encountered during preprocessing. If no handler is provided,
    /// include directives will be ignored.
    ///
    /// [`IncludeFileHandler`]: crate::parser::IncludeFileHandler
    pub fn with_include_file_handler<IFH: IncludeFileHandler + 'static>(
        mut self,
        handler: IFH,
    ) -> Self {
        self.include_file_handler = Some(Rc::new(handler));

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

    /// Called from [`Header::parse()`] for a value that is derived from parsing
    /// the header (except for attribute lines).
    pub(crate) fn set_attribute_by_value_from_header<N: AsRef<str>, V: AsRef<str>>(
        &mut self,
        name: N,
        value: V,
    ) {
        let attr_name = name.as_ref().to_lowercase();

        let attribute_value = AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::Anywhere,
            value: InterpretedValue::Value(value.as_ref().to_owned()),
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

impl Default for Parser {
    fn default() -> Self {
        Self {
            attribute_values: built_in_attrs(),
            default_attribute_values: built_in_default_values(),
            renderer: Rc::new(HtmlSubstitutionRenderer {}),
            primary_file_name: None,
            path_resolver: PathResolver::default(),
            include_file_handler: None,
        }
    }
}

fn built_in_attrs() -> HashMap<String, AttributeValue> {
    let mut attrs: HashMap<String, AttributeValue> = HashMap::new();

    attrs.insert(
        "empty".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::ApiOnly,
            value: InterpretedValue::Value("".into()),
        },
    );

    attrs.insert(
        "sp".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::ApiOnly,
            value: InterpretedValue::Value(" ".into()),
        },
    );

    attrs.insert(
        "deg".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::ApiOnly,
            value: InterpretedValue::Value("&#176;".into()),
        },
    );

    attrs.insert(
        "plus".to_owned(),
        AttributeValue {
            allowable_value: AllowableValue::Any,
            modification_context: ModificationContext::ApiOnly,
            value: InterpretedValue::Value("&#43;".into()),
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
