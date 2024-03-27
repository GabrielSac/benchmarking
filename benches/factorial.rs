use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num::BigInt;
/*
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");
    group.bench_function("Factorial iterative", |b| {
        b.iter(|| factorial_iterative(black_box(100)))
    });
    group.bench_function("Factorial recursive", |b| {
        b.iter(|| factorial_recursive(black_box(100)))
    });
    group.finish();
}
*/

fn factorial_recursive(n: BigInt) -> BigInt {
    if n == BigInt::from(0) {
        BigInt::from(1)
    } else {
        n.clone() * factorial_recursive(n - 1)
    }
}

fn factorial_iterative(n: BigInt) -> BigInt {
    let mut res = BigInt::from(1);
    let mut i = BigInt::from(1);
    while i <= n {
        res *= i.clone();
        i += 1;
    }
    res
}

fn factorial_iterative_benchmark(c: &mut Criterion) {
    c.bench_function("Factorial iterative", |b| {
        b.iter(|| factorial_iterative(black_box(BigInt::from(10000))))
    });
}

fn factorial_recursive_benchmark(c: &mut Criterion) {
    c.bench_function("Factorial recursive", |b| {
        b.iter(|| factorial_recursive(black_box(BigInt::from(10000))))
    });
}

criterion_group!(
    benches,
    factorial_iterative_benchmark,
    factorial_recursive_benchmark
);
criterion_main!(benches);
