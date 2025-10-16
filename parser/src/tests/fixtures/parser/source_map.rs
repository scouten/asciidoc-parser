use crate::tests::fixtures::parser::SourceLine;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SourceMap(pub &'static [(usize, SourceLine)]);

impl<'src> PartialEq<crate::parser::SourceMap> for SourceMap {
    fn eq(&self, other: &crate::parser::SourceMap) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<SourceMap> for crate::parser::SourceMap {
    fn eq(&self, other: &SourceMap) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &SourceMap, observed: &crate::parser::SourceMap) -> bool {
    if fixture.0.len() != observed.0.len() {
        return false;
    }

    for (fixture_source_line, observed_source_line) in fixture.0.iter().zip(&observed.0) {
        if fixture_source_line.0 != observed_source_line.0
            || fixture_source_line.1 != observed_source_line.1
        {
            return false;
        }
    }

    true
}
