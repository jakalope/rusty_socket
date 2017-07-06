all: listener

listener: src/foo.rs
	rustc src/listener.rs

src/foo.rs:
	protoc --rust_out src/ src/foo.proto

clean:
	rm src/foo.rs
