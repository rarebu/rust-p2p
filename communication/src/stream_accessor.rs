use super::message::Message;
use super::stream_handler::StreamHandler;
use std::sync::{Mutex, Arc};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct StreamAccessor {
    stream: Arc<Mutex<StreamHandler>>
}

impl StreamAccessor {
    pub fn new(stream: TcpStream) -> StreamAccessor {
        StreamAccessor {
            stream: Arc::new(Mutex::new(StreamHandler::new(stream)))
        }
    }

    pub fn get_local_peer(&self) -> String {
        let stream = self.stream.lock().unwrap();
        stream.local_peer.clone()
    }

    pub fn get_remote_peer(&self) -> String {
        let stream = self.stream.lock().unwrap(); //wieder 1
        stream.remote_peer.clone()
    }

    pub fn write_message(&self, message: Message) {
        let mut stream = self.stream.lock().unwrap(); //hier wird strong 2
        stream.send_message(message);
    }

    pub fn read_message(&self) -> Option<Message> {
        let mut stream = self.stream.lock().unwrap(); //hier strong 2
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

    pub fn equals(&self, other: &StreamAccessor) -> bool{
        self.get_remote_peer() == other.get_remote_peer()
    }

    #[allow(dead_code)]
    pub fn consume_reference(self) {

    }
}