use super::router::Router;
use http::http_request::HttpRequest;
use std::{io::prelude::*, net::TcpListener};

pub struct Server<'a> {
    socket: &'a str,
}
impl<'a> Server<'a> {
    pub fn new(socket: &'a str) -> Self {
        Self { socket }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket).unwrap();
        println!("Starting server on socket: {} 🔗✅", self.socket);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Peer connected 🌐");
            let mut read_buffer = Vec::from([0; 96]);
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
