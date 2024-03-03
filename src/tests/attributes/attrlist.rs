use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::Attrlist,
    tests::fixtures::{attributes::TAttrlist, TSpan},
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let (_, b1) = Attrlist::parse(Span::new("abc", true)).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    let (rem, attrlist) = Attrlist::parse(Span::new("", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    assert_eq!(
        attrlist,
        TAttrlist {
            attributes: vec!(),
            source: TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert!(attrlist.named_attribute("foo").is_none());

    assert!(attrlist.nth_attribute(0).is_none());
    assert!(attrlist.nth_attribute(1).is_none());
    assert!(attrlist.nth_attribute(42).is_none());

    assert_eq!(
        attrlist.span(),
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}
