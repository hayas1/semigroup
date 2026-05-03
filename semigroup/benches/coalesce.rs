use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use semigroup::{CombineIterator, Semigroup, op::Coalesce};

// fold_all_some: all elements are Some — first wins immediately on every op
fn bench_fold_all_some(c: &mut Criterion) {
    let mut group = c.benchmark_group("coalesce/fold_all_some");
    for n in [10u32, 100, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| (0..n).map(|i| Coalesce(Some(black_box(i)))).lreduce());
        });
    }
    group.finish();
}

// fold_leading_nones: N-1 Nones then one Some — worst case, traverses all elements
fn bench_fold_leading_nones(c: &mut Criterion) {
    let mut group = c.benchmark_group("coalesce/fold_leading_nones");
    for n in [10u32, 100, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                (0..n)
                    .map(|i| Coalesce(if i + 1 == n { Some(black_box(i)) } else { None }))
                    .lreduce()
            });
        });
    }
    group.finish();
}

// derive_struct: realistic layered-config merge via #[derive(Semigroup)]
#[derive(Clone, Semigroup)]
#[semigroup(with = "semigroup::op::Coalesce")]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u32>,
}

fn bench_derive_struct(c: &mut Criterion) {
    let cli = Config { host: Some("localhost".into()), port: Some(8080), timeout: None };
    let env = Config { host: None, port: None, timeout: Some(30) };
    let file = Config { host: Some("default".into()), port: Some(80), timeout: Some(60) };
    c.bench_function("coalesce/derive_struct/merge_3", |b| {
        b.iter(|| black_box(cli.clone().semigroup(env.clone()).semigroup(file.clone())))
    });
}

criterion_group!(benches, bench_fold_all_some, bench_fold_leading_nones, bench_derive_struct);
criterion_main!(benches);
