use std::{cmp::PartialEq, collections::HashMap, fmt};

use crate::tests::fixtures::document::RefEntry;

#[derive(Eq, PartialEq)]
pub(crate) struct Catalog {
    pub(crate) refs: HashMap<&'static str, RefEntry>,
    pub(crate) reftext_to_id: HashMap<&'static str, &'static str>,
}

impl fmt::Debug for Catalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("crate::document::Catalog")
            .field("refs", &self.refs)
            .field("reftext_to_id", &self.reftext_to_id)
            .finish()
    }
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            refs: HashMap::default(),
            reftext_to_id: HashMap::default(),
        }
    }
}

impl PartialEq<crate::document::Catalog> for Catalog {
    fn eq(&self, other: &crate::document::Catalog) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Catalog> for crate::document::Catalog {
    fn eq(&self, other: &Catalog) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<Catalog> for &crate::document::Catalog {
    fn eq(&self, other: &Catalog) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Catalog, observed: &crate::document::Catalog) -> bool {
    if fixture.refs.len() != observed.refs.len()
        || fixture.reftext_to_id.len() != observed.reftext_to_id.len()
    {
        return false;
    }

    // Compare refs HashMap.
    for (key, value) in &fixture.refs {
        match observed.refs.get(*key) {
            Some(observed_value) if value == observed_value => {}
            _ => return false,
        }
    }

    // Compare reftext_to_id HashMap.
    for (key, value) in &fixture.reftext_to_id {
        match observed.reftext_to_id.get(*key) {
            Some(observed_value) if *value == observed_value.as_str() => {}
            _ => return false,
        }
    }

    true
}
