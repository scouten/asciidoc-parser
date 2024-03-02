use std::{cmp::PartialEq, fmt};

use super::THeader;
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

impl<'a> PartialEq<AttrList<'a>> for TAttrlist {
    fn eq(&self, other: &Attrlist<'a>) -> bool {
        tattrlist_eq(self, other)
    }
}

impl<'a> PartialEq<TAttrlist> for Attrlist<'a> {
    fn eq(&self, other: &TAttrlist) -> bool {
        tattrlist_eq(other, self)
    }
}

impl<'a> PartialEq<TAttrlist> for &Attrlist<'a> {
    fn eq(&self, other: &TAttrlist) -> bool {
        tattrlist_eq(other, self)
    }
}

fn tattrlist_eq(tattrlist: &TAttrlist, attrlist: &Attrlist) -> bool {
    if &tattrlist.source != attrlist.span() {
        return false;
    }

    if tattrlist.attributes.len() != tattrlist.attributes().len() {
        return false;
    }
    
    for (ta_attr, attr) in tattrlist.attributes.iter().zip(attrlist.attributes()) {
        if ta_attr != attr {
            return false;
        }
    }

    true
}
