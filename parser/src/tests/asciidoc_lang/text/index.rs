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

    #[test]
    fn unconstrained_formatting_pair() {
        verifies!(
            r#"
[#unconstrained]
=== Unconstrained formatting pair

An [.term]*unconstrained formatting pair* can be used anywhere in the text.
When the conditions are not met for a constrained formatting pair, the situation calls for an unconstrained formatting pair.
An unconstrained pair consists of a double opening mark and a double closing mark that encloses the text to be styled (e.g., `+Sara**h**+`).

For example, you'd use an unconstrained pair to format one or more letters in a word.

----
The man page, short for **man**ual page, is a form of software documentation.
----

The unconstrained pair provides a more brute force approach to formatting at the tradeoff of being more verbose.
You'll typically switch to an unconstrained pair when a constrained pair doesn't do the trick.
See xref:troubleshoot-unconstrained-formatting.adoc#use-unconstrained[When should I use an unconstrained pair?] for more examples of when to use an unconstrained pair.

"#
        );

        let doc = Parser::default()
            .parse("The man page, short for **man**ual page, is a form of software documentation.");

        let mut blocks = doc.nested_blocks();

        let block1 = blocks.next().unwrap();
        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            sb1.content().rendered(),
            r#"The man page, short for <strong>man</strong>ual page, is a form of software documentation."#
        );

        assert!(blocks.next().is_none());
    }
}

non_normative!(
    r#"
== Inline text and punctuation styles

AsciiDoc provides six inline text styles and one punctuation style that are applied solely with formatting marks.

xref:bold.adoc[Bold] (type: strong)::
Text that is bold will stand out against the regular, surrounding text due to the application of a thicker and/or darker font.
Bold is useful when the text needs to catch the attention of a person visually scanning a page.
The formatting mark for bold is an asterisk (`*`).

xref:italic.adoc[Italic] (type: emphasis)::
Text is often italicized in order to stress a word or phrase, quote a speaker, or introduce a term.
Italic type slants slightly to the right, and depending on the font, may have cursive swashes and flourishes.
The formatting mark for italic is an underscore (`+_+`).

xref:monospace.adoc[Monospace] (type: monospaced)::
Technical content often requires text to be styled in a way that indicates a command or source code.
Such text is usually emphasized using a fixed-width (i.e., monospace) font.
The formatting mark for monospace is a backtick (`++`++`).

xref:highlight.adoc[Highlight] (type: mark)::
Another way to draw attention to text is to highlight it.
This semantic style is used for reference or notation purposes, or to mark the importance of a key subject or point.
The formatting mark for highlight is a hash (`+#+`).

xref:custom-inline-styles.adoc[Styled phrase] (type: unquoted)::
Adding a role to a span of text that uses the highlight formatting mark (`+#+`) converts to generic phrase that can be styled.
AsciiDoc defines several built-in roles that you can use to style text, and the style/theming system of the converter can allow you to define styles for a custom role.

xref:subscript-and-superscript.adoc[Subscript and superscript] (type: subscript/superscript)::
Subscript and superscript text is common in mathematical expressions and chemical formulas.
The formatting mark for subscript is a tilde (`{tilde}`).
The formatting mark for superscript is a caret (`{caret}`).

////
AsciiDoc also provides two built-in styles that are applied with an additional role.

Strike through::

Underline::
////

xref:quotation-marks-and-apostrophes.adoc[Curved quotation marks and apostrophes] (type: double/single)::
By default, the AsciiDoc processor outputs straight quotation marks and apostrophes.
They can be changed to curved by adding backticks (`++`++`) as a formatting hint.

== Quotes substitution

When the AsciiDoc processor encounters text enclosed by designated formatting marks, those marks are replaced by the start and end tags of the corresponding HTML or XML element, depending on your backend, during the xref:subs:quotes.adoc[quotes substitution step].
You can control when inline formatting is applied to inline text, macros, or blocks with the xref:subs:quotes.adoc#quotes-value[quotes substitution value].

////
CAUTION: You may not always want these symbols to indicate text formatting.
In those cases, you'll need to use additional markup to xref:subs:prevent.adoc[escape the text formatting markup].
////
"#
);
