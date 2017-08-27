extern crate protobuf;

mod foo;

use protobuf::Message;
use common::SOCKET_PATH;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

mod common;

fn main() {
    let mut foo_proto = foo::Foo::new();
    foo_proto.set_id(12345i64);
    foo_proto.set_data(23456i64);

    let mut stream = UnixStream::connect(SOCKET_PATH).unwrap();

    {
        let mut coded_stream = protobuf::CodedOutputStream::new(&mut stream);
        foo_proto.write_to_with_cached_sizes(&mut coded_stream).unwrap();
    } {
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        println!("{}", response);
    }
}
