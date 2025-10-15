use crate::parser::{SourceLine, SourceMap};

#[test]
fn empty() {
    let sm = SourceMap::default();
    assert_eq!(sm.original_file_and_line(1), Some(SourceLine(None, 1)));
}

#[test]
fn one_entry() {
    let mut sm = SourceMap::default();
    sm.append(1, SourceLine(None, 1));

    assert_eq!(sm.original_file_and_line(0), Some(SourceLine(None, 0)));
    assert_eq!(sm.original_file_and_line(1), Some(SourceLine(None, 1)));
    assert_eq!(sm.original_file_and_line(4), Some(SourceLine(None, 4)));

    assert_eq!(
        sm.original_file_and_line(4000),
        Some(SourceLine(None, 4000))
    );
}

#[test]
fn multiple_entries() {
    let mut sm = SourceMap::default();
    sm.append(1, SourceLine(None, 1));
    sm.append(10, SourceLine(Some("foo.adoc".to_owned()), 1));
    sm.append(20, SourceLine(Some("bar.adoc".to_owned()), 18));
    sm.append(30, SourceLine(None, 11));

    assert_eq!(sm.original_file_and_line(1), Some(SourceLine(None, 1)));
    assert_eq!(sm.original_file_and_line(4), Some(SourceLine(None, 4)));

    assert_eq!(
        sm.original_file_and_line(10),
        Some(SourceLine(Some("foo.adoc".to_owned()), 1))
    );
    assert_eq!(
        sm.original_file_and_line(19),
        Some(SourceLine(Some("foo.adoc".to_owned()), 10))
    );

    assert_eq!(
        sm.original_file_and_line(20),
        Some(SourceLine(Some("bar.adoc".to_owned()), 18))
    );
    assert_eq!(
        sm.original_file_and_line(21),
        Some(SourceLine(Some("bar.adoc".to_owned()), 19))
    );
    assert_eq!(
        sm.original_file_and_line(29),
        Some(SourceLine(Some("bar.adoc".to_owned()), 27))
    );

    assert_eq!(sm.original_file_and_line(30), Some(SourceLine(None, 11)));
    assert_eq!(sm.original_file_and_line(40), Some(SourceLine(None, 21)));
}

#[test]
fn impl_debug() {
    let mut sm = SourceMap::default();
    sm.append(1, SourceLine(None, 1));

    assert_eq!(
        format!("{sm:#?}"),
        "[\n    (\n        1,\n        SourceLine(\n            None,\n            1,\n        ),\n    ),\n]"
    );
}
