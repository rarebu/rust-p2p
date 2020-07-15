use super::stream_accessor::StreamAccessor;
use std::net::TcpStream;
use super::connectable::Connectable;
use crate::communication_error::CommunicationError;

#[derive(Debug)]
pub struct Client {
    connected_streams: Vec<StreamAccessor>
}

impl Connectable for Client {
    fn get_connections(&self) -> Result<Vec<String>, CommunicationError> {
        let mut tmp = Vec::new();
        for stream in self.connected_streams.iter() {
            tmp.push(stream.get_remote_peer()?);
        }
        Ok(tmp)
    }

    fn get_connection(&self, peer_address: String) -> Result<Option<StreamAccessor>, CommunicationError> {
        for stream in self.connected_streams.iter() {
            if peer_address == stream.get_remote_peer()? {
                return Ok(Option::from(stream.clone()));
            }
        }
        Ok(None)
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
                self.close_disconnect(true).unwrap();
            } else { break; }
        }
    }

    pub fn connect(&mut self, ip: String, port: usize) -> Result<(), CommunicationError> {

        let address = format!("{}:{}", ip, port);
        let stream = TcpStream::connect(address).unwrap();
        self.connected_streams.push(StreamAccessor::new(stream)?);
        Ok(())
    }


    pub fn disconnect(&mut self, connection: StreamAccessor, send_all: bool)  -> Result<(), CommunicationError> {
        let index = self.connected_streams.iter().position(|stream| stream.equals(&connection)
            .unwrap_or(false)).ok_or(CommunicationError::new("Stream could not be found".to_string()))?;
        self.connected_streams.remove(index);
        connection.close(send_all);
        Ok(())
    }

    fn close_disconnect(&mut self, send_all: bool) -> Result<(), CommunicationError> {
        let stream = self.connected_streams.pop().ok_or(CommunicationError::new("Connection could not be found".to_string()))?;
        stream.close(send_all);
        Ok(())
    }
}
