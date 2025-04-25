use asciidoc_parser::Parser;
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

const BENCH_NAME: &str = "section with 2 blocks";
const PARSE_TEXT: &str = "== Section Title\n\nabc\n\ndef";

pub fn section_with_two_blocks(c: &mut Criterion) {
    let parser = Parser::default();
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| parser.parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, section_with_two_blocks);
criterion_main!(benches);
