use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/attributes/pages/document-attributes.adoc");
// Tracking commit 375dfb99, current as of 2025-04-11.

non_normative!(
    r#"
= Document Attributes

Each document holds a set of name-value pairs called document attributes.
//Document attributes are an unordered collection of key-value pairs named attributes that are stored directly on the document.
These attributes provide a means of configuring the AsciiDoc processor, declaring document metadata, and defining reusable content.
This page introduces document attributes and answers some questions about the terminology used when referring to them.

== What are document attributes?

Document attributes are effectively document-scoped variables for the AsciiDoc language.
The AsciiDoc language defines a set of built-in attributes, and also allows the author (or extensions) to define additional document attributes, which may replace built-in attributes when permitted.

Built-in attributes either provide access to read-only information about the document and its environment or allow the author to configure behavior of the AsciiDoc processor for a whole document or select regions.
Built-in attributes are effectively unordered.
User-defined attribute serve as a powerful text replacement tool.
User-defined attributes are stored in the order in which they are defined.

Here's a summary of some of the things document attributes are used for:

* Provide access to document information
* Define document metadata
* Turn on or turn off built-in features
* Configure built-in features
* Declare the location of assets, like images
* Store content for reuse throughout a document

Let's look closer at the different types of document attributes.

== Types of document attributes

Document attributes fall into the following groups.

Built-in attributes:: Built-in attributes add, configure, and control common features in a document.
Many built-in attributes only take effect when defined in the document header with an attribute entry.
+
Boolean attributes are a subgroup of the built-in attribute.
If a boolean attribute is defined, but not given a value (i.e., set), it's in the "on" state.
If the attribute is not defined (i.e., not set), it's in the "off" state.
In that regard, these attributes act as a switch.
Their sole function is to turn on or turn off a feature.

User-defined attributes::
A user-defined attribute is any attribute that the author sets that isn't reserved by the AsciiDoc language or an extension.
Most of the time, user-defined attributes are used as a text replacement tool.
These attributes allow the author to define named, reusable content.
Thus, instead of having to repeat text throughout the document, such as a product name, that text can be defined in an attribute and referenced by its name instead.
This technique helps to keep the document DRY, which stands for "`Don't Repeat Yourself`".

== What does defining a document attribute mean?

* have default values in the case of built-in attributes
* have no value in the case of boolean attributes and built-in attributes with default values
* have a single line value
* have a value that xref:wrap-values.adoc[spans multiple, contiguous lines]
* have a value that includes basic inline AsciiDoc syntax, such as:
** attribute references
** text formatting (if wrapped in a xref:pass:pass-macro.adoc#inline-pass[pass macro])
** inline macros (if wrapped in a xref:pass:pass-macro.adoc#inline-pass[pass macro])

But there are certain limitations to be aware of.
Document attributes cannot:

* have a value that includes AsciiDoc block content, such as:
** lists
** multiple paragraphs
** blocks (tables, sidebars, examples, etc)
** other whitespace-dependent markup

== What does setting a document attribute mean?

* be set (turned on)

== What does unsetting a document attribute mean?

* be unset (turned off) with a leading (preferred) or trailing `!` added to the name

== Where are document attributes defined, set, and unset?

Document attributes can be declared in the:

* document header as an xref:attribute-entries.adoc[attribute entry]
* document body as an xref:attribute-entries.adoc[attribute entry]
* API via the `:attributes` option
* CLI via the `-a` option
* override locked attributes assigned from the command line

== What does referencing a document attribute mean?

Referencing a document attribute means replacing an attribute name with that attribute's value.
A document attribute can be referenced in the document using the syntax `+{name}+`, where `name` is the name of the attribute.

== Where can document attributes be referenced?

A document attribute can be referenced anywhere in the document where the attributes substitution is applied.
Generally speaking, the xref:subs:attributes.adoc[attributes substitution] is applied to the value of an attribute entry, titles, paragraph text, list text, the value of an element attribute, and the target of a macro.

A document attribute can only be referenced after it has been defined.
"#
);
