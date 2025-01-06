// use pretty_assertions_sorted::assert_eq;

use crate::{
    tests::{
        fixtures::{
            attributes::TAttrlist,
            blocks::{TBlock, TCompoundDelimitedBlock, TMacroBlock, TSimpleBlock},
            document::{TDocument, THeader},
            inlines::TInline,
            TSpan,
        },
        sdd::{non_normative, track_file, verifies},
    },
    Document,
};

track_file!("docs/modules/blocks/pages/build-basic-block.adoc");

non_normative!(
    r#"
= Build a Basic Block
:y: Yes
:n: No

"#
);

#[test]
fn build_delimited_block() {
    non_normative!(
        r#"
== Build a delimited block

In this section, we'll create a delimited sidebar block.
The delimiter for the sidebar style is four asterisks (`+****+`).

. Enter the opening delimiter at the beginning of a new line and then press kbd:[Enter].
+
----
Text in your document.

****

----

. On the new line, enter your content, such as paragraphs, delimited blocks, directives, and macros.
The delimited block's style will be applied to all of this content until the closing delimiter.
+
----
Text in your document.

****
This is content in a sidebar block.

image::name.png[]

This is more content in the sidebar block.
----

. To end the delimited block, press kbd:[Enter] at the end of your last line of content.
On the new line, type the closing delimiter.
+
"#
    );

    verifies!(
        r#"
----
Text in your document.

****
This is content in a sidebar block.

image::name.png[]

This is more content in the sidebar block.
****
----
"#
    );

    let doc = Document::parse(
    "Text in your document.\n\n****\nThis is content in a sidebar block.\n\nimage::name.png[]\n\nThis is more content in the sidebar block.\n****",
);

    assert_eq!(doc, TDocument {
        header: THeader {
            title: None,
            attributes: vec![],
            source: TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            },
        },
        blocks: vec![
            TBlock::Simple(
                TSimpleBlock {
                    inline: TInline::Uninterpreted(
                        TSpan {
                            data: "Text in your document.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    ),
                    source: TSpan {
                        data: "Text in your document.\n",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    attrlist: None,
                },
            ),
            TBlock::CompoundDelimited(
                TCompoundDelimitedBlock {
                    blocks: vec![
                        TBlock::Simple(
                            TSimpleBlock {
                                inline: TInline::Uninterpreted(
                                    TSpan {
                                        data: "This is content in a sidebar block.",
                                        line: 4,
                                        col: 1,
                                        offset: 29,
                                    },
                                ),
                                source: TSpan {
                                    data: "This is content in a sidebar block.\n",
                                    line: 4,
                                    col: 1,
                                    offset: 29,
                                },
                                title: None,
                                attrlist: None,
                            },
                        ),
                        TBlock::Macro(
                            TMacroBlock {
                                name: TSpan {
                                    data: "image",
                                    line: 6,
                                    col: 1,
                                    offset: 66,
                                },
                                target: Some(
                                    TSpan {
                                        data: "name.png",
                                        line: 6,
                                        col: 8,
                                        offset: 73,
                                    },
                                ),
                                macro_attrlist: TAttrlist {
                                    attributes: vec![],
                                    source: TSpan {
                                        data: "",
                                        line: 6,
                                        col: 17,
                                        offset: 82,
                                    },
                                },
                                source: TSpan {
                                    data: "image::name.png[]",
                                    line: 6,
                                    col: 1,
                                    offset: 66,
                                },
                                title: None,
                                attrlist: None,
                            },
                        ),
                        TBlock::Simple(
                            TSimpleBlock {
                                inline: TInline::Uninterpreted(
                                    TSpan {
                                        data: "This is more content in the sidebar block.",
                                        line: 8,
                                        col: 1,
                                        offset: 85,
                                    },
                                ),
                                source: TSpan {
                                    data: "This is more content in the sidebar block.\n",
                                    line: 8,
                                    col: 1,
                                    offset: 85,
                                },
                                title: None,
                                attrlist: None,
                            },
                        ),
                    ],
                    context: "sidebar",
                    source: TSpan {
                        data: "****\nThis is content in a sidebar block.\n\nimage::name.png[]\n\nThis is more content in the sidebar block.\n****",
                        line: 3,
                        col: 1,
                        offset: 24,
                    },
                    title: None,
                    attrlist: None,
                },
            ),
        ],
        source: TSpan {
            data: "Text in your document.\n\n****\nThis is content in a sidebar block.\n\nimage::name.png[]\n\nThis is more content in the sidebar block.\n****",
            line: 1,
            col: 1,
            offset: 0,
        },
        warnings: vec![],
    });

    non_normative!(
        r#"
That's it.
You've built a delimited block.

"#
    );
}

/*
== Build a block from a paragraph

In some cases, you can style a block using the style's name.
If the content is contiguous (not interrupted by empty lines or comment lines), you can assign the block style's name in an attribute list placed above the content.
This format is often used for single-line listings:

[source]
....
include::example$block.adoc[tag=opt-listing]
....

or single-line quotes:

----
include::example$block.adoc[tag=quote-name]
----

However, note that the lines of a styled paragraph are first parsed like a paragraph, then promoted to the specified block type.
That means that line comments will be dropped, which can impact verbatim blocks such as a listing block.
Thus, the delimited block form is preferred, especially when creating a verbatim block.

}

mod summary {
== Summary of built-in blocks
// This section is just hanging here for the moment, it's not its final destination, I just didn't want to comment it out.

The following table identifies the built-in block styles, their delimiter syntax, purposes, and the substitutions performed on their contents.

include::partial$block-name-table.adoc[]

////
This table shows the substitutions performed by each substitution group referenced in the previous table.

include::partial$subs-group-table.adoc[]

OMG. What does this random tip apply to????
TIP: Surround an attribute value with single quotes in order to apply normal substitutions.
////
}
*/
