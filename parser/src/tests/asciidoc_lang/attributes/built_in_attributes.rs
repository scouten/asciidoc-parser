use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    tests::sdd::{non_normative, track_file, verifies},
};

track_file!("docs/modules/attributes/pages/built-in-attributes.adoc");

non_normative!(
    r#"
= Declare Built-In Document Attributes
:navtitle: Declare Built-In Attributes

An AsciiDoc processor has numerous attributes reserved for special purposes.
*Built-in attributes* add, configure, and control common features in a document.
Many built-in attributes only take effect when defined in the document header with an attribute entry.

"#
);

#[test]
fn use_default_value() {
    verifies!(
        r#"
== Use an attribute's default value

Many built-in attributes have a default value.
When you want to activate a built-in attribute and assign it its default value, you can leave the value in the attribute entry empty.

For example, to turn on the xref:toc:index.adoc[Table of Contents for a document], you set the `toc` attribute using an attribute entry in the document header.

[source]
----
= Title of Document
:toc:
----

The default value of an activated attribute will be assigned at processing time, if:

. it has a default value, and
. the value in the attribute entry is left empty

In the example above, the default value of `auto` will be assigned to `toc` since the value was left empty in the attribute entry.

"#
    );

    let mut parser = Parser::default();
    parser.parse("= Title of Document\n:toc:");

    assert_eq!(
        parser.attribute_value("toc").as_maybe_str().unwrap(),
        "auto"
    );
}
