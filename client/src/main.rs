mod team_gui;
mod game;
mod network;
mod game_gui;
mod app;

use eframe::run_native;
use std::sync::mpsc::channel;
use crate::app::MainApp;
use crate::team_gui::RegistrationData;
use crate::game::GameClient;

fn main() {
    // Création des canaux
    let (tx_gui_net, rx_gui_net) = channel::<RegistrationData>();
    let (tx_net_gui, rx_net_gui) = channel::<app::NetworkMessage>();

    // Lancement du thread réseau
    std::thread::spawn(move || {
        if let Ok(reg_data) = rx_gui_net.recv() {
            match reg_data {
                RegistrationData::Create { team_name, team_members } => {
                    println!("[Network] Création d'équipe: {} {:?}", team_name, team_members);
                    let server_address = "127.0.0.1:8778";
                    let mut client = GameClient::new(server_address);
                    client.register_team(&team_name);
                    if let Some(token) = client.registration_token.clone() {
                        tx_net_gui.send(app::NetworkMessage::RegistrationComplete {
                            token: token.clone(),
                            team_name: team_name.clone(),
                            team_members: team_members.clone(),
                        }).ok();
                        for m in &team_members {
                            println!("[Network] Inscription du joueur: {}", m);
                            client.subscribe_player(m);
                        }
                        listen_server_loop(server_address.to_string(), tx_net_gui);
                    } else {
                        tx_net_gui.send(app::NetworkMessage::RegistrationFailed("Registration error".into())).ok();
                    }
                },
                RegistrationData::Join { token, player_name } => {
                    println!("[Network] Rejoindre l'équipe: token={} player={}", token, player_name);
                    let server_address = "127.0.0.1:8778";
                    GameClient::join_game(server_address, &token, &player_name);
                    tx_net_gui.send(app::NetworkMessage::RegistrationComplete {
                        token: token.clone(),
                        team_name: String::new(),
                        team_members: vec![],
                    }).ok();
                    listen_server_loop(server_address.to_string(), tx_net_gui);
                }
            }
        }
        loop {
            std::thread::sleep(std::time::Duration::from_secs(9999));
        }
    });

    let app = MainApp::new(rx_net_gui, tx_gui_net);
    let native_options = eframe::NativeOptions::default();
    let _ = run_native("Sauve Qui Peut - Main App", native_options, Box::new(|_cc| Box::new(app)));
}

fn listen_server_loop(server_address: String, tx_net_gui: std::sync::mpsc::Sender<app::NetworkMessage>) {
    use std::net::TcpStream;
    use crate::network::receive_message;
    let stream = TcpStream::connect(&server_address);
    if stream.is_err() {
        eprintln!("Impossible de se connecter à {} dans listen_server_loop", server_address);
        return;
    }
    let mut stream = stream.unwrap();
    stream.set_nonblocking(false).ok();

    loop {
        if let Some(json_str) = receive_message(&mut stream) {
            if json_str.contains("\"RadarView\"") {
                if let Ok(rv) = serde_json::from_str::<RadarViewWrapper>(&json_str) {
                    tx_net_gui.send(app::NetworkMessage::RadarViewReceived(rv.radar_view.0)).ok();
                }
            } else if json_str.contains("\"Hint\"") {
                if let Ok(hw) = serde_json::from_str::<HintWrapper>(&json_str) {
                    tx_net_gui.send(app::NetworkMessage::HintReceived(serde_json::to_value(hw.hint).unwrap())).ok();
                }
            } else if json_str.contains("\"Challenge\"") {
                if let Ok(cw) = serde_json::from_str::<ChallengeWrapper>(&json_str) {
                    tx_net_gui.send(app::NetworkMessage::ChallengeReceived(serde_json::to_value(cw.challenge).unwrap())).ok();
                }
            } else if json_str.contains("\"ActionError\"") {
                tx_net_gui.send(app::NetworkMessage::ActionErrorReceived(json_str)).ok();
            } else {
                eprintln!("Message inconnu: {}", json_str);
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}

#[derive(serde::Deserialize)]
struct RadarViewWrapper {
    #[serde(rename = "RadarView")]
    radar_view: shared::messages::RadarView,
}

#[derive(serde::Deserialize)]
struct HintWrapper {
    #[serde(rename = "Hint")]
    hint: shared::messages::Hint,
}

#[derive(serde::Deserialize)]
struct ChallengeWrapper {
    #[serde(rename = "Challenge")]
    challenge: shared::messages::Challenge,
}
