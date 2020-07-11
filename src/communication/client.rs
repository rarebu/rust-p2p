use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::net::lookup_host;
use super::message::Message;

#[tokio::main]
pub async fn client() {
    println!("client\n");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("created stream");

    let message = Message::Body(String::from("Hey i am client"));
    let bytes = bincode::serialize(&message).unwrap();
    stream.write(bytes.as_slice()).await;
    println!("wrote to stream; \n");
}