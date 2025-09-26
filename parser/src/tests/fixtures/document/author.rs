use std::{cmp::PartialEq, fmt};

#[derive(Eq, PartialEq)]
pub(crate) struct Author {
    pub(crate) name: &'static str,
    pub(crate) firstname: &'static str,
    pub(crate) middlename: Option<&'static str>,
    pub(crate) lastname: Option<&'static str>,
    pub(crate) email: Option<&'static str>,
}

impl fmt::Debug for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("crate::document::Author")
            .field("name", &self.name)
            .field("firstname", &self.firstname)
            .field("middlename", &self.middlename)
            .field("lastname", &self.lastname)
            .field("email", &self.email)
            .finish()
    }
}

impl PartialEq<crate::document::Author> for Author {
    fn eq(&self, other: &crate::document::Author) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Author> for crate::document::Author {
    fn eq(&self, other: &Author) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<Author> for &crate::document::Author {
    fn eq(&self, other: &Author) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Author, observed: &crate::document::Author) -> bool {
    fixture.name == observed.name()
        && fixture.firstname == observed.firstname()
        && fixture.middlename.as_deref() == observed.middlename()
        && fixture.lastname.as_deref() == observed.lastname()
        && fixture.email.as_deref() == observed.email()
}
