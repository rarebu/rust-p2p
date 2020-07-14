use tokio::net::TcpListener;
use tokio::prelude::*;
use super::message::Message;
use crate::communication::message::Message::HeaderSize;


// #[tokio::main]
pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    println!("server\n");
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut read = 0;
            let mut size_header = 0;

            // In a loop, read data from the socket and write the data back.
            loop {
                println!("vv");
                let n = match socket.read(&mut buf).await {
                    // socket closed                    0 <= n <= buf.len()
                    Ok(n) if n == 0 => {
                        println!("AAA");
                        if size_header < 1024 {
                            //größeren buffer
                            let (mut size, message) = handle_message(&buf, &mut size_header);

                            match message {
                                Message::HeaderSize(_) => {
                                    println!("AAAX");
                                    let ack = Message::Ack(true);
                                    let bytes = bincode::serialize(&ack).unwrap();
                                    socket.write_all(bytes.as_slice()).await;
                                },
                                _ => (),
                            }

                            while size != read {
                                let (mut size2, message) = handle_message(&buf[size..], &mut size_header);
                                size = size + size2;
                                match message {
                                    Message::HeaderSize(_) => {
                                        println!("AAAC");
                                        let ack = Message::Ack(true);
                                        let bytes = bincode::serialize(&ack).unwrap();
                                        socket.write_all(bytes.as_slice()).await;
                                    },
                                    _ => (),
                                }
                            }
                        }
                        read = 0;
                        return
                    },
                    Ok(n) => read = {
                        println!("n is: {}", n);
                        n
                    },
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                //if let Err(e) = socket.write_all(&buf[0..n]).await {
                let message = Message::Body(String::from("Hey test message"));
                let bytes = bincode::serialize(&message).unwrap();
                // handle_message(&buf);

                socket.write_all(bytes.as_slice());

                println!("got message, wrote back")
            }

        });
    }
}

pub fn handle_message(buf: &[u8], size_header: &mut usize) -> (usize, Message) {
    let msg: Message = bincode::deserialize(&buf).unwrap();
    let bytes = bincode::serialize(&msg).unwrap();
    println!("HEE {:?}", msg);
    match &msg {
        Message::HeaderSize(size) => *size_header = *size,
        Message::Body(body) => println!("{}", body),
        _ => (),
    }
    (bytes.len(), msg)
}

