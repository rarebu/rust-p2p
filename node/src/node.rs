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
    pub fn new(ip: String, port: usize) -> Node {
        // start server with listener and client
        Node {
            client: Client::start(),
            server: Server::start(ip, port),
        }

    }

    pub fn connect(&mut self, ip: String, port: usize) -> Result<(), &'static str> {
        match self.search_connection(ip.clone(), port) {
            None => self.client.connect(ip.clone(), port),
            _ => {
                println!("Drop reference to arc({}:{})", ip, port);
                return Err("Stream does already exist!")
            },
        }
        Ok(())
    }

    pub fn disconnect(&mut self, ip: String, port: usize) {
        //sende disconnect message mit: close_stream_and_send_all_messages(self)
        // let stream =

        let stream = self.search_connection(ip.clone(), port).unwrap(); //existiert stream? wenn ja hole ihn

        let ccons = self.client.get_connections();


        let pattern = format!("{}:{}", ip, port);

        let stream_in_client = ccons.iter().find(|connection| *connection == &pattern).is_some();
        if stream_in_client {
            self.client.disconnect(stream, true);
        } else {
            self.server.disconnect(stream, true);
        }
        println!("Drop reference to arc({}:{})", ip, port);
    }

    pub fn send_message(&self, content: String, ip: String, port: usize) {
        let content = Message::Content(content);
        let stream = self.search_connection(ip.clone(), port).unwrap(); //stream holen
        stream.write_message(content);
        println!("Drop reference to arc({}:{})", ip, port);
    }

    pub fn receive_message_from_peer(&self, ip: String, port: usize) -> Result<String, &'static str> {
        // reagiere auf disconnect und schliesse stream mit: close_stream(self)

        let stream = self.search_connection(ip.clone(), port).unwrap(); //stream holen
        loop {
            let message = stream.read_message().unwrap();//crash
            match message {
                Message::Disconnect => {
                    stream.close(false);
                    println!("Drop reference to arc({}:{})", ip, port);
                    return Err("Stream closed!");
                },
                Message::Content(message) => {
                    println!("Drop reference to arc({}:{})", ip, port);
                    return Ok(format!("{:?}", message));
                },
            }
        }
    }

    pub fn get_connected_peers(&self) -> Vec<String>{
        let mut connections = self.server.get_connections();
        connections.extend(self.client.get_connections());
        connections
    }

    pub fn shutdown(self) {
        self.server.stop();
        self.client.stop();
    }

    fn search_connection(&self, ip: String, port: usize) -> Option<StreamAccessor> {
        let x = Node::search_into_connactable(&self.client, &ip, port);
        if let None = x {
            return Node::search_into_connactable(&self.server, &ip, port);
        }
        x
    }

    fn search_into_connactable(connectable: &Connectable, ip: &str, port: usize) -> Option<StreamAccessor>{
        let tmp = connectable.get_connections().iter().find(|x| {
            let address: Vec<&str> = x.split(":").collect();
            address.get(0).unwrap() == &ip && address.get(1).unwrap() == &port.to_string()
        }).map(|f|f.to_string());
        let v = tmp.map(|x| connectable.get_connection(x.clone()).unwrap());
        // println!("v is defined: {}", v.is_some());
        v
    }
}
