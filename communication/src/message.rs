

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub header_information: String,
    pub content: String
}
