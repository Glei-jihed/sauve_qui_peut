use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json;

pub struct GameClient {
    pub stream: TcpStream,
    
    pub registration_token: Option<String>,
}

impl GameClient {
    pub fn new(server_address: &str) -> Self {
        match TcpStream::connect(server_address) {
            Ok(stream) => {
                println!("✅ Connecté au serveur sur {}", server_address);
                GameClient { 
                    stream, 
                    registration_token: None 
                }
            }
            Err(e) => {
                eprintln!("❌ Erreur de connexion au serveur : {}", e);
                std::process::exit(1);
            }
        }
    }


    pub fn register_team(&mut self, team_name: &str) {
        let message = serde_json::json!({
            "RegisterTeam": {
                "name": team_name
            }
        }).to_string();

        let message_size = (message.len() as u32).to_le_bytes();
        println!("📤 Envoi de la taille : {} octets", message.len());

        if self.stream.write_all(&message_size).is_err() {
            eprintln!("❌ Erreur d'envoi de la taille !");
            return;
        }

        if self.stream.write_all(message.as_bytes()).is_err() {
            eprintln!("❌ Erreur d'envoi du message !");
            return;
        }

        let mut size_buffer = [0; 4];
        if self.stream.read_exact(&mut size_buffer).is_err() {
            eprintln!("❌ Erreur de lecture de la taille de la réponse !");
            return;
        }

        let response_size = u32::from_le_bytes(size_buffer);
        let mut buffer = vec![0; response_size as usize];

        match self.stream.read_exact(&mut buffer) {
            Ok(_) => {
                let response = String::from_utf8_lossy(&buffer).to_string();
                println!("📩 Réponse du serveur : {:?}", response);

            
            }
            Err(e) => {
                eprintln!("❌ Erreur de lecture du message : {}", e);
            }
        }
    }

    
    pub fn subscribe_player(&mut self, player_name: &str) {
        if let Some(token) = &self.registration_token {
            let message = serde_json::json!({
                "SubscribePlayer": {
                    "name": player_name,
                    "registration_token": token
                }
            }).to_string();

            let message_size = (message.len() as u32).to_le_bytes();
            println!("📤 Envoi de la taille : {} octets", message.len());

            if self.stream.write_all(&message_size).is_err() {
                eprintln!("❌ Erreur d'envoi de la taille !");
                return;
            }

            if self.stream.write_all(message.as_bytes()).is_err() {
                eprintln!("❌ Erreur d'envoi du message !");
                return;
            }

            let mut size_buffer = [0; 4];
            if self.stream.read_exact(&mut size_buffer).is_err() {
                eprintln!("❌ Erreur de lecture de la taille de la réponse !");
                return;
            }

            let response_size = u32::from_le_bytes(size_buffer);
            let mut buffer = vec![0; response_size as usize];

            match self.stream.read_exact(&mut buffer) {
                Ok(_) => {
                    let response = String::from_utf8_lossy(&buffer).to_string();
                    println!("📩 Réponse du serveur : {:?}", response);
                }
                Err(e) => {
                    eprintln!("❌ Erreur de lecture du message : {}", e);
                }
            }
        } else {
            eprintln!("❌ Erreur : Aucun token d'inscription disponible !");
        }
    }

    pub fn process_radar_view(&self, radar: &str) {
        println!("Radar view encoded: {}", radar);
    }
}
