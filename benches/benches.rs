#![feature(test)]

extern crate test;
use test::Bencher;
extern crate rdp;
use rdp::rdp;
use rdp::visvalingam;

#[bench]
fn bench_rdp(b: &mut Bencher) {
    let points = include!("../src/mk_route.rs");
    b.iter(||{
        rdp(&points, &0.001);
    });
}

#[bench]
fn bench_visvalingam(b: &mut Bencher) {
    let points = include!("../src/mk_route.rs");
    b.iter(||{
        visvalingam(&points, &0.0000075);
    });
}
