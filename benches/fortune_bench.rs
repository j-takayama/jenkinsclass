use std::env;
use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use fortune::{parse_name_from_args, print_fortune};

fn env_u64_or(default: u64, key: &str) -> u64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default)
}

fn maybe_assert_print_fortune_threshold() {
    let Some(raw_threshold) = env::var("BENCH_PRINT_FORTUNE_MAX_NS").ok() else {
        return;
    };

    let threshold_ns = raw_threshold.parse::<u128>().unwrap_or_else(|_| {
        panic!(
            "BENCH_PRINT_FORTUNE_MAX_NS must be an integer number of nanoseconds, got: {raw_threshold}"
        )
    });

    let iterations = env_u64_or(20_000, "BENCH_PRINT_FORTUNE_THRESHOLD_ITERS");
    let start = Instant::now();

    for _ in 0..iterations {
        black_box(print_fortune(
            black_box("The best code is no code at all."),
            black_box("Taylor"),
        ));
    }

    let elapsed = start.elapsed().as_nanos();
    let mean_ns = elapsed / u128::from(iterations);

    assert!(
        mean_ns <= threshold_ns,
        "print_fortune mean time {mean_ns}ns exceeded threshold {threshold_ns}ns over {iterations} iterations"
    );
}

fn bench_print_fortune(c: &mut Criterion) {
    maybe_assert_print_fortune_threshold();

    c.bench_function("print_fortune", |b| {
        b.iter(|| {
            black_box(print_fortune(
                black_box("The best code is no code at all."),
                black_box("Taylor"),
            ))
        });
    });
}

fn bench_parse_name_from_args(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_name_from_args");

    group.bench_with_input(BenchmarkId::new("with_name", "short"), &(), |b, _| {
        b.iter(|| black_box(parse_name_from_args(["--name", "Avery"])));
    });

    group.bench_with_input(BenchmarkId::new("without_name", "flags_only"), &(), |b, _| {
        b.iter(|| black_box(parse_name_from_args(["--list", "--seed", "7"])));
    });

    group.finish();
}

fn configure_criterion() -> Criterion {
    let measurement_ms = env_u64_or(2_000, "BENCH_MEASUREMENT_MS");
    let warm_up_ms = env_u64_or(800, "BENCH_WARMUP_MS");

    Criterion::default()
        .warm_up_time(Duration::from_millis(warm_up_ms))
        .measurement_time(Duration::from_millis(measurement_ms))
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = bench_print_fortune, bench_parse_name_from_args
}
criterion_main!(benches);
