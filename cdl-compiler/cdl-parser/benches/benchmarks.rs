use cdl_parser::parse_text;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let file = include_str!("../../test_script/test.cdl");

  c.bench_function("parse large file (30k lines, 130kb)", |b| b.iter(|| parse_text(black_box(file))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
