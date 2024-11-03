use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let addr = addr.parse::<SocketAddr>().expect("Not a valid IP address");

    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("TCP port {} should be available", addr.port()).as_str());

    loop {
        listener
            .accept()
            .await
            .map(|(stream, _)| tokio::spawn(handle_connection(stream)))
            .map_err(|err| eprintln!("error: {:?}", err))
            .ok()
            .take();
    }
}

async fn handle_connection(mut stream: TcpStream) {
    loop {
        match stream.read(&mut [0; 128]).await {
            // Connection closed
            Ok(0) => break,
            // Request received.
            Ok(_) => stream
                .write_all(b"+PONG\r\n")
                .await
                .expect("response should've been sent"),
            // Error while reading.
            Err(err) => {
                println!("error: {:?}", err);
                stream
                    .shutdown()
                    .await
                    .expect("connection should've shutdown gracefully");
                break;
            }
        }
    }
}
