use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use log::{info, warn};
use serde_json;
use shared::messages::RegisterTeam;

fn handle_client(mut stream: TcpStream) {
    
    let mut size_buffer = [0u8; 4];
    if let Err(e) = stream.read_exact(&mut size_buffer) {
        warn!("Erreur lors de la lecture de la taille du message: {}", e);
        return;
    }
    let msg_size = u32::from_le_bytes(size_buffer) as usize;
    if msg_size > 1_048_576 {
        warn!("Too large message size: {}", msg_size);
        return;
    }
    let mut buffer = vec![0u8; msg_size];
    if let Err(e) = stream.read_exact(&mut buffer) {
        warn!("Erreur lors de la lecture du message: {}", e);
        return;
    }
    let msg_str = String::from_utf8_lossy(&buffer);
    info!("Message reçu: {}", msg_str);

    
    if let Ok(parsed) = serde_json::from_str::<RegisterTeam>(&msg_str) {
        info!("RegisterTeam reçu: {:?}", parsed);
        
        let response = serde_json::json!({
            "RegisterTeamResult": {
                "Ok": {
                    "expected_players": 3,
                    "registration_token": "SECRET_TOKEN"
                }
            }
        }).to_string();
        let response_bytes = response.as_bytes();
        let response_size = (response_bytes.len() as u32).to_le_bytes();
        if stream.write_all(&response_size).is_err() {
            warn!("Erreur lors de l'envoi de la taille de réponse");
            return;
        }
        if stream.write_all(response_bytes).is_err() {
            warn!("Erreur lors de l'envoi de la réponse");
            return;
        }
    } else {
        // Si la désérialisation échoue, on envoie une erreur générique.
        let response = serde_json::json!({
            "ActionError": "Unknown message"
        }).to_string();
        let response_bytes = response.as_bytes();
        let response_size = (response_bytes.len() as u32).to_le_bytes();
        if stream.write_all(&response_size).is_err() {
            warn!("Erreur lors de l'envoi de la taille de réponse");
            return;
        }
        if stream.write_all(response_bytes).is_err() {
            warn!("Erreur lors de l'envoi de la réponse");
            return;
        }
    }
}

fn main() {
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:8778").expect("Impossible de lier sur l'adresse");
    info!("Serveur lancé sur 127.0.0.1:8778");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                warn!("Échec d'une connexion: {}", e);
            }
        }
    }
}
