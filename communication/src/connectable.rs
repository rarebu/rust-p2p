use super::stream_accessor::StreamAccessor;

pub trait Connectable {
    fn get_connections(&self) -> Vec<String> ;
    fn get_connection(&self, peer_address: String) -> Option<StreamAccessor>;
}