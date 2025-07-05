use criterion::{Criterion, criterion_group, criterion_main};
use std::{
    hint::black_box,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    thread,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("atomic", |b| {
        static A: AtomicU64 = AtomicU64::new(0);
        b.iter(|| {
            black_box(&A);
            for _ in 0..1_000_000 {
                black_box(A.load(Ordering::Relaxed));
            }
        })
    });

    c.bench_function("same cache-line", |b| {
        let a: [AtomicU64; 3] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
        b.iter(|| {
            black_box(&a);
            let flag = AtomicBool::new(false);
            thread::scope(|s| {
                s.spawn(|| {
                    loop {
                        if flag.load(Ordering::Acquire) {
                            break;
                        }
                        a[0].store(0, Ordering::Relaxed);
                        a[2].store(0, Ordering::Relaxed);
                    }
                });

                for _ in 0..1_000_000 {
                    black_box(a[1].load(Ordering::Relaxed));
                }
                flag.store(true, Ordering::Release);
            });
        })
    });

    c.bench_function("different cache-line", |b| {
        #[repr(align(64))]
        struct AlignedAtomic(AtomicU64);

        let a: [AlignedAtomic; 3] = [
            AlignedAtomic(AtomicU64::new(0)),
            AlignedAtomic(AtomicU64::new(0)),
            AlignedAtomic(AtomicU64::new(0)),
        ];
        b.iter(|| {
            black_box(&a);
            let flag = AtomicBool::new(false);
            thread::scope(|s| {
                s.spawn(|| {
                    loop {
                        if flag.load(Ordering::Acquire) {
                            break;
                        }
                        a[0].0.store(0, Ordering::Relaxed);
                        a[2].0.store(0, Ordering::Relaxed);
                    }
                });

                for _ in 0..1_000_000 {
                    black_box(a[1].0.load(Ordering::Relaxed));
                }
                flag.store(true, Ordering::Release);
            });
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
