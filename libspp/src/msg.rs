extern crate bytes;

use bytes::*;
use ::mapper::*;
use std::io;
use std::fmt;

static MAGIC: [u8; 3] = [0x53, 0x50, 0x50];

const PK_STATE_STRING: u8 = 0x01;
const PK_STATE_INTEGER: u8 = 0x02;

#[derive(Eq, PartialEq, Debug)]
pub struct SppMessage {
    pub identifier: String,
    pub payload: Vec<u8>
}

pub fn serialize(pk: &SppMessage, mapper: &SppMapper, ans: &mut BytesMut) {
    ans.extend_from_slice(&MAGIC);
    match mapper.string_to_integer(&pk.identifier) {
        Some(id) => {
            ans.put_u8(PK_STATE_INTEGER);
            ans.put_u16_be(id as u16);
        },
        None => {
            ans.put_u8(PK_STATE_STRING);
            let id = &pk.identifier;
            ans.reserve(2 + id.len()); //size of u16 + string
            ans.put_u16_be(id.len() as u16);
            ans.put_slice(id.as_ref());
        }
    }
    ans.put_u16_be(pk.payload.len() as u16);
    ans.extend_from_slice(&pk.payload);
}

#[derive(Debug)]
pub enum DeserializeError {
    NoState,
    Magic,
    Length { required: usize, actual: usize },
    WrongState { state: u8 },
    NoSuchId { id: u16 },
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializeError::NoState => write!(f, "no packet state"),
            DeserializeError::Magic => write!(f, "incorrect magic"),
            DeserializeError::Length { required, actual } =>
                write!(f, "packet length: required {}, got {}", required, actual),
            DeserializeError::WrongState { state } => write!(f, "illegal state : {}", state),
            DeserializeError::NoSuchId { id } => write!(f, "no such id : {}", id),
        }
    }
}

macro_rules! ensure_length {
    ($cur: ident, $size: expr) => {
        if $cur.remaining() < $size {
            return Err(DeserializeError::Length
            { required: $size, actual: $cur.remaining() });
        }
    };
}

pub fn deserialize(mapper: &SppMapper, cur: &mut io::Cursor<Bytes>)
    -> Result<SppMessage, DeserializeError> {
    if !validate_magic(cur) {
        return Err(DeserializeError::Magic);
    }
    if cur.remaining() < 1 {
        return Err(DeserializeError::NoState);
    }
    let string_id = match cur.get_u8() {
        PK_STATE_STRING => {
            ensure_length!(cur, 2);
            let len = cur.get_u16_be();
            ensure_length!(cur, len as usize);
            let mut string = String::with_capacity(len as usize);
            for _ in 0..len {
                string.push(cur.get_u8() as char);
            }
            string
        },
        PK_STATE_INTEGER => {
            ensure_length!(cur, 2);
            let int = cur.get_u16_be();
            let string_id;
            if let Some(string) = mapper.integer_to_string(int) {
                string_id = String::from(string);
            } else {
                return Err(DeserializeError::NoSuchId { id: int });
            }
            string_id
        },
        others => return Err(DeserializeError::WrongState { state: others }),
    };
    ensure_length!(cur, 2);
    let len = cur.get_u16_be();
    ensure_length!(cur, len as usize);
    let mut payload = Vec::with_capacity(len as usize);
    for _ in 0..len {
        payload.push(cur.get_u8());
    }
    let pk = SppMessage {
        identifier: string_id,
        payload,
    };
    Ok(pk)
}

fn validate_magic(cur: &mut io::Cursor<Bytes>) -> bool {
    if cur.remaining() < 3 {
        return false;
    }
    for i in 0usize..3 {
        if cur.get_u8() != MAGIC[i] {
            return false;
        }
    }
    return true;
}