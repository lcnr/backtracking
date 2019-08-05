#[macro_use]
extern crate criterion;
use criterion::{Criterion, ParameterizedBenchmark};

use backtracking::{
    b::{b, w},
    queens::{b_star, Queens},
};

fn bench_fibs(c: &mut Criterion) {
    c.bench(
        "Queens",
        ParameterizedBenchmark::new(
            "B",
            |bencher, &i| bencher.iter(|| b(Queens::new(i), i)),
            1..12,
        )
        .with_function("W", |bencher, &i| bencher.iter(|| w(Queens::new(i), i)))
        .with_function("BStar", |bencher, &i| bencher.iter(|| b_star(i))),
    );
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
