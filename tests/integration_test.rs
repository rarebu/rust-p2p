extern crate ipaddress;
use std::net::Ipv4Addr;
use ipaddress::IPAddress;

#[test]
fn create_node() {
    let node = Node::new();
    assert_eq!(node.is_running, true)
}

fn try_connection() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let node = Node::new();
    node.connect(ip);
    assert_eq!(node.is_connected, true)
}