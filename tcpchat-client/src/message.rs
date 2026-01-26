use std::{fmt::Display, str::FromStr};

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

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Rename => write!(f, "/name"),
            Event::JoinRoom => write!(f, "/join"),
            Event::GetRooms => write!(f, "/rooms"),
            Event::Message => write!(f, "/msg"),
            Event::Error => write!(f, "/err"),
        }
    }
}

impl FromStr for Event {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/name" => Ok(Event::Rename),
            "/join" => Ok(Event::JoinRoom),
            "/rooms" => Ok(Event::GetRooms),
            "/msg" => Ok(Event::Message),
            "/err" => Ok(Event::Error),
            _ => Err("Unknow event type"),
        }
    }
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
