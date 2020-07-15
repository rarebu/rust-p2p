use std::net::TcpStream;
use std::thread::JoinHandle;
use std::io::{Read, Write};
use std::thread::spawn;
use std::collections::VecDeque;
use std::time::Duration;
use super::message::{Message, Message2};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};
use crate::communication_error::CommunicationError;

static PANIC_MESSAGE_THREAD: &str = "Fatal error, Message-Queue should contain elements after not empty check!";

#[derive(Debug)]
pub struct StreamHandler {
    pub local_peer: String,
    pub remote_peer: String,
    to_send: Arc<Mutex<VecDeque<Message>>>,
    received: Arc<Mutex<VecDeque<Message>>>,
    shutdown: Arc<AtomicBool>,
    write_all_shutdown: Arc<AtomicBool>,
    handle: Option<JoinHandle<Result<(), CommunicationError>>>
}

impl StreamHandler {
    pub fn new(mut stream: TcpStream) -> Result<StreamHandler, CommunicationError> {
        let sends = Arc::new(Mutex::new(VecDeque::new()));
        let receives = Arc::new(Mutex::new(VecDeque::new()));
        let shutdown_switch = Arc::new(AtomicBool::new(false));
        let write_all_shutdown_switch = Arc::new(AtomicBool::new(false));
        let stream_peer = stream.peer_addr().map_err(|_| CommunicationError::new("StreamHandler: peer_address lookup failed!".to_string()))?;
        let stream_loc = stream.local_addr().map_err(|_| CommunicationError::new("StreamHandler: local_address lookup failed!".to_string()))?;
        let mut header_information: Option<usize> = None;
        let mut buf_vec: Vec<u8> = Vec::new();
            Ok(StreamHandler {
            local_peer: format!("{}:{}", stream_loc.ip(), stream_loc.port()),
            remote_peer: format!("{}:{}", stream_peer.ip(), stream_peer.port()),
            to_send: sends.clone(),
            received: receives.clone(),
            shutdown: shutdown_switch.clone(),
            write_all_shutdown: write_all_shutdown_switch.clone(),
            handle: Option::from(spawn(move || {
                loop {
                    //set timeout to avoid blocking loop
                    stream.set_read_timeout(Option::from(Duration::new(0,100000000))).map_err(|err| CommunicationError::new(err.to_string()))?;
                    //read operation
                    let mut buffer = [0u8; 4096];
                    let count = match stream.read(&mut buffer) {
                        Ok(count) => count,
                        Err(_) => 0
                    };
                    //read successful?
                    if count > 0 {
                        if header_information.is_none() {
                            let header_msg: Message2 = bincode::deserialize(&buffer[..])?;
                            match header_msg {
                                Message2::Header(s) => header_information = Some(s),
                                _ => (),
                            }
                            // ack absenden
                            continue;
                        } else {
                            let vec_tmp: Vec<u8> = buffer.iter().cloned().collect();
                            buf_vec.extend(vec_tmp.iter().cloned());
                            if header_information.unwrap() > count { //hier ist unnwrap explizit erlaubt, da durch else sichergestellt ist, dass kein None vorhanden
                                header_information = Some(header_information.unwrap() - count);
                                continue;
                            } else {
                                let msg: Message = bincode::deserialize(&buf_vec[..])?;
                                {
                                    let mut receives = receives.lock()?;
                                    receives.push_back(msg);
                                    header_information = None;
                                }
                            }
                        }
                    }
                    //write operation
                    let mut closure_serialize = |content| -> Result<(), CommunicationError>{

                        let content_message = bincode::serialize(&content)?;
                        let header = Message2::Header(content_message.len());
                        let header_message = bincode::serialize(&header)?;

                        // println!("Header: {:?}", header);
                        // println!("Content: {:?}", content);

                        stream.write_all(header_message.as_slice()).map_err(|_| CommunicationError::new_write_error())?;
                        stream.write_all(content_message.as_slice()).map_err(|_| CommunicationError::new_write_error())?;
                        Ok(())
                    };
                    {
                    let mut sends = sends.lock()?;
                    if !sends.is_empty() {
                        let content = sends.pop_front().expect(PANIC_MESSAGE_THREAD);
                        closure_serialize(content);
                    }
                    }
                    //shutdown option
                    if shutdown_switch.load(Ordering::SeqCst) {
                        if write_all_shutdown_switch.load(Ordering::SeqCst) {
                            let mut sends = sends.lock()?;
                            loop {
                                let tmp = sends.pop_back();
                                match tmp {
                                    Some(message) => closure_serialize(message)?,
                                    None => break,
                                }
                            }
                        }
                        //über boolean steuern ob noch alle nachrichten gesendet werden müssen
                        break;
                    }
                }
                Ok(())
            }))
        })
    }

    pub fn get_message(&mut self) -> Result<Option<Message>, CommunicationError> {
        let mut received = self.received.lock()?;
        Ok(received.pop_front())
    }

    pub fn send_message(&mut self, message: Message) -> Result<(), CommunicationError> {
        let mut to_send = self.to_send.lock()?;
        to_send.push_back(message);
        Ok(())
    }

    pub fn close_stream_and_send_all_messages(self) {

        //alle messages die in der queue sind sollten gesendet werde, damit close message gesendet wird
        // hier boolean setzen
        self.write_all_shutdown.store(true, Ordering::SeqCst);
        self.close_stream();
    }

    pub fn close_stream(self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.handle.unwrap().join().expect(PANIC_MESSAGE_THREAD); //First unwrap should succeed everytime
    }
}