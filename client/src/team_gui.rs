use eframe::{egui, App, Frame};
use std::sync::mpsc::Sender;

/// Application GUI pour la création d'équipe.
pub struct TeamRegistrationApp {
    pub team_name: String,
    pub team_members: Vec<String>,
}

impl Default for TeamRegistrationApp {
    fn default() -> Self {
        Self {
            team_name: String::new(),
            // Par défaut, nous prévoyons 3 membres ; vous pouvez ajuster selon vos besoins.
            team_members: vec![String::new(), String::new(), String::new()],
        }
    }
}

/// Wrapper de l'application GUI incluant un canal pour transmettre les infos de création d'équipe.
pub struct TeamRegistrationAppWithChannel {
    pub inner: TeamRegistrationApp,
    pub tx: Sender<(String, Vec<String>)>,
}

impl App for TeamRegistrationAppWithChannel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Création d'équipe");

            ui.label("Nom de l'équipe :");
            ui.text_edit_singleline(&mut self.inner.team_name);

            ui.separator();
            ui.heading("Membres de l'équipe");
            for (i, member) in self.inner.team_members.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Membre {} :", i + 1));
                    ui.text_edit_singleline(member);
                });
            }

            ui.separator();
            if ui.button("Enregistrer l'équipe").clicked() {
                println!("Enregistrement de l'équipe : {}", self.inner.team_name);
                println!("Membres : {:?}", self.inner.team_members);
                // Envoi des infos via le canal au thread réseau.
                if let Err(e) = self.tx.send((self.inner.team_name.clone(), self.inner.team_members.clone())) {
                    eprintln!("Erreur lors de l'envoi via le canal: {}", e);
                }
            }
        });
    }
}
