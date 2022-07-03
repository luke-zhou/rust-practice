use sort::{
    bubble_sort,
    merge_sort,
    quick_sort,
};
use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion,
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut list: Vec<i64> = Vec::with_capacity(100_000);
    for _ in 0..list.capacity() {
        list.push(rand::random());
    };
    let mut group = c.benchmark_group("sort");
    group.bench_function("bubble sort 100_000", |b| b.iter(|| bubble_sort(black_box(& mut list))));
    group.bench_function("merge sort 100_000", |b| b.iter(|| merge_sort(black_box(& mut list))));
    group.bench_function("quick sort 100_000", |b| b.iter(|| quick_sort(black_box(& mut list), 0, 99_999)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);