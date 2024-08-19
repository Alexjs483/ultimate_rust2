// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started

use testing::sploosh;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// benchmark  functions: need to input a mutable-reference to a Criterion struct
// note: Using the black_box function stops the compiler from
// constant-folding away the whole function and replacing it with a constant.
pub fn sploosh_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh 2", |b| b.iter(|| sploosh(black_box(2), black_box(2), black_box(2))));
}

criterion_group!(benches, sploosh_benchmark);
criterion_main!(benches);

