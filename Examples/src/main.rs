extern crate node;
extern crate pnet;

use pnet::datalink;
use node::node::Node;

fn main() {
    println!("Hello world");
    let node = Node::new(String::from("192.168.1.101"), 15000).unwrap();
    node.shutdown();

    // for iface in datalink::interfaces() {
    //     println!("{:?}", iface.ips);
    // }
}