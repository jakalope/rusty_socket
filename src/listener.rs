extern crate protobuf;

use std::os::unix::net::{UnixStream, UnixListener};

mod foo;

fn handle_client(mut stream: UnixStream) {
    let foo_proto: foo::Foo = protobuf::parse_from_reader(&mut stream).unwrap();
    let foo_str: String = protobuf::text_format::print_to_string(&foo_proto);
    println!("Got Foo({:?})", foo_str);
}

fn main() {
    let listener = UnixListener::bind("/tmp/rust_listener_00").unwrap();

    // accept connections and process them, spawning a new thread for each one
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
