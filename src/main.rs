extern crate rust_p2p;
use std::thread::sleep;
use futures::executor::block_on;
use rust_p2p::communication::server::server;
use rust_p2p::communication::client::client;
use std::{time, thread};
use futures::join;
use rust_p2p::communication::StreamHandler::main as abc;

#[tokio::main]
pub async fn main() {
    //abc();
    let s = server();
    let c = client();
    join!(s,c);
    // let server_thread = thread::spawn(|| {
    //     match server() {
    //         Ok(_) => print!("ok\n"),
    //         Err(_) => print!("notok\n"),
    //     }
    // });
    // sleep(time::Duration::from_secs(2));
    // let server_thread2 = thread::spawn(|| client());
    // println!("Join handle now");
    // server_thread.join();
    // server_thread2.join();
}
