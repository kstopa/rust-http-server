#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_addr = "127.0.0.1:9090";
    let server_addr = env::var("SERVER_ADDRESS").unwrap_or(default_addr.to_string());
    println!("Starting server at {server_addr}");

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public data read from {public_path}");

    let server = Server::new(server_addr);
    server.run(&WebsiteHandler::new(public_path));
}
