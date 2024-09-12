use asciidoc_parser::Document;
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

const BENCH_NAME: &str = "inline_macro";
const PARSE_TEXT: &str = "= Example Title\n\nblah foo:bar[blah] bonus";

pub fn perf(c: &mut Criterion) {
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| Document::parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, perf);
criterion_main!(benches);
