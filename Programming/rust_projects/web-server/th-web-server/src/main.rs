use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:8080").unwrap();

    for conn in listener.incoming() {
        let conn: TcpStream = conn.unwrap();
        handle_connection(conn);
    }
}

fn handle_connection(mut conn: TcpStream) {
    let mut buff = [0; 1024];
    conn.read(&mut buff).unwrap();

    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    conn.write(response);
}
