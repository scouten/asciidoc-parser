use asciidoc_parser::Parser;
use codspeed_criterion_compat::{Criterion, black_box, criterion_group, criterion_main};

const BENCH_NAME: &str = "section with 2 blocks";
const PARSE_TEXT: &str = "== Section Title\n\nabc\n\ndef";

pub fn section_with_two_blocks(c: &mut Criterion) {
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| Parser::default().parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, section_with_two_blocks);
criterion_main!(benches);
