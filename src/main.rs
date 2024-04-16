mod conn_man;
mod state;
mod logging;

fn main() {
    let server = conn_man::server::Server::new();
    server.start_server();
}
