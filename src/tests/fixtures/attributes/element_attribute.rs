use std::{cmp::PartialEq, fmt};

use crate::{attributes::ElementAttribute, tests::fixtures::TSpan, HasSpan};

#[derive(Eq, PartialEq)]
pub(crate) struct TElementAttribute {
    pub name: Option<TSpan>,
    pub shorthand_items: Vec<TSpan>,
    pub value: TSpan,
    pub source: TSpan,
}

impl fmt::Debug for TElementAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElementAttribute")
            .field("name", &self.name)
            .field("shorthand_items", &self.shorthand_items)
            .field("value", &self.value)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<ElementAttribute<'src>> for TElementAttribute {
    fn eq(&self, other: &ElementAttribute<'src>) -> bool {
        tattribute_eq(self, other)
    }
}

impl<'src> PartialEq<TElementAttribute> for ElementAttribute<'src> {
    fn eq(&self, other: &TElementAttribute) -> bool {
        tattribute_eq(other, self)
    }
}

impl<'src> PartialEq<TElementAttribute> for &ElementAttribute<'src> {
    fn eq(&self, other: &TElementAttribute) -> bool {
        tattribute_eq(other, self)
    }
}

fn tattribute_eq(tattribute: &TElementAttribute, attribute: &ElementAttribute) -> bool {
    if &tattribute.source != attribute.span() {
        return false;
    }

    if tattribute.value != attribute.raw_value() {
        return false;
    }

    if &tattribute.shorthand_items != attribute.shorthand_items() {
        return false;
    }

    match tattribute.name {
        Some(ref name) => {
            if let Some(attr_name) = attribute.name() {
                name == attr_name
            } else {
                false
            }
        }
        None => attribute.name().is_none(),
    }
}
