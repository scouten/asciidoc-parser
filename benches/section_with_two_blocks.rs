use asciidoc_parser::Document;
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

const SECTION_WITH_TWO_BLOCKS: &str = "== Section Title\n\nabc\n\ndef";

pub fn section_with_two_blocks(c: &mut Criterion) {
    c.bench_function("Section with 2 blocks", |b| {
        b.iter(|| Document::parse(black_box(SECTION_WITH_TWO_BLOCKS)))
    });
}

criterion_group!(benches, section_with_two_blocks);
criterion_main!(benches);
