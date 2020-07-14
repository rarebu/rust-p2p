use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    HeaderSize(usize),  // True
    Body(String),       // False
    Ack(bool),
}