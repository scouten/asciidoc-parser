use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/attributes/pages/id.adoc");
// Tracking commit 493cbec4, current as of 2025-04-10.

non_normative!(
    r#"
= ID Attribute
:page-aliases: ids.adoc

You can assign an identifier (i.e., unique name) to a block or inline element using the `id` attribute.
The `id` attribute is a xref:positional-and-named-attributes.adoc#named[named attribute].
Its purpose is to identify the element when linking, scripting, or styling.
Thus, the identifier can only be used once in a document.

An ID:

. provides an internal link or cross reference anchor for an element
. can be used for adding additional styling to specific elements (e.g., via a CSS ID selector)

You can assign an ID to blocks using the shorthand hash (`+#+`) syntax, longhand (`id=`) syntax, or the anchor (`[[]]`) syntax.
You can assign an ID to inline elements using the shorthand hash (`+#+`) syntax or by adding an anchor adjacent to the inline element using the anchor (`[[]]`) syntax.
You can assign an ID to a table cell by using an anchor (`[[]]`) at the start of the cell.
Likewise, you can assign an ID to a list item by using an anchor (`[[]]`) at the start of the principal text.

"#
);

mod valid_id_characters {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::Block,
        tests::{
            fixtures::{
                blocks::{TBlock, TSimpleBlock},
                content::TContent,
                warnings::TWarning,
                TSpan,
            },
            sdd::{non_normative, verifies},
        },
        warnings::WarningType,
        Parser, Span,
    };

    non_normative!(
        r#"
== Valid ID characters

AsciiDoc does not restrict the set of characters that can be used for an ID when the ID is defined using the named `id` attribute.
"#
    );

    #[test]
    fn value_non_empty() {
        verifies!(
            r#"
All the language requires in this case is that the value be non-empty.
"#
        );

        let mut parser = Parser::default();

        let maw = Block::parse(
            Span::new("[[]]\nThis paragraph gets a lot of attention.\n"),
            &mut parser,
        );

        assert_eq!(
            maw.warnings,
            vec![TWarning {
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 3,
                    offset: 2,
                },
                warning: WarningType::EmptyBlockAnchorName,
            },]
        );

        let mi = maw.item.unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "This paragraph gets a lot of attention.",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "This paragraph gets a lot of attention.",
                },
                source: TSpan {
                    data: "[[]]\nThis paragraph gets a lot of attention.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: Some(TSpan {
                    data: "",
                    line: 1,
                    col: 3,
                    offset: 2,
                },),
                attrlist: None,
            })
        );
    }

    non_normative!(
        r#"
When the ID is defined using the shorthand hash syntax or the anchor syntax, the acceptable characters is more limited (for example, spaces are not permitted).
Regardless, it's not advisable to exploit the ability to use any characters the AsciiDoc syntax allows.
The reason to be cautious is because the ID is passed through to the output, and not all output formats afford the same latitude.
For example, XML is far more restrictive about which characters are permitted in an ID value.

"#
    );

    #[test]
    fn check_valid_xml_name() {
        verifies!(
            r#"
To ensure portability of your IDs, it's best to conform to a universal standard.
The standard we recommend following is a https://www.w3.org/TR/REC-xml/#NT-Name[Name] value as defined by the XML specification.
At a high level, the first character of a Name must be a letter, colon, or underscore and the optional following characters must be a letter, colon, underscore, hyphen, period, or digit.
You should not use any space characters in an ID.
Starting the ID with a digit is less likely to be problematic, but still best to avoid.
It's best to use lowercase letters whenever possible as this solves portability problem when using case-insensitive platforms.

When the AsciiDoc processor auto-generates IDs for section titles and discrete headings, it adheres to this standard.

Here are examples of valid IDs (according to the recommendations above):

[listing]
----
install
data-structures
error-handling
subject-and-body
unset_an_attribute
----

Here are examples of invalid IDs:

[listing]
----
install the gem
3 blind mice
-about-the-author
----

"#
        );

        let mut parser = Parser::default();

        let maw = Block::parse(
            Span::new("[[3 blind mice]]\nThis paragraph gets a lot of attention.\n"),
            &mut parser,
        );

        assert_eq!(
            maw.warnings,
            vec![TWarning {
                source: TSpan {
                    data: "3 blind mice",
                    line: 1,
                    col: 3,
                    offset: 2,
                },
                warning: WarningType::InvalidBlockAnchorName,
            },]
        );

        let mi = maw.item.unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "This paragraph gets a lot of attention.",
                        line: 2,
                        col: 1,
                        offset: 17,
                    },
                    rendered: "This paragraph gets a lot of attention.",
                },
                source: TSpan {
                    data: "[[3 blind mice]]\nThis paragraph gets a lot of attention.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: Some(TSpan {
                    data: "3 blind mice",
                    line: 1,
                    col: 3,
                    offset: 2,
                },),
                attrlist: None,
            })
        );
    }

    #[test]
    #[ignore]
    fn block_id() {
        non_normative!(
            r#"
////
BlockId

NOTE: Section pending
////

"#
        );
    }
}

mod block_assignment {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::Block,
        tests::{
            fixtures::{
                attributes::{TAttrlist, TElementAttribute},
                blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
                content::TContent,
                TSpan,
            },
            sdd::{non_normative, verifies},
        },
        Parser, Span,
    };

    non_normative!(
        r#"
== Block assignment

// tag::bl[]
You can assign an ID to a block using the shorthand syntax, the longhand syntax, or a block anchor.

"#
    );

    #[test]
    fn shorthand_syntax() {
        verifies!(
            r#"
In the shorthand syntax, you prefix the name with a hash (`#`) in the first position attribute.

[source]
----
[#goals]
* Goal 1
* Goal 2
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("[#goals]\n* Goal 1\n* Goal 2"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        // NOTE: This test will have to be revised when we support lists.

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "* Goal 1\n* Goal 2",
                        line: 2,
                        col: 1,
                        offset: 9,
                    },
                    rendered: "* Goal 1\n* Goal 2",
                },
                source: TSpan {
                    data: "[#goals]\n* Goal 1\n* Goal 2",
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
                            data: "#goals",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },],
                        value: TSpan {
                            data: "#goals",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                        source: TSpan {
                            data: "#goals",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },],
                    source: TSpan {
                        data: "#goals",
                        line: 1,
                        col: 2,
                        offset: 1,
                    }
                })
            })
        );
    }

    #[test]
    fn longhand_syntax() {
        verifies!(
            r#"
In the longhand syntax, you use a standard named attribute.

[source]
----
[id=goals]
* Goal 1
* Goal 2
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("[id=goals]\n* Goal 1\n* Goal 2"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        // NOTE: This test will have to be revised when we support lists.

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "* Goal 1\n* Goal 2",
                        line: 2,
                        col: 1,
                        offset: 11,
                    },
                    rendered: "* Goal 1\n* Goal 2",
                },
                source: TSpan {
                    data: "[id=goals]\n* Goal 1\n* Goal 2",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: vec![TElementAttribute {
                        name: Some(TSpan {
                            data: "id",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "goals",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        source: TSpan {
                            data: "id=goals",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },],
                    source: TSpan {
                        data: "id=goals",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );
    }

    #[test]
    fn block_anchor_syntax() {
        verifies!(
            r#"
In the block anchor syntax, you surround the name with double square brackets:

[source]
----
[[goals]]
* Goal 1
* Goal 2
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("[[goals]]\n* Goal 1\n* Goal 2"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        // NOTE: This test will have to be revised when we support lists.

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "* Goal 1\n* Goal 2",
                        line: 2,
                        col: 1,
                        offset: 10,
                    },
                    rendered: "* Goal 1\n* Goal 2",
                },
                source: TSpan {
                    data: "[[goals]]\n* Goal 1\n* Goal 2",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: Some(TSpan {
                    data: "goals",
                    line: 1,
                    col: 3,
                    offset: 2,
                },),
                attrlist: None,
            },)
        );
    }

    #[test]
    fn block_style_with_shorthand_syntax() {
        verifies!(
            r#"
Let's say you want to create a blockquote from an open block and assign it an ID and xref:role.adoc[role].
You add `quote` (the block style) in front of the `#` (the ID) in the first attribute position, as this example shows:

[source]
----
[quote.movie#roads,Dr. Emmett Brown]
____
Roads? Where we're going, we don't need roads.
____
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("[quote.movie#roads,Dr. Emmett Brown]\n____\nRoads? Where we're going, we don't need roads.\n____"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(mi.item, TBlock::CompoundDelimited(
                TCompoundDelimitedBlock {
                    blocks: vec![
                        TBlock::Simple(
                            TSimpleBlock {
                                content: TContent {
                                    original: TSpan {
                                        data: "Roads? Where we're going, we don't need roads.",
                                        line: 3,
                                        col: 1,
                                        offset: 42,
                                    },
                                    rendered: "Roads? Where we&#8217;re going, we don&#8217;t need roads.",
                                },
                                source: TSpan {
                                    data: "Roads? Where we're going, we don't need roads.",
                                    line: 3,
                                    col: 1,
                                    offset: 42,
                                },
                                title: None,
                                anchor: None,
                                attrlist: None,
                            },
                        ),
                    ],
                    context: "quote",
                    source: TSpan {
                        data: "[quote.movie#roads,Dr. Emmett Brown]\n____\nRoads? Where we're going, we don't need roads.\n____",
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
                                            data: ".movie",
                                            line: 1,
                                            col: 7,
                                            offset: 6,
                                        },
                                        TSpan {
                                            data: "#roads",
                                            line: 1,
                                            col: 13,
                                            offset: 12,
                                        },
                                    ],
                                    value: TSpan {
                                        data: "quote.movie#roads",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                    source: TSpan {
                                        data: "quote.movie#roads",
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
                                        col: 20,
                                        offset: 19,
                                    },
                                    source: TSpan {
                                        data: "Dr. Emmett Brown",
                                        line: 1,
                                        col: 20,
                                        offset: 19,
                                    },
                                },
                            ],
                            source: TSpan {
                                data: "quote.movie#roads,Dr. Emmett Brown",
                                line: 1,
                                col: 2,
                                offset: 1,
                            },
                        },
                    ),
                },
            ));
    }

    non_normative!(
        r#"
TIP: The order of ID and role values in the shorthand syntax does not matter.

CAUTION: If the ID contains a `.`, you must define it using either a longhand assignment (e.g., `id=classname.propertyname`) or the anchor shorthand (e.g., `+[[classname.propertyname]]+`).
This is necessary since the `.` character in the shorthand syntax is the delimiter for a role, and thus gets misinterpreted as such.
// end::bl[]
"#
    );
}

// No coverage as yet ...

// == Inline assignment

// // tag::in[]
// The id (`#`) shorthand can be used on inline quoted text.

// .Quoted text with ID assignment using shorthand syntax
// ----
// [#free_the_world]#free the world#
// ----

// .General text with preceding ID assignment using inline anchor syntax
// ----
// [[free_the_world]]free the world
// ----
// // end::in[]

// [#anchor]
// == Use an ID as an anchor

// An anchor (aka ID) can be defined almost anywhere in the document, including
// on a section title, on a discrete heading, on a paragraph, on an image, on a
// delimited block, on an inline phrase, and so forth. The anchor is declared by
// enclosing a _valid_ XML Name in double square brackets (e.g., `+[[idname]]+`)
// or using the shorthand ID syntax (e.g., `[#idname]`) at the start of an
// attribute list. The shorthand form is the preferred syntax.

// The double square bracket form requires the ID to start with a letter, an
// underscore, or a colon, ensuring the ID is portable. According to the https://www.w3.org/TR/REC-xml/#NT-Name[XML Name] rules, a portable ID may not begin with a number, even though a number is allowed elsewhere in the name.
// The shorthand form in an attribute list does not impose this restriction.

// === On block element

// To reference a block element, you must assign an ID to that block.
// You can define an ID using the shorthand syntax:

// .Assign an ID to a paragraph using shorthand syntax
// [source]
// ----
// include::example$id.adoc[tag=block-id-shorthand]
// ----

// or you can define it using the block anchor syntax:

// .Assign an ID to a paragraph using block anchor syntax
// [source]
// ----
// include::example$id.adoc[tag=block-id-brackets]
// ----

// === As an inline anchor

// You can also define an anchor anywhere in content that receives normal
// substitutions (specifically the macros substitution). You can enclose the ID
// in double square brackets:

// .Define an inline anchor
// [source]
// ----
// include::example$id.adoc[tag=anchor-brackets]
// ----

// or using the shorthand ID syntax.

// .Define an inline anchor using shorthand syntax
// [source]
// ----
// include::example$id.adoc[tag=anchor-shorthand]
// ----

// === On a list item

// In addition to being able to define anchors on sections and blocks, anchors
// can be defined inline wherever you can type normal text (anchors are a macros
// substitution). The anchors in the text get replaced with invisible anchor
// points in the output.

// For example, you would not put an anchor in front of a list item:

// .*Invalid* position for an anchor ID in front of a list item
// [source]
// ----
// include::example$id.adoc[tag=anchor-wrong]
// ----

// Instead, you would put it at the start of the text of the list item:

// .Define an inline anchor on a list item
// [source]
// ----
// include::example$id.adoc[tag=anchor-list-item]
// ----

// For a description list, the anchor must be placed at the start of the term:

// .Define an inline anchor on a description list item
// [source]
// ----
// include::example$id.adoc[tag=anchor-dlist-item]
// ----

// You can add multiple anchors to a list item or description list term.
// However, only the first anchor is registered for use as an xref within the
// document. The remaining anchors are auxiliary and are used for making deep
// links (i.e., accessible from a URL fragment).

// === On a table cell

// You can assign an ID to a table cell by placing an inline anchor at the start
// of the cell.

// .Assigning an ID to a table cell using an inline anchor
// [source]
// ----
// |===
// |[[my_cell]]The table cell I want to jump to.
// |===
// ----

// === On an inline image

// You cannot currently define an ID on an inline image.
// Instead you need to place an inline anchor adjacent to it.

// .Placing an inline anchor adjacent to an inline image using shorthand
// [source]
// ----
// include::example$id.adoc[tag=inline-anchor-brackets]
// ----

// Instead of the shorthand form, you can use the macro `anchor` to achieve the
// same goal.

// .Placing an inline anchor adjacent to an inline image using a macro
// [source]
// ----
// include::example$id.adoc[tag=inline-anchor-macro]
// ----

// == Add additional anchors to a section

// To add additional anchors to a section (with or without an autogenerated ID),
// place the anchors in front of the title (without any spaces).

// .Add additional anchors to a section using inline anchors
// [source]
// ----
// include::example$id.adoc[tag=anchor-header-extra]
// ----

// CAUTION: You cannot use inline anchors in a section title to make internal
// references to that section. The processor will flag these as possible invalid
// references. These additional anchors are only intended for making deep links
// using an alternate ID.

// Remember that inline anchors are discovered wherever the macros substitution
// is applied (e.g., paragraph text). If text content doesn't belong somewhere,
// neither does an inline anchor point.

// == Customize automatic xreftext

// It's possible to customize the text that will be used in the cross reference
// link (called `xreflabel`). If not defined, the AsciiDoc processor does it
// best to find suitable text (the solution differs from case to case).
// In case of an image, the image caption will be used.
// In case of a section header, the text of the section's title will be used.

// To define the `xreflabel`, add it in the anchor definition right after the ID
// (separated by a comma).

// .An anchor ID with a defined xreflabel. The caption will not be used as link
// text. [source]
// ----
// include::example$id.adoc[tag=anchor-xreflabel]
// ----
