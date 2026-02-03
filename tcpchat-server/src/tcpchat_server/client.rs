use std::net::SocketAddr;

use tokio::sync::mpsc::Sender;

pub struct Client {
    pub socket_addr: SocketAddr,
    pub tx: Sender<String>,
}

impl Client {
    pub fn new(socket_addr: SocketAddr, tx: Sender<String>) -> Self {
        Self { socket_addr, tx }
    }
}
