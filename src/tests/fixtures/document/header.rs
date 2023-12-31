use std::{cmp::PartialEq, fmt};

use crate::{document::Header, tests::fixtures::TSpan, HasSpan};

// Approximate mock of Header type that we can use
// to declare expected values for easier test writing.
//
// Primary difference is that the data members are public
// so we can declare them inline.
#[derive(Eq, PartialEq)]
pub(crate) struct THeader {
    pub title: Option<TSpan>,
    pub source: TSpan,
}

impl fmt::Debug for THeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Intentionally mimic the output of nom_span::Spanned
        // so diffs point the unit test author to the important
        // differences.
        f.debug_struct("Header")
            .field("title", &self.title)
            .field("source", &self.source)
            .finish()
    }
}

impl<'a> PartialEq<Header<'a>> for THeader {
    fn eq(&self, other: &Header<'a>) -> bool {
        theader_eq(self, other)
    }
}

impl<'a> PartialEq<THeader> for Header<'a> {
    fn eq(&self, other: &THeader) -> bool {
        theader_eq(other, self)
    }
}

impl<'a> PartialEq<THeader> for &Header<'a> {
    fn eq(&self, other: &THeader) -> bool {
        theader_eq(other, self)
    }
}

fn theader_eq(theader: &THeader, header: &Header) -> bool {
    if &theader.source != header.span() {
        return false;
    }

    if theader.title.is_some() != header.title().is_some() {
        return false;
    }

    if let Some(ref th_title) = theader.title {
        if let Some(ref h_title) = header.title() {
            if th_title != h_title {
                return false;
            }
        }
    }

    true
}
