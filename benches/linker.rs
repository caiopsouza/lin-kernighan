use criterion::{criterion_group, criterion_main, Criterion};
use lin_kernighan::{load_problem, gls};

fn gls_benchmark(c: &mut Criterion) {
    let tsp = load_problem();

    let mut group = c.benchmark_group("PCB3038");
    group.sample_size(10);

    group.bench_function("gls(1000)", |b| b.iter(|| {
        gls(&tsp, 1000);
    }));

    group.finish();
}

criterion_group!(benches, gls_benchmark);
criterion_main!(benches);
