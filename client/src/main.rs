mod team_gui;
mod game;
mod network; // (déjà présent pour la logique réseau si besoin)

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use eframe::run_native;
use team_gui::TeamRegistrationAppWithChannel;

fn main() {
    // Création d'un canal pour transmettre les infos de création d'équipe (nom et membres)
    let (tx, rx) = mpsc::channel::<(String, Vec<String>)>();

    // Lancement du thread réseau qui attend les infos de création d'équipe
    let network_handle = thread::spawn(move || {
        // Attente de réception des infos de l'interface GUI
        if let Ok((team_name, team_members)) = rx.recv() {
            println!("[Network] Infos reçues : {} {:?}", team_name, team_members);
            // Connexion au serveur
            let server_address = "127.0.0.1:8778";
            let mut client = game::GameClient::new(server_address);
            // Envoi de la demande d'enregistrement d'équipe
            client.register_team(&team_name);
            // Pour chaque membre, on peut appeler subscribe_player (à implémenter ultérieurement)
            for member in team_members {
                println!("[Network] Inscription du joueur : {}", member);
                // client.subscribe_player(&member);
            }
        }
        // Boucle de maintien du thread réseau
        loop {
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Lancement de l'interface graphique sur le thread principal,
    // en passant le transmetteur (tx) via un wrapper.
    let app = TeamRegistrationAppWithChannel {
        inner: team_gui::TeamRegistrationApp::default(),
        tx,
    };
    let native_options = eframe::NativeOptions::default();
    let _ = run_native(
        "Sauve Qui Peut - Création d'équipe",
        native_options,
        Box::new(|_cc| -> Box<dyn eframe::App> {
            Box::new(app)
        })
    );

    network_handle.join().unwrap();
}
