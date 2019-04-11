use criterion::{criterion_group, criterion_main, Criterion};
use geo::simplify::Simplify;
use geo::simplifyvw::{SimplifyVW, SimplifyVWPreserve};
use geo_types::LineString;

fn bench_rdp(c: &mut Criterion) {
    c.bench_function("bench_rdp", |b| {
        let points = include!("../src/mk_route.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplify(&0.001);
        });
    });
}

fn bench_visvalingam(c: &mut Criterion) {
    c.bench_function("bench_visvalingam", |b| {
        let points = include!("../src/mk_route.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplifyvw(&0.0000075);
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

fn bench_visvalingam_long(c: &mut Criterion) {
    c.bench_function("bench_visvalingam_long", |b| {
        let points = include!("../src/mk_route_long.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplifyvw(&0.0000075);
        });
    });
}

fn bench_visvalingamp_long(c: &mut Criterion) {
    c.bench_function("bench_visvalingamp_long", |b| {
        let points = include!("../src/mk_route_long.rs");
        let ls: LineString<f64> = points.into();
        b.iter(|| {
            ls.simplifyvw_preserve(&0.0000075);
        });
    });
}

criterion_group!(
    benches,
    bench_rdp,
    bench_visvalingam,
    bench_rdp_long,
    bench_visvalingam_long,
    bench_visvalingamp_long
);
criterion_main!(benches);
