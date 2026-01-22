use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::sync::RwLock;

use crate::server::client::Client;

pub struct Room {
    pub room_name: String,
    members: Arc<RwLock<HashMap<SocketAddr, Arc<Client>>>>,
}

impl Room {
    pub fn new(room_name: String) -> Self {
        Self {
            room_name,
            members: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_member(&self, mem_id: &SocketAddr) -> Option<Arc<Client>> {
        self.members
            .read()
            .await
            .get(mem_id)
            .and_then(|c| Some(c.clone()))
    }

    pub async fn get_all_members(&self) -> Vec<Arc<Client>> {
        self.members
            .read()
            .await
            .values()
            .map(|mem| mem.clone())
            .collect()
    }

    pub async fn add_member(&self, mem: Arc<Client>) {
        self.members.write().await.insert(mem.socket_addr, mem);
    }

    pub async fn remove_member(&self, mem_id: &SocketAddr) {
        let mut members = self.members.write().await;
        members.remove(mem_id);
    }
}
