extern crate bimap;
extern crate bytes;

pub mod mapper;
pub mod msg;
pub mod net;

pub mod prelude {
    pub use ::mapper::SppMapper;
    pub use ::msg::{SppMessage, serialize, deserialize, DeserializeError};
}