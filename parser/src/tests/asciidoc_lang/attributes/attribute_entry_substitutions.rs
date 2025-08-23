#![allow(unused)] // TEMPORARY

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser, Span,
    document::{Attribute, InterpretedValue},
    tests::{
        fixtures::{
            TSpan,
            blocks::{TBlock, TSimpleBlock},
            content::TContent,
            document::{TAttribute, TDocument, THeader, TInterpretedValue},
        },
        sdd::{non_normative, track_file, verifies},
    },
};

track_file!("docs/modules/attributes/pages/attribute-entry-substitutions.adoc");

non_normative!(
    r#"
= Attribute Entry Substitutions

The AsciiDoc processor automatically applies substitutions from the header substitution group to the value of an attribute entry prior to the assignment, regardless of where the attribute entry is declared in the document.
"#
);

#[test]
fn applies_header_subs() {
    verifies!(
        r#"
The header substitution group, which replaces xref:subs:special-characters.adoc[special characters] followed by xref:subs:attributes.adoc[attribute references], is applied to the values of attribute entries, regardless of whether the entries are defined in the header or in the document body.
This is the same group that gets applied to metadata lines (author and revision information) in the document header.

That means that any inline formatting in an attribute value isn't interpreted because:

. inline formatting is not applied when the AsciiDoc processor sets an attribute, and
. inline formatting is not applied when an attribute is referenced since the relevant substitutions come before attributes are resolved.

"#
    );

    let mut parser = Parser::default();
    parser.parse(":special_chars: <tag_this>\n:y: yes\n:answer: {y}\n:format: *bold*");

    assert_eq!(
        parser
            .attribute_value("special_chars")
            .as_maybe_str()
            .unwrap(),
        "&lt;tag_this&gt;"
    );

    assert_eq!(
        parser.attribute_value("answer").as_maybe_str().unwrap(),
        "yes"
    );

    assert_eq!(
        parser.attribute_value("format").as_maybe_str().unwrap(),
        "*bold*"
    );
}

mod change_subs_when_assigning {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser, Span,
        blocks::{ContentModel, IsBlock},
        content::{SubstitutionGroup, SubstitutionStep},
        document::{Attribute, InterpretedValue},
        tests::{
            fixtures::{
                TSpan,
                attributes::{TAttrlist, TElementAttribute},
                blocks::{TBlock, TRawDelimitedBlock, TSimpleBlock},
                content::TContent,
                document::{TAttribute, TDocument, THeader, TInterpretedValue},
            },
            sdd::{non_normative, track_file, verifies},
        },
    };

    non_normative!(
        r#"
[#pass-macro]
== Change substitutions when assigning a value

If you want the value of an attribute entry to be used *as is* (not subject to substitutions), or you want to alter the substitutions that are applied, you can enclose the value in the xref:pass:pass-macro.adoc[inline pass macro] (i.e., `\pass:[]`).
The inline pass macro accepts a list of zero or more substitutions in the target slot, which can be used to control which substitutions are applied to the value.
If no substitutions are specified, no substitutions will be applied.

In order for the inline macro to work in this context, it must completely surround the attribute value.
If it's used anywhere else in the value, it will be ignored.

    "#
    );

    #[test]
    fn prevent_subs_on_assignment() {
        verifies!(
            r#"
Here's how to prevent substitutions from being applied to the value of an attribute entry:

[source]
----
:cols: pass:[.>2,.>4]
----

This might be useful if you're referencing the attribute in a place that depends on the unaltered text, such as the value of the `cols` attribute on a table.

    "#
        );

        let mut parser = Parser::default();
        parser.parse(":cols: pass:[.>2,.>4]");

        assert_eq!(
            parser.attribute_value("cols").as_maybe_str().unwrap(),
            ".>2,.>4"
        );
    }

    #[test]
    fn apply_quotes_subs() {
        verifies!(
            r#"
Here's how we can apply the xref:subs:quotes.adoc[quotes substitution] to the value of an attribute entry:

[source]
----
:app-name: pass:quotes[MyApp^2^]
----

Internally, the value is stored as `MyApp<sup>2</sup>`.
You can inspect the value stored in an attribute using this trick:

[source]
----
[subs=attributes+]
------
{app-name}
------
----
    "#
        );

        let mut parser = Parser::default();
        let doc = parser.parse(
            ":app-name: pass:quotes[MyApp^2^]\n\n[subs=attributes+]\n------\n{app-name}\n------",
        );

        assert_eq!(
            parser.attribute_value("app-name").as_maybe_str().unwrap(),
            "MyApp<sup>2</sup>"
        );

        let mut blocks = doc.nested_blocks();

        let block1 = blocks.next().unwrap();

        assert_eq!(
            block1,
            &TBlock::RawDelimited(TRawDelimitedBlock {
                content: TContent {
                    original: TSpan {
                        data: "{app-name}",
                        line: 5,
                        col: 1,
                        offset: 60,
                    },
                    rendered: "MyApp&lt;sup&gt;2&lt;/sup&gt;",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "[subs=attributes+]\n------\n{app-name}\n------",
                    line: 3,
                    col: 1,
                    offset: 34,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: &[TElementAttribute {
                        name: Some("subs"),
                        value: "attributes+",
                        shorthand_items: &[],
                    },],
                    source: TSpan {
                        data: "subs=attributes+",
                        line: 3,
                        col: 2,
                        offset: 35,
                    },
                },),
                substitution_group: SubstitutionGroup::Custom(vec![
                    SubstitutionStep::AttributeReferences,
                    SubstitutionStep::SpecialCharacters,
                ],),
            },)
        );
    }
}
