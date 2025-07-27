#![allow(unused)]

use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/subs/pages/special-characters.adoc");

non_normative!(
    r#"
= Special Character Substitutions
:navtitle: Special Characters
:table-caption: Table
:y: Yes
:n: No

"#
);

mod substitutions {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        span::content::SubstitutionGroup,
        tests::sdd::{non_normative, to_do_verifies, verifies},
    };

    non_normative!(
        r#"
The special characters substitution step searches for three characters (`<`, `>`, `&`) and replaces them with their xref:replacements.adoc#char-ref-sidebar[named character references].

"#
    );

    #[test]
    fn less_than_symbol() {
        verifies!(
            r#"
* The less than symbol, `<`, is replaced with the named character reference `\&lt;`.
"#
        );

        let doc = Parser::default().parse("Replace this < with lt");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "Replace this &lt; with lt");
    }

    #[test]
    fn greater_than_symbol() {
        verifies!(
            r#"
* The greater than symbol, `>`, is replaced with the named character reference `\&gt;`.
"#
        );

        let doc = Parser::default().parse("Replace this > with gt");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "Replace this &gt; with gt");
    }

    #[test]
    fn ampersand() {
        verifies!(
            r#"
* An ampersand, `&`, is replaced with the named character reference `\&amp;`.
"#
        );

        let doc = Parser::default().parse("Replace this & with amp");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(block1.content().rendered(), "Replace this &amp; with amp");
    }
}

mod default_special_characters_substitution {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        parser::ModificationContext,
        span::content::SubstitutionGroup,
        tests::sdd::{non_normative, to_do_verifies, verifies},
    };

    non_normative!(
        r#"
== Default special characters substitution

<<table-special>> lists the specific blocks and inline elements the special characters substitution step applies to automatically.

.Blocks and inline elements subject to the special characters substitution
[#table-special%autowidth,cols="~,^~"]
|===
|Blocks and elements |Substitution step applied by default

"#
    );

    #[test]
    fn attribute_entry_values() {
        verifies!(
            r#"
|Attribute entry values |{y}

"#
        );

        let doc = Parser::default().parse(":lt-attr: <\n\nGoodbye {lt-attr} hello");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "Goodbye &lt; hello");
    }

    #[test]
    fn comments() {
        verifies!(
            r#"
|Comments |{n}

"#
        );

        let doc = Parser::default().parse("////\nabc < def\n////");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::RawDelimited(block1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(block1.content().rendered(), "abc < def");
    }
}
