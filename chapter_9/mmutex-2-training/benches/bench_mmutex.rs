use criterion::{Criterion, criterion_group, criterion_main};
use mmutex_2_training::MMutex2;
use std::{hint::black_box, thread};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mmutex2", |b| {
        let m = MMutex2::new(0);
        black_box(&m);
        b.iter(|| {
            thread::scope(|s| {
                s.spawn(|| {
                    for _ in 0..500_000 {
                        *m.lock() += 1;
                    }
                });
                s.spawn(|| {
                    for _ in 0..500_000 {
                        *m.lock() += 1;
                    }
                });
            });
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
