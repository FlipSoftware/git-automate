use server::Server;

mod handler;
mod router;
mod server;

fn main() {
    let server = Server::new("localhost:8080");
    server.run();
}
