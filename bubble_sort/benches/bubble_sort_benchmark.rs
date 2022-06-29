use bubble_sort::bubble_sort;
use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut list: Vec<i64> = Vec::with_capacity(100_000);
    for _ in 0..list.capacity() {
        list.push(rand::random());
    };
    c.bench_function("bubble sort 100_000", |b| b.iter(|| bubble_sort(black_box(& mut list))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
