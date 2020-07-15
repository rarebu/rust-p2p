use node::node::Node;

use std::{thread, time};


fn main() {
    let localhost = String::from("127.0.0.1");
    let mut node1 = Node::new(localhost.clone(), 11000).unwrap(); //client

    let mut node2 = Node::new(localhost.clone(), 12000).unwrap(); //server

    println!("Hallo");
    node1.connect(localhost.clone(), 12000).unwrap(); //node1 wird zum client
    println!("Node1: {:?}", node1.get_connected_peers().unwrap());
    println!("Node2: {:?}", node2.get_connected_peers().unwrap());
    // let vec:Vec<&String> = node2.get_connected_peers().get(0).unwrap().split(":").collect();
    let connected_peers = node2.get_connected_peers().unwrap();

    let tmp = connected_peers;
    let address: Vec<&str> = tmp.get(0).unwrap().split(":").collect();

    let port: usize = address.get(1).unwrap().parse().unwrap();

    node1.send_message(String::from("Hey this is a message by Node 1"), localhost.clone(), 12000).unwrap();
    println!("Node1: {:?}", node1.get_connected_peers());
    println!("Node2: {:?}", node2.get_connected_peers());
    println!("Port: {}", port);

    thread::sleep(time::Duration::from_millis(1000));
    let msg = node2.receive_message_from_peer(localhost.clone(), port).unwrap();

    node2.send_message(String::from("Hey this is a message by Node 2"), localhost.clone(), port).unwrap();

    println!("Node2 received: {}", msg);
    thread::sleep(time::Duration::from_millis(1000));
    let msg = node1.receive_message_from_peer(localhost.clone(), 12000).unwrap();
    println!("Node1 received: {}", msg);

    println!("Node1: {:?}", node1.get_connected_peers());
    println!("Node2: {:?}", node2.get_connected_peers());


    thread::sleep(time::Duration::from_millis(1000));
    println!("node2 disconnects node1");

    node2.disconnect(localhost.clone(), port).unwrap();


    println!("Jetzt könnte es aufgrund unserer fehlenden Fehlerbehandlung krachen");
    node1.receive_message_from_peer(localhost.clone(), 12000).unwrap_err();
    node1.receive_message_from_peer(localhost.clone(), 12000).unwrap_err();
    node1.receive_message_from_peer(localhost.clone(), 12000).unwrap_err();
    println!("Jetzt sollten alle streams geschlossen sein!");
    println!("Node1: {:?}", node1.get_connected_peers());
    println!("Node2: {:?}", node2.get_connected_peers());

}