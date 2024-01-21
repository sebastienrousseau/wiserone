// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing
// terms. Copyright © 2024 WiserOne. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use wiserone::{run, wiserone_join, wiserone_map, wiserone_vec};

fn wiserone_vec_benchmark(c: &mut Criterion) {
    c.bench_function("wiserone_vec_macro", |b| {
        b.iter(|| wiserone_vec![1, 2, 3, 4, 5])
    });
}

fn wiserone_map_benchmark(c: &mut Criterion) {
    c.bench_function("wiserone_map_macro", |b| {
        b.iter(|| {
            wiserone_map!["a" => 1, "b" => 2, "c" => 3, "d" => 4, "e" => 5]
        })
    });
}

fn wiserone_join_benchmark(c: &mut Criterion) {
    c.bench_function("wiserone_join_macro", |b| {
        b.iter(|| wiserone_join!["a", "b", "c", "d", "e"])
    });
}

fn wiserone_benchmark(c: &mut Criterion) {
    c.bench_function("wiserone", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                run().unwrap();
            }
        })
    });
}

criterion_group!(
    wiserone_macros_benchmark,
    wiserone_vec_benchmark,
    wiserone_map_benchmark,
    wiserone_join_benchmark,
    wiserone_benchmark
);
criterion_main!(wiserone_macros_benchmark);
