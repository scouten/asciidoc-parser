use crate::{attributes::ElementAttribute, Span};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attrlist<'a> {
    attributes: Vec<ElementAttribute<'a>>,
    source: Span<'a>,
}
