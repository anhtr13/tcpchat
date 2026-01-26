use std::{env, net::TcpStream, thread};

use crate::handler::{handle_messages_from_server, handle_user_input};

mod handler;
mod message;

fn main() -> std::io::Result<()> {
    println!("Hello world");

    let server_host = env::var("HOST").unwrap_or("localhost".to_string());
    let server_port: u32 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Port number is u32 integer");

    let stream = TcpStream::connect(format!("{server_host}:{server_port}"))?;
    let write_stream = stream.try_clone()?;

    let server_response_thread = thread::spawn(move || {
        handle_messages_from_server(stream);
    });

    let user_interactive_thread = thread::spawn(move || {
        handle_user_input(write_stream);
    });

    server_response_thread
        .join()
        .expect("server_response_thread panicked");
    user_interactive_thread
        .join()
        .expect("user_interactive_thread panicked");

    Ok(())
}
