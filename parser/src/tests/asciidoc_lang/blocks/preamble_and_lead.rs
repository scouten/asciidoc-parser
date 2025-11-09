use std::collections::HashMap;

use pretty_assertions_sorted::assert_eq;

use crate::{Parser, document::RefType, tests::prelude::*};

track_file!("docs/modules/blocks/pages/preamble-and-lead.adoc");

non_normative!(
    r#"
= Preamble and Lead Style

"#
);

#[test]
fn preamble_style() {
    verifies!(
        r#"
[#preamble-style]
== Preamble

Content between the end of the xref:document:header.adoc[document header] and the first section title in the document body is called the preamble.
A preamble is optional.

.Preamble
[#ex-preamble]
----
include::example$preamble.adoc[]
----

"#
    );

    let doc = Parser::default().parse(
        "= The Intrepid Chronicles\n\nThis adventure begins on a frigid morning.\nWe've run out of coffee beans, but leaving our office means venturing into certain peril.\nYesterday, a colony of ravenous Wolpertingers descended from the foothills.\nNo one can find the defensive operations manual, and our security experts are on an off-the-grid team-building retreat in Katchanga.\n\nWhat are we going to do?\n\n== Certain Peril\n\nDaylight trickles across the cobblestones..."
    );

    assert_eq!(
        doc,
        Document {
            header: Header {
                title_source: Some(Span {
                    data: "The Intrepid Chronicles",
                    line: 1,
                    col: 3,
                    offset: 2,
                },),
                title: Some("The Intrepid Chronicles",),
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
                source: Span {
                    data: "= The Intrepid Chronicles",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            },
            blocks: &[
                Block::Preamble(Preamble {
                    blocks: &[
                        Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "This adventure begins on a frigid morning.\nWe've run out of coffee beans, but leaving our office means venturing into certain peril.\nYesterday, a colony of ravenous Wolpertingers descended from the foothills.\nNo one can find the defensive operations manual, and our security experts are on an off-the-grid team-building retreat in Katchanga.",
                                    line: 3,
                                    col: 1,
                                    offset: 27,
                                },
                                rendered: "This adventure begins on a frigid morning.\nWe&#8217;ve run out of coffee beans, but leaving our office means venturing into certain peril.\nYesterday, a colony of ravenous Wolpertingers descended from the foothills.\nNo one can find the defensive operations manual, and our security experts are on an off-the-grid team-building retreat in Katchanga.",
                            },
                            source: Span {
                                data: "This adventure begins on a frigid morning.\nWe've run out of coffee beans, but leaving our office means venturing into certain peril.\nYesterday, a colony of ravenous Wolpertingers descended from the foothills.\nNo one can find the defensive operations manual, and our security experts are on an off-the-grid team-building retreat in Katchanga.",
                                line: 3,
                                col: 1,
                                offset: 27,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                        },),
                        Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "What are we going to do?",
                                    line: 8,
                                    col: 1,
                                    offset: 370,
                                },
                                rendered: "What are we going to do?",
                            },
                            source: Span {
                                data: "What are we going to do?",
                                line: 8,
                                col: 1,
                                offset: 370,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                        },),
                    ],
                    source: Span {
                        data: "This adventure begins on a frigid morning.\nWe've run out of coffee beans, but leaving our office means venturing into certain peril.\nYesterday, a colony of ravenous Wolpertingers descended from the foothills.\nNo one can find the defensive operations manual, and our security experts are on an off-the-grid team-building retreat in Katchanga.\n\nWhat are we going to do?",
                        line: 3,
                        col: 1,
                        offset: 27,
                    },
                },),
                Block::Section(SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Certain Peril",
                            line: 10,
                            col: 4,
                            offset: 399,
                        },
                        rendered: "Certain Peril",
                    },
                    blocks: &[Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "Daylight trickles across the cobblestones...",
                                line: 12,
                                col: 1,
                                offset: 414,
                            },
                            rendered: "Daylight trickles across the cobblestones&#8230;&#8203;",
                        },
                        source: Span {
                            data: "Daylight trickles across the cobblestones...",
                            line: 12,
                            col: 1,
                            offset: 414,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),],
                    source: Span {
                        data: "== Certain Peril\n\nDaylight trickles across the cobblestones...",
                        line: 10,
                        col: 1,
                        offset: 396,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_type: SectionType::Normal,
                    section_id: Some("_certain_peril",),
                    section_number: None,
                },),
            ],
            source: Span {
                data: "= The Intrepid Chronicles\n\nThis adventure begins on a frigid morning.\nWe've run out of coffee beans, but leaving our office means venturing into certain peril.\nYesterday, a colony of ravenous Wolpertingers descended from the foothills.\nNo one can find the defensive operations manual, and our security experts are on an off-the-grid team-building retreat in Katchanga.\n\nWhat are we going to do?\n\n== Certain Peril\n\nDaylight trickles across the cobblestones...",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
            source_map: SourceMap(&[]),
            catalog: Catalog {
                refs: HashMap::from([(
                    "_certain_peril",
                    RefEntry {
                        id: "_certain_peril",
                        reftext: Some("Certain Peril",),
                        ref_type: RefType::Section,
                    },
                ),]),
                reftext_to_id: HashMap::from([("Certain Peril", "_certain_peril",),]),
            },
        }
    );
}
