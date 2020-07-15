extern crate communication;

use std::error;
use std::fmt;
use communication::communication_error::CommunicationError;


#[derive(Debug, Clone)]
pub enum NodeError {
    CommunicationError(CommunicationError),
    StreamAllreadyExistsError,
    StreamNotExistsAnymoreError,
    NoMessageReceivedError,
    MapFailedError,
    StreamAccessorNotReachableError,
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeError::CommunicationError(ref e) => e.fmt(f),
            NodeError::StreamAllreadyExistsError => write!(f, "Stream does allready exist"),
            NodeError::StreamNotExistsAnymoreError => write!(f, "Stream does not exist anymore"),
            NodeError::NoMessageReceivedError => write!(f, "No messages received"),
            NodeError::MapFailedError => write!(f, "Map function failed"),
            NodeError::StreamAccessorNotReachableError => write!(f, "Stream_accessor not reachable"),
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
        NodeError::CommunicationError(item)
    }
}