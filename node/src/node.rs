extern crate communication;

use communication::connectable::Connectable;
use communication::server::Server;
use communication::client::Client;
use communication::message::Message;
use communication::stream_accessor::StreamAccessor;
use std::error::Error;


use std::{thread, time};
// use std::fmt::Error;

pub struct Node {
    client: Client,
    server: Server,
}

impl Node {
    pub fn new(ip: String, port: i32) -> Node {
        // start server with listener and client
        Node {
            client: Client::start(),
            server: Server::start(ip, port),
        }

    }

    pub fn connect(mut self, ip: String, port: i32) {
        self.client.connect(ip.clone(), port);
    }

    pub fn disconnect(self, ip: String, port: i32) {
        //sende disconnect message mit: close_stream_and_send_all_messages(self)
        let stream = self.search_connection(ip, port).unwrap(); //stream holen
        stream.close(true);
    }

    pub fn send_message(self, content: String, ip: String, port: i32) {
        let content = Message::Content(content);
        let stream = self.search_connection(ip, port).unwrap(); //stream holen
        stream.write_message(content);
    }

    pub fn receive_message(self, ip: String, port: i32) -> Result<String, &'static str> {
        // reagiere auf disconnect und schliesse stream mit: close_stream(self)

        let stream = self.search_connection(ip, port).unwrap(); //stream holen
        loop {
            let message = stream.read_message().unwrap();
            match message {
                Message::Disconnect => {
                    stream.close(false);
                    return Err("Stream closed!");
                },
                Message::Content(message) => {

                    return Ok(format!("{:?}", message));
                },
            }
        }
    }

    fn search_connection(self, ip: String, port: i32) -> Option<StreamAccessor> {
        let x = Node::search_into_connactable(&self.client, &ip, port);
        if let None = x {
            return Node::search_into_connactable(&self.server, &ip, port);
        }
        x
    }

    fn search_into_connactable(connectable: &Connectable, ip: &str, port: i32) -> Option<StreamAccessor>{
        connectable.get_connections().iter().find(|x| {
            let address: Vec<&str> = x.split(":").collect();
            address.get(0).unwrap() == &ip && address.get(1).unwrap() == &port.to_string()
        }).map(|x| connectable.get_connection(x.clone()).unwrap())
    }
}
