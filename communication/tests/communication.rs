//mod node;

extern crate communication;

use communication::connectable::Connectable;
use communication::server::Server;
use communication::client::Client;
use communication::message::Message;
use communication::stream_accessor::StreamAccessor;

use std::{thread, time};


#[test]
fn integration_test() {
    let ip = String::from("127.0.0.1");
    let port = 11224;

    // start server with listener and client
    let server = Server::start(ip.clone(), port).unwrap();
    let mut client = Client::start();
    let mut client2 = Client::start();
    thread::sleep(time::Duration::from_millis(100));

    // client connecting to server
    client.connect(ip.clone(), port).unwrap();
    client2.connect(ip, port).unwrap();

    //get list of connections
    let ccons = client.get_connections().unwrap();
    let ccons2 = client2.get_connections().unwrap();
    let scons = server.get_connections().unwrap();

    // get specific connection name
    let ccon_string: String = ccons.get(0).unwrap().to_string();
    let ccon_string2: String = ccons2.get(0).unwrap().to_string();
    let scon_string: String = scons.get(0).unwrap().to_string();
    let scon_string2: String = scons.get(1).unwrap().to_string();
    println!("clients connections are: {:?}", ccons);
    // println!("clients2 connections are: {:?}", ccons2);
    println!("servers connections are: {:?}", scons);

    // get specific connection
    let ccon = client.get_connection(ccon_string).unwrap().unwrap();
    let ccon2 = client2.get_connection(ccon_string2).unwrap().unwrap();
    let scon = server.get_connection(scon_string).unwrap().unwrap();
    let scon2 = server.get_connection(scon_string2).unwrap().unwrap();

    // client sends message
    send_message(&ccon, String::from("This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybThis is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?This is the client, anybody thasdfere?"));
    // send_message(&ccon, String::from("This is the client, anybode there?"));
    thread::sleep(time::Duration::from_millis(1000));

    // server receives message
    let a = receive_message(&scon);
    println!("Server received from client:\n{}", a);

    // server answers
    send_message(&scon, String::from("This is the server, yes of course!"));
    thread::sleep(time::Duration::from_millis(1000));

    // client receives answer
    let b = receive_message(&ccon);
    println!("Client received from server:\n{}", b);

    // consume accessor to drop arc reference
//    ccon.consume();
//    scon.consume();

    // client disconnects stream
    println!("clients connections are1: {:?}", client.get_connections().unwrap());
    println!("clients2 connections are2: {:?}", client2.get_connections().unwrap());
    client2.disconnect(ccon2, true).unwrap();
    println!("clients2 connections are3: {:?}", client2.get_connections().unwrap());
    client.disconnect(ccon, true).unwrap();
    println!("clients connections are4: {:?}", client.get_connections().unwrap());

    // server disconnects stream
    server.disconnect(scon, false).unwrap();
    server.disconnect(scon2, false).unwrap();
    println!("clients connections are: {:?}", client.get_connections().unwrap());
    println!("servers connections are: {:?}", server.get_connections().unwrap());

    // shutting down client and server
    client.stop().unwrap();
    server.stop().unwrap();
    println!("Bye");
}

fn send_message(stream: &StreamAccessor, content: String) {
    let content = Message::Content(content);
    stream.write_message(content).unwrap();
}

fn receive_message(stream: &StreamAccessor) -> String {
    format!("{:?}", stream.read_message().unwrap())
}