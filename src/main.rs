use server::Server;

use crate::logger::LogType;

mod config;
mod db;
mod server;
mod save;
mod packet;
mod logger;

fn main() {
    let server = Server::new();
    server.start_server();
}
