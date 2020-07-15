use super::stream_accessor::StreamAccessor;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};
use super::connectable::Connectable;
use crate::communication_error::CommunicationError;

#[derive(Debug)]
pub struct Server {
    connected_streams: Arc<Mutex<Vec<StreamAccessor>>>,
    shutdown: Arc<AtomicBool>,
    address: String,
    handle_listener: JoinHandle<Result<(), CommunicationError>>
}

impl Connectable for Server {
    fn get_connections(&self) -> Result<Vec<String>, CommunicationError> {
        let connected_streams = self.connected_streams.lock()?;
        let mut tmp = Vec::new();
        for stream in connected_streams.iter() {
            tmp.push(stream.get_remote_peer()?);
        }
        Ok(tmp)
    }


    fn get_connection(&self, peer_address: String) -> Result<Option<StreamAccessor>, CommunicationError> {
        let connected_streams = self.connected_streams.lock()?;
        for stream in connected_streams.iter() {
            if peer_address == stream.get_remote_peer()? {
                return Ok(Option::from(stream.clone()));
            }
        }
        Ok(None)
    }
}

impl Server {
    pub fn start(ip: String, port: usize) -> Result<Server, CommunicationError> {
        let address = format!("{}:{}", ip, port);
        let streams = Arc::new(Mutex::new(Vec::new()));
        let switch = Arc::new(AtomicBool::new(false));
        Ok(Server {
            connected_streams: streams.clone(),
            shutdown: switch.clone(),
            address: address.clone(),
            handle_listener: spawn(move || -> Result<(), CommunicationError> {
                let listener = TcpListener::bind(address).map_err(|_| CommunicationError::new("Server could not bind address".to_string()))?;
                for stream in listener.incoming() {
                    if switch.load(Ordering::SeqCst) {
                        break;
                    }
                    let stream = stream.map_err(|_| CommunicationError::new("Incoming stream corrupted".to_string()))?;
                    let mut streams = streams.lock()?;
                    streams.push(StreamAccessor::new(stream)?);
                }
                Ok(())
            }),
        })
    }

    pub fn stop(self) -> Result<(), CommunicationError> {
        //send poison pill
        self.shutdown.store(true, Ordering::SeqCst);
        let address = self.address.clone();
        TcpStream::connect(address).map_err(|_| CommunicationError::new("Client could not connect stream".to_string()))?;

        //close all streams
        let connected_streams = self.connected_streams.lock()?;
        loop {
            if connected_streams.len() > 0 {
                self.close_disconnect(true)?;
            } else { break; }
        }

        //shut down listener thread
        self.handle_listener.join().map_err(|_| CommunicationError::new("Server could not join handle".to_string()))??;
        Ok(())
    }

    pub fn disconnect(&self, connection: StreamAccessor, send_all: bool) -> Result<(), CommunicationError> {
        let mut connected_streams = self.connected_streams.lock()?;
        let index= connected_streams.iter().position(|stream| stream.equals(&connection)
            .unwrap_or(false)).ok_or(CommunicationError::new("Stream could not be found".to_string()))?;
        connected_streams.remove(index);
        connection.close(send_all)?;
        Ok(())
    }

    fn close_disconnect(&self, send_all: bool) -> Result<(), CommunicationError> {
        let mut connected_streams = self.connected_streams.lock()?;
        let stream = connected_streams.pop().ok_or(CommunicationError::new("Connection could not be found".to_string()))?;
        stream.close(send_all)?;
        Ok(())
    }
}
