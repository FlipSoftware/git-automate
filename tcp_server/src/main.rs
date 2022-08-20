use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let connection_listener = TcpListener::bind("localhost:8080").unwrap();
    println!("Server started on port: 8080 ✅");

    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Pong: peer connected ↔");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
