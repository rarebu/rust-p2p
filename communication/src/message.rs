use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message2 {
    // Sender(String),
    // Receiver(String),
    Header(usize),
    Content(Message),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Content(String),
    Disconnect(String),
}
