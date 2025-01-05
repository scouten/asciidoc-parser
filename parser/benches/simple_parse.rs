use asciidoc_parser::Document;
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

const BENCH_NAME: &str = "2 blocks + title";
const PARSE_TEXT: &str = "= Example Title\n\nabc\n\ndef";

pub fn two_blocks_and_title(c: &mut Criterion) {
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| Document::parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, two_blocks_and_title);
criterion_main!(benches);
