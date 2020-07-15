use super::stream_accessor::StreamAccessor;
use std::net::TcpStream;
use super::connectable::Connectable;

#[derive(Debug)]
pub struct Client {
    connected_streams: Vec<StreamAccessor>
}

impl Connectable for Client {
    fn get_connections(&self) -> Vec<String> {
        let mut tmp = Vec::new();
        for stream in self.connected_streams.iter() {
            tmp.push(stream.get_remote_peer());
        }
        tmp
    }

    fn get_connection(&self, peer_address: String) -> Option<StreamAccessor> {
        println!("Pass reference to arc({}) outside", peer_address);
        for stream in self.connected_streams.iter() {
            if peer_address == stream.get_remote_peer() {
                return Option::from(stream.clone());
            }
        }
        None
    }
}


impl Client {
    pub fn start() -> Client {
        Client {
            connected_streams: Vec::new()
        }
    }

    pub fn stop(mut self) {
        loop {
            if self.connected_streams.len() > 0 {
                self.close_disconnect(true);
            } else { break; }
        }
    }

    pub fn connect(&mut self, ip: String, port: usize) {

        let address = format!("{}:{}", ip, port);
        let stream = TcpStream::connect(address).unwrap();
        self.connected_streams.push(StreamAccessor::new(stream));
    }


    pub fn disconnect(&mut self, connection: StreamAccessor, send_all: bool) {
        let index = self.connected_streams.iter().position(|stream| stream.equals(&connection)).unwrap();
        self.connected_streams.remove(index);
        connection.close(send_all);
    }

    fn close_disconnect(&mut self, send_all: bool) {
        let stream = self.connected_streams.pop().unwrap();
        stream.close(send_all);
    }
}
