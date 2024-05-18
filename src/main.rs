use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server is running on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

