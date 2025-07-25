use crate::{
    Parser,
    blocks::{Block, IsBlock},
    tests::sdd::{non_normative, track_file, verifies},
};

track_file!("docs/modules/text/pages/literal-monospace.adoc");

non_normative!(
    r#"
= Literal Monospace

Unlike other markup languages, monospaced text in AsciiDoc is not synonymous with literal text.
Instead, it gets interpreted just like normal text.
In other words, it's subject to all text substitutions by default.

This might be surprising at first.
But there's good reason for this difference.
In AsciiDoc, you can take advantage of attribute references and inline macros inside of a monospaced text span.
The drawback, of course, is that you have to be careful to escape these special characters if you intend to output them without special formatting (i.e., as literal text).

One way to prevent the processor from interpreting special characters in monospaced text is to escape them using backslash characters, just as you would with normal text.
However, escaping individual occurrences that way can be tedious.
That's why AsciiDoc offers a special type of monospace formatting called the literal monospace.

"#
);

#[test]
fn ex_literal() {
    verifies!(
        r#"
To make a true literal codespan in AsciiDoc, you must enclose the monospaced text in a passthrough.
Rather than using a single pair of backtick characters, you'll use the combination of the backtick and plus characters, where the plus characters fall on the inside of the backtick characters (e.g., `pass:[`+text+`]`).
The plus characters are a shorthand for the `\pass:c[]` enclosure.

<<ex-literal>> contains literal, monospaced text.

.Literal monospace syntax
[#ex-literal]
----
include::example$text.adoc[tag=literal-mono]
----

"#
    );

    let doc = Parser::default().parse(
        "You can reference the value of a document attribute using\nthe syntax `+{name}+`, where `name` is the attribute name.",
    );

    let mut blocks = doc.nested_blocks();

    let block1 = blocks.next().unwrap();
    let Block::Simple(sb1) = block1 else {
        panic!("Unexpected block type: {block1:?}");
    };

    assert_eq!(
        sb1.content().rendered(),
        "You can reference the value of a document attribute using\nthe syntax <code>{name}</code>, where <code>name</code> is the attribute name."
    );

    assert!(blocks.next().is_none());
}

#[test]
fn ex_plus() {
    verifies!(
        r#"
This shorthand syntax can accommodate most of the literal monospace cases.
The main exception is when the text itself contains plus characters.
To avoid confusing the processor, you'll need to switch to using the more formal passthrough macro to handle these cases.

<<ex-plus>> shows literal, monospaced text that contains plus characters.

.Literal monospace syntax with + characters
[#ex-plus]
----
include::example$text.adoc[tag=literal-mono-with-plus]
----

"#
    );

    let doc = Parser::default().parse("`pass:[++]` is the increment operator in C.");

    let mut blocks = doc.nested_blocks();

    let block1 = blocks.next().unwrap();
    let Block::Simple(sb1) = block1 else {
        panic!("Unexpected block type: {block1:?}");
    };

    assert_eq!(
        sb1.content().rendered(),
        "<code>++</code> is the increment operator in C."
    );

    assert!(blocks.next().is_none());
}

non_normative!(
    r#"
Passthroughs are a general purpose utility in AsciiDoc.
You can learn about the various passthrough options in xref:pass:pass-macro.adoc[].

////
==== Using legacy monospace markup

In Asciidoctor 1.5, the backtick replaces the single and double plus sign for indicating monospace formatting and normal substitutions.
However, you can forgo this change to the syntax by setting the `compat-mode` attribute either in the document header:

----
:compat-mode:
----

or by passing the attribute to the API:

----
-a compat-mode
----

.Why the output behavior of `+` and `++` changed in Asciidoctor 1.5
****
The meaning of `+` in the AsciiDoc syntax was inconsistent in AsciiDoc.py and previous Asciidoctor versions.
// I believe we can make the AsciiDoc syntax easier to understand and remember if we consistently implement `+` as a passthrough (literal) mark.

Where the quad plus (`++++`) and triple plus (`+++`) pass content without any processing, the double plus (`++`) and single plus (`+`) pass content "`as you see it`" (which means escaping special characters).
Double plus (`++`) is an unconstrained match (can be used anywhere inline, just like double dollar (`$$`)), while single plus (`+`) is a constrained match (can be used at word boundaries). For example:

* +{conf}+ becomes {conf}
* dev++{conf}++ becomes dev{conf}

TIP: You may recognize that `++` is now equivalent to `$$` (and perhaps easier to read).

Notice that the text is not formatted as monospace.
To apply monospace formatting to content marked by `+` or `++`, surround it with a pair of backticks.


The backtick characters around text only mean that the text should be formatted as monospace. The backtick characters *do not add* passthrough semantics. In most cases, the passthrough semantics aren't necessary, so using the backticks for monospaced formatting is all that's necessary.

* \`literal\` becomes `literal`
* \`{backend}\` becomes `html5`
* a\`\`|\`\`b becomes a`|`b

So the question remains, how do you prevent substitutions in text formatted as monospaced (i.e., monospaced literal)? Answer: You either escape the substitution or combine the formatting and the passthrough.

* \`\\{backend}\` becomes `{backend}`
* \`+{backend}+\` becomes `{backend}`

By not mixing monospace formatting with passthrough (literal) semantics, we are deviating slightly from the behavior of backticks in Markdown. However, that's because AsciiDoc has additional features, such as attribute references, that we want to be able to leverage when formatting text as monospace (just as we would anywhere). As it turns out, the lack of ability for substitutions when creating monospaced text in Markdown is quite limiting (and frustrating).

Let's give this separation of inline passthroughs (using single and double plus) and monospace formatting (using single and double backticks) a try and see how it goes. If we need to remove or reduce the number of substitution applied when formatting text as monospace, we can entertain the idea once we've given this configuration a trial.

Since we're swapping behavior between two existing formatting marks, we had to introduce a transitional syntax that will work in with both 0.1.4 and 1.5.0 to ease migration. I'll document those options here.

---

**IMPORTANT:** I want to reiterate that you have the option of enabling legacy mode if you don't want to deal with migration. Legacy mode will be supported for the foreseeable future to avoid unnecessary disruption. If that's the option you want to take, simply add the following document attribute at the start of your document, or wherever you want to enable legacy mode.

----
:compat-mode:
----

If you want to begin migration to the modern syntax, read on.

---

There are three scenarios that are affected by this syntax change.

* Monospaced normal text (it doesn't contain AsciiDoc syntax, so the text doesn't get interpreted)
* Monospaced text without substitutions (you want to prevent the text from being interpreted)
* Monospaced text with substitutions (you want text to be interpreted)

Let's first consider the legacy syntax used in each of these examples.

* pass:[+monospaced normal text+ or `monospaced normal text`]
* pass:[`Use {asciidoctor-version} to print the version of Asciidoctor`] (attribute is _not_ replaced)
* pass:[+The version of Asciidoctor is {asciidoctor-version}+] (attribute _is_ replaced)

If you want the previous examples to work in Asciidoctor 1.5.0, you need to switch to the modern syntax, as shown below:

* pass:[`monospaced normal text`]
* pass:[`+Use {asciidoctor-version} to print the version of Asciidoctor+`] (attribute is _not_ replaced)
* pass:[`The version of Asciidoctor is {asciidoctor-version}`] (attribute _is_ replaced)

While the previous examples will work in Asciidoctor 1.5.0, the last two are now broken in Asciidoctor 0.1.4. Unfortunately, we can't control when Asciidoctor is upgraded on services like GitHub, so there will be a period of time when you will need to use a syntax that works on both versions. So what do we do? The answer, use the transitional syntax.

To use the transitional syntax, add the role "x-" in front of the legacy syntax to indicate that you want Asciidoctor 1.5.0 to use the old behavior. Of course, Asciidoctor 0.1.4 already understands the old syntax.

* pass:[[x-\]`Use {asciidoctor-version} to print the version of Asciidoctor`] (attribute is _not_ replaced)
* pass:[[x-\]+The version of Asciidoctor is {asciidoctor-version}+] (attribute _is_ replaced)

By using the "x-" role, you not only enable the legacy behavior, you also mark the locations that need to be migrated to the modern syntax in Asciidoctor 1.5.0 once Asciidoctor has been upgraded everywhere.

If you aren't worried about how the document renders on services like GitHub, you can start using the modern syntax immediately.


// The <<user-manual#back-pass,inline literal passthrough>> also applies monospace text formatting.

////

////
Monospace text formatting is often used to emulate how source code appears in computer terminals, simple text editors, and integrated development environments (IDEs).
A word or phrase is rendered using a fixed-width font, i.e. monospaced font, when it is enclosed in a single pair of plus signs (+{plus}+).
A character contained within a string of characters must be enclosed in a double pair of plus signs (+{plus}{plus}+).

----
include::ex-text.adoc[tag=mono]
----

Monospaced text can be bold and italicized, as long as the markup pairs are entered in the right order.
The monospace markup must be the outermost pair, then the bold pair, and the italic markup must always be the innermost pair.

.Rendered monospace formatted text
====
include::ex-text.adoc[tag=mono]
====

The <<user-manual#back-pass,inline literal passthrough>> also applies monospace text formatting.
////
"#
);
