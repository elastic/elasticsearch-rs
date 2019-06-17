use crate::{
    connection::Connection,
    node_pool::NodePool
};

pub struct ConnectionSettings<T> where T: Connection<TNodePool>, TNodePool: NodePool {
    connection: T
}

impl<T> ConnectionSettings<T> where T: Connection<TNodePool>, TNodePool: NodePool {
    pub fn get_connection(&self) -> T {
        &self.connection;
    }
}
