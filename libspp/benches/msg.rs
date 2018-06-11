#![feature(test)]
extern crate test;
extern crate libspp;
extern crate bytes;

use test::*;
use libspp::msg::*;
use bytes::*;

#[bench]
fn encode(b: &mut Bencher) {
    let mapper = libspp::mapper::new();
    b.iter(|| {
        let mut ans = BytesMut::new();
        let pk = SppMessage {
            identifier: String::from("ec:gui_define"),
            payload: vec![0xea, 0x5e, 0xca, 0x71, 0x02],
        };
        libspp::msg::serialize(&pk, &mapper, &mut ans);
    });
}

#[bench]
fn decode(b: &mut Bencher) {
    let mapper = libspp::mapper::new();
    b.iter(|| {
        let mut ans = Bytes::from(vec![
            0x53, 0x50, 0x50,
            0x01,
            0x00, 0x0d,
            0x65, 0x63, 0x3a, 0x67, 0x75, 0x69, 0x5f, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65,
            0x00, 0x05,
            0xea, 0x5e, 0xca, 0x71, 0x02
        ]).into_buf();
        libspp::msg::deserialize(&mapper, &mut ans).unwrap();
    });
}
