#[macro_use]
extern crate criterion;
use criterion::{Criterion, ParameterizedBenchmark, PlotConfiguration, AxisScale};

use backtracking::{
    b, w,
    queens::{b_star, Queens},
    langford::{l, LangfordPairsBrute}
};

fn bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

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

    c.bench(
        "Langford Pairs",
        ParameterizedBenchmark::new(
            "B",
            |bencher, &i| bencher.iter(|| b(LangfordPairsBrute::new(i as isize), i * 2)),
            1..5,
        )
        .with_function("W", |bencher, &i| bencher.iter(|| w(LangfordPairsBrute::new(i as isize), i * 2)))
        .with_function("L", |bencher, &i| bencher.iter(|| l(i)))
        .plot_config(plot_config),
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
