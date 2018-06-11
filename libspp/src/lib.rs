extern crate bimap;
extern crate bytes;
extern crate core;

pub mod mapper;
pub mod msg;

pub mod prelude {
    pub use ::mapper::SppMapper;
    pub use ::msg::{SppMessage, serialize, deserialize, DeserializeError};
}