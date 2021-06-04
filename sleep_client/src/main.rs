extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;

use std::time::{SystemTime};
use clap::{Arg, App};

use async_std::io::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task::spawn;
use futures::StreamExt;

#[async_std::main]
async fn main() {
    let matches = App::new("Sleep Client")
        .about("http client which return current machine time by `/` path")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .help("port of the `sleep client`"))
        .get_matches();
    // let port = "8080";
    let mut port = matches.value_of("port");
    match port {
        None => port = Some("8080"),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(p) => println!("Server started on port :{}", p),
                Err(_) => println!("port value is not valid {}", s),
            }
        }
    }

    let listener = TcpListener::bind(["0.0.0.0", ":", port.unwrap()].join("")).await.unwrap();
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

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let header = String::from("HTTP/1.1 200 OK\r\n\r\n");
    let time: DateTime<Utc> = SystemTime::now().into();
    let response = format!("{}{:?}", header, time);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

