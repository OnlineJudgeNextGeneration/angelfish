extern crate bytes;
extern crate libbedrock;
use bytes::{BytesMut, IntoBuf};

#[test]
fn test_magic() {
    use libbedrock::packet::*;

    let mut bm = BytesMut::with_capacity(16);
    bm.put_pk_magic();
    assert_eq!(
        bm.as_mut(),
        b"\0\xff\xff\0\xfe\xfe\xfe\xfe\xfd\xfd\xfd\xfd\x124Vx"
    );
    let mut cur = bm.freeze().into_buf();
    assert!(cur.validate_pk_magic());
}
