use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use semigroup::{CombineIterator, Semigroup, op::HdrHistogram};

// chain_merge: fold N single-value HdrHistograms via semigroup
// exercises the Value+Value→Histogram and Histogram+Value paths in HdrHistogramInner
fn bench_chain_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("hdr_histogram/chain_merge");
    for n in [10u64, 100, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| (0..n).map(HdrHistogram::<u32>::from).lreduce());
        });
    }
    group.finish();
}

// collect: build a histogram from N u64 values via FromIterator
fn bench_collect(c: &mut Criterion) {
    let mut group = c.benchmark_group("hdr_histogram/collect");
    for n in [10u64, 100, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| black_box((0..n).collect::<HdrHistogram<u32>>()));
        });
    }
    group.finish();
}

// merge_pair: merge two pre-built histograms, exercising the Histogram+Histogram path
fn bench_merge_pair(c: &mut Criterion) {
    let mut group = c.benchmark_group("hdr_histogram/merge_pair");
    for n in [500u64, 5000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let a: HdrHistogram<u32> = (0..n).collect();
            let rhs: HdrHistogram<u32> = (n..n * 2).collect();
            b.iter(|| black_box(a.clone().semigroup(rhs.clone())));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_chain_merge, bench_collect, bench_merge_pair);
criterion_main!(benches);
