use std::net::TcpStream;
use std::thread::JoinHandle;
use std::io::{Read, Write};
use std::thread::spawn;
use std::collections::VecDeque;
use std::time::Duration;
use super::message::Message;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};

#[derive(Debug)]
pub struct StreamHandler {
    pub local_peer: String,
    pub remote_peer: String,
    to_send: Arc<Mutex<VecDeque<Message>>>,
    received: Arc<Mutex<VecDeque<Message>>>,
    shutdown: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>
}

impl StreamHandler {
    pub fn new(mut stream: TcpStream) -> StreamHandler {
        let sends = Arc::new(Mutex::new(VecDeque::new()));
        let receives = Arc::new(Mutex::new(VecDeque::new()));
        let switch = Arc::new(AtomicBool::new(false));
        let stream_peer = stream.peer_addr().unwrap();
        let stream_loc = stream.local_addr().unwrap();
        StreamHandler {
            local_peer: format!("{}:{}", stream_loc.ip(), stream_loc.port()),
            remote_peer: format!("{}:{}", stream_peer.ip(), stream_peer.port()),
            to_send: sends.clone(),
            received: receives.clone(),
            shutdown: switch.clone(),
            handle: Option::from(spawn(move || {
                loop {
                    //set timeout to avoid blocking loop
                    stream.set_read_timeout(Option::from(Duration::new(0,100000000))).unwrap();
                    //read operation
                    let mut buffer = [0u8; 4096];
                    let count = match stream.read(&mut buffer) {
                        Ok(count) => count,
                        Err(_) => 0
                    };
                    //read successful?
                    if count > 0 {
                        let msg: Message = bincode::deserialize(&buffer).unwrap();
                        {
                            let mut receives = receives.lock().unwrap();
                            receives.push_back(msg);
                        }
                    }
                    //write operation
                    {
                    let mut sends = sends.lock().unwrap();
                    if !sends.is_empty() {
                        let message = sends.pop_front().unwrap();
                        let bytes = bincode::serialize(&message).unwrap();
                        stream.write_all(bytes.as_slice()).unwrap();
                    }}
                    //shutdown option
                    if switch.load(Ordering::SeqCst) {
                        break;
                    }
                }

            }))
        }
    }

    pub fn get_message(&mut self) -> Option<Message> {
        let mut received = self.received.lock().unwrap();
        received.pop_front()
    }

    pub fn send_message(&mut self, message: Message) {
        let mut to_send = self.to_send.lock().unwrap();
        to_send.push_back(message);
    }

    pub fn close_stream(self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.handle.unwrap().join().unwrap();
    }
}