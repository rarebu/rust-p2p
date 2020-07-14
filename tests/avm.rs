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
    let server = Server::start(ip.clone(), port);
    let mut client = Client::start();
    thread::sleep(time::Duration::from_millis(100));

    // client connecting to server
    client.connect(ip, port);

    //get list of connections
    let ccons = client.get_connections();
    let scons = server.get_connections();

    // get specific connection name
    let ccon_string: String = ccons.get(0).unwrap().to_string();
    let scon_string: String = scons.get(0).unwrap().to_string();
    println!("clients connections are: {:?}", ccons);
    println!("servers connections are: {:?}", scons);

    // get specific connection
    let ccon = client.get_connection(ccon_string).unwrap();
    let scon = server.get_connection(scon_string).unwrap();

    // client sends message
    send_message(&ccon, String::from("This is the client, anybody there?"));
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
    client.disconnect(ccon);
    println!("servers connections are:{:?}", server.get_connections());

    // server disconnects stream
    server.disconnect(scon);
    println!("clients connections are: {:?}", client.get_connections());
    println!("servers connections are: {:?}", server.get_connections());

    // shutting down client and server
    client.stop();
    server.stop();
    assert_eq!(1, 1);
}

fn send_message(stream: &StreamAccessor, content: String) {
    let sender = stream.get_local_peer();
    let receiver = stream.get_remote_peer();
    let message = Message {
        sender: sender,
        receiver: receiver,
        header_information: String::from("text_string"),
        content: content,
    };

    stream.write_message(message);
}

fn receive_message(stream: &StreamAccessor) -> String {
    format!("{:?}", stream.read_message().unwrap())
}