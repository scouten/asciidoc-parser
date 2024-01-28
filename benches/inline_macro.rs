use asciidoc_parser::Document;
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

const INLINE_MACRO: &str = "= Example Title\n\nblah foo:bar[blah] bonus";

pub fn inline_macro(c: &mut Criterion) {
    c.bench_function("inline_macro", |b| {
        b.iter(|| Document::parse(black_box(INLINE_MACRO)))
    });
}

criterion_group!(benches, inline_macro);
criterion_main!(benches);
