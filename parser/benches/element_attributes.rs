use asciidoc_parser::Parser;
use codspeed_criterion_compat::{Criterion, black_box, criterion_group, criterion_main};

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
    let parser = Parser::default();
    c.bench_function(BENCH_NAME, |b| {
        b.iter(|| parser.parse(black_box(PARSE_TEXT)))
    });
}

criterion_group!(benches, perf);
criterion_main!(benches);
