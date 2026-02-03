use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    #[serde(rename = "/name")]
    Rename,
    #[serde(rename = "/join")]
    JoinRoom,
    #[serde(rename = "/rooms")]
    GetRooms,
    #[serde(rename = "/msg")]
    Message,
    #[serde(rename = "/err")]
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub event: Event,
    pub payload: String,
}

pub fn create_message(event: Event, payload: String) -> String {
    let mut data =
        serde_json::to_string(&Message { event, payload }).expect("Failed to serialize data");
    data.push('\n');
    data
}
