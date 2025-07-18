use crate::{
    Span,
    document::{Attribute, InterpretedValue},
    tests::{
        fixtures::{
            TSpan,
            document::{TAttribute, TRawAttributeValue},
        },
        sdd::{non_normative, track_file, verifies},
    },
};

track_file!("docs/modules/attributes/pages/attribute-entries.adoc");
// Tracking commit 76c9fe63, current as of 2025-04-11.

non_normative!(
    r#"
= Attribute Entries

== What is an attribute entry?

Before you can use a document attribute in your document, you have to declare it.
An [.term]*attribute entry* is the primary mechanism for defining a document attribute in an AsciiDoc document.
You can think of an attribute entry as a global variable assignment for AsciiDoc.
The document attribute it creates becomes available from that point forward in the document.
Attribute entries are also frequently used to toggle features.

"#
);

#[test]
fn set_boolean_attribute() {
    verifies!(
        r#"
An attribute entry consists of two parts: an attribute *name* and an attribute *value*.
The attribute name comes first, followed by the optional value.
Each attribute entry must be entered on its own line.
An attribute entry starts with an opening colon (`:`), directly followed by the attribute's name, and then a closing colon (`:`).
This [.term]*sets* -- that is, turns on -- the document attribute so you can use it in your document.

[source]
----
:name-of-an-attribute: <.>
----
<.> The attribute's name is directly preceded with a opening colon (`:`) and directly followed by a closing colon (`:`).

"#
    );

    let mi = Attribute::parse(Span::new(":name-of-an-attribute:")).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "name-of-an-attribute",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Set,
            source: TSpan {
                data: ":name-of-an-attribute:",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(mi.item.value(), InterpretedValue::Set);
}

#[test]
fn explicit_value() {
    verifies!(
        r#"
In many cases, you explicitly assign a value to a document attribute by entering information after its name in the attribute entry.
The value must be offset from the closing colon (`:`) by at least one space.

[source]
----
:name-of-an-attribute: value of the attribute <.>
----
<.> An explicitly assigned value is offset from the closing colon (`:`) by at least one space.
At the end of the value, press kbd:[Enter].

"#
    );

    let mi = Attribute::parse(Span::new(":name-of-an-attribute: value of the attribute")).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "name-of-an-attribute",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Value(TSpan {
                data: "value of the attribute",
                line: 1,
                col: 24,
                offset: 23,
            },),
            source: TSpan {
                data: ":name-of-an-attribute: value of the attribute",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    if let InterpretedValue::Value(value) = mi.item.value() {
        assert_eq!(value.as_ref(), "value of the attribute");
    } else {
        panic!("unexpected value type {v:?}", v = mi.item.value());
    }
}

// No coverage after this until we do substitutions.

// Take note that xref:attribute-entry-substitutions.adoc[header substitutions]
// automatically get applied to the value by default. That means you don't need
// to escape special characters such in an HTML tag. It also means you can
// reference the value of attributes which have already been defined when
// defining the value of an attribute. Attribute references in the value of an
// attribute entry are resolved immediately.

// [source]
// ----
// :url-org: https://example.org/projects
// :url-project: {url-org}/project-name <.>
// ----
// <.> You can reuse the value of an attribute which has already been set using
// using an attribute reference in the value.

// Some built-in attributes don't require a value to be explicitly assigned in
// an attribute entry because they're a boolean attribute or have an implied
// value.

// [source]
// ----
// :name-of-an-attribute: <.>
// ----
// <.> If you don't want to explicitly assign a value to the attribute, press
// kbd:[Enter] after the closing colon (`:`).

// When set, the value of a built-in boolean attribute is always empty (i.e., an
// _empty string_). If you set a built-in attribute and leave its value empty,
// the AsciiDoc processor may infer a value at processing time.

// == Where can an attribute entry be declared?

// An attribute entry is most often declared in the document header.
// For attributes that allow it (which includes general purpose attributes), the
// attribute entry can alternately be declared between blocks in the document
// body (i.e., the portion of the document below the header).

// WARNING: An attribute entry should not be declared inside the boundaries of a
// delimited block. When an attribute entry is declared inside a delimited
// block, the behavior is undefined.

// When an attribute is defined in the document header using an attribute entry,
// that's referred to as a header attribute. A header attribute is available to
// the entire document until it is unset. A header attribute is also accessible
// from the document metadata for use by built-in behavior, extensions, and
// other applications that need to consult its value (e.g.,
// `source-highlighter`).

// When an attribute is defined in the document body using an attribute entry,
// that's simply referred to as a document attribute. For any attribute defined
// in the body, the attribute is available from the point it is set until it is
// unset. Attributes defined in the body are not available via the document
// metadata.

// Unless the attribute is unlocked, it can be unset or assigned a new value in
// the document header or body. However, note that unsetting or redefining a
// header attribute that controls behavior in the document body usually has no
// affect. See the xref:document-attributes-ref.adoc[] for where in a document
// each attribute can be set.

// == Defining document attributes without an attribute entry

// Document attributes can also be declared (set with an optional value or
// unset) outside the document via the CLI and API. The attribute entry syntax
// is not used in these cases. Rather, they are declared using the provided
// option. For the API, attributes are declared using the `:attributes` option
// (which supports various entry formats). For the CLI, the attribute is
// declared using the `-a` option.

// When an attribute is assigned a value outside of the document, the value is
// stored as is, meaning substitutions are not applied to it. That also means
// that the xref:subs:index.adoc[special characters and quote substitutions] are
// not applied to the value of that attribute when it is referenced in the
// document. However, subsequent substitutions, such as the macro substitution,
// do get applied. This behavior is due to that fact that the attributes
// substitution is applied after the special characters and quote substitutions.
// In order to force these substitutions to be applied to the value of the
// attribute, you must alter the substitution order at the point of reference.
// Here's an example using the inline pass macro.

// [,asciidoc]
// ----
// pass:a,q[{attribute-with-formatted-text}]
// ----

// When an attribute is declared from the command line or API, it is implicitly
// a document header attribute. By default, the attribute becomes locked (i.e.,
// hard set or unset) and thus cannot be changed by the document. This behavior
// can be changed by adding an `@` to the end of the attribute name or value
// (i.e., the soft set modifier). See xref:assignment-precedence.adoc[] for more
// information.

// The one exception to this rule is the `sectnums` attribute, which can always
// be changed.

// ////
// An exclamation point (`!`) before (or after) the attribute name unsets the
// attribute.

// [source]
// ----
// :!name: <1>
// ----
// <1> The leading `!` indicates this attribute should be unset.
// In this case, the value is ignored.

// An attribute entry must start at the beginning of the line.
// If the attribute entry follows a paragraph, it must be offset by an empty
// line. ////
// "#
// );
