#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    loop {
        stream
            .read(&mut [0; 128])
            .map(|_| stream.write_all(b"+PONG\r\n"))
            .map_err(|err| {
                println!("error: {:?}", err);
                stream.shutdown(Shutdown::Both)
            })
            .expect("Connection failed to drop")
            .ok();
    }
}

fn main() {
    let addr = "127.0.0.1:6379";

    TcpListener::bind(addr)
        .expect("TCP port should be available")
        .incoming()
        .for_each(|stream| match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("error: {}", e),
        });
}
