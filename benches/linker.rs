use criterion::{criterion_group, criterion_main, Criterion};
use lin_kernighan::{load_problem, local_search};

fn criterion_benchmark(c: &mut Criterion) {
    let tsp = load_problem();

    let mut group = c.benchmark_group("PCB3038");
    group.sample_size(100);

    group.bench_function("PCB3038", |b| b.iter(|| {
        let candidate_route = tsp.nearest_neighbor();
        let mut candidate = candidate_route.path;
        local_search(&tsp, &mut candidate);
    }));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
