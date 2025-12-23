use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

pub fn vec_with_cap(i: usize) {
    let mut v: Vec<u16> = Vec::with_capacity(1 << 16);
    for i in 0..i {
        v.push(i as u16);
    }
}

pub fn vec_without_cap(i: usize) {
    let mut v: Vec<u16> = Vec::new();
    for i in 0..i {
        v.push(i as u16);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec alloc");
    let test_i = 1usize << 16;
    group.bench_with_input("cap", &test_i, |b, i| {
        b.iter(|| vec_with_cap(black_box(*i)))
    });
    group.bench_with_input("no cap", &test_i, |b, i| {
        b.iter(|| vec_without_cap(black_box(*i)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
