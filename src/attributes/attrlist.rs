use crate::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attrlist<'a> {
    attributes: Vec<Attribute<'a>>,
    source: Span<'a>,
}
