use std::{cmp::PartialEq, fmt};

use crate::{
    attributes::Attrlist,
    tests::fixtures::{attributes::TElementAttribute, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TAttrlist {
    pub attributes: Vec<TElementAttribute>,
    pub source: TSpan,
}

impl fmt::Debug for TAttrlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attrlist")
            .field("attributes", &self.attributes)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<Attrlist<'src>> for TAttrlist {
    fn eq(&self, other: &Attrlist<'src>) -> bool {
        tattrlist_eq(self, other)
    }
}

impl PartialEq<TAttrlist> for Attrlist<'_> {
    fn eq(&self, other: &TAttrlist) -> bool {
        tattrlist_eq(other, self)
    }
}

impl PartialEq<TAttrlist> for &Attrlist<'_> {
    fn eq(&self, other: &TAttrlist) -> bool {
        tattrlist_eq(other, self)
    }
}

fn tattrlist_eq(tattrlist: &TAttrlist, attrlist: &Attrlist) -> bool {
    if &tattrlist.source != attrlist.span() {
        return false;
    }

    if tattrlist.attributes.len() != attrlist.attributes().len() {
        return false;
    }

    for (ta_attr, attr) in tattrlist.attributes.iter().zip(attrlist.attributes()) {
        if ta_attr != attr {
            return false;
        }
    }

    true
}
