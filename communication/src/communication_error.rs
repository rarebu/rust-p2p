use std::error;
use std::fmt;
use bincode;
use std::sync::PoisonError;

#[derive(Debug, Clone)]
pub struct CommunicationError {
    error_message: String,
}

impl CommunicationError {
    pub fn new(msg: String) -> CommunicationError {
        CommunicationError {
            error_message: msg,
        }
    }

    pub fn new_write_error() -> CommunicationError {
        CommunicationError {
            error_message: String::from("write_all failed!"),
        }
    }
}

impl fmt::Display for CommunicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is our First own error")
    }
}

impl error::Error for CommunicationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<bincode::Error> for CommunicationError {
    fn from(item: bincode::Error) -> Self {
        CommunicationError::new(item.to_string())
    }
}

impl<T> From<PoisonError<T>> for CommunicationError {
    fn from(item: PoisonError<T>) -> Self {
        CommunicationError::new(format!("PoisonError: {}", item))
    }
}