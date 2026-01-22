use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::server::room::Room;

pub struct Server {
    rooms: Arc<RwLock<HashMap<String, Arc<Room>>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_room(&self, room_name: &String) -> Option<Arc<Room>> {
        self.rooms.read().await.get(room_name).cloned()
    }

    pub async fn get_all_rooms(&self) -> Vec<String> {
        self.rooms
            .read()
            .await
            .values()
            .map(|r| r.room_name.clone())
            .collect()
    }

    pub async fn add_room(&self, room: Arc<Room>) {
        self.rooms
            .write()
            .await
            .insert(room.room_name.clone(), room);
    }

    pub async fn remove_room(&self, room_name: &String) {
        self.rooms.write().await.remove(room_name);
    }
}
