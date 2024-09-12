use asciidoc_parser::Document;
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

const BENCH_NAME: &str = "element attributes";
const PARSE_TEXT: &str = r#"= Example Title

[#rules.prominent%incremental]
abc

[appendix#custom-id]
def

[style,second-positional,named="value of named"]
ghi
"#;

pub fn perf(c: &mut Criterion) {
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| Document::parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, perf);
criterion_main!(benches);
