use criterion::{criterion_group, criterion_main, Criterion};
use lin_kernighan::{load_problem, gls};

fn criterion_benchmark(c: &mut Criterion) {
    let tsp = load_problem();

    let mut group = c.benchmark_group("PCB3038");
    group.sample_size(10);

    group.bench_function("gls(10000)", |b| b.iter(|| {
        gls(&tsp, 10000);
    }));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
