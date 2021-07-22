use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
}

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    }
}
