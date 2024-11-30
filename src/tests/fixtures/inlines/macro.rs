use std::fmt;

use crate::{inlines::InlineMacro, tests::fixtures::TSpan, HasSpan};

#[derive(Eq, PartialEq)]
pub(crate) struct TInlineMacro {
    pub name: TSpan,
    pub target: Option<TSpan>,
    pub attrlist: Option<TSpan>,
    pub source: TSpan,
}

impl fmt::Debug for TInlineMacro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InlineMacro")
            .field("name", &self.name)
            .field("target", &self.target)
            .field("attrlist", &self.attrlist)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<InlineMacro<'src>> for TInlineMacro {
    fn eq(&self, other: &InlineMacro<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TInlineMacro> for InlineMacro<'_> {
    fn eq(&self, other: &TInlineMacro) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TInlineMacro, inline_macro: &InlineMacro) -> bool {
    if &fixture.name != inline_macro.name() {
        return false;
    }

    if fixture.target.is_some() != inline_macro.target().is_some() {
        return false;
    }

    if let Some(ref th_target) = fixture.target {
        if let Some(ref h_target) = inline_macro.target() {
            if &th_target != h_target {
                return false;
            }
        }
    }

    if fixture.attrlist.is_some() != inline_macro.attrlist().is_some() {
        return false;
    }

    if let Some(ref th_attrlist) = fixture.attrlist {
        if let Some(ref h_attrlist) = inline_macro.attrlist() {
            if &th_attrlist != h_attrlist {
                return false;
            }
        }
    }

    &fixture.source == inline_macro.span()
}
