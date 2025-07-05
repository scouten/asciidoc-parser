use std::{cmp::PartialEq, fmt};

use crate::{attributes::ElementAttribute, tests::fixtures::TSpan, HasSpan};

#[derive(Eq, PartialEq)]
pub(crate) struct TElementAttribute {
    pub name: Option<&'static str>,
    pub shorthand_items: Vec<&'static str>,
    pub value: &'static str,
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
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TElementAttribute> for ElementAttribute<'_> {
    fn eq(&self, other: &TElementAttribute) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TElementAttribute> for &ElementAttribute<'_> {
    fn eq(&self, other: &TElementAttribute) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TElementAttribute, observed: &ElementAttribute) -> bool {
    if &fixture.source != observed.span() {
        return false;
    }

    if fixture.value != observed.value() {
        return false;
    }

    if fixture.shorthand_items != observed.shorthand_items() {
        return false;
    }

    match fixture.name {
        Some(fixture_name) => {
            if let Some(observed_name) = observed.name() {
                fixture_name == observed_name
            } else {
                false
            }
        }
        None => observed.name().is_none(),
    }
}
