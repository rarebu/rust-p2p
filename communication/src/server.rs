use super::stream_accessor::StreamAccessor;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};
use super::connectable::Connectable;

#[derive(Debug)]
pub struct Server {
    connected_streams: Arc<Mutex<Vec<StreamAccessor>>>,
    shutdown: Arc<AtomicBool>,
    address: String,
    handle_listener: JoinHandle<()>
}

impl Connectable for Server {
    fn get_connections(&self) -> Vec<String> {
        let connected_streams = self.connected_streams.lock().unwrap();
        let mut tmp = Vec::new();
        for stream in connected_streams.iter() {
            //let stream = stream.lock().unwrap();
            tmp.push(stream.get_remote_peer().clone());
        }
        tmp
    }


    fn get_connection(&self, peer_address: String) -> Option<StreamAccessor> {
        println!("Pass reference to arc({}) outside", peer_address);
        let connected_streams = self.connected_streams.lock().unwrap();
        for stream in connected_streams.iter() {
            if peer_address == stream.get_remote_peer() {
                return Option::from(stream.clone());
            }
        }
        None
    }
}

impl Server {
    pub fn start(ip: String, port: usize) -> Server {
        let address = format!("{}:{}", ip, port);
        let streams = Arc::new(Mutex::new(Vec::new()));
        let switch = Arc::new(AtomicBool::new(false));
        Server {
            connected_streams: streams.clone(),
            shutdown: switch.clone(),
            address: address.clone(),
            handle_listener: spawn(move || {
                let listener = TcpListener::bind(address).unwrap();
                for stream in listener.incoming() {
                    if switch.load(Ordering::SeqCst) {
                        break;
                    }
                    let stream = stream.unwrap();
                    let mut streams = streams.lock().unwrap();
                    // let v = stream.local_addr().unwrap().port();
                    // println!("V: {}", v);
                    streams.push(StreamAccessor::new(stream));
                }
            })
        }
    }

    pub fn stop(self) {
        //send poison pill
        self.shutdown.store(true, Ordering::SeqCst);
        let address = self.address.clone();
        TcpStream::connect(address).unwrap();

        //close all streams
        let connected_streams = self.connected_streams.lock().unwrap();
        loop {
            if connected_streams.len() > 0 {
                self.close_disconnect(true);
            } else { break; }
        }

        //shut down listener thread
        self.handle_listener.join().unwrap();
    }

    pub fn disconnect(&self, connection: StreamAccessor, send_all: bool) {
        let mut connected_streams = self.connected_streams.lock().unwrap();
        let index= connected_streams.iter().position(|stream| stream.equals(&connection)).unwrap();
        connected_streams.remove(index);
        connection.close(send_all);
    }

    fn close_disconnect(&self, send_all: bool) {
        let mut connected_streams = self.connected_streams.lock().unwrap();
        let stream = connected_streams.pop().unwrap();
        stream.close(send_all);
    }
}
