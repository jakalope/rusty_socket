extern crate protobuf;

use std::fs;
use std::path::Path;
use common::SOCKET_PATH;
use std::os::unix::net::{UnixStream, UnixListener};

mod foo;
mod common;

fn handle_client(mut stream: UnixStream) {
    let foo_proto: foo::Foo = protobuf::core::
        parse_length_delimited_from_reader(&mut stream).unwrap();
    let foo_str: String = protobuf::text_format::
        print_to_string(&foo_proto);
    println!("Got Foo({:?})", foo_str);
}

fn main() {
    let socket = Path::new(SOCKET_PATH);

    // Delete old socket if necessary
    if socket.exists() {
        fs::remove_file(&socket).unwrap();
    }

    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    // accept connections and process them, spawning a new thread for each one
    println!("Now accepting incoming streams.");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                println!("Connection succeeded.");
                std::thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                /* connection failed */
                println!("Connection failed: {:?}.", err);
                break;
            }
        }
    }
}
