extern crate bytes;
extern crate std;

use std::net;
// use std::sync::atomic;
use std::io;
use std::convert;
use std::fmt;
use std::io::Read;
use bytes::*;

/*
 *
 * ===== Types of bedrock packet (outer frame, or "rak net") =====
 *
 */

type PkId = u8;
type GUID = u64;
type MTU = i16;
type PlayerCount = i32;
type BedrockNetVer = i32;
type PingPongTime = u64;
type BedrockGameVer = String;
type SocketAddress = net::SocketAddr;

//#[derive(Debug)]
//struct ContentReliable(Index);
//
//#[derive(Debug)]
//enum ContentType {
//    Ordered {
//        ordered_frame_index: Index,
//        order_channel: u8,
//    },
//    OrderedAndSequenced {
//        ordered_frame_index: Index,
//        order_channel: u8,
//        sequenced_frame_index: Index,
//    },
//    Fragmented {
//        compound_size: i32,
//        compound_id: i16,
//        index: i32,
//    },
//}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Pk {
    //    ConnectedPing {
    //        ping_time: PingPongTime,
    //    },
    UnconnectedPing {
        ping_time: PingPongTime,
    },
    //    UnconnectedPingOpenConnections {
    //        ping_time: PingPongTime,
    //    },
    //    ConnectedPong {
    //        ping_time: PingPongTime,
    //        pong_time: PingPongTime,
    //    },
    OfflineConnectionRequest1 {
        protocol_version: u8,
    },
    OfflineConnectionResponse1 {
        server_id: GUID,
        mtu: MTU,
    },
    OfflineConnectionRequest2 {
        server_address: SocketAddress,
        mtu: MTU,
        client_id: GUID,
    },
    OfflineConnectionResponse2 {
        server_id: GUID,
        client_address: SocketAddress,
        mtu: MTU,
    },
    OnlineConnectionRequest {
        client_id: GUID,
        ping_time: PingPongTime,
    },
    //    OnlineConnectionRequestAccepted {
    //        client_address: SocketAddress,
    //        ping_time: PingPongTime,
    //        pong_time: PingPongTime,
    //    },
    UnconnectedPong {
        ping_time: PingPongTime,
        server_id: GUID,
        msg_of_today: String,
        online_count: PlayerCount,
        max_count: PlayerCount,
        bedrock_net_ver: BedrockNetVer,
        bedrock_game_ver: BedrockGameVer,
    },
    //    FrameSet {
    //        index: Index,
    //        content_reliable: ContentReliable,
    //        content_type: ContentType,
    //    },
    //    Game, //consider which type of [] to use
    //    AcknowledgeWithRange {
    //        record_count: i16,
    //        start_index: Index,
    //        end_index: Index,
    //    },
    //    AcknowledgeWithoutRange {
    //        record_count: i16,
    //        index: Index,
    //    },
}

//const ID_CONNECTED_PING: PkId = 0x00;
const ID_UNCONNECTED_PING: PkId = 0x01;
//const ID_UNCONNECTED_PING_OPEN_CONNECTIONS: PkId = 0x02;
//const ID_CONNECTED_PONG: PkId = 0x03;
const ID_OFFLINE_CONNECTION_REQUEST_1: PkId = 0x05;
const ID_OFFLINE_CONNECTION_RESPONSE_1: PkId = 0x06;
const ID_OFFLINE_CONNECTION_REQUEST_2: PkId = 0x07;
const ID_OFFLINE_CONNECTION_RESPONSE_2: PkId = 0x08;
const ID_ONLINE_CONNECTION_REQUEST: PkId = 0x09;
//const ID_ONLINE_CONNECTION_REQUEST_ACCEPTED: PkId = 0x10;
const ID_UNCONNECTED_PONG: PkId = 0x1c;
//const ID_FRAME_SET: (PkId, PkId) = (0x80, 0x8d);
//const ID_GAME: PkId = 0x8e;
//const ID_NACK: PkId = 0xa0;
//const ID_ACK: PkId = 0xc0;

/*
 *
 * ===== Serializers and deserializers =====
 *
 */

pub trait PkPut {
    fn reserve_pk_size(&mut self, size: usize);

    fn put_pk_id(&mut self, pk_id: PkId);

    fn put_pk_ping_pong_time(&mut self, time: PingPongTime);

    fn put_pk_guid(&mut self, guid: GUID);

    fn put_pk_mtu(&mut self, mtu: MTU);

    fn put_pk_address(&mut self, address: SocketAddress);

    fn put_pk_string(&mut self, string: String);

    fn put_pk_magic(&mut self);
}

impl PkPut for BytesMut {
    #[inline]
    fn reserve_pk_size(&mut self, size: usize) {
        self.reserve(size);
    }

    #[inline]
    fn put_pk_id(&mut self, pk_id: PkId) {
        self.put_u8(pk_id);
    }

    #[inline]
    fn put_pk_ping_pong_time(&mut self, time: PingPongTime) {
        self.put_u64_be(time);
    }

    #[inline]
    fn put_pk_guid(&mut self, guid: GUID) {
        self.put_u64_be(guid);
    }

    #[inline]
    fn put_pk_mtu(&mut self, mtu: MTU) {
        self.put_i16_be(mtu);
    }

    #[inline]
    fn put_pk_address(&mut self, address: SocketAddress) {
        match address {
            net::SocketAddr::V4(v4) => {
                self.reserve_pk_size(1 + 4 + 2);
                self.put_u8(0x04);
                self.put_slice(&v4.ip().octets());
            }
            net::SocketAddr::V6(v6) => {
                self.reserve_pk_size(1 + 16 + 2);
                self.put_u8(0x06);
                self.put_slice(&v6.ip().octets());
            }
        }
        self.put_u16_be(address.port());
    }

    #[inline]
    fn put_pk_string(&mut self, string: String) {
        self.reserve(2 + string.len()); //size of u16 + string
        self.put_u16_be(string.len() as u16);
        self.put_slice(string.as_ref());
    }

    #[inline]
    fn put_pk_magic(&mut self) {
        self.extend_from_slice(&MAGIC);
    }
}

impl convert::From<Pk> for Bytes {
    fn from(pk: Pk) -> Bytes {
        let mut ans = BytesMut::new();
        serialize(pk, &mut ans);
        ans.freeze()
    }
}

fn serialize(pk: Pk, ans: &mut BytesMut) {
    match pk {
        Pk::UnconnectedPing { ping_time } => {
            ans.put_pk_id(ID_UNCONNECTED_PING);
            ans.put_pk_ping_pong_time(ping_time);
            ans.put_pk_magic();
        }
        Pk::UnconnectedPong {
            ping_time,
            server_id,
            msg_of_today,
            online_count,
            max_count,
            bedrock_net_ver,
            bedrock_game_ver,
        } => {
            ans.put_pk_id(ID_UNCONNECTED_PONG);
            ans.put_pk_ping_pong_time(ping_time);
            ans.put_pk_guid(server_id);
            ans.put_pk_magic();
            ans.put_pk_string(format!(
                "MCPE;{};{};{};{};{}",
                msg_of_today, bedrock_net_ver, bedrock_game_ver, online_count, max_count
            ));
        }
        Pk::OfflineConnectionRequest1 { protocol_version } => {
            ans.put_pk_id(ID_OFFLINE_CONNECTION_REQUEST_1);
            ans.put_pk_magic();
            ans.put_u8(protocol_version);
            ans.reserve_pk_size(46);
            ans.put_slice(
                &[ // 我数了数，正好46个 // might change in the future (udp v6)
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            );
        }
        Pk::OfflineConnectionResponse1 { server_id, mtu } => {
            ans.put_pk_id(ID_OFFLINE_CONNECTION_RESPONSE_1);
            ans.put_pk_magic();
            ans.put_pk_guid(server_id);
            ans.put_u8(0); // use security is always 0
            ans.put_pk_mtu(mtu);
        }
        Pk::OfflineConnectionRequest2 {
            server_address,
            mtu,
            client_id,
        } => {
            ans.put_pk_id(ID_OFFLINE_CONNECTION_REQUEST_2);
            ans.put_pk_magic();
            ans.put_pk_address(server_address);
            ans.reserve_pk_size(2 + 8);
            ans.put_pk_mtu(mtu);
            ans.put_pk_guid(client_id);
        }
        Pk::OfflineConnectionResponse2 {
            server_id,
            client_address,
            mtu,
        } => {
            ans.put_pk_id(ID_OFFLINE_CONNECTION_RESPONSE_2);
            ans.put_pk_magic();
            ans.reserve_pk_size(8);
            ans.put_pk_guid(server_id);
            ans.put_pk_address(client_address);
            ans.reserve_pk_size(2 + 1);
            ans.put_pk_mtu(mtu);
            ans.put_u8(0); // encryption enabled
        }
        Pk::OnlineConnectionRequest {
            client_id,
            ping_time,
        } => {
            ans.put_pk_id(ID_ONLINE_CONNECTION_REQUEST);
            ans.put_pk_guid(client_id);
            ans.put_pk_ping_pong_time(ping_time);
        }
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    Header,
    Id(PkId),
    Length { required: usize, actual: usize },
    Magic,
    IpProto { proto: u8 },
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializeError::Header => write!(f, "no header found"),
            DeserializeError::Id(id) => write!(f, "unsupported packet id: {}", id),
            DeserializeError::Length { required, actual } => {
                write!(f, "packet length: required {}, got {}", required, actual)
            }
            DeserializeError::Magic => write!(f, "incorrect magic"),
            DeserializeError::IpProto { proto } => write!(f, "ip version: {}", proto),
        }
    }
}

pub trait PkGet {
    fn pk_remaining(&mut self) -> usize;

    fn get_pk_id(&mut self) -> PkId;

    fn get_pk_ping_pong_time(&mut self) -> PingPongTime;

    fn get_pk_guid(&mut self) -> GUID;

    fn get_pk_mtu(&mut self) -> MTU;

    fn try_get_pk_address(&mut self) -> std::result::Result<SocketAddress, DeserializeError>;

    fn validate_pk_magic(&mut self) -> bool;
}

impl PkGet for io::Cursor<Bytes> {
    #[inline]
    fn pk_remaining(&mut self) -> usize {
        self.remaining()
    }

    #[inline]
    fn get_pk_id(&mut self) -> PkId {
        self.get_u8()
    }

    #[inline]
    fn get_pk_ping_pong_time(&mut self) -> PingPongTime {
        self.get_u64_be()
    }

    #[inline]
    fn get_pk_guid(&mut self) -> GUID {
        self.get_u64_be()
    }

    #[inline]
    fn get_pk_mtu(&mut self) -> MTU {
        self.get_i16_be()
    }

    #[inline]
    fn try_get_pk_address(&mut self) -> std::result::Result<SocketAddress, DeserializeError> {
        macro_rules! check_remaining {
            ($cur: ident, $size: expr) => {
                if $cur.pk_remaining() < $size {
                    return Err(DeserializeError::Length
                    { required: $size, actual: $cur.pk_remaining() });
                }
            };
        };
        check_remaining!(self, 1);
        let ip_addr: net::IpAddr;
        match self.get_u8() {
            0x04 => {
                check_remaining!(self, 4 + 2);
                ip_addr = net::IpAddr::V4(net::Ipv4Addr::new(
                    self.get_u8(),
                    self.get_u8(),
                    self.get_u8(),
                    self.get_u8(),
                ));
            }
            0x06 => {
                check_remaining!(self, 8 + 2);
                ip_addr = net::IpAddr::V6(net::Ipv6Addr::new(
                    self.get_u16_be(),
                    self.get_u16_be(),
                    self.get_u16_be(),
                    self.get_u16_be(),
                    self.get_u16_be(),
                    self.get_u16_be(),
                    self.get_u16_be(),
                    self.get_u16_be(),
                ));
            }
            proto => return Err(DeserializeError::IpProto { proto }),
        };
        let port = self.get_u16_be();
        Ok(net::SocketAddr::new(ip_addr, port))
    }

    #[inline]
    fn validate_pk_magic(&mut self) -> bool {
        if self.remaining() < 16 {
            return false;
        }
        for i in 0usize..16 {
            if self.get_u8() != MAGIC[i] {
                return false;
            }
        }
        return true;
    }
}

macro_rules! check_remaining {
    ($cur: ident, $size: expr) => {
        if $cur.pk_remaining() < $size {
            return Err(DeserializeError::Length
            { required: $size, actual: $cur.pk_remaining() });
        }
    };
}

macro_rules! check_magic {
    ($cur: ident) => {
        if !$cur.validate_pk_magic() { return Err(DeserializeError::Magic); }
    };
}

pub trait IntoPk {
    fn try_into_pk(self) -> Result<Pk, DeserializeError>;
}

impl IntoPk for Bytes {
    fn try_into_pk(self) -> Result<Pk, DeserializeError> {
        let mut cur = self.into_buf();
        deserialize(&mut cur)
    }
}

fn deserialize(cur: &mut io::Cursor<Bytes>) -> Result<Pk, DeserializeError> {
    if cur.pk_remaining() < 1 {
        return Err(DeserializeError::Header);
    }
    let pk_id: PkId = cur.get_u8();
    match pk_id {
        ID_UNCONNECTED_PING => {
            check_remaining!(cur, 8);
            let ping_time = cur.get_pk_ping_pong_time();
            check_magic!(cur);
            Ok(Pk::UnconnectedPing { ping_time })
        }
        ID_UNCONNECTED_PONG => {
            check_remaining!(cur, 8 + 8 + 16 + 2);
            let ping_time = cur.get_pk_ping_pong_time();
            let server_id = cur.get_pk_guid();
            check_magic!(cur);
            check_remaining!(cur, 2);
            let len = cur.get_u16_be() as usize;
            let mut string = String::new();
            let r = cur.reader().read_to_string(&mut string).unwrap_or(0usize);
            if r != len {
                return Err(DeserializeError::Length {
                    required: len,
                    actual: r,
                });
            }
            let v: Vec<&str> = string.split(';').collect::<Vec<&str>>();
            let msg_of_today = v.get(1).unwrap_or(&"0").to_string();
            let bedrock_net_ver = v.get(2).unwrap_or(&"0").parse::<i32>().unwrap_or(0);
            let bedrock_game_ver = v.get(3).unwrap_or(&"0").to_string();
            let online_count = v.get(4).unwrap_or(&"0").parse::<i32>().unwrap_or(0);
            let max_count = v.get(5).unwrap_or(&"0").parse::<i32>().unwrap_or(0);
            Ok(Pk::UnconnectedPong {
                ping_time,
                server_id,
                msg_of_today,
                online_count,
                max_count,
                bedrock_net_ver,
                bedrock_game_ver,
            })
        }
        ID_OFFLINE_CONNECTION_REQUEST_1 => {
            check_magic!(cur);
            check_remaining!(cur, 1);
            let protocol_version = cur.get_u8();
            Ok(Pk::OfflineConnectionRequest1 { protocol_version })
        }
        ID_OFFLINE_CONNECTION_RESPONSE_1 => {
            check_magic!(cur);
            check_remaining!(cur, 8 + 1 + 1);
            let server_id = cur.get_pk_guid();
            cur.get_u8(); // ignore security
            let mtu = cur.get_pk_mtu();
            Ok(Pk::OfflineConnectionResponse1 { server_id, mtu })
        }
        ID_OFFLINE_CONNECTION_REQUEST_2 => {
            check_magic!(cur);
            let server_address: SocketAddress;
            match cur.try_get_pk_address() {
                Ok(a) => server_address = a,
                Err(e) => return Err(e),
            }
            check_remaining!(cur, 2 + 8);
            let mtu = cur.get_pk_mtu();
            let client_id = cur.get_pk_guid();
            Ok(Pk::OfflineConnectionRequest2 {
                server_address,
                mtu,
                client_id,
            })
        }
        ID_OFFLINE_CONNECTION_RESPONSE_2 => {
            check_magic!(cur);
            check_remaining!(cur, 8);
            let server_id = cur.get_pk_guid();
            let client_address: SocketAddress;
            match cur.try_get_pk_address() {
                Ok(a) => client_address = a,
                Err(e) => return Err(e),
            }
            check_remaining!(cur, 2);
            let mtu = cur.get_pk_mtu();
            Ok(Pk::OfflineConnectionResponse2 {
                server_id,
                client_address,
                mtu,
            })
        }
        _ => Err(DeserializeError::Id(pk_id)),
    }
}


/*
 *
 * ===== Bedrock net index (unsigned int24 little endian) =====
 *
 */

// #[derive(Debug)]
// struct Index {
//     inner: atomic::AtomicUsize,
// }

// impl Index {
//     #[inline]
//     fn new_zero() -> Index {
//         Index { inner: atomic::AtomicUsize::new(0) }
//     }
// }

// trait IndexWrite {
//     fn write_index_inc(&mut self, index: &mut Index);
// }

// trait IndexRead {
//     fn read_index(&mut self) -> Index;
// }

// impl IndexWrite for BytesMut {
//     #[inline]
//     fn write_index_inc(&mut self, index: &mut Index) {
//         let n = index.inner.fetch_add(1, atomic::Ordering::SeqCst) as u32;
//         self.put_slice(&[
//             (n & 0xff) as u8,
//             ((n >> 8) & 0xff) as u8,
//             ((n >> 16) & 0xff) as u8] as &[u8]);
//     }
// }

// impl IndexRead for io::Cursor<BytesMut> {
//     #[inline]
//     fn read_index(&mut self) -> Index {
//         let n: usize = (
//             (self.get_u8() as u32) +
//                 ((self.get_u8() as u32) << 8) +
//                 ((self.get_u8() as u32) << 16)
//         ) as usize;
//         Index { inner: atomic::AtomicUsize::new(n) }
//     }
// }

// #[test]
// fn test_index_write() {
//     use pk::IndexWrite;
//     use pk::Index;

//     let mut i = Index::new_zero();
//     let i = &mut i;
//     let mut bm = BytesMut::with_capacity(450000);
//     for _ in 0..150000 { bm.write_index_inc(i); }
//     assert_eq!(*bm.get(449997).unwrap(), 0xefu8);
//     assert_eq!(*bm.get(449998).unwrap(), 0x49u8);
//     assert_eq!(*bm.get(449999).unwrap(), 0x02u8);
// }

// #[test]
// fn test_index_read() {
//     use pk::IndexRead;
//     use bytes::IntoBuf;
//     use std::sync::atomic;

//     let a = BytesMut::from(vec![0xefu8, 0x49u8, 0x02u8]);
//     let mut a = a.into_buf();
//     let i = a.read_index();
//     let n = i.inner.load(atomic::Ordering::SeqCst);
//     assert_eq!(n, 149999);
// }

/*
 *
 * ===== Magic value =====
 *
 */

pub static MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78
];
