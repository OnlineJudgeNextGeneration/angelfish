#![feature(test)]
extern crate test;
extern crate libspp;

use test::*;

#[bench]
fn mapper_define(b: &mut Bencher) {
    let mut mapper = libspp::new_mapper();
    b.iter(|| {
        mapper.define_string_id("test");
    })
}

#[bench]
fn mapper_get(b: &mut Bencher) {
    let mut mapper = libspp::new_mapper();
    b.iter(|| {
        mapper.define_string_id("test");
    })
}

#[bench]
fn xor_100_integers(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..100 {
            let a = test::black_box(123235456);
            let b = test::black_box(123123123);
            let _ = a^b;
        }
    });
}