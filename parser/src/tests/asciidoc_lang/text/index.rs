use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/text/pages/index.adoc");

non_normative!(
    r#"
= Text Formatting and Punctuation

Just as we emphasize certain words and phrases when we speak, we emphasize words and phrases in text using formatting and punctuation.
AsciiDoc provides an assortment of formatting marks for applying visual emphasis and typographic punctuation to your document.
You can build on these basic formatting marks using built-in and user-defined roles.
This page covers the formatting marks that AsciiDoc provides and the rules for applying and customizing them.

"#
);

mod formatting_terms_and_concepts {
    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        tests::sdd::{non_normative, verifies},
    };

    non_normative!(
        r#"
== Formatting terms and concepts

=== Formatting marks and pairs

A [#def-format-mark.term]*formatting mark* is a symbolic character, such as `+*+`, `_`, or `~`, that indicates the inline style you want the AsciiDoc converter to apply to the text.
Formatting marks come in pairs.

A [#def-format-pair.term]*formatting pair* consists of an identical opening mark and closing mark that encloses the text you want to style.
The formatted text (i.e., the text enclosed by a formatting pair) can span multiple, contiguous lines.

The [#def-open-mark.term]*opening mark* specifies where you want the style to start.
The [#def-close-mark.term]*closing mark* specifies where you want the style to end.

Formatting pairs can be nested, but they cannot be overlapped.
If the pairs are overlapped, the behavior is unspecified and the AsciiDoc processor may produce malformed output.

A formatting pair is defined as either constrained or unconstrained, depending on where it's allowed to be applied.
An unconstrained pair can be applied anywhere, whereas the application of a constrained pair is more limited.

"#
    );

    #[test]
    fn constrained_formatting_pair() {
        verifies!(
            r#"
[#constrained]
=== Constrained formatting pair

When a space-like character directly precedes the text to format, and a space-like character or punctuation mark (`,`, `;`, `"`, `.`, `?`, or `!`) directly follows the text, and the text does not start or end with a space-like character, a [.term]*constrained formatting pair* can be used.
A constrained pair uses a single opening mark and a single closing mark to enclose the text to be styled (e.g., `+*strong*+`).

For example, you use this form to format a word that stands alone,

----
That is *strong* stuff!
----

to format a sequence of words,

----
That is *really strong* stuff!
----

or to format a word adjacent to punctuation, like an exclamation mark.

----
This stuff is *strong*!
----

As you can see, the constrained pair offers a more succinct markup at the tradeoff of having more limited (constrained) use.
However, it should suffice in most cases, so the abbreviated markup is a benefit.
You can think of a constrained pair as being a weaker markup than an unconstrained pair.

"#
        );

        // -- first example --

        let doc = Parser::default().parse("That is *strong* stuff!");

        let mut blocks = doc.nested_blocks();

        let block1 = blocks.next().unwrap();
        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            sb1.content().rendered(),
            r#"That is <strong>strong</strong> stuff!"#
        );

        assert!(blocks.next().is_none());

        // -- second example --

        let doc = Parser::default().parse("That is *really strong* stuff!");

        let mut blocks = doc.nested_blocks();

        let block1 = blocks.next().unwrap();
        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            sb1.content().rendered(),
            r#"That is <strong>really strong</strong> stuff!"#
        );

        assert!(blocks.next().is_none());

        // -- third example --

        let doc = Parser::default().parse("This stuff is *strong*!");

        let mut blocks = doc.nested_blocks();

        let block1 = blocks.next().unwrap();
        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            sb1.content().rendered(),
            r#"This stuff is <strong>strong</strong>!"#
        );

        assert!(blocks.next().is_none());
    }
}
