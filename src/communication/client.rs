use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::net::lookup_host;

#[tokio::main]
pub async fn client() {
    println!("client\n");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("created stream");

    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());
}