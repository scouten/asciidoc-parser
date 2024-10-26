//! Tracks https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang/-/blob/main/docs/modules/attributes/pages/positional-and-named-attributes.adoc?ref_type=heads
//!
//! Tracking commit 3474df92, current as of 2024-10-26.

// = Positional and Named Attributes

// This page breaks down the difference between positional and named attributes
// on an element and the rules for parsing an attribute list.

mod positional_attribute {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::MacroBlock,
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        Span,
    };

    // [#positional]
    // == Positional attribute

    // // tag::pos[]
    // Entries in an attribute list that only consist of a value are referred to
    // as positional attributes. The position is the 1-based index of the
    // entry once all named attributes have been removed (so they may be
    // interspersed).

    #[test]
    fn implicit_attribute_name() {
        // The positional attribute may be dually assigned to an implicit attribute
        // name if the block or macro defines a mapping for positional attributes.
        // Here are some examples of those mappings:

        // * `icon:` 1 => size
        // * `image:` and `image::` 1 => alt (text), 2 => width, 3 => height
        // * Delimited blocks: 1 => block style and attribute shorthand
        // * Other inline quoted text: 1 => attribute shorthand
        // * `link:` and `xref:` 1 => text
        // * Custom blocks and macros can also specify positional attributes

        // For example, the following two image macros are equivalent.

        // [source]
        // ----
        // image::sunset.jpg[Sunset,300,400]

        // image::sunset.jpg[alt=Sunset,width=300,height=400]
        // ----

        // The second macro is the same as the first, but written out in longhand
        // form.

        // end::pos[]

        let m1 = MacroBlock::parse(Span::new("image::sunset.jpg[Sunset,300,400]"))
            .unwrap_if_no_warnings()
            .unwrap();

        let m2 = MacroBlock::parse(Span::new(
            "image::sunset.jpg[alt=Sunset,width=300,height=400]",
        ))
        .unwrap_if_no_warnings()
        .unwrap();

        let a1 = m1.item.attrlist();
        let a2 = m2.item.attrlist();

        assert_eq!(
            a1.named_or_positional_attribute("alt", 1).unwrap(),
            TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: "Sunset",
                    line: 1,
                    col: 19,
                    offset: 18,
                }],
                value: TSpan {
                    data: "Sunset",
                    line: 1,
                    col: 19,
                    offset: 18,
                },
                source: TSpan {
                    data: "Sunset",
                    line: 1,
                    col: 19,
                    offset: 18,
                },
            },
        );

        assert_eq!(
            a2.named_or_positional_attribute("alt", 1).unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "alt",
                    line: 1,
                    col: 19,
                    offset: 18,
                },),
                shorthand_items: vec![],
                value: TSpan {
                    data: "Sunset",
                    line: 1,
                    col: 23,
                    offset: 22,
                },
                source: TSpan {
                    data: "alt=Sunset",
                    line: 1,
                    col: 19,
                    offset: 18,
                },
            }
        );
    }

    // === Block style and attribute shorthand

    // The first positional attribute on all blocks (including sections) is
    // special. It's used to define the
    // xref:blocks:index.adoc#block-style[block style]. It also supports a
    // shorthand syntax for defining the ID, role, and options attributes.
    // This shorthand syntax can also be used on formatted text, even though
    // formatted text doesn't technically support attributes.

    // The attribute shorthand is inspired by the HAML and Slim template
    // languages as a way of saving the author some typing. Instead of
    // having to use the longhand form of a name attribute, it's possible to
    // compress the assignment to a value prefixed by a special marker.
    // The markers are mapped as follows:

    // * `#` - ID
    // * `.` - role
    // * `%` - option

    // Each shorthand entry is placed directly adjacent to previous one,
    // starting immediately after the optional block style. The order of the
    // entries does not matter, except for the style, which must come first.

    // Here's an example that shows how to set an ID on a section using this
    // shorthand syntax:

    // ----
    // [#custom-id]
    // == Section with Custom ID
    // ----

    // The shorthand entry must follow the block style, if present.
    // Here's an example that shows how to set an ID on an appendix section
    // using this shorthand syntax:

    // ----
    // [appendix#custom-id]
    // == Appendix with Custom ID
    // ----

    // Here's an example of a block that uses the shorthand syntax to set the
    // ID, a role, and an option for a list. Specifically, this syntax sets
    // the ID to `rules`, adds the role `prominent`, and sets the option
    // `incremental`.

    // ----
    // [#rules.prominent%incremental]
    // * Work hard
    // * Play hard
    // * Be happy
    // ----

    // A block can have multiple roles and options, so these shorthand entries
    // may be repeated. Here's an example that shows how to set several
    // options on a table. Specifically, this syntax sets the `header`,
    // `footer`, and `autowidth` options.

    // ----
    // [%header%footer%autowidth]
    // |===
    // |Header A |Header B
    // |Footer A |Footer B
    // |===
    // ----

    // This shorthand syntax also appears on formatted text.
    // Here's an example that shows how to set the ID and add a role to a strong
    // phrase. Specifically, this syntax sets the ID to `free-world` and
    // adds the `goals` role.

    // ----
    // [#free-world.goals]*free the world*
    // ----

    // Formatted text does not support a style, so the first and only positional
    // attribute is always the shorthand syntax.
}

// [#named]
// == Named attribute

// // tag::name[]
// A named attribute consists of a name and a value separated by an `=`
// character (e.g., `name=value`).

// If the value contains a space, comma, or quote character, it must be enclosed
// in double or single quotes (e.g., `name="value with space"`). In all other
// cases, the surrounding quotes are optional.

// If the value contains the *same* quote character used to enclose the value,
// the quote character in the value must be escaped by prefixing it with a
// backslash (e.g., `value="the song \"Dark Horse\""`).

// If enclosing quotes are used, they are dropped from the parsed value and the
// preceding backslash is dropped from any escaped quotes.

// [#unset]
// === Unset a named attribute

// To undefine a named attribute, set the value to `None` (case sensitive).
// // end::name[]

// == Attribute list parsing

// The source text that's used to define attributes for an element is referred
// to as an [.term]*attrlist*. An attrlist is always enclosed in a pair of
// square brackets. This applies for block attributes as well as attributes on a
// block or inline macro. The processor splits the attrlist into individual
// attribute entries, determines whether each entry is a positional or named
// attribute, parses the entry accordingly, and assigns the result as an
// attribute on the node.

// The rules for what defines the boundaries of an individual attribute, and
// whether the attribute is positional or named, are defined below.
// In these rules, `name` consists of a word character (letter or numeral)
// followed by any number of word or `-` characters (e.g., `see-also`).

// * Attribute references are expanded before the attrlist is parsed (i.e., the
//   attributes substitution is applied).
// * Parsing an attribute proceeds from the beginning of the attribute list
//   string or after a previously identified delimiter (`,`).
// ** The first character of an attribute list cannot be a tab or space.
// For subsequent attributes, any leading space or tab characters are skipped.
// * If a valid attribute name is found, and it is followed by an equals sign
//   (=), then the parser recognizes this as a named attribute.
// The text after the equals sign (=) and up to the next comma or end of list is
// taken as the attribute value. Space and tab characters around the equals sign
// (=) and at the end of the value are ignored.
// * Otherwise, this is a positional attribute with a value that ends at the
//   next delimiter or end of list.
// Any space or tab characters at the boundaries of the value are ignored.
// * To parse the attribute value:
// ** If the first character is not a quote, the string is read until the next
// delimiter or end of string. ** If the first character is a double quote
// (i.e., `"`), then the string is read until the next unescaped double quote
// or, if there is no closing double quote, the next delimiter. If there is a
// closing double quote, the enclosing double quote characters are removed and
// escaped double quote characters are unescaped; if not, the initial double
// quote is retained. ** If the next character is a single quote (i.e., `'`),
// then the string is read until the next unescaped single quote or, if there is
// no closing single quote, the next delimiter. If there is a closing single
// quote, the enclosing single quote characters are removed and escaped single
// quote characters are unescaped; if not, the initial single quote is retained.
// If there is a closing single quote, and the first character is not an escaped
// single quote, substitutions are performed on the value as described in
// <<Substitutions>>.

// .When to escape a closing square bracket
// ****
// Since the terminal of an attrlist is a closing square bracket, it's sometimes
// necessary to escape a closing square bracket if it appears in the value of an
// attribute.

// In line-oriented syntax such as a block attribute list, a block macro, and an
// include directive, you do not have to escape closing square brackets that
// appear in the attrlist itself. That's because the parser already knows to
// look for the closing square bracket at the end of the line.

// If a closing square bracket appears in the attrlist of an inline element,
// such as an inline macro, it usually has to be escaped using a backslash or by
// using the character reference `+&#93;+`. There are some exceptions to this
// rule, such as a link macro in a footnote, which are influenced by the
// substitution order. ****

// == Substitutions

// // tag::subs[]
// Recall that attribute references are expanded before the attrlist is parsed.
// Therefore, it's not necessary to force substitutions to be applied to a value
// if you're only interested in applying the attributes substitution.
// The attributes substitution has already been applied at this point.

// If the attribute name (in the case of a positional attribute) or value (in the case of a named attribute) is enclosed in single quotes (e.g., `+citetitle='Processed by https://asciidoctor.org'+`), and the attribute is defined in an attrlist on a block, then the xref:subs:index.adoc#normal-group[normal substitution group] is applied to the value at assignment time.
// No special processing is performed, aside from the expansion of attribute
// references, if the value is not enclosed in quotes or is enclosed in double
// quotes.

// If the value contains the same quote character used to enclose the value, escape the quote character in the value by prefixing it with a backslash (e.g., `+citetitle='A \'use case\' diagram, generated by https://plantuml.com'+`).
// // end::subs[]
