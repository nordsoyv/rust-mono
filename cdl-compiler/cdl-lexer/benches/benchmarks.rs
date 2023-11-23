use cdl_lexer::lex;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let file = include_str!("../test_script/test.cdl");

  c.bench_function("lex large file (5k lines, 130kb)", |b| b.iter(|| lex(black_box(file))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
