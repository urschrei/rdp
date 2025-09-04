use criterion::{criterion_group, criterion_main, Criterion};
use geo::simplify::{Simplify, SimplifyIdx};
use geo::simplify_vw::{SimplifyVw, SimplifyVwIdx, SimplifyVwPreserve};
use geo_types::LineString;

fn bench_rdp(c: &mut Criterion) {
    c.bench_function("bench_rdp", |b| {
        let points = include!("../src/mk_route.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify(0.001);
        });
    });
}

fn bench_rdp_idx(c: &mut Criterion) {
    c.bench_function("bench_rdp_idx", |b| {
        let points = include!("../src/mk_route.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify_idx(0.001);
        });
    });
}

fn bench_visvalingam(c: &mut Criterion) {
    c.bench_function("bench_visvalingam", |b| {
        let points = include!("../src/mk_route.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify_vw(&0.0000075);
        });
    });
}

fn bench_visvalingam_idx(c: &mut Criterion) {
    c.bench_function("bench_visvalingam_idx", |b| {
        let points = include!("../src/mk_route.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify_vw_idx(&0.0000075);
        });
    });
}

fn bench_rdp_long(c: &mut Criterion) {
    c.bench_function("bench_rdp_long", |b| {
        let points = include!("../src/mk_route_long.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify(&0.001);
        });
    });
}

fn bench_rdp_long_idx(c: &mut Criterion) {
    c.bench_function("bench_rdp_long_idx", |b| {
        let points = include!("../src/mk_route_long.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify_idx(&0.001);
        });
    });
}

fn bench_visvalingam_long(c: &mut Criterion) {
    c.bench_function("bench_visvalingam_long", |b| {
        let points = include!("../src/mk_route_long.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify_vw(&0.0000075);
        });
    });
}

fn bench_visvalingam_long_idx(c: &mut Criterion) {
    c.bench_function("bench_visvalingam_long_idx", |b| {
        let points = include!("../src/mk_route_long.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify_vw_idx(&0.0000075);
        });
    });
}

fn bench_visvalingamp_long(c: &mut Criterion) {
    c.bench_function("bench_visvalingamp_long", |b| {
        let points = include!("../src/mk_route_long.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify_vw_preserve(&0.0000075);
        });
    });
}

criterion_group!(
    benches,
    bench_rdp,
    bench_rdp_idx,
    bench_visvalingam,
    bench_visvalingam_idx,
    bench_rdp_long,
    bench_rdp_long_idx,
    bench_visvalingam_long,
    bench_visvalingam_long_idx,
    bench_visvalingamp_long
);
criterion_main!(benches);
