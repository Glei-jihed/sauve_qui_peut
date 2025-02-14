use eframe::{egui, App, Frame};
use std::sync::mpsc::{Receiver, Sender};
use crate::team_gui::TeamRegistrationApp;
pub use crate::team_gui::RegistrationData; // Ré-exportation publique de RegistrationData
use crate::game_gui::GameView;

#[derive(Debug)]
pub enum AppState {
    Registration,
    Game { token: String },
}

#[derive(Debug)]
pub enum NetworkMessage {
    RegistrationComplete(String), // Le token
    RegistrationFailed(String),
}

pub struct MainApp {
    pub state: AppState,
    pub team_registration: TeamRegistrationApp,
    pub rx_net: Receiver<NetworkMessage>,
    pub tx_gui_net: Sender<RegistrationData>,
}

impl MainApp {
    pub fn new(rx_net: Receiver<NetworkMessage>, tx_gui_net: Sender<RegistrationData>) -> Self {
        Self {
            state: AppState::Registration,
            team_registration: TeamRegistrationApp::default(),
            rx_net,
            tx_gui_net,
        }
    }
}

impl App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Vérifier les messages du réseau pour changer l'état de l'application.
        while let Ok(msg) = self.rx_net.try_recv() {
            match msg {
                NetworkMessage::RegistrationComplete(token) => {
                    self.state = AppState::Game { token };
                },
                NetworkMessage::RegistrationFailed(err) => {
                    eprintln!("Registration failed: {}", err);
                }
            }
        }
        
        match &self.state {
            AppState::Registration => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.team_registration.ui(ui, |reg_data| {
                        if let Err(e) = self.tx_gui_net.send(reg_data) {
                            eprintln!("Erreur lors de l'envoi via le canal: {}", e);
                        }
                    });
                });
            },
            AppState::Game { token: _ } => {
                GameView::default().update(ctx, frame);
            },
        }
    }
}
