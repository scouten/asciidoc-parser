use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/subs/pages/replacements.adoc");

non_normative!(
    r#"
= Character Replacement Substitutions
:navtitle: Character Replacements
:table-caption: Table
:y: Yes
//icon:check[role="green"]
:n: No
//icon:times[role="red"]
:url-html-ref: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references
:url-unicode: https://en.wikipedia.org/wiki/List_of_Unicode_characters

"#
);

mod replacements {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        tests::sdd::{non_normative, verifies},
    };

    non_normative!(
        r#"
The character replacement substitution step processes textual characters such as marks, arrows and dashes and replaces them with the decimal format of their Unicode code point, i.e., their <<char-ref-sidebar,numeric character reference>>.
The replacements step depends on the substitutions completed by the xref:special-characters.adoc[special characters step].

"#
    );

    verifies!(
        r#"
// Table of Textual symbol replacements is inserted below
include::partial$subs-symbol-repl.adoc[]

"#
    );

    #[test]
    fn copyright() {
        let doc = Parser::default().parse("(C)");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#169;");
    }

    #[test]
    fn registered() {
        let doc = Parser::default().parse("(R)");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#174;");
    }

    #[test]
    fn trademark() {
        let doc = Parser::default().parse("(TM)");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#8482;");
    }

    #[test]
    fn em_dash() {
        let doc = Parser::default().parse("abc--def");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "abc&#8212;&#8203;def");
    }

    #[test]
    fn em_dash_surrounded_by_spaces() {
        let doc = Parser::default().parse("abc -- def");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "abc&#8201;&#8212;&#8201;def");
    }

    #[test]
    fn ellipsis() {
        let doc = Parser::default().parse("...");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#8230;&#8203;");
    }

    #[test]
    fn single_right_arrow() {
        let doc = Parser::default().parse("->");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#8594;");
    }

    #[test]
    fn double_right_arrow() {
        let doc = Parser::default().parse("=>");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#8658;");
    }

    #[test]
    fn single_left_arrow() {
        let doc = Parser::default().parse("<-");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#8592;");
    }

    #[test]
    fn double_left_arrow() {
        let doc = Parser::default().parse("<=");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "&#8656;");
    }

    #[test]
    fn typographic_apostrophe() {
        let doc = Parser::default().parse("Sam's");

        let block1 = doc.nested_blocks().next().unwrap();

        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(sb1.content().rendered(), "Sam&#8217;s");
    }
}
