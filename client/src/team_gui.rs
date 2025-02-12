use eframe::{egui, App, Frame};

/// Application GUI pour la création d'équipe.
pub struct TeamRegistrationApp {
    pub team_name: String,
    pub team_members: Vec<String>,
}

impl Default for TeamRegistrationApp {
    fn default() -> Self {
        Self {
            team_name: String::new(),
            // Par défaut, on prévoit 3 membres ; ajustez selon vos besoins.
            team_members: vec![String::new(), String::new(), String::new()],
        }
    }
}

impl App for TeamRegistrationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Création d'équipe");

            ui.label("Nom de l'équipe :");
            ui.text_edit_singleline(&mut self.team_name);

            ui.separator();
            ui.heading("Membres de l'équipe");
            for (i, member) in self.team_members.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Membre {} :", i + 1));
                    ui.text_edit_singleline(member);
                });
            }

            ui.separator();
            if ui.button("Enregistrer l'équipe").clicked() {
                println!("Enregistrement de l'équipe : {}", self.team_name);
                println!("Membres : {:?}", self.team_members);
                // Vous pourrez ajouter ici la logique pour envoyer ces données au thread réseau.
            }
        });
    }
}
