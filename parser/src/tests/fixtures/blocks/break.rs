use std::fmt;

use crate::{
    HasSpan,
    blocks::{BreakType, IsBlock},
    tests::fixtures::{Span, attributes::Attrlist},
};

#[derive(Eq, PartialEq)]
pub(crate) struct Break {
    pub type_: BreakType,
    pub source: Span,
    pub title_source: Option<Span>,
    pub title: Option<String>,
    pub anchor: Option<Span>,
    pub attrlist: Option<Attrlist>,
}

impl fmt::Debug for Break {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Break")
            .field("type_", &self.type_)
            .field("source", &self.source)
            .field("title_source", &self.title_source)
            .field("title", &self.title)
            .field("anchor", &self.anchor)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

impl<'src> PartialEq<crate::blocks::Break<'src>> for Break {
    fn eq(&self, other: &crate::blocks::Break<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Break> for crate::blocks::Break<'_> {
    fn eq(&self, other: &Break) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Break, observed: &crate::blocks::Break) -> bool {
    if fixture.type_ != observed.type_() {
        return false;
    }

    if fixture.title_source.is_some() != observed.title_source().is_some() {
        return false;
    }

    if let Some(ref fixture_title_source) = fixture.title_source
        && let Some(ref observed_title_source) = observed.title_source()
        && fixture_title_source != observed_title_source
    {
        return false;
    }

    if fixture.title.is_some() != observed.title().is_some() {
        return false;
    }

    if let Some(ref fixture_title) = fixture.title
        && let Some(ref observed_title) = observed.title()
        && fixture_title != observed_title
    {
        return false;
    }

    if fixture.anchor.is_some() != observed.anchor().is_some() {
        return false;
    }

    if let Some(ref tcdb_anchor) = fixture.anchor
        && let Some(ref cdb_anchor) = observed.anchor()
        && tcdb_anchor != cdb_anchor
    {
        return false;
    }

    if fixture.attrlist.is_some() != observed.attrlist().is_some() {
        return false;
    }

    if let Some(ref fixture_attrlist) = fixture.attrlist
        && let Some(ref observed_attrlist) = observed.attrlist()
        && &fixture_attrlist != observed_attrlist
    {
        return false;
    }

    fixture.source == observed.span()
}
