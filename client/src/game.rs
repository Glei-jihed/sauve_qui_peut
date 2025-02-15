// client/src/game.rs

use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json;
use shared::messages::{RegisterTeamResultWrapper, RegisterTeamResult, SubscribePlayer};
use shared::messages::RelativeDirection; // Importation pour les actions de déplacement

pub struct GameClient {
    pub stream: TcpStream,
    pub registration_token: Option<String>,
    pub server_address: String,
}

impl GameClient {
    pub fn new(server_address: &str) -> Self {
        match TcpStream::connect(server_address) {
            Ok(stream) => {
                println!("✅ Connecté au serveur sur {}", server_address);
                GameClient {
                    stream,
                    registration_token: None,
                    server_address: server_address.to_string(),
                }
            }
            Err(e) => {
                eprintln!("❌ Erreur de connexion au serveur: {}", e);
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
        println!("📤 Envoi de la taille: {} octets", message.len());

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
                println!("📩 Réponse du serveur: {:?}", response);
                let wrapper: Result<RegisterTeamResultWrapper, _> = serde_json::from_str(&response);
                match wrapper {
                    Ok(w) => match w.register_team_result {
                        RegisterTeamResult::OkVariant { ok } => {
                            println!("✅ Enregistrement réussi: {} joueurs attendus, token = {}", ok.expected_players, ok.registration_token);
                            self.registration_token = Some(ok.registration_token);
                        },
                        RegisterTeamResult::ErrVariant { err } => {
                            eprintln!("❌ Erreur lors de l'enregistrement: {}", err);
                        }
                    },
                    Err(e) => {
                        eprintln!("❌ Erreur de désérialisation: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("❌ Erreur de lecture du message: {}", e);
            }
        }
    }

    pub fn subscribe_player(&self, player_name: &str) {
        if let Some(token) = &self.registration_token {
            match TcpStream::connect(&self.server_address) {
                Ok(mut new_stream) => {
                    let message = serde_json::json!({
                        "SubscribePlayer": {
                            "name": player_name,
                            "registration_token": token
                        }
                    }).to_string();
                    let message_size = (message.len() as u32).to_le_bytes();
                    println!("📤 Envoi de la taille pour SubscribePlayer: {} octets", message.len());
                    if new_stream.write_all(&message_size).is_err() {
                        eprintln!("❌ Erreur d'envoi de la taille pour SubscribePlayer!");
                        return;
                    }
                    if new_stream.write_all(message.as_bytes()).is_err() {
                        eprintln!("❌ Erreur d'envoi du message SubscribePlayer!");
                        return;
                    }
                    let mut size_buffer = [0; 4];
                    if new_stream.read_exact(&mut size_buffer).is_err() {
                        eprintln!("❌ Erreur de lecture de la taille de la réponse SubscribePlayer!");
                        return;
                    }
                    let response_size = u32::from_le_bytes(size_buffer);
                    let mut buffer = vec![0; response_size as usize];
                    if new_stream.read_exact(&mut buffer).is_err() {
                        eprintln!("❌ Erreur de lecture du message SubscribePlayer!");
                        return;
                    }
                    let response = String::from_utf8_lossy(&buffer).to_string();
                    println!("📩 Réponse du serveur (SubscribePlayer): {:?}", response);
                },
                Err(e) => {
                    eprintln!("❌ Erreur lors de la connexion pour subscribe_player: {}", e);
                }
            }
        } else {
            eprintln!("❌ Aucun token disponible pour subscribe_player!");
        }
    }

    pub fn join_game(server_address: &str, token: &str, player_name: &str) {
        match TcpStream::connect(server_address) {
            Ok(mut stream) => {
                let message = serde_json::json!({
                    "SubscribePlayer": {
                        "name": player_name,
                        "registration_token": token
                    }
                }).to_string();
                let message_size = (message.len() as u32).to_le_bytes();
                println!("📤 Envoi de la taille pour SubscribePlayer (join): {} octets", message.len());
                if stream.write_all(&message_size).is_err() {
                    eprintln!("❌ Erreur d'envoi de la taille pour SubscribePlayer (join)!");
                    return;
                }
                if stream.write_all(message.as_bytes()).is_err() {
                    eprintln!("❌ Erreur d'envoi du message SubscribePlayer (join)!");
                    return;
                }
                let mut size_buffer = [0; 4];
                if stream.read_exact(&mut size_buffer).is_err() {
                    eprintln!("❌ Erreur de lecture de la taille de la réponse SubscribePlayer (join)!");
                    return;
                }
                let response_size = u32::from_le_bytes(size_buffer);
                let mut buffer = vec![0; response_size as usize];
                if stream.read_exact(&mut buffer).is_err() {
                    eprintln!("❌ Erreur de lecture du message SubscribePlayer (join)!");
                    return;
                }
                let response = String::from_utf8_lossy(&buffer).to_string();
                println!("📩 Réponse du serveur (SubscribePlayer join): {:?}", response);
            },
            Err(e) => {
                eprintln!("❌ Erreur lors de la connexion pour join_game: {}", e);
            }
        }
    }

    /// Envoie une action de déplacement (MoveTo) au serveur en utilisant une nouvelle connexion.
    pub fn send_move_action_static(server_address: &str, direction: RelativeDirection) {
        if let Ok(mut stream) = TcpStream::connect(server_address) {
            let message = serde_json::json!({
                "Action": {
                    "MoveTo": format!("{:?}", direction)
                }
            })
            .to_string();
            let message_size = (message.len() as u32).to_le_bytes();
            if stream.write_all(&message_size).is_err() {
                eprintln!("❌ Erreur d'envoi de la taille pour MoveTo!");
                return;
            }
            if stream.write_all(message.as_bytes()).is_err() {
                eprintln!("❌ Erreur d'envoi du message MoveTo!");
            }
        } else {
            eprintln!("❌ Erreur de connexion pour envoyer MoveTo!");
        }
    }
}
