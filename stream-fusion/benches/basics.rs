
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate stream_fusion;
use stream_fusion::{closure,r#trait};

fn criterion_benchmark(c: &mut Criterion) {
    let lim = 1_000_000;
    c.bench_function(&*format!("trait {}", lim),
        |b| b.iter(|| r#trait::basic_bench(black_box(lim))));
    c.bench_function(&*format!("closure {}", lim),
        |b| b.iter(|| closure::basic_bench(black_box(lim))));
    c.bench_function(&*format!("basic trait {}", lim),
        |b| b.iter(|| trait_bench1(black_box(lim))));
    c.bench_function(&*format!("basic closure {}", lim),
        |b| b.iter(|| closure_bench1(black_box(lim))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use r#trait::Stream;

// arith seems to get optimized away
pub fn trait_bench1(n: usize) -> i64 {
    r#trait::range(0, n as i64)
        .map(|n| n + 2)
        .foldl(|n, x| n + x, 0)
}
// arith seems to get optimized away
pub fn closure_bench1(n: usize) -> i64 {
    let s0 = closure::range(0, n as i64);
    let s1 = closure::map(|n| n + 2, s0);
    closure::foldl(|n, x| n + x, 0, &s1)
}
