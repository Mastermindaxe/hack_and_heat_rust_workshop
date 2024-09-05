use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benchmarks::{fibonacci, fibonacci_recursive};

fn fibonacci_recursive_bench(c: &mut Criterion) {
    c.bench_function("fib rec 20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });
}

fn fibonacci_iterative_bench(c: &mut Criterion) {
    c.bench_function("fib iter 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(
    basic_benches,
    fibonacci_recursive_bench,
    fibonacci_iterative_bench
);
criterion_main!(basic_benches);
