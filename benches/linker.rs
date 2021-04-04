use criterion::{criterion_group, criterion_main, Criterion};
use tsplib::Tsp;
use lin_kernighan::matrix::SymmetricMatrix;
use lin_kernighan::path::Path;
use lin_kernighan::{load_problem, local_search_step};

fn criterion_benchmark(c: &mut Criterion) {
    let tsp = load_problem();

    let mut group = c.benchmark_group("PCB3038");
    group.sample_size(10);

    group.bench_function("PCB3038", |b| b.iter(|| {
        let candidate_route = tsp.nearest_neighbor();
        let mut candidate = candidate_route.path;
        while local_search_step(&tsp, &mut candidate) {}
    }));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
