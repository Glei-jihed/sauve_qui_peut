use std::net::TcpStream;
use shared::messages::{RegisterTeam, RegisterTeamResult, SubscribePlayer};
use serde_json;

pub struct GameClient {
    pub stream: TcpStream,
    pub registration_token: Option<String>,
}

impl GameClient {
    pub fn new(server_address: &str) -> Self {
        let stream = TcpStream::connect(server_address).expect("Could not connect to server");
        GameClient { stream, registration_token: None }
    }

    pub fn register_team(&mut self, team_name: &str) {
        let register_team = RegisterTeam { name: team_name.to_string() };
        let message = serde_json::to_string(&register_team).unwrap();
        self.stream.write_all(message.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let size = self.stream.read(&mut buffer).unwrap();
        let response: String = String::from_utf8_lossy(&buffer[..size]).to_string();

        if let Ok(result) = serde_json::from_str::<RegisterTeamResult>(&response) {
            println!("Team registered: {:?}", result);
            self.registration_token = Some(result.registration_token);
        }
    }

    pub fn subscribe_player(&mut self, player_name: &str) {
        if let Some(token) = &self.registration_token {
            let subscribe_player = SubscribePlayer {
                name: player_name.to_string(),
                registration_token: token.clone(),
            };
            let message = serde_json::to_string(&subscribe_player).unwrap();
            self.stream.write_all(message.as_bytes()).unwrap();

            let mut buffer = [0; 512];
            let size = self.stream.read(&mut buffer).unwrap();
            let response: String = String::from_utf8_lossy(&buffer[..size]).to_string();

            println!("Player subscribed: {:?}", response);
        }
    }
}
