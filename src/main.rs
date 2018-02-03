#![allow(unused_imports,dead_code)]
extern crate tungstenite;

use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::Message;
use tungstenite::server::accept;

pub mod kipac;
mod calc;
fn message_to_string(msg: Message) -> String {
    match msg {
        Message::Text(s) => s,
        _ => String::from("Hello"),
    }
}

fn handle(stream: TcpStream) {
    let mut ws = accept(stream).unwrap();
    while let Ok(_msg) = ws.read_message() {
        let msg = message_to_string(_msg);
        println!("{}", msg);
    }
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in server.incoming() {
        spawn(move || handle(stream.unwrap()));
    }
}
