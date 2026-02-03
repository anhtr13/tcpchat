use std::{env, sync::Arc};

use local_ip_address::local_ip;
use tcpchat_server::tcpchat_server::{handler::handler, server::Server};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let server = Arc::new(Server::new());

    let port = env::var("PORT").unwrap_or(String::from("8080"));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    if let Ok(my_ip) = local_ip() {
        println!("Server is running on {}:{}", my_ip, port);
    } else {
        println!("Could not get local IP address.");
    }

    loop {
        let (socket, addr) = listener.accept().await.expect("Failed to accept socket");

        tokio::spawn(handler(server.clone(), socket, addr));
    }
}
