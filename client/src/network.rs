use std::io::{Write, Read};
use std::net::TcpStream;

pub fn send_message(stream: &mut TcpStream, message: &str) {
    let size = (message.len() as u32).to_le_bytes();
    if stream.write_all(&size).is_err() {
        eprintln!("❌ Erreur lors de l'envoi de la taille du message!");
        return;
    }
    if stream.write_all(message.as_bytes()).is_err() {
        eprintln!("❌ Erreur lors de l'envoi du message!");
    }
}

pub fn receive_message(stream: &mut TcpStream) -> Option<String> {
    let mut size_buffer = [0; 4];
    if stream.read_exact(&mut size_buffer).is_err() {
        return None; // On retourne None si on ne parvient pas à lire (EOF ou autre)
    }
    let size = u32::from_le_bytes(size_buffer) as usize;
    if size > 1_048_576 {
        eprintln!("⚠️ Message trop grand: {} octets!", size);
        return None;
    }
    let mut buffer = vec![0; size];
    if stream.read_exact(&mut buffer).is_err() {
        return None;
    }
    Some(String::from_utf8_lossy(&buffer).to_string())
}
