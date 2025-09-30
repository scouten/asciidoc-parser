use pretty_assertions_sorted::assert_eq;

use crate::{Parser, parser::ModificationContext, tests::prelude::*};

#[test]
fn empty_line() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(crate::Span::new(""), &mut parser);

    assert_eq!(
        &al,
        AuthorLine {
            authors: &[],
            source: Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn one_simple_author() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(
        crate::Span::new("Kismet R. Lee <kismet@asciidoctor.org>"),
        &mut parser,
    );

    assert_eq!(
        &al,
        AuthorLine {
            authors: &[Author {
                name: "Kismet R. Lee",
                firstname: "Kismet",
                middlename: Some("R.",),
                lastname: Some("Lee",),
                email: Some("kismet@asciidoctor.org",),
            },],
            source: Span {
                data: "Kismet R. Lee <kismet@asciidoctor.org>",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn author_without_middle_name() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(
        crate::Span::new("Doc Writer <doc@example.com>"),
        &mut parser,
    );

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "Doc Writer",
                firstname: "Doc",
                middlename: None,
                lastname: Some("Writer",),
                email: Some("doc@example.com",),
            },],
            source: Span {
                data: "Doc Writer <doc@example.com>",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn too_many_names() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(
        crate::Span::new("Four Names Not Supported <doc@example.com>"),
        &mut parser,
    );

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "Four Names Not Supported &lt;doc@example.com&gt;",
                firstname: "Four Names Not Supported &lt;doc@example.com&gt;",
                middlename: None,
                lastname: None,
                email: None,
            },],
            source: Span {
                data: "Four Names Not Supported <doc@example.com>",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn one_name() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(
        crate::Span::new("John <john@example.com>"),
        &mut parser,
    );

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "John",
                firstname: "John",
                middlename: None,
                lastname: None,
                email: Some("john@example.com",),
            },],
            source: Span {
                data: "John <john@example.com>",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn underscore_join() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(crate::Span::new("Mary_Sue Brontë"), &mut parser);

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "Mary_Sue Brontë",
                firstname: "Mary_Sue",
                middlename: None,
                lastname: Some("Brontë",),
                email: None,
            },],
            source: Span {
                data: "Mary_Sue Brontë",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn greek() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(
        crate::Span::new("Αλέξανδρος Παπαδόπουλος"),
        &mut parser,
    );

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "Αλέξανδρος Παπαδόπουλος",
                firstname: "Αλέξανδρος",
                middlename: None,
                lastname: Some("Παπαδόπουλος",),
                email: None,
            },],
            source: Span {
                data: "Αλέξανδρος Παπαδόπουλος",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn japanese() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(crate::Span::new("山田太郎"), &mut parser);

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "山田太郎",
                firstname: "山田太郎",
                middlename: None,
                lastname: None,
                email: None,
            },],
            source: Span {
                data: "山田太郎",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn arabic() {
    let mut parser = Parser::default();

    let al = crate::document::AuthorLine::parse(crate::Span::new("عبد_الله"), &mut parser);

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "عبد_الله",
                firstname: "عبد_الله",
                middlename: None,
                lastname: None,
                email: None,
            },],
            source: Span {
                data: "عبد_الله",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn attr_sub_email() {
    let mut parser = Parser::default()
        .with_intrinsic_attribute(
            "jane-email",
            "jane@example.com",
            ModificationContext::Anywhere,
        )
        .with_intrinsic_attribute(
            "john-email",
            "john@example.com",
            ModificationContext::Anywhere,
        );

    let al = crate::document::AuthorLine::parse(
        crate::Span::new("Jane Smith <{jane-email}>; John Doe <{john-email}>"),
        &mut parser,
    );

    assert_eq!(
        al,
        AuthorLine {
            authors: &[
                Author {
                    name: "Jane Smith",
                    firstname: "Jane",
                    middlename: None,
                    lastname: Some("Smith",),
                    email: Some("jane@example.com",),
                },
                Author {
                    name: "John Doe",
                    firstname: "John",
                    middlename: None,
                    lastname: Some("Doe",),
                    email: Some("john@example.com",),
                },
            ],
            source: Span {
                data: "Jane Smith <{jane-email}>; John Doe <{john-email}>",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn attr_sub_applied_after_parsing() {
    // This is to demonstrate compatibility with Ruby asciidoctor behavior. In that
    // implementation, the attribute substitution is applied *after* parsing for
    // individual authors, which results in the unexpected treatment that the entire
    // list is one author with mangled results.
    let mut parser = Parser::default().with_intrinsic_attribute(
        "author-list",
        "Jane Smith <jane@example.com>; John Doe <john@example.com>",
        ModificationContext::Anywhere,
    );

    let al = crate::document::AuthorLine::parse(crate::Span::new("{author-list}"), &mut parser);

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "Jane Smith &lt;jane@example.com&gt;; John Doe &lt;john@example.com&gt;",
                firstname: "Jane Smith &lt;jane@example.com&gt;; John Doe &lt;john@example.com&gt;",
                middlename: None,
                lastname: None,
                email: None,
            },],
            source: Span {
                data: "{author-list}",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn attr_sub_for_individual_author() {
    let mut parser = Parser::default().with_intrinsic_attribute(
        "full-author",
        "John Doe <john@example.com>",
        ModificationContext::Anywhere,
    );

    let al = crate::document::AuthorLine::parse(crate::Span::new("{full-author}"), &mut parser);

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "{full-author}",
                firstname: "John",
                middlename: None,
                lastname: Some("Doe",),
                email: Some("john@example.com",),
            },],
            source: Span {
                data: "{full-author}",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn err_individual_name_components_as_attributes() {
    // This approach doesn't work in Ruby AsciiDoctor either.
    let mut parser = Parser::default()
        .with_intrinsic_attribute("first-name", "Jane", ModificationContext::Anywhere)
        .with_intrinsic_attribute("last-name", "Smith", ModificationContext::Anywhere)
        .with_intrinsic_attribute(
            "author-email",
            "jane@example.com",
            ModificationContext::Anywhere,
        );

    let al = crate::document::AuthorLine::parse(
        crate::Span::new("{first-name} {last-name} <{author-email}>"),
        &mut parser,
    );

    assert_eq!(
        al,
        AuthorLine {
            authors: &[Author {
                name: "Jane Smith &amp;lt;jane@example.com&amp;gt;",
                firstname: "Jane Smith &amp;lt;jane@example.com&amp;gt;",
                middlename: None,
                lastname: None,
                email: None,
            },],
            source: Span {
                data: "{first-name} {last-name} <{author-email}>",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );
}

#[test]
fn sets_author_attributes_single_author_with_all_parts() {
    let mut parser = Parser::default();
    let _doc = parser.parse("= Document Title\nKismet R. Lee <kismet@asciidoctor.org>");

    // Primary author attributes
    assert_eq!(
        parser.attribute_value("author"),
        InterpretedValue::Value("Kismet R. Lee")
    );
    assert_eq!(
        parser.attribute_value("firstname"),
        InterpretedValue::Value("Kismet")
    );
    assert_eq!(
        parser.attribute_value("middlename"),
        InterpretedValue::Value("R.")
    );
    assert_eq!(
        parser.attribute_value("lastname"),
        InterpretedValue::Value("Lee")
    );
    assert_eq!(
        parser.attribute_value("authorinitials"),
        InterpretedValue::Value("KRL")
    );
    assert_eq!(
        parser.attribute_value("email"),
        InterpretedValue::Value("kismet@asciidoctor.org")
    );
}

#[test]
fn sets_author_attributes_single_author_without_middle_name() {
    let mut parser = Parser::default();
    let _doc = parser.parse("= Document Title\nDoc Writer <doc@example.com>");

    assert_eq!(
        parser.attribute_value("author"),
        InterpretedValue::Value("Doc Writer")
    );
    assert_eq!(
        parser.attribute_value("firstname"),
        InterpretedValue::Value("Doc")
    );
    assert_eq!(
        parser.attribute_value("middlename"),
        InterpretedValue::Unset
    );
    assert_eq!(
        parser.attribute_value("lastname"),
        InterpretedValue::Value("Writer")
    );
    assert_eq!(
        parser.attribute_value("authorinitials"),
        InterpretedValue::Value("DW")
    );
    assert_eq!(
        parser.attribute_value("email"),
        InterpretedValue::Value("doc@example.com")
    );
}

#[test]
fn sets_author_attributes_single_author_first_name_only() {
    let mut parser = Parser::default();
    let _doc = parser.parse("= Document Title\nJohn <john@example.com>");

    assert_eq!(
        parser.attribute_value("author"),
        InterpretedValue::Value("John")
    );
    assert_eq!(
        parser.attribute_value("firstname"),
        InterpretedValue::Value("John")
    );
    assert_eq!(
        parser.attribute_value("middlename"),
        InterpretedValue::Unset
    );
    assert_eq!(parser.attribute_value("lastname"), InterpretedValue::Unset);
    assert_eq!(
        parser.attribute_value("authorinitials"),
        InterpretedValue::Value("J")
    );
    assert_eq!(
        parser.attribute_value("email"),
        InterpretedValue::Value("john@example.com")
    );
}

#[test]
fn sets_author_attributes_single_author_without_email() {
    let mut parser = Parser::default();
    let _doc = parser.parse("= Document Title\nMary Sue Brontë");

    assert_eq!(
        parser.attribute_value("author"),
        InterpretedValue::Value("Mary Sue Brontë")
    );
    assert_eq!(
        parser.attribute_value("firstname"),
        InterpretedValue::Value("Mary")
    );
    assert_eq!(
        parser.attribute_value("middlename"),
        InterpretedValue::Value("Sue")
    );
    assert_eq!(
        parser.attribute_value("lastname"),
        InterpretedValue::Value("Brontë")
    );
    assert_eq!(
        parser.attribute_value("authorinitials"),
        InterpretedValue::Value("MSB")
    );
    assert_eq!(parser.attribute_value("email"), InterpretedValue::Unset);
}

#[test]
fn sets_author_attributes_multiple_authors() {
    let mut parser = Parser::default();
    let _doc = parser
        .parse("= Document Title\nJane Smith <jane@example.com>; John Doe <john@example.com>");

    // First author (primary)
    assert_eq!(
        parser.attribute_value("author"),
        InterpretedValue::Value("Jane Smith")
    );
    assert_eq!(
        parser.attribute_value("firstname"),
        InterpretedValue::Value("Jane")
    );
    assert_eq!(
        parser.attribute_value("lastname"),
        InterpretedValue::Value("Smith")
    );
    assert_eq!(
        parser.attribute_value("authorinitials"),
        InterpretedValue::Value("JS")
    );
    assert_eq!(
        parser.attribute_value("email"),
        InterpretedValue::Value("jane@example.com")
    );

    // Second author
    assert_eq!(
        parser.attribute_value("author_2"),
        InterpretedValue::Value("John Doe")
    );
    assert_eq!(
        parser.attribute_value("firstname_2"),
        InterpretedValue::Value("John")
    );
    assert_eq!(
        parser.attribute_value("lastname_2"),
        InterpretedValue::Value("Doe")
    );
    assert_eq!(
        parser.attribute_value("authorinitials_2"),
        InterpretedValue::Value("JD")
    );
    assert_eq!(
        parser.attribute_value("email_2"),
        InterpretedValue::Value("john@example.com")
    );

    // Verify middlename attributes are unset for both authors
    assert_eq!(
        parser.attribute_value("middlename"),
        InterpretedValue::Unset
    );
    assert_eq!(
        parser.attribute_value("middlename_2"),
        InterpretedValue::Unset
    );
}

#[test]
fn sets_author_attributes_unicode_names() {
    let mut parser = Parser::default();
    let _doc = parser.parse("= Document Title\nΑλέξανδρος Μ. Παπαδόπουλος");

    assert_eq!(
        parser.attribute_value("author"),
        InterpretedValue::Value("Αλέξανδρος Μ. Παπαδόπουλος")
    );
    assert_eq!(
        parser.attribute_value("firstname"),
        InterpretedValue::Value("Αλέξανδρος")
    );
    assert_eq!(
        parser.attribute_value("middlename"),
        InterpretedValue::Value("Μ.")
    );
    assert_eq!(
        parser.attribute_value("lastname"),
        InterpretedValue::Value("Παπαδόπουλος")
    );
    assert_eq!(
        parser.attribute_value("authorinitials"),
        InterpretedValue::Value("ΑΜΠ")
    );
}
