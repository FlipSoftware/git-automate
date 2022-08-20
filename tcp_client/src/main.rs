use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();
    stream.write("Ping".as_bytes()).unwrap();
    let mut buffer = [0; 4];
    stream.read(&mut buffer).unwrap();
    println!("Response: {:?}", std::str::from_utf8(&buffer).unwrap());
}
