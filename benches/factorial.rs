use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

fn factorial_iterative_benchmark(c: &mut Criterion) {
    c.bench_function("Factorial iterative", |b| {
        b.iter(|| factorial_iterative(black_box(1000000)))
    });
}

fn factorial_recursive_benchmark(c: &mut Criterion) {
    c.bench_function("Factorial recursive", |b| {
        b.iter(|| factorial_recursive(black_box(1000000)))
    });
}

fn factorial_recursive(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

fn factorial_iterative(n: u128) -> u128 {
    let mut res = 1;
    for i in 1..n + 1 {
        res *= i;
    }
    res
}

criterion_group!(
    benches,
    factorial_iterative_benchmark,
    factorial_recursive_benchmark
);
criterion_main!(benches);
