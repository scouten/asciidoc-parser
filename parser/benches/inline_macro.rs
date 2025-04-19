use asciidoc_parser::Parser;
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

const BENCH_NAME: &str = "inline macro";
const PARSE_TEXT: &str = "= Example Title\n\nblah foo:bar[blah] bonus";

pub fn inline_macro(c: &mut Criterion) {
    let parser = Parser::default();
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| parser.parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, inline_macro);
criterion_main!(benches);
