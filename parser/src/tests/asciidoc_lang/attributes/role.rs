use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/attributes/pages/role.adoc");
// Tracking commit c7d2b3e4, current as of 2025-04-10.

non_normative!(
    r#"
= Role Attribute
:page-aliases: roles.adoc

You can assign one or more roles to blocks and most inline elements using the `role` attribute.
The `role` attribute is a xref:positional-and-named-attributes.adoc#named[named attribute].
Even though the attribute name is singular, it may contain multiple (space-separated) roles.
Roles may also be defined using a shorthand (dot-prefixed) syntax.

A role:

. adds additional semantics to an element
. can be used to apply additional styling to a group of elements (e.g., via a CSS class selector)
. may activate additional behavior if recognized by the converter

TIP: The `role` attribute in AsciiDoc always get mapped to the `class` attribute in the HTML output.
In other words, role names are synonymous with HTML class names, thus allowing output elements to be identified and styled in CSS using class selectors (e.g., `sidebarblock.role1`).

"#
);

mod assign_roles_to_blocks {
    non_normative!(
        r#"
== Assign roles to blocks

"#
    );

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        tests::{
            fixtures::{
                Span,
                attributes::{TAttrlist, TElementAttribute},
                blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
                content::TContent,
            },
            sdd::{non_normative, verifies},
        },
    };

    non_normative!(
        r#"
You can assign roles to blocks using the shorthand dot (`.`) syntax or the longhand (`role=`) syntax.

"#
    );

    #[test]
    fn shorthand_role_syntax_single() {
        verifies!(
            r#"
=== Shorthand role syntax for blocks

To assign a role to a block, prefix the value with a dot (`.`) in style style position of an attribute list.
The dot implicitly sets the `role` attribute.

.Sidebar block with a role assigned using the shorthand dot
[source#ex-block]
----
[.rolename]
****
This is a sidebar with a role assigned to it, rolename.
****
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(
            crate::Span::new(
                "[.rolename]\n****\nThis is a sidebar with a role assigned to it, rolename.\n****",
            ),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "This is a sidebar with a role assigned to it, rolename.",
                            line: 3,
                            col: 1,
                            offset: 17,
                        },
                        rendered: "This is a sidebar with a role assigned to it, rolename.",
                    },
                    source: Span {
                        data: "This is a sidebar with a role assigned to it, rolename.",
                        line: 3,
                        col: 1,
                        offset: 17,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "sidebar",
                source: Span {
                    data: "[.rolename]\n****\nThis is a sidebar with a role assigned to it, rolename.\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: &[TElementAttribute {
                        name: None,
                        shorthand_items: &[".rolename"],
                        value: ".rolename"
                    },],
                    source: Span {
                        data: ".rolename",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );
    }

    #[test]
    fn shorthand_role_syntax_multiple() {
        verifies!(
            r#"
You can assign multiple roles to a block by prefixing each value with a dot (`.`).

.Sidebar with two roles assigned using the shorthand dot
[source#ex-two-roles]
----
[.role1.role2]
****
This is a sidebar with two roles assigned to it, role1 and role2.
****
----

The role values are turned into a space-separated list of values, `role1 role2`.

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(crate::Span::new(
            "[.role1.role2]\n****\nThis is a sidebar with two roles assigned to it, role1 and role2.\n****",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "This is a sidebar with two roles assigned to it, role1 and role2.",
                            line: 3,
                            col: 1,
                            offset: 20,
                        },
                        rendered: "This is a sidebar with two roles assigned to it, role1 and role2.",
                    },
                    source: Span {
                        data: "This is a sidebar with two roles assigned to it, role1 and role2.",
                        line: 3,
                        col: 1,
                        offset: 20,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "sidebar",
                source: Span {
                    data: "[.role1.role2]\n****\nThis is a sidebar with two roles assigned to it, role1 and role2.\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: &[TElementAttribute {
                        name: None,
                        shorthand_items: &[".role1", ".role2"],
                        value: ".role1.role2"
                    },],
                    source: Span {
                        data: ".role1.role2",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );
    }

    #[test]
    fn formal_role_syntax_single() {
        verifies!(
            r#"
=== Formal role syntax for blocks

You can define the roles using a named attribute instead, which is the longhand syntax for adding roles to an element.
When using this syntax, add the attribute name `role` followed by the equals sign (`=`) then the role name or names to any position in the block attribute list.

.Sidebar block with a role assigned using the formal syntax
[source#ex-block-formal]
----
[role=rolename]
****
This is a sidebar with one role assigned to it, rolename.
****
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(crate::Span::new(
            "[role=rolename]\n****\nThis is a sidebar with one role assigned to it, rolename.\n****",
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "This is a sidebar with one role assigned to it, rolename.",
                            line: 3,
                            col: 1,
                            offset: 21,
                        },
                        rendered: "This is a sidebar with one role assigned to it, rolename.",
                    },
                    source: Span {
                        data: "This is a sidebar with one role assigned to it, rolename.",
                        line: 3,
                        col: 1,
                        offset: 21,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "sidebar",
                source: Span {
                    data: "[role=rolename]\n****\nThis is a sidebar with one role assigned to it, rolename.\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: &[TElementAttribute {
                        name: Some("role"),
                        shorthand_items: &[],
                        value: "rolename"
                    },],
                    source: Span {
                        data: "role=rolename",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(roles.next().unwrap(), &"rolename");

        assert!(roles.next().is_none());
    }

    #[test]
    fn formal_role_syntax_multiple() {
        verifies!(
            r#"
Separate multiple role values using spaces.
Since the value has spaces, it's easier to read if enclosed in quotes, though the quotes are not strictly required.

.Sidebar with two roles assigned using the formal syntax
[source#ex-two-roles-formal]
----
[role="role1 role2"]
****
This is a sidebar with two roles assigned to it, role1 and role2.
****
----

"#
        );

        let mut parser = Parser::default();

        let mi = Block::parse(crate::Span::new(
            "[role=\"role1 role2\"]\n****\nThis is a sidebar with two roles assigned to it, role1 and role2.\n****"
        ), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "This is a sidebar with two roles assigned to it, role1 and role2.",
                            line: 3,
                            col: 1,
                            offset: 26,
                        },
                        rendered: "This is a sidebar with two roles assigned to it, role1 and role2.",
                    },
                    source: Span {
                        data: "This is a sidebar with two roles assigned to it, role1 and role2.",
                        line: 3,
                        col: 1,
                        offset: 26,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "sidebar",
                source: Span {
                    data: "[role=\"role1 role2\"]\n****\nThis is a sidebar with two roles assigned to it, role1 and role2.\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: &[TElementAttribute {
                        name: Some("role"),
                        shorthand_items: &[],
                        value: "role1 role2"
                    },],
                    source: Span {
                        data: "role=\"role1 role2\"",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(roles.next().unwrap(), &"role1");

        assert_eq!(roles.next().unwrap(), &"role2");

        assert!(roles.next().is_none());
    }

    non_normative!(
        r#"
In this form, the value of the role attribute is already in the right form to be passed through to the output.
No additional processing is done on it.

This longhand syntax can also be used on inline macros, but it cannot be used with formatted (aka quoted) text.

"#
    );
}

mod assign_roles_to_formatted_inline_elements {
    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        tests::sdd::{non_normative, verifies},
    };

    non_normative!(
        r#"
== Assign roles to formatted inline elements

        "#
    );

    #[test]
    fn assign_roles_to_inline_elements() {
        verifies!(
            r#"
You can assign roles to inline elements that are enclosed in formatting syntax, such as bold (`+*+`), italic (`+_+`), and monospace (`++`++`).
To assign a role to an inline element that's enclosed in formatting syntax block, prefix the value with a dot (`.`) in an attribute list.

.Inline role assignments using shorthand syntax
[source#ex-role-dot]
----
This sentence contains [.application]*bold inline content* that's assigned a role.

This sentence contains [.varname]`monospace text` that's assigned a role.
----

The HTML source code that is output from <<ex-role-dot>> is shown below.

.HTML source code produced by <<ex-role-dot>>
[source#ex-role-html,html]
----
<p>This sentence contains <strong class="application">bold inline content</strong> that&#8217;s assigned a role.</p>

<p>This sentence contains <code class="varname">monospace text</code> that&#8217;s assigned a role.</p>
</div>
----

As you can see from this output, roles in AsciiDoc are translated to CSS class names in HTML.
Thus, roles are an ideal way to annotated elements in your document so you can use CSS to uniquely style them.

"#
        );

        let doc = Parser::default().parse(
            "This sentence contains [.application]*bold inline content* that's assigned a role.\n\nThis sentence contains [.varname]`monospace text` that's assigned a role."
        );

        let mut blocks = doc.nested_blocks();

        let block1 = blocks.next().unwrap();
        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            sb1.content().rendered(),
            r#"This sentence contains <strong class="application">bold inline content</strong> that&#8217;s assigned a role."#
        );

        let block2 = blocks.next().unwrap();
        let Block::Simple(sb2) = block2 else {
            panic!("Unexpected block type: {block2:?}");
        };

        assert_eq!(
            sb2.content().rendered(),
            r#"This sentence contains <code class="varname">monospace text</code> that&#8217;s assigned a role."#
        );

        assert!(blocks.next().is_none());
    }

    non_normative!(
        r#"
The role is often used on a phrase to represent semantics you might have expressed using a dedicated element in DocBook or DITA.

////
Using the shorthand notation, an id can also be specified:

[source]
----
[#idname.rolename]`monospace text`
----

which produces:

[source,html]
----
<a id="idname"></a><code class="rolename">monospace text</code>
----
////
        "#
    );
}
