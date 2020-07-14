// extern crate communication;
//
// use communication::connectable::Connectable;
// use communication::server::Server;
// use communication::client::Client;
// use communication::message::Message;
// use communication::stream_accessor::StreamAccessor;
//
// use std::{thread, time};
//
// pub struct Node {
//     client: Client,
//     server: Server,
// }
//
// impl Node {
//     pub fn new(ip: String, port: i32) -> Node {
//         // start server with listener and client
//         Node {
//             client: Client::start(),
//             server: Server::start(ip, port),
//         }
//
//     }
//
//     pub fn connect(mut self, ip: String, port: i32) {
//         self.client.connect(ip.clone(), port);
//     }
//
//     pub fn disconnect(self, ip: String, port: i32) {
//         let stream = self.search_connection(ip, port).unwrap(); //stream holen
//
//     }
//
//     pub fn send_message(self, content: String, ip: String, port: i32) {
//         let content = Message::Content(content);
//         let stream = self.search_connection(ip, port).unwrap(); //stream holen
//         stream.write_message(content);
//     }
//
//     pub fn receive_message(self, ip: String, port: i32) -> String {
//         let stream = self.search_connection(ip, port).unwrap(); //stream holen
//         format!("{:?}", stream.read_message().unwrap())
//     }
//
//     fn search_connection(self, ip: String, port: i32) -> Option<StreamAccessor> {
//         let x = Node::search_into_connactable(&self.client, &ip, port);
//         if let None = x {
//             return Node::search_into_connactable(&self.server, &ip, port);
//         }
//         x
//     }
//
//     fn search_into_connactable(connectable: &Connectable, ip: &str, port: i32) -> Option<StreamAccessor>{
//         connectable.get_connections().iter().find(|x| {
//             let address: Vec<&str> = x.split(":").collect();
//             address.get(0).unwrap() == &ip && address.get(1).unwrap() == &port.to_string()
//         }).map(|x| connectable.get_connection(x.clone()).unwrap())
//     }
// }
