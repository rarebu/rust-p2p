extern crate communication;

use std::error;
use std::fmt;
use communication::communication_error::CommunicationError;


#[derive(Debug, Clone)]
pub enum NodeError {
    communication_error(CommunicationError),
    stream_allready_exists_error,
    stream_not_exists_anymore_error,
    no_message_received_error,
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeError::communication_error(ref e) => e.fmt(f),
            NodeError::stream_allready_exists_error => write!(f, "Stream does allready exist"),
            NodeError::stream_not_exists_anymore_error => write!(f, "Stream does not exist anymore"),
            NodeError::no_message_received_error => write!(f, "No messages received"),
        }

    }
}

impl error::Error for NodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<CommunicationError> for NodeError {
    fn from(item: CommunicationError) -> Self {
        NodeError::communication_error(item)
    }
}