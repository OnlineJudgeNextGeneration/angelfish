//! A chat server that broadcasts a message to all connections.
//!
//! This example is explicitly more verbose than it has to be. This is to
//! illustrate more concepts.
//!
//! A chat server for telnet clients. After a telnet client connects, the first
//! line should contain the client's name. After that, all lines sent by a
//! client are broadcasted to all other connected clients.
//!
//! Because the client is telnet, lines are delimited by "\r\n".
//!
//! You can test this out by running:
//!
//!     cargo run --example chat
//!
//! And then in another terminal run:
//!
//!     telnet localhost 6142
//!
//! You can run the `telnet` command in any number of additional windows.
//!
//! You can run the second command in multiple windows and then chat between the
//! two, seeing the messages from the other client as they're received. For all
//! connected clients they'll all join the same room and see everyone else's
//! messages.

#![deny(deprecated)]

extern crate tokio;
extern crate futures;
extern crate bytes;
extern crate libspp;

use tokio::net::{TcpListener};
use tokio::prelude::*;
use libspp::prelude::*;
use std::io;

fn new_spp_handle<'a, R, W>(reader: R, writer: W) -> SppHandle<'a, R, W>
    where R: AsyncRead,
          W: AsyncWrite {
    SppHandle {
        reader,
        writer,
        mapper: libspp::mapper::new(),
    }
}

#[derive(Debug)]
pub struct SppHandle<'a, R, W> {
    reader: Option<R>,
    writer: Option<W>,
    mapper: SppMapper<'a>
}

impl<R, W> Future for SppHandle<R, W>
    where R: AsyncRead,
          W: AsyncWrite,
{
    type Item = (R, W);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {

        }
    }
}


#[test]
pub fn main() {
    let address = "127.0.0.1:6142".parse().unwrap();

    let listener = TcpListener::bind(&address).unwrap();
    println!("server running on localhost:6142");

    let server = listener.incoming().for_each(move |stream| {

        let address = stream.peer_addr().unwrap();
        println!("New connection from {} ", address);

        let (reader, writer) = stream.split();

        tokio::spawn(
            tokio::io::copy(reader, writer).map(|amt| {
                println!("wrote {:?} bytes", amt)
            }).map_err(|err| {
                eprintln!("IO error {:?}", err)
            })
        );

        Ok(())
    })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    tokio::run(server);

}
