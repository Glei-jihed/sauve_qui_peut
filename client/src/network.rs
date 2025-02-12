use std::io::{Write, Read};
use std::net::TcpStream;

/// Envoie un message sur le stream en préfixant par sa taille encodée en little-endian.
pub fn send_message(stream: &mut TcpStream, message: &str) {
    let size = (message.len() as u32).to_le_bytes();
    if stream.write_all(&size).is_err() {
        eprintln!("❌ Erreur lors de l'envoi de la taille du message !");
        return;
    }
    if stream.write_all(message.as_bytes()).is_err() {
        eprintln!("❌ Erreur lors de l'envoi du message !");
    }
}

/// Lit un message depuis le stream en lisant d'abord 4 octets pour obtenir la taille (little-endian),
/// puis en lisant le nombre d’octets indiqué.
pub fn receive_message(stream: &mut TcpStream) -> Option<String> {
    let mut size_buffer = [0; 4];
    if stream.read_exact(&mut size_buffer).is_err() {
        eprintln!("❌ Erreur lors de la lecture de la taille du message !");
        return None;
    }
    let size = u32::from_le_bytes(size_buffer) as usize;
    if size > 1_048_576 {
        eprintln!("⚠️ Message reçu trop grand : {} octets !", size);
        return None;
    }
    let mut buffer = vec![0; size];
    if stream.read_exact(&mut buffer).is_err() {
        eprintln!("❌ Erreur lors de la lecture du message !");
        return None;
    }
    String::from_utf8(buffer).ok()
}
