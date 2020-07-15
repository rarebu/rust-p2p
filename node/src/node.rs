extern crate communication;

use communication::connectable::Connectable;
use communication::server::Server;
use communication::client::Client;
use communication::message::Message;
use communication::stream_accessor::StreamAccessor;
use std::error::Error;
use crate::node_error::NodeError;


use std::{thread, time};
// use std::fmt::Error;

pub struct Node {
    client: Client,
    server: Server,
}

impl Node {
    pub fn new(ip: String, port: usize) -> Result<Node, NodeError> {
        // start server with listener and client
        Ok(Node {
            client: Client::start(),
            server: Server::start(ip, port)?,
        })

    }

    pub fn connect(&mut self, ip: String, port: usize) -> Result<(), NodeError> {
        match self.search_connection(ip.clone(), port)? {
            None => self.client.connect(ip.clone(), port)?,
            _ => {
                println!("Drop reference to arc({}:{})", ip, port);
                return Err(NodeError::stream_allready_exists_error);
            },
        }
        Ok(())
    }

    pub fn disconnect(&mut self, ip: String, port: usize) -> Result<(), NodeError> {
        //sende disconnect message mit: close_stream_and_send_all_messages(self)
        // let stream =

        let stream = self.search_connection(ip.clone(), port)?.ok_or(NodeError::stream_not_exists_anymore_error)?; //existiert stream? wenn ja hole ihn

        let ccons = self.client.get_connections()?;


        let pattern = format!("{}:{}", ip, port);

        let stream_in_client = ccons.iter().find(|connection| *connection == &pattern).is_some();
        if stream_in_client {
            self.client.disconnect(stream, true);
        } else {
            self.server.disconnect(stream, true);
        }
        Ok(())
    }

    pub fn send_message(&self, content: String, ip: String, port: usize) -> Result<(), NodeError>{
        let content = Message::Content(content);
        let stream = self.search_connection(ip.clone(), port)?.ok_or(NodeError::stream_not_exists_anymore_error)?; //stream holen
        stream.write_message(content);
        Ok(())
    }

    pub fn receive_message_from_peer(&self, ip: String, port: usize) -> Result<String, NodeError> {
        // reagiere auf disconnect und schliesse stream mit: close_stream(self)

        let stream = self.search_connection(ip.clone(), port)?.ok_or(NodeError::stream_not_exists_anymore_error)?; //stream holen
        loop {
            let message = stream.read_message()?.ok_or(NodeError::no_message_received_error)?;//crash
            match message {
                Message::Disconnect => {
                    stream.close(false);
                    println!("Drop reference to arc({}:{})", ip, port);
                    return Err(NodeError::stream_not_exists_anymore_error);
                },
                Message::Content(message) => {
                    println!("Drop reference to arc({}:{})", ip, port);
                    return Ok(format!("{:?}", message));
                },
            }
        }
    }

    pub fn get_connected_peers(&self) -> Result<Vec<String>, NodeError>{
        let mut connections = self.server.get_connections()?;
        connections.extend(self.client.get_connections()?);
        Ok(connections)
    }

    pub fn shutdown(self) {
        self.server.stop();
        self.client.stop();
    }

    fn search_connection(&self, ip: String, port: usize) -> Result<Option<StreamAccessor>, NodeError> {
        let x = Node::search_into_connactable(&self.client, &ip, port)?;
        if let None = x {
            return Node::search_into_connactable(&self.server, &ip, port);
        }
        Ok(x)
    }

    fn search_into_connactable(connectable: &Connectable, ip: &str, port: usize) -> Result<Option<StreamAccessor>, NodeError>{
        let tmp = connectable.get_connections()?.iter().find(|x| {
            let address: Vec<&str> = x.split(":").collect();
            address.get(0).unwrap() == &ip && address.get(1).unwrap() == &port.to_string()
        }).map(|f|f.to_string());
        let v = tmp.map(|x| connectable.get_connection(x.clone()).unwrap().unwrap());
        // println!("v is defined: {}", v.is_some());
        Ok(v)
    }
}
