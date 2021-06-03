extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::{SystemTime};

use async_std::io::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task::spawn;
use futures::StreamExt;

#[async_std::main]
async fn main() {
    let port = ":8080";
    println!("Server started on port {}", port);

    let listener = TcpListener::bind(["127.0.0.1", port].join("")).await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}


async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await.unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let header = String::from("HTTP/1.1 200 OK\r\n\r\n");
    let time: DateTime<Utc> = SystemTime::now().into();
    let response = format!("{}{:?}", header, time);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

