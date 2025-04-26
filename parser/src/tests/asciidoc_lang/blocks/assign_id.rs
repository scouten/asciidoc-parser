use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, IsBlock},
    tests::{
        fixtures::{
            attributes::{TAttrlist, TElementAttribute},
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            TSpan,
        },
        sdd::{non_normative, track_file, verifies},
    },
    Parser, Span,
};

track_file!("docs/modules/blocks/pages/assign-id.adoc");

non_normative!(
    r#"
= Assign an ID

You can assign an ID to any block using an attribute list.
Once you've assigned an ID to a block, you can use that ID to link to it using a cross reference.

"#
);

#[test]
fn block_id_syntax() {
    verifies!(
        r#"
== Block ID syntax

An ID is assigned to a block by prefixing the ID value with a hash (`#`) and placing it in the block's attribute list.

----
[#the-id-of-this-block]
====
Content of delimited example block
====
----

"#
    );

    let mut parser = Parser::default();

    let block = Block::parse(
        Span::new("[#the-id-of-this-block]\n====\nContent of delimited example block\n===="),
        &mut parser,
    )
    .unwrap_if_no_warnings()
    .unwrap()
    .item;

    assert_eq!(
        block,
        TBlock::CompoundDelimited(TCompoundDelimitedBlock {
            blocks: vec![TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "Content of delimited example block",
                    line: 3,
                    col: 1,
                    offset: 29,
                },
                source: TSpan {
                    data: "Content of delimited example block",
                    line: 3,
                    col: 1,
                    offset: 29,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },),],
            context: "example",
            source: TSpan {
                data: "[#the-id-of-this-block]\n====\nContent of delimited example block\n====",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: Some(TAttrlist {
                attributes: vec![TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: "#the-id-of-this-block",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },],
                    value: TSpan {
                        data: "#the-id-of-this-block",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                    source: TSpan {
                        data: "#the-id-of-this-block",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },],
                source: TSpan {
                    data: "#the-id-of-this-block",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            },),
        },)
    );

    assert_eq!(
        block.id().unwrap(),
        TSpan {
            data: "the-id-of-this-block",
            line: 1,
            col: 3,
            offset: 2,
        }
    );
}

non_normative!(
    r#"
Let's go through some examples of assigning an ID to a block with several attributes, a title, and delimiters.

== Assign an ID to a block with attributes

In this section, we'll assign an ID to this blockquote:

[quote#roads,Dr. Emmett Brown,Back to the Future]
Roads? Where we're going, we don't need roads.

"#
);

#[test]
fn style_and_id() {
    verifies!(
        r#"
When the style attribute is explicitly assigned to a block, the style name is always placed in the first position of the attribute list.
Then, the ID is attached directly to the end of the style name.

The blockquote with an assigned style and ID in <<ex-style-id>> shows this order of attributes.


.Assign a style and ID to a block
[#ex-style-id]
----
[quote#roads]
Roads? Where we're going, we don't need roads.
----

"#
    );

    let mut parser = Parser::default();

    let block = Block::parse(
        Span::new("[quote#roads]\nRoads? Where we're going, we don't need roads."),
        &mut parser,
    )
    .unwrap_if_no_warnings()
    .unwrap()
    .item;

    assert_eq!(
        block,
        TBlock::Simple(TSimpleBlock {
            content: TSpan {
                data: "Roads? Where we're going, we don't need roads.",
                line: 2,
                col: 1,
                offset: 14,
            },
            source: TSpan {
                data: "[quote#roads]\nRoads? Where we're going, we don't need roads.",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: Some(TAttrlist {
                attributes: vec![TElementAttribute {
                    name: None,
                    shorthand_items: vec![
                        TSpan {
                            data: "quote",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                        TSpan {
                            data: "#roads",
                            line: 1,
                            col: 7,
                            offset: 6,
                        },
                    ],
                    value: TSpan {
                        data: "quote#roads",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                    source: TSpan {
                        data: "quote#roads",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },],
                source: TSpan {
                    data: "quote#roads",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            },),
        },)
    );

    assert_eq!(
        block.id().unwrap(),
        TSpan {
            data: "roads",
            line: 1,
            col: 8,
            offset: 7,
        }
    );
}

#[test]
fn style_id_and_positional_attributes() {
    verifies!(
        r#"
Since <<ex-style-id>> is a blockquote it should have some attribution and citation information.
In <<ex-cite>>, let's attribute this quote to its speaker and original context using the positional attribution attributes that are built into the `quote` style.

.Assign a style, ID, and positional attributes to a block
[#ex-cite]
----
[quote#roads,Dr. Emmett Brown,Back to the Future]
Roads? Where we're going, we don't need roads.
----

"#
    );

    let mut parser = Parser::default();

    let block = Block::parse(Span::new(
        "[quote#roads,Dr. Emmett Brown,Back to the Future]\nRoads? Where we're going, we don't need roads.",
    ), &mut parser)
    .unwrap_if_no_warnings()
    .unwrap()
    .item;

    assert_eq!(block,
        TBlock:: Simple(
            TSimpleBlock {
                content: TSpan {
                    data: "Roads? Where we're going, we don't need roads.",
                    line: 2,
                    col: 1,
                    offset: 50,
                },
                source: TSpan {
                    data: "[quote#roads,Dr. Emmett Brown,Back to the Future]\nRoads? Where we're going, we don't need roads.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: Some(
                    TAttrlist {
                        attributes: vec![
                            TElementAttribute {
                                name: None,
                                shorthand_items: vec![
                                    TSpan {
                                        data: "quote",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                    TSpan {
                                        data: "#roads",
                                        line: 1,
                                        col: 7,
                                        offset: 6,
                                    },
                                ],
                                value: TSpan {
                                    data: "quote#roads",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                                source: TSpan {
                                    data: "quote#roads",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                            },
                            TElementAttribute {
                                name: None,
                                shorthand_items: vec![],
                                value: TSpan {
                                    data: "Dr. Emmett Brown",
                                    line: 1,
                                    col: 14,
                                    offset: 13,
                                },
                                source: TSpan {
                                    data: "Dr. Emmett Brown",
                                    line: 1,
                                    col: 14,
                                    offset: 13,
                                },
                            },
                            TElementAttribute {
                                name: None,
                                shorthand_items: vec![],
                                value: TSpan {
                                    data: "Back to the Future",
                                    line: 1,
                                    col: 31,
                                    offset: 30,
                                },
                                source: TSpan {
                                    data: "Back to the Future",
                                    line: 1,
                                    col: 31,
                                    offset: 30,
                                },
                            },
                        ],
                        source: TSpan {
                            data: "quote#roads,Dr. Emmett Brown,Back to the Future",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },
                ),
            },
        ));

    assert_eq!(
        block.id().unwrap(),
        TSpan {
            data: "roads",
            line: 1,
            col: 8,
            offset: 7,
        }
    );
}

non_normative!(
    r#"
Except when the `role` and `options` attributes are assigned values using their shorthand syntax (`.` and `%`, respectively), all other block attributes are typically separated by commas (`,`).

////

In addition to a title, a block can be assigned additional metadata including:

* ID (xref:attributes:id.adoc#anchor[anchor])
* Block style (first positional attribute)
* Block attributes

Here's an example of a quote block with metadata:

----
include::example$block.adoc[tag=meta-co]
----
<1> Title: Gettysburg Address
<2> ID: gettysburg
<3> Block name: quote
<4> attribution: Abraham Lincoln (Named block attribute)
<5> citetitle: Address delivered at the dedication of the Cemetery at Gettysburg (Named block attribute)

TIP: A block can have multiple block attribute lines.
The attributes will be aggregated.
If there is a name conflict, the last attribute defined wins.

Some metadata is used as supplementary content, such as the title, whereas other metadata controls how the block is converted, such as the block name.
////
"#
);
