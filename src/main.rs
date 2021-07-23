use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;
use std::fs::File;

fn handle_stream(mut stream: TcpStream) {
    let mut request_buffer = [0; 512];
    stream.read(&mut request_buffer).unwrap();

    let (resource_location, response_header) = if request_buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("./resource/hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else {
        ("./resource/404.html", "HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };
    
    let mut html_template = File::open(resource_location).unwrap();
    let mut content = String::new();
    html_template.read_to_string(&mut content).unwrap();

    let response = format!("{}{}", response_header, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    }
}
