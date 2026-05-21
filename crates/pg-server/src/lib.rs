use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:5432").unwrap();
    println!("pg-server running on 5432");

    for stream in listener.incoming() {
        handle(stream.unwrap());
    }
}

fn handle(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf);
    let _ = stream.write_all(b"OK\n");
}