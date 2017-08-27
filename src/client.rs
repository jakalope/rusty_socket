extern crate protobuf;

mod common;
mod foo;

use common::SOCKET_PATH;
use protobuf::Message;
use std::io::prelude::*;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

fn main() {
    let mut foo_proto = foo::Foo::new();
    foo_proto.set_id(12345i64);
    foo_proto.set_data(23456i64);

    let mut stream = UnixStream::connect(SOCKET_PATH).unwrap();
    {
        foo_proto.write_length_delimited_to_writer(
                &mut stream).unwrap();
    }

    stream.shutdown(Shutdown::Write).expect(
            "Unable to shutdown Write mode.");
    {
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        println!("{}", response);
    }
}
