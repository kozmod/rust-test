extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let header = String::from("HTTP/1.1 200 OK\r\n\r\n");
    let time: DateTime<Utc> = SystemTime::now().into();
    let response = format!("{}{:?}", header, time);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

