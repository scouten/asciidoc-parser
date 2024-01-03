use asciidoc_parser::Document;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const TWO_BLOCKS_AND_TITLE: &str = "= Example Title\n\nabc\n\ndef";

pub fn two_blocks_and_title(c: &mut Criterion) {
    c.bench_function("2 blocks + title", |b| {
        b.iter(|| Document::parse(black_box(TWO_BLOCKS_AND_TITLE)))
    });
}

criterion_group!(benches, two_blocks_and_title);
criterion_main!(benches);
