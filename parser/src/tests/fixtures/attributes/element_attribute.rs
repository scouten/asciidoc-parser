use std::{cmp::PartialEq, fmt};

#[derive(Eq, PartialEq)]
pub(crate) struct ElementAttribute {
    pub name: Option<&'static str>,
    pub shorthand_items: &'static [&'static str],
    pub value: &'static str,
}

impl fmt::Debug for ElementAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElementAttribute")
            .field("name", &self.name)
            .field("shorthand_items", &self.shorthand_items)
            .field("value", &self.value)
            .finish()
    }
}

impl<'src> PartialEq<crate::attributes::ElementAttribute<'src>> for ElementAttribute {
    fn eq(&self, other: &crate::attributes::ElementAttribute<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<ElementAttribute> for crate::attributes::ElementAttribute<'_> {
    fn eq(&self, other: &ElementAttribute) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<ElementAttribute> for &crate::attributes::ElementAttribute<'_> {
    fn eq(&self, other: &ElementAttribute) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(
    fixture: &ElementAttribute,
    observed: &crate::attributes::ElementAttribute,
) -> bool {
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
