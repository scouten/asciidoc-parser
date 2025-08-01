use crate::{
    Parser,
    blocks::{Block, IsBlock},
    tests::sdd::{non_normative, track_file, verifies},
};

track_file!("docs/modules/text/pages/text-span-built-in-roles.adoc");

non_normative!(
    r#"
= Text Span and Built-in Roles

Instead of applying explicit formatting to text, you can enclose a span of a text in a non-formatting element.
This type of markup is referred to as a text span (formerly known as _unquoted text_).
It's purpose is to allow attributes such as role and ID to be applied to unformatted text.
Though those attributes can still be used to apply styles to the text.

"#
);

#[test]
fn text_span_syntax() {
    verifies!(
        r#"
== Text span syntax

When text is enclosed in a pair of single or double hash symbols (`#`) *and* has at least one role, the role(s) will be applied to that text without adding any other implicit formatting.

CAUTION: If no attrlist is present, the formatting pair will be interpreted as xref:highlight.adoc[highlighted text] instead.

.Text span syntax
[#ex-text-span]
----
include::example$text.adoc[tag=text-span]
----

When <<ex-text-span>> is converted to HTML, it translates into the following output.

.Text span HTML output
[,html]
----
include::example$text.adoc[tag=text-span-html]
----

As you can see, it's up to the stylesheet to provide styles for this element.
Typically, this means you'll need to xref:custom-inline-styles.adoc[define custom inline styles] that map to the corresponding class.
In this case, since `underline` is a built-in role, the style is provided for you.

"#
    );

    let doc = Parser::default().parse("The text [.underline]#underline me# is underlined.");

    let mut blocks = doc.nested_blocks();

    let block1 = blocks.next().unwrap();
    let Block::Simple(sb1) = block1 else {
        panic!("Unexpected block type: {block1:?}");
    };

    assert_eq!(
        sb1.content().rendered(),
        r#"The text <span class="underline">underline me</span> is underlined."#
    );

    assert!(blocks.next().is_none());
}

non_normative!(
    r#"
[#built-in]
== Built-in roles for text

The AsciiDoc language provides a handful of built-in roles you can use to provide formatting hints for the text.
While these roles are often used with a text span, they can also be used with any other formatted text for which a role is accepted.

WARNING: Not all converters recognize these roles, though you can expect them to at least be supported by the HTML converter.

These roles are as follows:

underline:: Applies an underline decoration to the span of text.
overline:: Applies an overline decoration to the span of text.
line-through:: Applies a line-through (aka strikethrough) decoration to the span of text.
nobreak:: Disables words within the span of text from being broken.
nowrap:: Prevents the span of text from wrapping at all.
pre-wrap:: Prevents sequences of space and space-like characters from being collapsed (i.e., all spaces are preserved).

[#deprecated]
=== Deprecated roles

There are several built-in roles that were once supported in AsciiDoc, but have since been deprecated.
These roles include `big`, `small`, named colors (e.g., `aqua`), and named background colors (e.g., `aqua-background`).
You should create your own semantic roles in place of these deprecated roles.
"#
);
