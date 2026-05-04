use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use semigroup::{op::Sum, segment_tree::SegmentTree};

fn new_sum_tree(n: usize) -> SegmentTree<Sum<u64>> {
    (0..n as u64).map(Sum).collect()
}

// build/collect: construct segment tree from N elements via FromIterator, O(N)
// build/push:    construct segment tree by pushing N elements one by one, O(N log N)
fn bench_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_tree/build");
    group.sample_size(20);
    for n in [1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::new("collect", n), &n, |b, &n| {
            b.iter(|| black_box(new_sum_tree(n)));
        });
        group.bench_with_input(BenchmarkId::new("push", n), &n, |b, &n| {
            b.iter(|| {
                let mut tree = SegmentTree::new();
                for i in 0u64..n as u64 {
                    tree.push(Sum(i));
                }
                black_box(tree)
            });
        });
    }
    group.finish();
}

// query: range queries across tree sizes, O(log N)
fn bench_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_tree/query");
    for n in [1_000, 10_000, 100_000] {
        let tree = new_sum_tree(n);
        // full-range query
        group.bench_with_input(BenchmarkId::new("full", n), &tree, |b, tree| {
            b.iter(|| black_box(tree.combine(..)))
        });
        // middle-half range query
        group.bench_with_input(BenchmarkId::new("half", n), &tree, |b, tree| {
            b.iter(|| black_box(tree.combine(n / 4..n * 3 / 4)))
        });
        // narrow 16-element range query
        group.bench_with_input(BenchmarkId::new("small16", n), &tree, |b, tree| {
            b.iter(|| black_box(tree.combine(n / 2..n / 2 + 16)))
        });
    }
    group.finish();
}

// update: single-point update, O(log N) — reuses the same tree across iterations
fn bench_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_tree/update");
    for n in [1_000, 10_000, 100_000] {
        let mut tree = new_sum_tree(n);
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter(|| tree.update(black_box(n / 2), Sum(black_box(42u64))))
        });
    }
    group.finish();
}

// push: append to end, amortized O(log N) — measures a single push into an N-element tree
// returns tree so criterion drops it outside the timed region
fn bench_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_tree/push");
    for n in [1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || new_sum_tree(n),
                |mut tree| {
                    tree.push(Sum(black_box(42u64)));
                    tree
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, bench_build, bench_query, bench_update, bench_push);
criterion_main!(benches);
