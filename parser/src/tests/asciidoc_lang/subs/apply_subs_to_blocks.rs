use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/subs/pages/apply-subs-to-blocks.adoc");

non_normative!(
    r#"
= Customize the Substitutions Applied to Blocks

Each block context is associated with a set default substitutions that best suit the content model.
However, there are situations where you may need a different set of substitutions to be applied.
For example, you may want the AsciiDoc processor to substitute attribute references in a listing block.
Therefore, the AsciiDoc language provides a mechanism for altering the substitutions on a block.

"#
);

mod subs_attribute {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        tests::sdd::{non_normative, to_do_verifies, verifies},
    };

    non_normative!(
        r#"
== The subs attribute

The substitutions that get applied to a block (and to certain inline elements) can be changed or modified using the `subs` element attribute.
This attribute accepts a comma-separated list of substitution steps or groups.

The names of those substitution steps and groups are as follows:

[#subs-groups]
"#
    );

    #[test]
    fn none() {
        verifies!(
            r#"
`none`:: Substitution group that disables all substitutions.

"#
        );

        let doc = Parser::default().parse(
            "[subs=none]\nThis & _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This & _that_ and icon:github[] +\nanother line with a{sp}space there ..."
        );
    }

    #[test]
    fn normal() {
        verifies!(
            r#"
`normal`:: Substitution group that performs all substitution types except callouts.

"#
        );

        let doc = Parser::default().parse(
            "[subs=normal]\nThis & _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &amp; <em>that</em> and <span class=\"icon\">[github&#93;</span><br>\nanother line with a space there &#8230;&#8203;"
        );
    }

    #[test]
    fn verbatim() {
        verifies!(
            r#"
`verbatim`:: Substitution group that replaces special characters and processes callouts.

"#
        );

        let doc = Parser::default().parse(
            "[subs=verbatim]\nThis & _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &amp; _that_ and icon:github[] +\nanother line with a{sp}space there ..."
        );
    }

    #[test]
    fn specialchars() {
        verifies!(
            r#"
`specialchars`:: Substitution step that replaces `<`, `>`, and `&` with their corresponding entities.
For source blocks, this substitution step enables syntax highlighting as well.

"#
        );

        let doc = Parser::default().parse(
            "[subs=verbatim]\nThis & _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &amp; _that_ and icon:github[] +\nanother line with a{sp}space there ..."
        );
    }

    #[ignore]
    #[test]
    fn callouts() {
        // TO DO (https://github.com/scouten/asciidoc-parser/issues/311):
        // Implement this test when implementing callouts.
        to_do_verifies!(
            r#"
`callouts`:: Substitution step that processes callouts in literal, listing, and source blocks.

"#
        );

        let doc = Parser::default().parse(
            "[subs=verbatim]\nThis & _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &amp; that and icon:github[] +\nanother line with a{sp}space there ..."
        );
    }

    #[test]
    fn quotes() {
        verifies!(
            r#"
`quotes`:: Substitution step that applies inline text formatting.

"#
        );

        let doc = Parser::default().parse(
            "[subs=quotes]\nThis & _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This & <em>that</em> and icon:github[] +\nanother line with a{sp}space there ..."
        );
    }

    #[test]
    fn attributes() {
        verifies!(
            r#"
`attributes`:: Substitution step that replaces attribute references.

"#
        );

        let doc = Parser::default().parse(
            "[subs=attributes]\nThis & _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This & _that_ and icon:github[] +\nanother line with a space there ..."
        );
    }

    #[test]
    fn replacements() {
        verifies!(
            r#"
`replacements`:: Substitution step that replaces hexadecimal Unicode code points and entity, HTML, and XML character references with the characters' decimal Unicode code point.
The output of `replacements` may depend on whether the `specialcharacters` substitution was previously applied.

"#
        );

        let doc = Parser::default().parse(
            "[subs=replacements]\nThis &#169; _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &#169; _that_ and icon:github[] +\nanother line with a{sp}space there &#8230;&#8203;"
        );
    }

    #[test]
    fn macros() {
        verifies!(
            r#"
`macros`:: Substitution step that processes inline and block macros.

"#
        );

        let doc = Parser::default().parse(
            "[subs=macros]\nThis &#169; _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &#169; _that_ and <span class=\"icon\">[github&#93;</span> +\nanother line with a{sp}space there ..."
        );
    }

    #[test]
    fn post_replacements() {
        verifies!(
            r#"
`post_replacements`:: Substitution step that processes the line break character (`{plus}`).

"#
        );

        let doc = Parser::default().parse(
            "[subs=post_replacements]\nThis &#169; _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &#169; _that_ and icon:github[]<br>\nanother line with a{sp}space there ..."
        );
    }

    #[test]
    fn plus_and_minus_modifiers() {
        verifies!(
            r#"
If a `+` or `-` modifier is added to a step, the existing substitutions are modified accordingly (see <<incremental,incremental subs>>).
Otherwise, the existing substitutions are replaced.
The value also specifies the order in which the substitutions are applied.

"#
        );

        let doc = Parser::default().parse(
            "[subs=+macros]\n----\nThis &#169; _that_ and icon:github[] +\nanother line with a{sp}space there ...\n----",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::RawDelimited(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &amp;#169; _that_ and <span class=\"icon\">[github&#93;</span> +\nanother line with a{sp}space there ..."
        );

        let doc = Parser::default().parse(
            "[subs=-macros]\nThis &#169; _that_ and icon:github[] +\nanother line with a{sp}space there ...",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &#169; <em>that</em> and icon:github[]<br>\nanother line with a space there &#8230;&#8203;"
        );
    }

    #[test]
    fn does_not_inherit() {
        verifies!(
            r#"
NOTE: The `subs` element attribute does not inherit to nested blocks.
It can only be applied to a leaf block, which is any block that cannot have child blocks (e.g., a paragraph or a listing block).

"#
        );

        let doc = Parser::default().parse(
            ":icons:\n\n[subs=none]\n****\nThis &#169; _that_ and icon:github[] +\nanother line with a{sp}space there ...\n****",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::CompoundDelimited(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        let block1 = block1.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "This &#169; <em>that</em> and <span class=\"icon\"><img src=\"./images/icons/github.png\" alt=\"github\"></span><br>\nanother line with a space there &#8230;&#8203;"
        );
    }
}

mod set_subs_attribute_on_block {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        tests::sdd::{non_normative, verifies},
    };

    non_normative!(
        r#"
w== Set the subs attribute on a block

CAUTION: You should always prefer to use <<incremental,incremental substitutions>>, and only revert to exact substitutions when you require the additional control.

"#
    );

    #[test]
    fn inline_formatting_in_source() {
        verifies!(
            r#"
Let's look at an example where you want to process inline formatting markup in a source block.
By default, source blocks (as well as other verbatim blocks) are only subject to the verbatim substitution group (specialchars and callouts).
You can change this behavior by setting the `subs` attribute in the block's attribute list.

[source,asciidoc]
....
include::example$subs.adoc[tag=subs-in]
....
<.> The `subs` attribute is set in the attribute list and assigned the `verbatim` and `quotes` values.
It's important to reinstate the `verbatim` substitution step to ensure special characters are encoded (which, for source blocks, also enables syntax highlighting).
<.> The formatting markup in this line will be replaced when the `quotes` substitution step runs.

Here's the result.

====
include::example$subs.adoc[tag=subs-out]
====
<.> The `verbatim` value enables any special characters and callouts to be processed.
<.> The `quotes` value enables the bold text formatting to be processed.

"#
        );

        let doc = Parser::default().parse(
            "[source,java,subs=\"verbatim,quotes\"]\n----\nSystem.out.println(\"Hello *<name>*\")\n----",
        );

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::RawDelimited(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            block1.content().rendered(),
            "System.out.println(\"Hello <strong>&lt;name&gt;</strong>\")"
        );
    }
}
