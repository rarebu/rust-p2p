use super::stream_accessor::StreamAccessor;
use crate::communication_error::CommunicationError;

pub trait Connectable {
    fn get_connections(&self) -> Result<Vec<String>, CommunicationError> ;
    fn get_connection(&self, peer_address: String) -> Result<Option<StreamAccessor>, CommunicationError>;
}