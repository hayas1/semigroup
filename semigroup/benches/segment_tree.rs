#![feature(test)]

extern crate test;
use test::{Bencher, black_box};

use semigroup::{op::Sum, segment_tree::SegmentTree};

fn make_tree(n: usize) -> SegmentTree<Sum<u64>> {
    (0..n as u64).map(Sum).collect()
}

// build: construct segment tree from N elements, O(N)

#[bench]
fn build_1k(b: &mut Bencher) {
    b.iter(|| black_box(make_tree(1_024)));
}

#[bench]
fn build_10k(b: &mut Bencher) {
    b.iter(|| black_box(make_tree(10_000)));
}

#[bench]
fn build_100k(b: &mut Bencher) {
    b.iter(|| black_box(make_tree(100_000)));
}

// query_full: full-range query, O(log N)

#[bench]
fn query_full_1k(b: &mut Bencher) {
    let tree = make_tree(1_024);
    b.iter(|| black_box(tree.combine(..)));
}

#[bench]
fn query_full_100k(b: &mut Bencher) {
    let tree = make_tree(100_000);
    b.iter(|| black_box(tree.combine(..)));
}

// query_half: middle-half range query, O(log N)

#[bench]
fn query_half_100k(b: &mut Bencher) {
    let tree = make_tree(100_000);
    b.iter(|| black_box(tree.combine(25_000..75_000)));
}

// query_small: narrow 16-element range query

#[bench]
fn query_small16_100k(b: &mut Bencher) {
    let tree = make_tree(100_000);
    b.iter(|| black_box(tree.combine(50_000..50_016)));
}

// update: single-point update, O(log N)

#[bench]
fn update_100k(b: &mut Bencher) {
    let mut tree = make_tree(100_000);
    let mut i = 0usize;
    b.iter(|| {
        tree.update(i % 100_000, Sum(black_box(42u64)));
        i += 1;
    });
}

// push: append to end, amortized O(log N)

#[bench]
fn push_1k(b: &mut Bencher) {
    b.iter(|| {
        let mut tree = SegmentTree::new();
        for i in 0u64..1_024 {
            tree.push(Sum(i));
        }
        black_box(tree)
    });
}
