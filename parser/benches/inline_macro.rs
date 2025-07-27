use asciidoc_parser::Parser;
use codspeed_criterion_compat::{Criterion, black_box, criterion_group, criterion_main};

const BENCH_NAME: &str = "inline macro";
const PARSE_TEXT: &str = "= Example Title\n\nblah foo:bar[blah] bonus";

pub fn inline_macro(c: &mut Criterion) {
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| Parser::default().parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, inline_macro);
criterion_main!(benches);
