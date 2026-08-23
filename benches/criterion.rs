// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing
// terms. Copyright © 2024 WiserOne. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Criterion benchmarks for the wiserone macros and `run()`.

use criterion::{criterion_group, criterion_main, Criterion};
use wiserone::{wiserone_join, wiserone_map, wiserone_vec};

fn wiserone_vec_benchmark(c: &mut Criterion) {
    let _ = c.bench_function("wiserone_vec_macro", |b| {
        b.iter(|| wiserone_vec![1, 2, 3, 4, 5])
    });
}

fn wiserone_map_benchmark(c: &mut Criterion) {
    let _ = c.bench_function("wiserone_map_macro", |b| {
        b.iter(|| {
            wiserone_map!["a" => 1, "b" => 2, "c" => 3, "d" => 4, "e" => 5]
        })
    });
}

fn wiserone_join_benchmark(c: &mut Criterion) {
    let _ = c.bench_function("wiserone_join_macro", |b| {
        b.iter(|| wiserone_join!["a", "b", "c", "d", "e"])
    });
}

// The former "wiserone" benchmark called `run()` 1000 times per
// iteration. It was removed rather than repaired, for three reasons:
//
//  * it could never run. `run()` parsed the process arguments, and
//    under `cargo bench` those contain `--bench`, so clap rejected it
//    and the whole benchmark binary failed;
//  * each call creates ./docs/logs, opens a log file, and writes
//    generated HTML into ./docs — a benchmark should not mutate the
//    project's output tree, still less 1000 times per sample;
//  * what it measured was filesystem I/O and argument parsing, not
//    anything specific to this crate.
//
// The macro benchmarks below, and benches/performance_suite.rs, cover
// the parts worth measuring. `run_cli_from` now exists if an
// end-to-end benchmark is ever wanted, but it would need a scratch
// output directory first.

criterion_group!(
    wiserone_macros_benchmark,
    wiserone_vec_benchmark,
    wiserone_map_benchmark,
    wiserone_join_benchmark
);
criterion_main!(wiserone_macros_benchmark);
