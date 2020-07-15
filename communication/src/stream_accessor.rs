use super::message::Message;
use super::stream_handler::StreamHandler;
use std::sync::{Mutex, Arc};
use std::net::TcpStream;
use crate::communication_error::CommunicationError;

#[derive(Debug, Clone)]
pub struct StreamAccessor {
    stream: Arc<Mutex<StreamHandler>>
}

impl StreamAccessor {
    pub fn new(stream: TcpStream) -> Result<StreamAccessor, CommunicationError> {
        Ok(StreamAccessor {
            stream: Arc::new(Mutex::new(StreamHandler::new(stream)?)),
        })
    }

    pub fn get_local_peer(&self) -> Result<String, CommunicationError> {
        let stream = self.stream.lock()?;
        Ok(stream.local_peer.clone())
    }

    pub fn get_remote_peer(&self) -> Result<String, CommunicationError> {
        let stream = self.stream.lock()?; //wieder 1
        Ok(stream.remote_peer.clone())
    }

    pub fn write_message(&self, message: Message) -> Result<(), CommunicationError>{
        let mut stream = self.stream.lock()?; //hier wird strong 2
        stream.send_message(message);
        Ok(())
    }

    pub fn read_message(&self) -> Result<Option<Message>, CommunicationError> {
        let mut stream = self.stream.lock()?; //hier strong 2
        stream.get_message()
    }

    pub fn close(self, send_all: bool) {
        let stream = Arc::try_unwrap(self.stream).unwrap(); //hier auch 2
        let stream = stream.into_inner().unwrap();
        if send_all {
            stream.close_stream_and_send_all_messages();
        } else {
            stream.close_stream();
        }
    }

    pub fn equals(&self, other: &StreamAccessor) -> Result<bool, CommunicationError> {
        Ok(self.get_remote_peer()? == other.get_remote_peer()?)
    }

    #[allow(dead_code)]
    pub fn consume_reference(self) {

    }
}