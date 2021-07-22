use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs::File;

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let mut html_template = File::open("./resource/hello.html").unwrap();
    let mut content = String::new();
    html_template.read_to_string(&mut content).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", content);
    println!("{}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    }
}
