use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};

use crate::tcpchat_server::{
    client::Client,
    message::{Event, Message, create_message},
    room::Room,
    server::Server,
};

pub async fn handler(server: Arc<Server>, socket: TcpStream, addr: SocketAddr) {
    let mut name = format!("Anonymous-{}", addr);
    let mut curr_room_name = String::from("");

    let (tx, mut rx) = mpsc::channel::<String>(10);
    let client = Arc::new(Client::new(addr, tx));
    println!("{} has connected", addr);

    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();

    loop {
        tokio::select! {
            bytes_read = reader.read_line(&mut buf) => {
                match bytes_read {
                    Ok(0) => {
                        break;
                    }
                    Err(e) => {
                        println!("Error: {e}");
                        break;
                    }
                    Ok(_) => {
                        let msg: Message = match serde_json::from_str(&buf) {
                            Ok(msg) => msg,
                            Err(e) => {
                                let err = create_message(Event::Error, e.to_string());
                                writer.write_all(err.as_bytes()).await.unwrap_or_else(|_| {
                                    panic!("Failed to response to {addr}");
                                });
                                buf.clear();
                                continue;
                            }
                        };

                        buf.clear();

                        match msg.event {
                            Event::Rename => {
                                name = msg.payload;
                                let res = create_message(Event::Message, format!("Your new name is {name}"));
                                writer
                                    .write_all(res.as_bytes())
                                    .await
                                    .unwrap_or_else(|_| {
                                        panic!("Failed to response to {addr}");
                                    });
                            }
                            Event::JoinRoom => {
                                let room_name = msg.payload.trim();
                                if room_name.is_empty() {
                                    let err = create_message(Event::Error, String::from("Invalid room name"));
                                    writer
                                        .write_all(err.as_bytes())
                                        .await
                                        .unwrap_or_else(|_| {
                                            panic!("Failed to response to {addr}");
                                        });
                                    continue;
                                }
                                let room_name = room_name.to_string();
                                if room_name != curr_room_name {
                                    if !curr_room_name.is_empty()
                                        && let prev_room = server.get_room(&curr_room_name).await
                                        && let Some(prev_room) = prev_room
                                    {
                                        prev_room.remove_member(&addr).await;
                                    }

                                    let room = server.get_room(&room_name).await;

                                    if let Some(room) = room {
                                        let m = create_message(Event::Message, format!("{name} has joined the room"));
                                        let room_members = room.get_all_members().await;
                                        for mem in room_members {
                                            mem.tx.send(m.clone()).await.unwrap_or_else(|_| {
                                                println!("Failed to send to {addr} tx");
                                            });
                                        }
                                        room.add_member(client.clone()).await;
                                        let m = create_message(Event::Message, format!("Joined room '{room_name}'"));
                                        writer
                                            .write_all(m.as_bytes())
                                            .await
                                            .unwrap_or_else(|_| {
                                                panic!("Failed to response to {addr}");
                                            });
                                    } else {
                                        let room = Arc::new(Room::new(room_name.clone()));
                                        room.add_member(client.clone()).await;
                                        server.add_room(room).await;
                                        let m = create_message(Event::Message, format!("Joined room '{room_name}'"));
                                        writer
                                            .write_all(m.as_bytes())
                                            .await
                                            .unwrap_or_else(|_| {
                                                panic!("Failed to response to {addr}");
                                            });
                                    }
                                    curr_room_name = room_name;
                                }
                            }
                            Event::GetRooms => {
                                let rooms = server.get_all_rooms().await;
                                let res = create_message(Event::Message, format!("[{}]", rooms.join(", ")));
                                writer
                                    .write_all(res.as_bytes())
                                    .await
                                    .unwrap_or_else(|_| {
                                        panic!("Failed to response to {addr}");
                                    });
                            }
                            Event::Message => {
                                if curr_room_name.is_empty() {
                                    let m = create_message(
                                        Event::Error,
                                        String::from("You're not in any room, join a room to send message."),
                                    );
                                    writer
                                        .write_all(m.as_bytes())
                                        .await
                                        .unwrap_or_else(|_| {
                                            panic!("Failed to response to {addr}");
                                        });
                                    continue;
                                }
                                let room = server.get_room(&curr_room_name).await;
                                if let Some(room) = room {
                                    let m = create_message(Event::Message, format!("{name}: {}", msg.payload));
                                    let room_members = room.get_all_members().await;
                                    for client in room_members {
                                        client.tx.send(m.clone()).await.unwrap_or_else(|_| {
                                            println!("Failed to send to {addr} tx");
                                        });
                                    }
                                }
                            }
                            _ => {
                                let err = create_message(Event::Error, String::from("Invalid event"));
                                writer
                                    .write_all(err.as_bytes())
                                    .await
                                    .unwrap_or_else(|_| {
                                        panic!("Failed to response to {addr}");
                                    });
                            }
                        }
                    }
                }
            },

            msg = rx.recv() => {
                if let Some(data) = msg {
                    writer
                        .write_all(data.as_bytes())
                        .await
                        .unwrap_or_else(|_| {
                            panic!("Failed to response to {addr}");
                        });
                }
            }
        }
    }

    if !curr_room_name.is_empty()
        && let Some(room) = server.get_room(&curr_room_name).await
    {
        room.remove_member(&addr).await;
    }

    println!("{} has disconnected", addr);
}
