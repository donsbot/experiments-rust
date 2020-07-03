
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate stream_fusion;
use stream_fusion::{closure,r#trait};

fn criterion_benchmark(c: &mut Criterion) {
    let lim = 10_000_000;
    c.bench_function(&*format!("trait {}", lim),
        |b| b.iter(|| r#trait::basic_bench(black_box(lim))));
    c.bench_function(&*format!("closure {}", lim),
        |b| b.iter(|| closure::basic_bench(black_box(lim))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
