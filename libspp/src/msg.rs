extern crate bytes;

use bytes::*;
use ::mapper::*;

static MAGIC: [u8; 3] = [0x53, 0x50, 0x50];

static PK_STATE_STRING: u8 = 0x01;
static PK_STATE_INTEGER: u8 = 0x02;

pub struct Pk<'a> {
    pub identifier: &'a str,
    pub payload: &'a [u8]
}

pub fn serialize(pk: Pk, mapper: SppMapper, ans: &mut BytesMut) {
    ans.extend_from_slice(&MAGIC);
    match mapper.string_to_integer(pk.identifier) {
        Some(id) => {
            ans.put_u8(PK_STATE_INTEGER);
            ans.put_u16_be(id as u16);
        },
        None => {
            ans.put_u8(PK_STATE_STRING);
            let id = String::from(pk.identifier);
            ans.reserve(2 + id.len()); //size of u16 + string
            ans.put_u16_be(id.len() as u16);
            ans.put_slice(id.as_ref());
        }
    }
    ans.extend_from_slice(pk.payload);
}


