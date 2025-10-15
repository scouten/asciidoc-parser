#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SourceLine(pub Option<&'static str>, pub usize);

impl PartialEq<crate::parser::SourceLine> for SourceLine {
    fn eq(&self, other: &crate::parser::SourceLine) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<SourceLine> for crate::parser::SourceLine {
    fn eq(&self, other: &SourceLine) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &SourceLine, observed: &crate::parser::SourceLine) -> bool {
    fixture.0 == observed.0.as_deref() && fixture.1 == observed.1
}
