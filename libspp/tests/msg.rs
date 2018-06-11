extern crate libspp;
extern crate bytes;

use libspp::msg::SppMessage;
use bytes::*;

#[test]
fn test_serialize() {
    let pk = SppMessage {
        identifier: String::from("ec:gui_define"),
        payload: vec![0xea, 0x5e, 0xca, 0x71, 0x02],
    };
    let b = vec![
        0x53, 0x50, 0x50,
        0x01,
        0x00, 0x0d,
        0x65, 0x63, 0x3a, 0x67, 0x75, 0x69, 0x5f, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65,
        0x00, 0x05,
        0xea, 0x5e, 0xca, 0x71, 0x02
    ];
    test_encode_decode(pk, b);
}

#[cfg(test)]
fn test_encode_decode(a: SppMessage, b: std::vec::Vec<u8>) {
    let mapper = libspp::mapper::new();
    let mut ans = BytesMut::new();
    libspp::msg::serialize(&a, &mapper, &mut ans);
    assert_eq!(ans.to_vec(), b);
    let mut ans = Bytes::from(ans).into_buf();
    let c = libspp::msg::deserialize(&mapper, &mut ans).unwrap();
    assert_eq!(c, a);
}