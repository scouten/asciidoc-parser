use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/attributes/pages/options.adoc");
// Tracking commit 6ef733aa, current as of 2025-04-10.

non_normative!(
    r#"
= Options Attribute

The `options` attribute (often abbreviated as `opts`) is a versatile xref:positional-and-named-attributes.adoc#named[named attribute] that can be assigned one or more values.
It can be defined globally as document attribute as well as a block attribute on an individual block.

There is no strict schema for options.
Any options which are not recognized are ignored.

"#
);

mod assign_options_to_blocks {
    non_normative!(
        r#"
== Assign options to blocks

You can assign one or more options to a block using the shorthand or formal syntax for the `options` attribute.

"#
    );

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, IsBlock},
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

    #[test]
    fn shorthand_syntax_single() {
        verifies!(
            r#"
=== Shorthand options syntax for blocks

To assign an option to a block, prefix the value with a percent sign (`%`) in an attribute list.
The percent sign implicitly sets the `options` attribute.

.Sidebar block with an option assigned using the shorthand dot
[source#ex-block]
----
[%option]
****
This is a sidebar with an option assigned to it, named option.
****
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[%option]\n****\nThis is a sidebar with an option assigned to it, named option.\n****",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(mi.item, TBlock::CompoundDelimited(
            TCompoundDelimitedBlock {
                blocks: vec![
                    TBlock::Simple(
                        TSimpleBlock {
                            content: TContent {
                                original: TSpan {
                                    data: "This is a sidebar with an option assigned to it, named option.",
                                    line: 3,
                                    col: 1,
                                    offset: 15,
                                },
                                rendered: None,
                                substitutions: vec!(),
                            },
                            source: TSpan {
                                data: "This is a sidebar with an option assigned to it, named option.",
                                line: 3,
                                col: 1,
                                offset: 15,
                            },
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },
                    ),
                ],
                context: "sidebar",
                source: TSpan {
                    data: "[%option]\n****\nThis is a sidebar with an option assigned to it, named option.\n****",
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
                                        data: "%option",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                ],
                                value: TSpan {
                                    data: "%option",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                                source: TSpan {
                                    data: "%option",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                            },
                        ],
                        source: TSpan {
                            data: "%option",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },
                ),
            },
        ));

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "option",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert!(options.next().is_none());
    }

    #[test]
    fn shorthand_syntax_multiple() {
        verifies!(
            r#"
You can assign multiple options to a block by prefixing each value with a percent sign (`%`).

.Sidebar with two options assigned using the shorthand dot
[source#ex-two-options]
----
[%option1%option2]
****
This is a sidebar with two options assigned to it, named option1 and option2.
****
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[%option1%option2]\n****\nThis is a sidebar with two options assigned to it, named option1 and option2.\n****",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(mi.item, TBlock::CompoundDelimited(
            TCompoundDelimitedBlock {
                blocks: vec![
                    TBlock::Simple(
                        TSimpleBlock {
                            content: TContent {
                                original: TSpan {
                                    data: "This is a sidebar with two options assigned to it, named option1 and option2.",
                                    line: 3,
                                    col: 1,
                                    offset: 24,
                                },
                                rendered: None,
                                substitutions: vec!(),
                            },
                            source: TSpan {
                                data: "This is a sidebar with two options assigned to it, named option1 and option2.",
                                line: 3,
                                col: 1,
                                offset: 24,
                            },
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },
                    ),
                ],
                context: "sidebar",
                source: TSpan {
                    data: "[%option1%option2]\n****\nThis is a sidebar with two options assigned to it, named option1 and option2.\n****",
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
                                        data: "%option1",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                    TSpan {
                                        data: "%option2",
                                        line: 1,
                                        col: 10,
                                        offset: 9,
                                    },
                                ],
                                value: TSpan {
                                    data: "%option1%option2",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                                source: TSpan {
                                    data: "%option1%option2",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                            },
                        ],
                        source: TSpan {
                            data: "%option1%option2",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },
                ),
            },
        ));

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "option1",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "option2",
                line: 1,
                col: 11,
                offset: 10,
            }
        );

        assert!(options.next().is_none());
    }

    #[test]
    fn table_with_three_options() {
        verifies!(
            r#"
For instance, consider a table with the three built-in option values, `header`, `footer`, and `autowidth`, assigned to it.
<<ex-table-short>> shows how the values are assigned using the shorthand notation.

.Table with three options assigned using the shorthand syntax
[source#ex-table-short]
----
[%header%footer%autowidth,cols=2*~]
|===
|Cell A1 |Cell B1

|Cell A2 |Cell B2

|Cell A3 |Cell B3
|===
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[%header%footer%autowidth,cols=2*~]\n|===\n|Cell A1 |Cell B1\n\n|Cell A2 |Cell B2\n\n|Cell A3 |Cell B3\n|===",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        // IMPORTANT: This test will have to be revised once we parse tables.

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "|===\n|Cell A1 |Cell B1",
                        line: 2,
                        col: 1,
                        offset: 36,
                    },
                    rendered: None,
                    substitutions: vec!(),
                },
                source: TSpan {
                    data: "[%header%footer%autowidth,cols=2*~]\n|===\n|Cell A1 |Cell B1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: vec![
                        TElementAttribute {
                            name: None,
                            shorthand_items: vec![
                                TSpan {
                                    data: "%header",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                                TSpan {
                                    data: "%footer",
                                    line: 1,
                                    col: 9,
                                    offset: 8,
                                },
                                TSpan {
                                    data: "%autowidth",
                                    line: 1,
                                    col: 16,
                                    offset: 15,
                                },
                            ],
                            value: TSpan {
                                data: "%header%footer%autowidth",
                                line: 1,
                                col: 2,
                                offset: 1,
                            },
                            source: TSpan {
                                data: "%header%footer%autowidth",
                                line: 1,
                                col: 2,
                                offset: 1,
                            },
                        },
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "cols",
                                line: 1,
                                col: 27,
                                offset: 26,
                            },),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "2*~",
                                line: 1,
                                col: 32,
                                offset: 31,
                            },
                            source: TSpan {
                                data: "cols=2*~",
                                line: 1,
                                col: 27,
                                offset: 26,
                            },
                        },
                    ],
                    source: TSpan {
                        data: "%header%footer%autowidth,cols=2*~",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "header",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "footer",
                line: 1,
                col: 10,
                offset: 9,
            }
        );

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "autowidth",
                line: 1,
                col: 17,
                offset: 16,
            }
        );

        assert!(options.next().is_none());
    }

    #[test]
    fn formal_options_syntax_single() {
        verifies!(
            r#"
=== Formal options syntax for blocks

Explicitly set `options` or `opts`, followed by the equals sign (`=`), and then the value in an attribute list.

.Sidebar block with an option assigned using the formal syntax
[source#ex-block-formal]
----
[opts=option]
****
This is a sidebar with an option assigned to it, named option.
****
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[opts=option]\n****\nThis is a sidebar with an option assigned to it, named option.\n****",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(mi.item, TBlock::CompoundDelimited(
            TCompoundDelimitedBlock {
                blocks: vec![
                    TBlock::Simple(
                        TSimpleBlock {
                            content: TContent {
                                original: TSpan {
                                    data: "This is a sidebar with an option assigned to it, named option.",
                                    line: 3,
                                    col: 1,
                                    offset: 19,
                                },
                                rendered: None,
                                substitutions: vec!(),
                            },
                            source: TSpan {
                                data: "This is a sidebar with an option assigned to it, named option.",
                                line: 3,
                                col: 1,
                                offset: 19,
                            },
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },
                    ),
                ],
                context: "sidebar",
                source: TSpan {
                    data: "[opts=option]\n****\nThis is a sidebar with an option assigned to it, named option.\n****",
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
                                name: Some(
                                    TSpan {
                                        data: "opts",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                ),
                                shorthand_items: vec![],
                                value: TSpan {
                                    data: "option",
                                    line: 1,
                                    col: 7,
                                    offset: 6,
                                },
                                source: TSpan {
                                    data: "opts=option",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                            },
                        ],
                        source: TSpan {
                            data: "opts=option",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },
                ),
            },
        ));

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "option",
                line: 1,
                col: 7,
                offset: 6,
            }
        );

        assert!(options.next().is_none());
    }

    #[test]
    fn formal_options_syntax_multiple() {
        verifies!(
            r#"
Separate multiple option values with commas (`,`).

.Sidebar with three options assigned using the formal syntax
[source#ex-three-roles-formal]
----
[opts="option1,option2"]
****
This is a sidebar with two options assigned to it, option1 and option2.
****
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[opts=\"option1,option2\"]\n****\nThis is a sidebar with two options assigned to it, option1 and option2.\n****",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(mi.item, TBlock::CompoundDelimited(
            TCompoundDelimitedBlock {
                blocks: vec![
                    TBlock::Simple(
                        TSimpleBlock {
                            content: TContent {
                                original: TSpan {
                                    data: "This is a sidebar with two options assigned to it, option1 and option2.",
                                    line: 3,
                                    col: 1,
                                    offset: 30,
                                },
                                rendered: None,
                                substitutions: vec!(),
                            },
                            source: TSpan {
                                data: "This is a sidebar with two options assigned to it, option1 and option2.",
                                line: 3,
                                col: 1,
                                offset: 30,
                            },
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },
                    ),
                ],
                context: "sidebar",
                source: TSpan {
                    data: "[opts=\"option1,option2\"]\n****\nThis is a sidebar with two options assigned to it, option1 and option2.\n****",
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
                                name: Some(
                                    TSpan {
                                        data: "opts",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                ),
                                shorthand_items: vec![],
                                value: TSpan {
                                    data: "option1,option2",
                                    line: 1,
                                    col: 8,
                                    offset: 7,
                                },
                                source: TSpan {
                                    data: "opts=\"option1,option2\"",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                            },
                        ],
                        source: TSpan {
                            data: "opts=\"option1,option2\"",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },
                ),
            },
        ));

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "option1",
                line: 1,
                col: 8,
                offset: 7,
            }
        );

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "option2",
                line: 1,
                col: 16,
                offset: 15,
            }
        );

        assert!(options.next().is_none());
    }

    #[test]
    fn table_with_three_options_formal() {
        verifies!(
            r#"
Let's revisit the table in <<ex-table-short>> that has the three built-in option values, `header`, `footer`, and `autowidth`, assigned to it using the shorthand notation (`%`).
Instead of using the shorthand notation, <<ex-table-formal>> shows how the values are assigned using the formal syntax.

.Table with three options assigned using the formal syntax
[source#ex-table-formal]
----
[cols=2*~,opts="header,footer,autowidth"]
|===
|Cell A1 |Cell B1

|Cell A2 |Cell B2

|Cell A3 |Cell B3
|===
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[cols=2*~,opts=\"header,footer,autowidth\"]\n|===\n|Cell A1 |Cell B1\n\n|Cell A2 |Cell B2\n\n|Cell A3 |Cell B3\n|===",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        // IMPORTANT: This test will have to be revised once we support tables.

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "|===\n|Cell A1 |Cell B1",
                        line: 2,
                        col: 1,
                        offset: 42,
                    },
                    rendered: None,
                    substitutions: vec!(),
                },
                source: TSpan {
                    data: "[cols=2*~,opts=\"header,footer,autowidth\"]\n|===\n|Cell A1 |Cell B1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: vec![
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "cols",
                                line: 1,
                                col: 2,
                                offset: 1,
                            },),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "2*~",
                                line: 1,
                                col: 7,
                                offset: 6,
                            },
                            source: TSpan {
                                data: "cols=2*~",
                                line: 1,
                                col: 2,
                                offset: 1,
                            },
                        },
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "opts",
                                line: 1,
                                col: 11,
                                offset: 10,
                            },),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "header,footer,autowidth",
                                line: 1,
                                col: 17,
                                offset: 16,
                            },
                            source: TSpan {
                                data: "opts=\"header,footer,autowidth\"",
                                line: 1,
                                col: 11,
                                offset: 10,
                            },
                        },
                    ],
                    source: TSpan {
                        data: "cols=2*~,opts=\"header,footer,autowidth\"",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "header",
                line: 1,
                col: 17,
                offset: 16,
            }
        );

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "footer",
                line: 1,
                col: 24,
                offset: 23,
            }
        );

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "autowidth",
                line: 1,
                col: 31,
                offset: 30,
            }
        );

        assert!(options.next().is_none());
    }
}

mod using_options_with_other_attributes {
    non_normative!(
        r#"
== Using options with other attributes

Let's consider `options` when combined with other attributes.
"#
    );

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, IsBlock},
        tests::{
            fixtures::{
                attributes::{TAttrlist, TElementAttribute},
                blocks::{TBlock, TSimpleBlock},
                content::TContent,
                TSpan,
            },
            sdd::{non_normative, verifies},
        },
        Parser, Span,
    };

    #[test]
    fn style_role_and_options() {
        verifies!(
            r#"
The following example shows how to structure an attribute list when you have style, role, and options attributes.

.Shorthand
[source]
----
[horizontal.properties%step] <.> <.> <.>
property 1:: does stuff
property 2:: does different stuff
----
<.> xref:blocks:styles.adoc[The block style attribute], declared as `horizontal` in this example, is a positional attribute.
A block style value is always placed at the start of the attribute list.
<.> `properties` is prefixed with a dot (`.`), signifying that it's assigned to the xref:role.adoc[role attribute].
The role and options attributes can be set in either order, i.e., `[horizontal%step.properties]`.
<.> The percent sign (`%`) sets the `options` attribute and assigns the `step` value to it.

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[horizontal.properties%step]\nproperty 1:: does stuff\nproperty 2:: does different stuff",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        // IMPORTANT: This test will have to be revised when we support attribute lists.

        assert_eq!(mi.item,
            TBlock::Simple(
                TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "property 1:: does stuff\nproperty 2:: does different stuff",
                            line: 2,
                            col: 1,
                            offset: 29,
                        },
                        rendered: None,
                        substitutions: vec!(),
                    },
                    source: TSpan {
                        data: "[horizontal.properties%step]\nproperty 1:: does stuff\nproperty 2:: does different stuff",
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
                                            data: "horizontal",
                                            line: 1,
                                            col: 2,
                                            offset: 1,
                                        },
                                        TSpan {
                                            data: ".properties",
                                            line: 1,
                                            col: 12,
                                            offset: 11,
                                        },
                                        TSpan {
                                            data: "%step",
                                            line: 1,
                                            col: 23,
                                            offset: 22,
                                        },
                                    ],
                                    value: TSpan {
                                        data: "horizontal.properties%step",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                    source: TSpan {
                                        data: "horizontal.properties%step",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                },
                            ],
                            source: TSpan {
                                data: "horizontal.properties%step",
                                line: 1,
                                col: 2,
                                offset: 1,
                            },
                        },
                    ),
                },
            ));

        assert_eq!(
            mi.item.declared_style().unwrap(),
            TSpan {
                data: "horizontal",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "step",
                line: 1,
                col: 24,
                offset: 23,
            }
        );

        assert!(options.next().is_none());

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "properties",
                line: 1,
                col: 13,
                offset: 12,
            }
        );

        assert!(roles.next().is_none());
    }

    #[test]
    fn style_role_and_options_formal() {
        verifies!(
            r#"
When you use the formal syntax, the positional and named attributes are separated by commas (`,`).

.Formal
[source]
----
[horizontal,role=properties,opts=step] <.>
property 1:: does stuff
property 2:: does different stuff
----
<.> Like in the shorthand example, named attributes such as `role` and `options` can be set in any order in the attribute list once any xref:positional-and-named-attributes.adoc#positional[positional attributes] are set.
"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(
            "[horizontal,role=properties,opts=step]\nproperty 1:: does stuff\nproperty 2:: does different stuff",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        // IMPORTANT: This test will have to be revised when we support attribute lists.

        assert_eq!(mi.item,
            TBlock::Simple(
            TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "property 1:: does stuff\nproperty 2:: does different stuff",
                        line: 2,
                        col: 1,
                        offset: 39,
                    },
                    rendered: None,
                    substitutions: vec!(),
                },
                source: TSpan {
                    data: "[horizontal,role=properties,opts=step]\nproperty 1:: does stuff\nproperty 2:: does different stuff",
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
                                        data: "horizontal",
                                        line: 1,
                                        col: 2,
                                        offset: 1,
                                    },
                                ],
                                value: TSpan {
                                    data: "horizontal",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                                source: TSpan {
                                    data: "horizontal",
                                    line: 1,
                                    col: 2,
                                    offset: 1,
                                },
                            },
                            TElementAttribute {
                                name: Some(
                                    TSpan {
                                        data: "role",
                                        line: 1,
                                        col: 13,
                                        offset: 12,
                                    },
                                ),
                                shorthand_items: vec![],
                                value: TSpan {
                                    data: "properties",
                                    line: 1,
                                    col: 18,
                                    offset: 17,
                                },
                                source: TSpan {
                                    data: "role=properties",
                                    line: 1,
                                    col: 13,
                                    offset: 12,
                                },
                            },
                            TElementAttribute {
                                name: Some(
                                    TSpan {
                                        data: "opts",
                                        line: 1,
                                        col: 29,
                                        offset: 28,
                                    },
                                ),
                                shorthand_items: vec![],
                                value: TSpan {
                                    data: "step",
                                    line: 1,
                                    col: 34,
                                    offset: 33,
                                },
                                source: TSpan {
                                    data: "opts=step",
                                    line: 1,
                                    col: 29,
                                    offset: 28,
                                },
                            },
                        ],
                        source: TSpan {
                            data: "horizontal,role=properties,opts=step",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },
                ),
            },
        ));

        assert_eq!(
            mi.item.declared_style().unwrap(),
            TSpan {
                data: "horizontal",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(
            options.next().unwrap(),
            TSpan {
                data: "step",
                line: 1,
                col: 34,
                offset: 33,
            }
        );

        assert!(options.next().is_none());

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "properties",
                line: 1,
                col: 18,
                offset: 17,
            }
        );

        assert!(roles.next().is_none());
    }
}
