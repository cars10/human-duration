use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_duration::human_duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("human_duration(0, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(0, 0);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(30, 5_000_000)", |b| {
        b.iter(|| {
            let duration = Duration::new(30, 5_000_000);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(60, 50_000_000)", |b| {
        b.iter(|| {
            let duration = Duration::new(60, 50_000_000);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(92, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(92, 0);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(3600, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(3600, 0);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(86400, 337_000_000)", |b| {
        b.iter(|| {
            let duration = Duration::new(86400, 337_000_000);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(86680, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(86680, 0);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(2_628_000, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(2_628_000, 0);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(2_828_000, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(2_828_000, 0);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(31_536_000, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(31_536_000, 0);
            human_duration(black_box(&duration));
        })
    });

    c.bench_function("human_duration(34_536_000, 0)", |b| {
        b.iter(|| {
            let duration = Duration::new(34_536_000, 0);
            human_duration(black_box(&duration));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
