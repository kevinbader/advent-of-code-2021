use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day6::*;

pub fn day6_bench(c: &mut Criterion) {
    let input = fs::read_to_string("./input/day6.txt").expect("failed to read input file");
    let n_days = 80;
    let lanternfish = parse(&input);

    let mut group = c.benchmark_group("lanternfish");
    // group.sample_size(10);
    group.bench_with_input("advance1", &lanternfish, |b, input| {
        b.iter(|| advance1(input.clone(), n_days))
    });
    group.bench_with_input("advance2", &lanternfish, |b, input| {
        b.iter(|| advance2(input.clone(), n_days))
    });
    group.bench_with_input("advance3", &lanternfish, |b, input| {
        b.iter(|| advance3(input, n_days))
    });
    group.finish();
}

criterion_group!(benches, day6_bench);
criterion_main!(benches);
