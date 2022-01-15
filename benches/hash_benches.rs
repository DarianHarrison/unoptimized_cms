#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(test)] // only comes in nightly build not stable channel
// to install ```nightly rustup toolchain install nightly```
// to run benches ```rustup run nightly cargo bench```

use fasthash::{mum, metro, murmur3, sea, t1ha, xx, spooky, farm, city };
extern crate test;
use test::Bencher;

#[cfg(test)]
mod hash_benches {

    use super::*; // bring all of the test module’s parent’s items into scope with use super::*
    use test::{Bencher};

    #[bench]
    fn test_metro_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = metro::hash64(b"hello world\xff");
        });
    }

    #[bench]
    fn test_mum_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = mum::hash64(b"hello world\xff");
        });
    }

    #[bench]
    fn test_murmur3_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = murmur3::hash128(b"hello world\xff");
        });
    }

    #[bench]
    fn test_sea_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = sea::hash64(b"hello world\xff");
        });
    }

    #[bench]
    fn test_t1ha_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = t1ha::hash64(b"hello world\xff");
        });
    }

    #[bench]
    fn test_xx_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = xx::hash64(b"hello world\xff");
        });
    }
    #[bench]
    fn test_spooky_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = spooky::hash64(b"hello world\xff");
        });
    }
    #[bench]
    fn test_farm_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = farm::hash64(b"hello world\xff");
        });
    }
    #[bench]
    fn test_city_hash(b: &mut Bencher) {
        b.iter(|| {
            let h = city::hash64(b"hello world\xff");
        });
    }
}


// results
// test hash_benches::test_metro_hash   ... bench:           5 ns/iter (+/- 0)
// test hash_benches::test_t1ha_hash    ... bench:           6 ns/iter (+/- 0)
// test hash_benches::test_xx_hash      ... bench:           6 ns/iter (+/- 0
// test hash_benches::test_mum_hash     ... bench:           8 ns/iter (+/- 0)
// test hash_benches::test_murmur3_hash ... bench:           8 ns/iter (+/- 0)
// test hash_benches::test_sea_hash     ... bench:           9 ns/iter (+/- 0)
