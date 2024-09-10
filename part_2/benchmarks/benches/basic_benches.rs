use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benchmarks::{fibonacci_iterative, fibonacci_recursive};

fn fibonacci_iterative_bench(c: &mut Criterion) {
    c.bench_function("fibonacci_iterative 20", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
}

fn fibonacci_recursive_bench(c: &mut Criterion) {
    c.bench_function("fibonacci_recursive 20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });
}

criterion_group!(
    basic_benches,
    fibonacci_recursive_bench,
    fibonacci_iterative_bench
);
criterion_main!(basic_benches);
