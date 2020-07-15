extern crate node;

use node::node::Node;

use std::{thread, time};


#[test]
fn integration_test() {
    let localhost = String::from("127.0.0.1");
    let mut node1 = Node::new(localhost.clone(), 11000);
    let mut node2 = Node::new(localhost.clone(), 12000);

    node1.connect(localhost.clone(), 12000);
    println!("Node1: {:?}", node1.get_connected_peers());
    println!("Node2: {:?}", node2.get_connected_peers());
//     let ip = String::from("127.0.0.1");
//     let port = 11224;
//
//     // start server with listener and client
//     let server = Server::start(ip.clone(), port);
//     let mut client = Client::start();
//     let mut client2 = Client::start();
//     thread::sleep(time::Duration::from_millis(100));
//
//     // client connecting to server
//     client.connect(ip.clone(), port);
//     client2.connect(ip, port);
//
//     //get list of connections
//     let ccons = client.get_connections();
//     let ccons2 = client2.get_connections();
//     let scons = server.get_connections();
//
//     // get specific connection name
//     let ccon_string: String = ccons.get(0).unwrap().to_string();
//     let ccon_string2: String = ccons2.get(0).unwrap().to_string();
//     let scon_string: String = scons.get(0).unwrap().to_string();
//     let scon_string2: String = scons.get(1).unwrap().to_string();
//     println!("clients connections are: {:?}", ccons);
//     // println!("clients2 connections are: {:?}", ccons2);
//     println!("servers connections are: {:?}", scons);
//
//     // get specific connection
//     let ccon = client.get_connection(ccon_string).unwrap();
//     let ccon2 = client2.get_connection(ccon_string2).unwrap();
//     let scon = server.get_connection(scon_string).unwrap();
//     let scon2 = server.get_connection(scon_string2).unwrap();
//
//     // client sends message
//     send_message(&ccon, String::from("This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybThis is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?"));
//     // send_message(&ccon, String::from("This is the client, anybode there?"));
//     thread::sleep(time::Duration::from_millis(1000));
//
//     // server receives message
//     let a = receive_message(&scon);
//     println!("Server received from client:\n{}", a);
//
//     // server answers
//     send_message(&scon, String::from("This is the server, yes of course!"));
//     thread::sleep(time::Duration::from_millis(1000));
//
//     // client receives answer
//     let b = receive_message(&ccon);
//     println!("Client received from server:\n{}", b);
//
//     // consume accessor to drop arc reference
// //    ccon.consume();
// //    scon.consume();
//
//     // client disconnects stream
//     println!("clients connections are1: {:?}", client.get_connections());
//     println!("clients2 connections are2: {:?}", client2.get_connections());
//     client2.disconnect(ccon2, true);
//     println!("clients2 connections are3: {:?}", client2.get_connections());
//     client.disconnect(ccon, true);
//     println!("clients connections are4: {:?}", client.get_connections());
//
//     // server disconnects stream
//     server.disconnect(scon, false);
//     server.disconnect(scon2, false);
//     println!("clients connections are: {:?}", client.get_connections());
//     println!("servers connections are: {:?}", server.get_connections());
//
//     // shutting down client and server
//     client.stop();
//     server.stop();
//     println!("Bye");
}
