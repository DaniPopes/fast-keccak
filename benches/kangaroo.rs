#![feature(test)]

extern crate test;

use fast_keccak::{Hasher, KangarooTwelve};
use test::Bencher;

#[bench]
fn bench_k12(b: &mut Bencher) {
    let data = [0u8; 32];
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res = [0u8; 32];
        let mut k12 = KangarooTwelve::new(&[]);
        k12.update(&data);
        k12.finalize(&mut res);
    });
}
