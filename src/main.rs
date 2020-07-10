extern crate rust_p2p;

use rust_p2p::communication::server::Server;

pub fn main() {
    let x = Server::server();
}