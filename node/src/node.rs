extern crate communication;

use communication::connectable::Connectable;
use communication::server::Server;
use communication::client::Client;
use communication::message::Message;
use communication::stream_accessor::StreamAccessor;

use std::{thread, time};

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
        let stream = self.client. //stream holen

    }

    pub fn send_message(self, content: String) {
        let content = Message::Content(content);
        let stream = self. //stream holen
        stream.write_message(content);
    }

    pub fn receive_message(self) -> String {
        let stream = self. //stream holen
        format!("{:?}", stream.read_message().unwrap())
    }

    fn search_connection(self, ip: String, port: i32) -> &StreamAccessor {
        &self.client.get_connection(self.client.get_connections().iter().find(|&&x| {
            let address: Vec<&str> = x.split(":").collect();
            address(0) == ip && address(1) == port
        }).unwrap().clone()).unwrap()
    }
}
