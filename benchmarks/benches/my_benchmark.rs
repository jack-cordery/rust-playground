use ahash::AHashMap;
use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::HashMap;
use std::hint::black_box;

fn iterate_over_hmap(h: &HashMap<u32, bool>) -> u32 {
    h.keys().map(|e| e * 2).sum()
}

fn iterate_over_fmap(h: &AHashMap<u32, bool>) -> u32 {
    h.keys().map(|e| e * 2).sum()
}

fn iterate_over_vec(v: &Vec<u32>) -> u32 {
    v.iter().map(|e| e * 2).sum()
}

fn iterate_over_slice(v: &[u32]) -> u32 {
    v.iter().map(|e| e * 2).sum()
}

fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration");

    let vec: Vec<u32> = (0..1_000_000).collect();
    let hmap: HashMap<u32, bool> = vec.iter().copied().map(|x| (x, true)).collect();
    let fmap: AHashMap<u32, bool> = vec.iter().copied().map(|x| (x, true)).collect();

    group.bench_with_input("vec", &vec, |b, i| {
        b.iter(|| iterate_over_vec(black_box(i)))
    });
    group.bench_with_input("slice", &vec, |b, i| {
        b.iter(|| iterate_over_slice(black_box(i)))
    });
    group.bench_with_input("hmap", &hmap, |b, i| {
        b.iter(|| iterate_over_hmap(black_box(i)))
    });
    group.bench_with_input("fmap", &fmap, |b, i| {
        b.iter(|| iterate_over_fmap(black_box(i)))
    });
}

criterion_group!(benches, bench_iteration);
criterion_main!(benches);
