use pretty_assertions_sorted::assert_eq;

#[test]
fn impl_clone_eq() {
    let span = crate::Span::new("abc");
    let pr1 = span.into_parse_result(2);
    let pr2 = pr1;
    assert_eq!(pr1, pr2);
}
