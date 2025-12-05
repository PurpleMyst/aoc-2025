use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! doit {
    ($($day:ident),*$(,)?) => {
        pub fn aoc_benchmark(c: &mut Criterion) {
            $(c.bench_function(stringify!($day), |b| b.iter($day::solve));)+
            c.bench_function("all", |b| b.iter(|| ($($day::solve()),+)));
        }

        criterion_group! {
            name = benches;

            config = Criterion::default();

            targets = aoc_benchmark
        }

        criterion_main!(benches);
    };
}

#[rustfmt::skip]
doit!(
    day01,
    day02,
    day03,
    day04,
    day05,
);
