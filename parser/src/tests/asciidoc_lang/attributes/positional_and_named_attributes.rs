use crate::tests::prelude::*;

track_file!("docs/modules/attributes/pages/positional-and-named-attributes.adoc");

non_normative!(
    r#"
= Positional and Named Attributes

This page breaks down the difference between positional and named attributes on an element and the rules for parsing an attribute list.

"#
);

mod positional_attribute {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{IsBlock, metadata::BlockMetadata},
        content::SubstitutionGroup,
        tests::prelude::*,
    };

    non_normative!(
        r#"
[#positional]
== Positional attribute

// tag::pos[]
Entries in an attribute list that only consist of a value are referred to as positional attributes.
The position is the 1-based index of the entry once all named attributes have been removed (so they may be interspersed).

"#
    );

    #[test]
    fn implicit_attribute_name() {
        verifies!(
            r#"
The positional attribute may be dually assigned to an implicit attribute name if the block or macro defines a mapping for positional attributes.
Here are some examples of those mappings:

* `icon:` 1 => size
* `image:` and `image::` 1 => alt (text), 2 => width, 3 => height
* Delimited blocks: 1 => block style and attribute shorthand
* Other inline quoted text: 1 => attribute shorthand
* `link:` and `xref:` 1 => text
* Custom blocks and macros can also specify positional attributes

For example, the following two image macros are equivalent.

[source]
----
image::sunset.jpg[Sunset,300,400]

image::sunset.jpg[alt=Sunset,width=300,height=400]
----

The second macro is the same as the first, but written out in longhand form.
// end::pos[]

"#
        );

        let mut parser = Parser::default();

        let m1 = crate::blocks::MediaBlock::parse(
            &BlockMetadata::new("image::sunset.jpg[Sunset,300,400]"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap();

        let mut parser = Parser::default();

        let m2 = crate::blocks::MediaBlock::parse(
            &BlockMetadata::new("image::sunset.jpg[alt=Sunset,width=300,height=400]"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap();

        let a1 = m1.item.macro_attrlist();
        let a2 = m2.item.macro_attrlist();

        assert_eq!(
            a1.named_or_positional_attribute("alt", 1).unwrap(),
            ElementAttribute {
                name: None,
                shorthand_items: &["Sunset"],
                value: "Sunset"
            },
        );

        assert_eq!(
            a2.named_or_positional_attribute("alt", 1).unwrap(),
            ElementAttribute {
                name: Some("alt"),
                shorthand_items: &[],
                value: "Sunset"
            }
        );
    }

    #[test]
    fn block_style_and_attribute_shorthand() {
        non_normative!(
            r#"
=== Block style and attribute shorthand

The first positional attribute on all blocks (including sections) is special.
It's used to define the xref:blocks:index.adoc#block-style[block style].
It also supports a shorthand syntax for defining the ID, role, and options attributes.
This shorthand syntax can also be used on formatted text, even though formatted text doesn't technically support attributes.

The attribute shorthand is inspired by the HAML and Slim template languages as a way of saving the author some typing.
Instead of having to use the longhand form of a name attribute, it's possible to compress the assignment to a value prefixed by a special marker.
The markers are mapped as follows:

* `#` - ID
* `.` - role
* `%` - option

Each shorthand entry is placed directly adjacent to previous one, starting immediately after the optional block style.
The order of the entries does not matter, except for the style, which must come first.
        
"#
        );

        verifies!(
            r#"
Here's an example that shows how to set an ID on a section using this shorthand syntax:

----
[#custom-id]
== Section with Custom ID
----

"#
        );

        let mut parser = Parser::default();

        let block = crate::blocks::Block::parse(
            crate::Span::new("[#custom-id]\n== Section with Custom ID\n"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap()
        .item;

        assert_eq!(
            block,
            Block::Section(SectionBlock {
                level: 1,
                section_title: Span {
                    data: "Section with Custom ID",
                    line: 2,
                    col: 4,
                    offset: 16,
                },
                blocks: &[],
                source: Span {
                    data: "[#custom-id]\n== Section with Custom ID",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: None,
                        shorthand_items: &["#custom-id"],
                        value: "#custom-id"
                    },],
                    source: Span {
                        data: "#custom-id",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        assert_eq!(block.id().unwrap(), "custom-id");

        verifies!(
            r#"
The shorthand entry must follow the block style, if present.
Here's an example that shows how to set an ID on an appendix section using this shorthand syntax:

----
[appendix#custom-id]
== Appendix with Custom ID
----

"#
        );

        let mut parser = Parser::default();

        let block = crate::blocks::Block::parse(
            crate::Span::new("[appendix#custom-id]\n== Appendix with Custom ID\n"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap()
        .item;

        assert_eq!(
            block,
            Block::Section(SectionBlock {
                level: 1,
                section_title: Span {
                    data: "Appendix with Custom ID",
                    line: 2,
                    col: 4,
                    offset: 24,
                },
                blocks: &[],
                source: Span {
                    data: "[appendix#custom-id]\n== Appendix with Custom ID",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: None,
                        shorthand_items: &["appendix", "#custom-id"],
                        value: "appendix#custom-id"
                    },],
                    source: Span {
                        data: "appendix#custom-id",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        assert_eq!(block.declared_style().unwrap(), "appendix");
        assert_eq!(block.id().unwrap(), "custom-id");
        assert_eq!(block.substitution_group(), SubstitutionGroup::Normal);

        verifies!(
            r#"
Here's an example of a block that uses the shorthand syntax to set the ID, a role, and an option for a list.
Specifically, this syntax sets the ID to `rules`, adds the role `prominent`, and sets the option `incremental`.

----
[#rules.prominent%incremental]
* Work hard
* Play hard
* Be happy
----

"#
        );

        let mut parser = Parser::default();

        let block = crate::blocks::Block::parse(
            crate::Span::new(
                "[#rules.prominent%incremental]\n* Work hard\n* Play hard\n* Be happy\n",
            ),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap()
        .item;

        // TO DO: This will change when we understand lists.
        assert_eq!(
            block,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "* Work hard\n* Play hard\n* Be happy",
                        line: 2,
                        col: 1,
                        offset: 31,
                    },
                    rendered: "* Work hard\n* Play hard\n* Be happy",
                },
                source: Span {
                    data: "[#rules.prominent%incremental]\n* Work hard\n* Play hard\n* Be happy",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: None,
                        shorthand_items: &["#rules", ".prominent", "%incremental"],
                        value: "#rules.prominent%incremental"
                    },],
                    source: Span {
                        data: "#rules.prominent%incremental",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        assert_eq!(block.id().unwrap(), "rules");
        assert_eq!(block.roles().first().unwrap(), &"prominent");
        assert_eq!(block.options().first().unwrap(), &"incremental");

        assert!(!block.has_option("rules"));
        assert!(!block.has_option("prominent"));
        assert!(block.has_option("incremental"));

        verifies!(
            r#"
A block can have multiple roles and options, so these shorthand entries may be repeated.
Here's an example that shows how to set several options on a table.
Specifically, this syntax sets the `header`, `footer`, and `autowidth` options.

----
[%header%footer%autowidth]
|===
|Header A |Header B
|Footer A |Footer B
|===
----

"#
        );

        let mut parser = Parser::default();

        let block = crate::blocks::Block::parse(crate::Span::new(
            "[%header%footer%autowidth]\n|===\n|Header A |Header B\n|Footer A |Footer B\n|===\n",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap()
        .item;

        assert_eq!(
            block,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "|===\n|Header A |Header B\n|Footer A |Footer B\n|===",
                        line: 2,
                        col: 1,
                        offset: 27,
                    },
                    rendered: "|===\n|Header A |Header B\n|Footer A |Footer B\n|===",
                },
                source: Span {
                    data: "[%header%footer%autowidth]\n|===\n|Header A |Header B\n|Footer A |Footer B\n|===",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: None,
                        shorthand_items: &["%header", "%footer", "%autowidth",],
                        value: "%header%footer%autowidth"
                    },],
                    source: Span {
                        data: "%header%footer%autowidth",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        verifies!(
            r#"
This shorthand syntax also appears on formatted text.
Here's an example that shows how to set the ID and add a role to a strong phrase.
Specifically, this syntax sets the ID to `free-world` and adds the `goals` role.

----
[#free-world.goals]*free the world*
----

Formatted text does not support a style, so the first and only positional attribute is always the shorthand syntax.

"#
        );

        let mut parser = Parser::default();

        let doc = parser.parse("[#free-world.goals]*free the world*");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "[#free-world.goals]*free the world*",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<strong id=\"free-world\" class=\"goals\">free the world</strong>",
                    },
                    source: Span {
                        data: "[#free-world.goals]*free the world*",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "[#free-world.goals]*free the world*",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}

mod named_attribute {
    use pretty_assertions_sorted::assert_eq;

    use crate::{Parser, tests::prelude::*};

    non_normative!(
        r#"
[#named]
== Named attribute

// tag::name[]
"#
    );

    #[test]
    fn basic_syntax() {
        verifies!(
            r#"
A named attribute consists of a name and a value separated by an `=` character (e.g., `name=value`).

"#
        );

        let mut parser = Parser::default();

        let doc = parser.parse("[foo=bar]\nSome text here.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "Some text here.",
                            line: 2,
                            col: 1,
                            offset: 10,
                        },
                        rendered: "Some text here.",
                    },
                    source: Span {
                        data: "[foo=bar]\nSome text here.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: Some(Attrlist {
                        attributes: &[ElementAttribute {
                            name: Some("foo"),
                            value: "bar",
                            shorthand_items: &[],
                        },],
                        source: Span {
                            data: "foo=bar",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: Span {
                    data: "[foo=bar]\nSome text here.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn enclose_value_in_quotes() {
        verifies!(
            r#"
If the value contains a space, comma, or quote character, it must be enclosed in double or single quotes (e.g., `name="value with space"`).
In all other cases, the surrounding quotes are optional.

"#
        );

        let mut parser = Parser::default();

        let doc = parser.parse("[name=\"value with space\"]\nSome text here.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "Some text here.",
                            line: 2,
                            col: 1,
                            offset: 26,
                        },
                        rendered: "Some text here.",
                    },
                    source: Span {
                        data: "[name=\"value with space\"]\nSome text here.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: Some(Attrlist {
                        attributes: &[ElementAttribute {
                            name: Some("name",),
                            value: "value with space",
                            shorthand_items: &[],
                        },],
                        source: Span {
                            data: "name=\"value with space\"",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: Span {
                    data: "[name=\"value with space\"]\nSome text here.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn escape_same_quote_with_backslash() {
        verifies!(
            r#"
If the value contains the *same* quote character used to enclose the value, the quote character in the value must be escaped by prefixing it with a backslash (e.g., `value="the song \"Dark Horse\""`).

If enclosing quotes are used, they are dropped from the parsed value and the preceding backslash is dropped from any escaped quotes.

"#
        );

        let mut parser = Parser::default();

        let doc = parser.parse("[value=\"the song \\\"Dark Horse\\\"\"]\nSome text here.");

        assert_eq!(
            &doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "Some text here.",
                            line: 2,
                            col: 1,
                            offset: 34,
                        },
                        rendered: "Some text here.",
                    },
                    source: Span {
                        data: "[value=\"the song \\\"Dark Horse\\\"\"]\nSome text here.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: Some(Attrlist {
                        attributes: &[ElementAttribute {
                            name: Some("value",),
                            value: "the song \"Dark Horse\"",
                            shorthand_items: &[],
                        },],
                        source: Span {
                            data: "value=\"the song \\\"Dark Horse\\\"\"",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: Span {
                    data: "[value=\"the song \\\"Dark Horse\\\"\"]\nSome text here.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}

mod attribute_list_parsing {
    use pretty_assertions_sorted::assert_eq;

    use crate::{Parser, tests::prelude::*};

    non_normative!(
        r#"
== Attribute list parsing

The source text that's used to define attributes for an element is referred to as an [.term]*attrlist*.
An attrlist is always enclosed in a pair of square brackets.
This applies for block attributes as well as attributes on a block or inline macro.
The processor splits the attrlist into individual attribute entries, determines whether each entry is a positional or named attribute, parses the entry accordingly, and assigns the result as an attribute on the node.

The rules for what defines the boundaries of an individual attribute, and whether the attribute is positional or named, are defined below.
"#
    );

    #[test]
    fn definition_of_name() {
        verifies!(
            r#"
In these rules, `name` consists of a word character (letter or numeral) followed by any number of word or `-` characters (e.g., `see-also`).

"#
        );

        let mut parser = Parser::default();

        let doc = parser.parse("[foo=bar,94-pages=94]\nSome text here.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "Some text here.",
                            line: 2,
                            col: 1,
                            offset: 22,
                        },
                        rendered: "Some text here.",
                    },
                    source: Span {
                        data: "[foo=bar,94-pages=94]\nSome text here.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: Some(Attrlist {
                        attributes: &[
                            ElementAttribute {
                                name: Some("foo"),
                                value: "bar",
                                shorthand_items: &[],
                            },
                            ElementAttribute {
                                name: Some("94-pages",),
                                value: "94",
                                shorthand_items: &[],
                            },
                        ],
                        source: Span {
                            data: "foo=bar,94-pages=94",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: Span {
                    data: "[foo=bar,94-pages=94]\nSome text here.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn attribute_substitution_before_parsing() {
        verifies!(
            r#"
* Attribute references are expanded before the attrlist is parsed (i.e., the attributes substitution is applied).
"#
        );

        let mut parser = Parser::default();

        let doc =
            parser.parse(":url: https://example.com\n\n[foo=bar,target={url}]\nSome text here.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[Attribute {
                        name: Span {
                            data: "url",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                        value_source: Some(Span {
                            data: "https://example.com",
                            line: 1,
                            col: 7,
                            offset: 6,
                        },),
                        value: InterpretedValue::Value("https://example.com",),
                        source: Span {
                            data: ":url: https://example.com",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },],
                    source: Span {
                        data: ":url: https://example.com",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "Some text here.",
                            line: 4,
                            col: 1,
                            offset: 50,
                        },
                        rendered: "Some text here.",
                    },
                    source: Span {
                        data: "[foo=bar,target={url}]\nSome text here.",
                        line: 3,
                        col: 1,
                        offset: 27,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: Some(Attrlist {
                        attributes: &[
                            ElementAttribute {
                                name: Some("foo",),
                                value: "bar",
                                shorthand_items: &[],
                            },
                            ElementAttribute {
                                name: Some("target",),
                                value: "https://example.com",
                                shorthand_items: &[],
                            },
                        ],
                        source: Span {
                            data: "foo=bar,target={url}",
                            line: 3,
                            col: 2,
                            offset: 28,
                        },
                    },),
                },),],
                source: Span {
                    data: ":url: https://example.com\n\n[foo=bar,target={url}]\nSome text here.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}
