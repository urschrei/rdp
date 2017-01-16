#![feature(test)]

extern crate test;
use test::Bencher;
extern crate rdp;
extern crate geo;
use geo::{LineString, Point};
use geo::simplify::Simplify;
use geo::simplifyvw::SimplifyVW;

#[bench]
fn bench_rdp(b: &mut Bencher) {
    let points = include!("../src/mk_route.rs");
    let points_ls: Vec<_> = points.iter().map(|e| Point::new(e[0], e[1])).collect();
    let ls = LineString(points_ls);
    b.iter(||{
        ls.simplify(&0.001);
    });
}

#[bench]
fn bench_visvalingam(b: &mut Bencher) {
    let points = include!("../src/mk_route.rs");
    let points_ls: Vec<_> = points.iter().map(|e| Point::new(e[0], e[1])).collect();
    let ls = LineString(points_ls);
    b.iter(||{
        ls.simplifyvw(&0.0000075);
    });
}

#[bench]
fn bench_rdp_long(b: &mut Bencher) {
    let points = include!("../src/mk_route_long.rs");
    let points_ls: Vec<_> = points.iter().map(|e| Point::new(e[0], e[1])).collect();
    let ls = LineString(points_ls);
    b.iter(||{
        ls.simplify(&0.001);
    });
}

#[bench]
fn bench_visvalingam_long(b: &mut Bencher) {
    let points = include!("../src/mk_route_long.rs");
    let points_ls: Vec<_> = points.iter().map(|e| Point::new(e[0], e[1])).collect();
    let ls = LineString(points_ls);
    b.iter(||{
        ls.simplifyvw(&0.0000075);
    });
}
