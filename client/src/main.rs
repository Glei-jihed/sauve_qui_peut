mod team_gui;

use eframe::run_native;
use team_gui::TeamRegistrationApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = run_native(
        "Sauve Qui Peut - Création d'équipe", // Titre de la fenêtre
        native_options,
        Box::new(|_cc| -> Box<dyn eframe::App> {
            Box::new(TeamRegistrationApp::default())
        })
    );
}
