// client/src/main.rs
mod team_gui;
mod game;
mod network;
mod game_gui;
mod app;

use eframe::run_native;
use app::{MainApp, NetworkMessage};
use crate::team_gui::RegistrationData; // Importez directement depuis team_gui
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;
use crate::game::GameClient;

fn main() {
    // Canal pour transmettre les infos de création/join d'équipe du GUI vers le thread réseau.
    let (tx_gui_net, rx_gui_net): (Sender<RegistrationData>, Receiver<RegistrationData>) = channel();
    // Canal pour transmettre les réponses du réseau vers le GUI.
    let (tx_net_gui, rx_net_gui): (Sender<NetworkMessage>, Receiver<NetworkMessage>) = channel();

    // Lancement du thread réseau.
    thread::spawn(move || {
        if let Ok(reg_data) = rx_gui_net.recv() {
            match reg_data {
                RegistrationData::Create { team_name, team_members } => {
                    println!("[Network] Infos reçues (Create): {} {:?}", team_name, team_members);
                    let server_address = "127.0.0.1:8778";
                    let mut client = GameClient::new(server_address);
                    client.register_team(&team_name);
                    if let Some(token) = client.registration_token.clone() {
                        tx_net_gui.send(NetworkMessage::RegistrationComplete {
                            token: token.clone(),
                            team_name: team_name.clone(),
                            team_members: team_members.clone(),
                        }).unwrap();
                        for member in team_members.iter() {
                            println!("[Network] Inscription du joueur : {}", member);
                            client.subscribe_player(member);
                        }
                    } else {
                        tx_net_gui.send(NetworkMessage::RegistrationFailed("Registration error".into())).unwrap();
                    }
                },
                RegistrationData::Join { token, player_name } => {
                    println!("[Network] Infos reçues (Join): token={} player={}", token, player_name);
                    let server_address = "127.0.0.1:8778";
                    GameClient::join_game(server_address, &token, &player_name);
                    tx_net_gui.send(NetworkMessage::RegistrationComplete {
                        token: token.clone(),
                        team_name: String::new(), // Optionnel : nom d'équipe vide pour le mode Join
                        team_members: Vec::new(),
                    }).unwrap();
                }
            }
        }
        loop {
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Lancement de l'application principale.
    let app = MainApp::new(rx_net_gui, tx_gui_net);
    let native_options = eframe::NativeOptions::default();
    let _ = run_native(
        "Sauve Qui Peut - Main App",
        native_options,
        Box::new(|_cc| -> Box<dyn eframe::App> {
            Box::new(app)
        })
    );
}
