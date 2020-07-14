use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::net::lookup_host;
use super::message::Message;
use std::thread::sleep;
use std::time;

// #[tokio::main]
pub async fn client() {
    println!("client\n");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("created stream");

    let message = Message::Body(String::from("Hey i am cliegggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggfgdahaedhadfghajhagnjafgnargdfjnafgdhnfghgfvhafgmnarfgtjnafgjhnafghbafgnbbadbgadghbnargdfhafdghafgnagdfnganhfgagrfhafdghadfgnagnadfghahdgfhdgfahtahtgrttarjhaetafhafdhhafehafdhefdafdfgshtaadfhafdafdahgdfasgsdafhsadufhsadjfghasdujfhaskdufhasdkjhfgsadjghaksdufjhgkajsdhgfkjsadhfasdkjhfaksjdhfaskdjhfsakdjfhasdkjfhasdkjfhaskjdfhadskjfhaskjhgfsakjdghasdkjghkasdjfghasdkfjghasdkxujfhasuidfhisuadjhfna,jvchakujdfshauosdfhkiausdhfgaskdufhadskfhadksufhaksudfhaskdjfhaskudhfgksadujfhasdukfhaskjdfhaskdjfhaskdujfhaksdjfhaskdjfghksduljghaksdughasdkfujghaskdUJGHALDSKGHFDSAKULFUHASDKJGFHBASDKJFGHSADKUJFHASKDUJFHASDKUGHAEKSDRFUJHGFAIUERHFKAUSDJHFKUASLDFHAESDFRASTGfdsafsdafsdasgfdgffhgsdfrsdfgretrdtrqtrerzearethaeraghffgadfdfgcgrdfgfdagfdghfdgdgfdgfdhgfhgdfhfdhdfghdgdhgfhdfjhatjarjatrdjatrejhteztehatehathathathdhttdhadhfyahgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggndt"));
    let message2 = Message::Body(String::from("Hey i am client"));

    let mut buf = [0; 1024];

    let bytes = bincode::serialize(&message).unwrap();
    let header = Message::HeaderSize(bytes.len());
    let bytes2 = bincode::serialize(&message2).unwrap();
    let bytes3 = bincode::serialize(&header).unwrap();
    let headerbytes = bincode::serialize(&message).unwrap();
    stream.write_all(bytes3.as_slice()).await;
    sleep(time::Duration::from_secs(2));
    println!("Length bytes: {}", bytes.len());
    loop {

    }
    // loop {
    //     println!("a");
    //     match stream.read(&mut buf).await {
    //         Ok(n) if n == 0 => {
    //             let mut a:usize = 0;
    //             let (a, v) = super::server::handle_message(&buf, &mut a);
    //             match v {
    //                 Message::Ack(message_type) => {
    //                     if message_type {
    //                         break;
    //                     }
    //                 },
    //                 _ => (),
    //             }
    //         },
    //         Ok(n) => println!("{}", n),
    //         Err(e) => {
    //             eprintln!("failed to read from socket; err = {:?}", e);
    //             return;
    //         }
    //     }
    // }
    // let mut buf = [0; 1024];
    //
    // stream.write_all(bytes.as_slice()).await;
    // // stream.write_all(bytes2.as_slice()).await;
    // println!("wrote to stream; \n");
}