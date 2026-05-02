#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use semigroup::{op::HdrHistogram, CombineIterator, Semigroup};

// chain_merge: fold N single-value HdrHistograms via semigroup
// exercises the Value+Value→Histogram and Histogram+Value paths in HdrHistogramInner

#[bench]
fn chain_merge_10(b: &mut Bencher) {
    b.iter(|| (0u64..10).map(HdrHistogram::<u32>::from).lreduce());
}

#[bench]
fn chain_merge_100(b: &mut Bencher) {
    b.iter(|| (0u64..100).map(HdrHistogram::<u32>::from).lreduce());
}

#[bench]
fn chain_merge_1000(b: &mut Bencher) {
    b.iter(|| (0u64..1000).map(HdrHistogram::<u32>::from).lreduce());
}

// collect: build a histogram from N u64 values via FromIterator

#[bench]
fn collect_10(b: &mut Bencher) {
    b.iter(|| black_box((0u64..10).collect::<HdrHistogram<u32>>()));
}

#[bench]
fn collect_100(b: &mut Bencher) {
    b.iter(|| black_box((0u64..100).collect::<HdrHistogram<u32>>()));
}

#[bench]
fn collect_1000(b: &mut Bencher) {
    b.iter(|| black_box((0u64..1000).collect::<HdrHistogram<u32>>()));
}

// merge_pair: merge two pre-built histograms, exercising the Histogram+Histogram path

#[bench]
fn merge_pair_500_500(b: &mut Bencher) {
    let a: HdrHistogram<u32> = (0u64..500).collect();
    let rhs: HdrHistogram<u32> = (500u64..1000).collect();
    b.iter(|| black_box(a.clone().semigroup(rhs.clone())));
}

#[bench]
fn merge_pair_5000_5000(b: &mut Bencher) {
    let a: HdrHistogram<u32> = (0u64..5000).collect();
    let rhs: HdrHistogram<u32> = (5000u64..10000).collect();
    b.iter(|| black_box(a.clone().semigroup(rhs.clone())));
}
