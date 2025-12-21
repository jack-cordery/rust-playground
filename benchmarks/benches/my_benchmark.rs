use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::collections::HashMap;
use std::hint::black_box;

fn iterate_over_hmap(h: &HashMap<u8, bool>) -> u8 {
    h.keys().copied().sum()
}

fn iterate_over_vec(v: &Vec<u8>) -> u8 {
    v.iter().copied().sum()
}

fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration");

    for &size in &[8usize, 32, 128, 1_024, 16_384] {
        let vec: Vec<u8> = (0..size as u8).collect();
        let hmap: HashMap<u8, bool> = vec.iter().copied().map(|x| (x, true)).collect();

        group.bench_with_input(BenchmarkId::new("Vec", size), &size, |b, _| {
            b.iter(|| {
                // black_box the **result**
                black_box(iterate_over_vec(black_box(&vec)))
            })
        });

        group.bench_with_input(BenchmarkId::new("HashMap", size), &size, |b, _| {
            b.iter(|| {
                // black_box the **result**
                black_box(iterate_over_hmap(black_box(&hmap)))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_iteration);
criterion_main!(benches);
