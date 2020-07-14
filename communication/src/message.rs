use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    // Sender(String),
    // Receiver(String),
    Header(usize),
    Content(String),
}
