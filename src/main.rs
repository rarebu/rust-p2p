extern crate rust_p2p;
use std::thread::sleep;
use futures::executor::block_on;
use rust_p2p::communication::server::server;
use rust_p2p::communication::client::client;
use std::{time, thread};

pub fn main() {
    let server_thread = thread::spawn(|| {
        match server() {
            Ok(_) => print!("ok\n"),
            Err(_) => print!("notok\n"),
            _ => {}
        }
    });
    sleep(time::Duration::from_secs(2));
    client();
    println!("main\n");
}
