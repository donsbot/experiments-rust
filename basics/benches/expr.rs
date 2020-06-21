use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

criterion_group!(benches, bench);
criterion_main!(benches);

use basics::sum_f1;
use basics::sum_f2;
use basics::sum_f3;

// criterion benchmarks
pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sums");
    for i in [10000000000].iter() {
        group.bench_with_input(BenchmarkId::new("Low level", i), i,
            |b, i| b.iter(|| sum_f2(*i)));
        group.bench_with_input(BenchmarkId::new("Iteration", i), i,
            |b, i| b.iter(|| sum_f3(*i)));
        group.bench_with_input(BenchmarkId::new("Recursive", i), i,
            |b, i| b.iter(|| sum_f1(*i)));
    }
    group.finish();
}
