use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use shared::messages::{RegisterTeam, RegisterTeamResult, SubscribePlayer};
use serde_json;

pub fn connect_to_server(address: &str) -> TcpStream {
    TcpStream::connect(address).expect("Could not connect to server")
}

pub fn send_message<T: serde::Serialize>(stream: &mut TcpStream, message: &T) {
    let serialized = serde_json::to_string(message).unwrap();
    stream.write_all(serialized.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn read_message(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 512];
    let size = stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..size]).to_string()
}
