#![feature(test)]

extern crate test;
use test::{Bencher, black_box};

use semigroup::{CombineIterator, Semigroup, op::Coalesce};

// fold_all_some: all elements are Some — first wins immediately on every op
#[bench]
fn fold_all_some_10(b: &mut Bencher) {
    b.iter(|| (0u32..10).map(|i| Coalesce(Some(i))).lreduce());
}

#[bench]
fn fold_all_some_100(b: &mut Bencher) {
    b.iter(|| (0u32..100).map(|i| Coalesce(Some(i))).lreduce());
}

#[bench]
fn fold_all_some_1000(b: &mut Bencher) {
    b.iter(|| (0u32..1000).map(|i| Coalesce(Some(i))).lreduce());
}

// fold_leading_nones: N-1 Nones then one Some — worst case, traverses all elements
#[bench]
fn fold_leading_nones_10(b: &mut Bencher) {
    b.iter(|| {
        (0u32..10)
            .map(|i| Coalesce(if i + 1 == 10 { Some(i) } else { None }))
            .lreduce()
    });
}

#[bench]
fn fold_leading_nones_100(b: &mut Bencher) {
    b.iter(|| {
        (0u32..100)
            .map(|i| Coalesce(if i + 1 == 100 { Some(i) } else { None }))
            .lreduce()
    });
}

#[bench]
fn fold_leading_nones_1000(b: &mut Bencher) {
    b.iter(|| {
        (0u32..1000)
            .map(|i| Coalesce(if i + 1 == 1000 { Some(i) } else { None }))
            .lreduce()
    });
}

// derive_struct: realistic layered-config merge via #[derive(Semigroup)]
#[derive(Clone, Semigroup)]
#[semigroup(with = "semigroup::op::Coalesce")]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u32>,
}
#[bench]
fn derive_struct_merge_3(b: &mut Bencher) {
    let cli = Config {
        host: Some("localhost".into()),
        port: Some(8080),
        timeout: None,
    };
    let env = Config {
        host: None,
        port: None,
        timeout: Some(30),
    };
    let file = Config {
        host: Some("default".into()),
        port: Some(80),
        timeout: Some(60),
    };
    b.iter(|| black_box(cli.clone().semigroup(env.clone()).semigroup(file.clone())));
}
