use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json;
use shared::messages::{RegisterTeamResultWrapper};

pub struct GameClient {
    pub stream: TcpStream,
    /// Le token d'inscription, extrait de la réponse du serveur.
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

    /// Envoie une demande d'enregistrement d'équipe.
    /// Le message est enveloppé sous la clé "RegisterTeam" et préfixé par sa taille (u32 little-endian).
    /// La réponse est ensuite désérialisée pour extraire le token.
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
                // Désérialisation de la réponse
                let wrapper: Result<RegisterTeamResultWrapper, _> = serde_json::from_str(&response);
                match wrapper {
                    Ok(w) => {
                        match w.RegisterTeamResult {
                            shared::messages::RegisterTeamResult::Ok { expected_players, registration_token } => {
                                println!("✅ Enregistrement réussi : {} joueurs attendus, token = {}", expected_players, registration_token);
                                self.registration_token = Some(registration_token);
                            },
                            shared::messages::RegisterTeamResult::Err(err) => {
                                eprintln!("❌ Erreur lors de l'enregistrement de l'équipe: {}", err);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("❌ Erreur de désérialisation: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("❌ Erreur de lecture du message : {}", e);
            }
        }
    }

    /// (Méthode à compléter pour inscrire un joueur)
    pub fn subscribe_player(&mut self, _player_name: &str) {
        // À implémenter ultérieurement...
    }
}
