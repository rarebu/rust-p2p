extern crate rust_p2p;
use std::thread::sleep;

use rust_p2p::communication::server::Server;

pub fn main() {
    let x = Server::start();

    print!("main");
    sleep(std::time::Duration::from_secs(5));
}