extern crate node;
use std::{thread, time};

use node::node::Node;

fn main() {
    println!("Hello world");
    let mut node = Node::new(String::from("192.168.1.101"), 15000).unwrap();
    node.connect(String::from("192.168.1.119"), 15000).unwrap();
    node.send_message("Hey this is a message from host machine".to_string(), String::from("192.168.1.119"), 15000).unwrap();
    thread::sleep(time::Duration::from_secs(20));
    node.shutdown().unwrap();

}