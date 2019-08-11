#[macro_use]
extern crate criterion;
use criterion::{AxisScale, Criterion, ParameterizedBenchmark, PlotConfiguration};

use backtracking::{
    b,
    langford::{l, LangfordPairsBrute},
    queens::{b_star, Queens},
    recursive, w,
};

fn bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    c.bench(
        "Queens",
        ParameterizedBenchmark::new(
            "B",
            |bencher, &i| bencher.iter(|| b(Queens::new(i), i)),
            11..13,
        )
        .with_function("BStar", |bencher, &i| bencher.iter(|| b_star(i)))
        .plot_config(plot_config),
    );

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    c.bench(
        "Langford Pairs",
        ParameterizedBenchmark::new(
            "B",
            |bencher, &i| bencher.iter(|| b(LangfordPairsBrute::new(i as isize), i * 2)),
            1..5,
        )
        .with_function("W", |bencher, &i| {
            bencher.iter(|| w(LangfordPairsBrute::new(i as isize), i * 2))
        })
        .with_function("Recursive", |bencher, &i| {
            bencher.iter(|| recursive(LangfordPairsBrute::new(i as isize), i * 2))
        })
        .with_function("L", |bencher, &i| bencher.iter(|| l(i)))
        .plot_config(plot_config),
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
