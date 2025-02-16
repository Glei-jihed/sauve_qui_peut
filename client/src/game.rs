use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json;
use shared::messages::{
    RegisterTeamResultWrapper,
    RegisterTeamResult,
    RelativeDirection,
};

/// Gère la connexion et les actions liées à l’équipe.
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
        let msg = serde_json::json!({
            "RegisterTeam": { "name": team_name }
        }).to_string();
        let size = (msg.len() as u32).to_le_bytes();

        println!("📤 Envoi RegisterTeam ({} octets)", msg.len());
        if self.stream.write_all(&size).is_err() {
            eprintln!("❌ Erreur d'envoi taille RegisterTeam");
            return;
        }
        if self.stream.write_all(msg.as_bytes()).is_err() {
            eprintln!("❌ Erreur d'envoi message RegisterTeam");
            return;
        }

        let mut size_buf = [0; 4];
        if self.stream.read_exact(&mut size_buf).is_err() {
            eprintln!("❌ Erreur lecture taille réponse RegisterTeam");
            return;
        }
        let resp_size = u32::from_le_bytes(size_buf);
        let mut buffer = vec![0; resp_size as usize];
        if self.stream.read_exact(&mut buffer).is_err() {
            eprintln!("❌ Erreur lecture message RegisterTeam");
            return;
        }
        let resp = String::from_utf8_lossy(&buffer).to_string();
        println!("📩 Réponse RegisterTeam: {}", resp);

        let parsed = serde_json::from_str::<RegisterTeamResultWrapper>(&resp);
        if let Ok(wrapper) = parsed {
            match wrapper.register_team_result {
                RegisterTeamResult::OkVariant { ok } => {
                    println!("✅ RegisterTeam OK, token={}", ok.registration_token);
                    self.registration_token = Some(ok.registration_token);
                }
                RegisterTeamResult::ErrVariant { err } => {
                    eprintln!("❌ RegisterTeam err: {}", err);
                }
            }
        } else {
            eprintln!("❌ Erreur de parsing RegisterTeam");
        }
    }

    pub fn subscribe_player(&self, player_name: &str) {
        if let Some(tok) = &self.registration_token {
            if let Ok(mut s) = TcpStream::connect(&self.server_address) {
                let msg = serde_json::json!({
                    "SubscribePlayer": {
                        "name": player_name,
                        "registration_token": tok
                    }
                }).to_string();
                let size = (msg.len() as u32).to_le_bytes();

                println!("📤 SubscribePlayer -> {}", msg);
                if s.write_all(&size).is_err() {
                    eprintln!("❌ Erreur envoi taille SubscribePlayer");
                    return;
                }
                if s.write_all(msg.as_bytes()).is_err() {
                    eprintln!("❌ Erreur envoi message SubscribePlayer");
                    return;
                }
                let mut sz_buf = [0; 4];
                if s.read_exact(&mut sz_buf).is_err() {
                    eprintln!("❌ Erreur lecture taille réponse SubscribePlayer");
                    return;
                }
                let rsize = u32::from_le_bytes(sz_buf);
                let mut buffer = vec![0; rsize as usize];
                if s.read_exact(&mut buffer).is_err() {
                    eprintln!("❌ Erreur lecture message SubscribePlayer");
                    return;
                }
                let resp = String::from_utf8_lossy(&buffer).to_string();
                println!("📩 Réponse SubscribePlayer: {}", resp);
            } else {
                eprintln!("❌ Impossible de se connecter pour SubscribePlayer");
            }
        } else {
            eprintln!("❌ Aucun token (register_team) pour subscribe_player");
        }
    }

    pub fn join_game(server_address: &str, token: &str, player_name: &str) {
        if let Ok(mut s) = TcpStream::connect(server_address) {
            let msg = serde_json::json!({
                "SubscribePlayer": {
                    "name": player_name,
                    "registration_token": token
                }
            }).to_string();
            let size = (msg.len() as u32).to_le_bytes();

            println!("📤 JoinGame Subscribe -> {}", msg);
            if s.write_all(&size).is_err() {
                eprintln!("❌ Erreur envoi taille JoinGame");
                return;
            }
            if s.write_all(msg.as_bytes()).is_err() {
                eprintln!("❌ Erreur envoi message JoinGame");
                return;
            }
            let mut sz_buf = [0; 4];
            if s.read_exact(&mut sz_buf).is_err() {
                eprintln!("❌ Erreur lecture taille réponse JoinGame");
                return;
            }
            let rsize = u32::from_le_bytes(sz_buf);
            let mut buffer = vec![0; rsize as usize];
            if s.read_exact(&mut buffer).is_err() {
                eprintln!("❌ Erreur lecture message JoinGame");
                return;
            }
            let resp = String::from_utf8_lossy(&buffer).to_string();
            println!("📩 Réponse JoinGame: {}", resp);
        } else {
            eprintln!("❌ Impossible de connecter pour JoinGame");
        }
    }

    /// Envoie une action MoveTo sur un nouveau flux.
    pub fn send_move_action_static(server_address: &str, dir: RelativeDirection) {
        if let Ok(mut s) = TcpStream::connect(server_address) {
            let msg = serde_json::json!({
                "Action": {
                    "MoveTo": format!("{:?}", dir)
                }
            }).to_string();
            let size = (msg.len() as u32).to_le_bytes();

            println!("📤 MoveTo: {}", msg);
            if s.write_all(&size).is_err() {
                eprintln!("❌ Erreur envoi taille MoveTo");
                return;
            }
            if s.write_all(msg.as_bytes()).is_err() {
                eprintln!("❌ Erreur envoi message MoveTo");
                return;
            }
        } else {
            eprintln!("❌ Impossible de se connecter pour MoveTo");
        }
    }
}
