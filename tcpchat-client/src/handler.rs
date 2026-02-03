use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    process::exit,
    str::FromStr,
};

use crate::message::{Event, Message, create_message};

pub fn handle_user_input(mut writer: TcpStream) {
    let mut buffer = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap_or_else(|_| {
            println!("Something wrong");
        });

        match std::io::stdin().read_line(&mut buffer) {
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                exit(1);
            }
            Ok(_) => 'handler: {
                let input = buffer.trim_matches('\n').trim();

                let args: Vec<&str> = input.split_whitespace().collect();
                if args.is_empty() || (args.len() == 1 && args[0] != Event::GetRooms.to_string()) {
                    print!("\r");
                    std::io::stdout().flush().unwrap_or_else(|_| {
                        println!("Something wrong");
                    });
                    println!("Error: Wrong format.");
                    break 'handler;
                }

                let e = match Event::from_str(args[0]) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("{e}");
                        break 'handler;
                    }
                };

                let m = create_message(e, args[1..].join(" "));

                writer
                    .write_all(m.as_bytes())
                    .expect("Cannot write to socket");
            }
        }

        buffer.clear();
    }
}

pub fn handle_messages_from_server(reader: TcpStream) {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    loop {
        match reader.read_line(&mut buffer) {
            Err(e) => {
                eprintln!("Error when read message from server {e}");
                exit(1);
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let msg: Message = match serde_json::from_str(&buffer) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("Error when deserialize server message {e}");
                        exit(1);
                    }
                };

                buffer.clear();
                print!("\033[2K\r");
                std::io::stdout().flush().unwrap_or_else(|_| {
                    println!("Something wrong");
                });

                match msg.event {
                    Event::Error => {
                        println!("Error: {}", msg.payload);
                    }
                    Event::Message => {
                        println!("{}", msg.payload);
                    }
                    _ => {
                        eprintln!("Unknow server message");
                    }
                }

                print!("> ");
                std::io::stdout().flush().unwrap_or_else(|_| {
                    println!("Something wrong");
                });
            }
        }
    }
}
