use eframe::{egui, Frame};
use egui::TextureOptions;
use std::path::Path;
use image::io::Reader as ImageReader;

#[derive(PartialEq)]
pub enum RegistrationMode {
    Create,
    Join,
}

#[derive(Debug)]
pub enum RegistrationData {
    Create { team_name: String, team_members: Vec<String> },
    Join { token: String, player_name: String },
}

/// Interface GUI pour la création ou la jonction d'une équipe.
pub struct TeamRegistrationApp {
    pub mode: RegistrationMode,
    // Pour le mode "Créer"
    pub team_name: String,
    pub team_members: Vec<String>,
    // Pour le mode "Rejoindre"
    pub join_token: String,
    pub player_name: String,
    pub title_texture: Option<egui::TextureHandle>,
}

impl Default for TeamRegistrationApp {
    fn default() -> Self {
        Self {
            mode: RegistrationMode::Create,
            team_name: String::new(),
            team_members: vec![String::new(), String::new(), String::new()],
            join_token: String::new(),
            player_name: String::new(),
            title_texture: None,
        }
    }
}

impl TeamRegistrationApp {
    /// Affiche l'interface de création/rejoindre et appelle le callback avec RegistrationData.
    pub fn ui<F: FnMut(RegistrationData)>(&mut self, ui: &mut egui::Ui, mut on_register: F) {
        // Charger l'image si non déjà chargée.
        if self.title_texture.is_none() {
            if let Some(texture) = load_title_image(ui.ctx(), "images/team-with-title.png") {
                self.title_texture = Some(texture);
            }
        }
        
        // Toggle entre les modes "Créer" et "Rejoindre"
        ui.horizontal(|ui| {
            if ui.selectable_label(self.mode == RegistrationMode::Create, "Créer une équipe").clicked() {
                self.mode = RegistrationMode::Create;
            }
            if ui.selectable_label(self.mode == RegistrationMode::Join, "Rejoindre une équipe").clicked() {
                self.mode = RegistrationMode::Join;
            }
        });
        
        ui.columns(2, |cols| {
            // Colonne gauche : Formulaire.
            cols[0].vertical(|ui| {
                match self.mode {
                    RegistrationMode::Create => {
                        ui.heading("Création d'équipe");
                        ui.add_space(10.0);
                        let team_name_id = egui::Id::new("team_name_edit");
                        let team_name_bg = if ui.memory(|mem| mem.has_focus(team_name_id)) {
                            egui::Color32::from_rgb(220, 240, 255)
                        } else {
                            egui::Color32::WHITE
                        };
                        ui.label("Nom de l'équipe :");
                        egui::Frame::none()
                            .fill(team_name_bg)
                            .rounding(egui::Rounding::same(5.0))
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [300.0, 40.0],
                                    egui::TextEdit::singleline(&mut self.team_name).id(team_name_id)
                                );
                            });
                        ui.add_space(10.0);
                        ui.heading("Membres de l'équipe");
                        for i in 0..self.team_members.len() {
                            let member_id = egui::Id::new(format!("team_member_{}", i));
                            let member_bg = if ui.memory(|mem| mem.has_focus(member_id)) {
                                egui::Color32::from_rgb(220, 240, 255)
                            } else {
                                egui::Color32::WHITE
                            };
                            ui.horizontal(|ui| {
                                ui.label(format!("Membre {} :", i + 1));
                                egui::Frame::none()
                                    .fill(member_bg)
                                    .rounding(egui::Rounding::same(5.0))
                                    .show(ui, |ui| {
                                        ui.add_sized(
                                            [300.0, 40.0],
                                            egui::TextEdit::singleline(&mut self.team_members[i]).id(member_id)
                                        );
                                    });
                            });
                            ui.add_space(5.0);
                        }
                        ui.add_space(10.0);
                        if ui.button("Enregistrer l'équipe").clicked() {
                            on_register(RegistrationData::Create {
                                team_name: self.team_name.clone(),
                                team_members: self.team_members.clone(),
                            });
                        }
                    },
                    RegistrationMode::Join => {
                        ui.heading("Rejoindre une équipe");
                        ui.add_space(10.0);
                        let token_id = egui::Id::new("join_token_edit");
                        let token_bg = if ui.memory(|mem| mem.has_focus(token_id)) {
                            egui::Color32::from_rgb(220, 240, 255)
                        } else {
                            egui::Color32::WHITE
                        };
                        ui.label("Token de l'équipe :");
                        egui::Frame::none()
                            .fill(token_bg)
                            .rounding(egui::Rounding::same(5.0))
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [300.0, 40.0],
                                    egui::TextEdit::singleline(&mut self.join_token).id(token_id)
                                );
                            });
                        ui.add_space(10.0);
                        let player_id = egui::Id::new("player_name_edit");
                        let player_bg = if ui.memory(|mem| mem.has_focus(player_id)) {
                            egui::Color32::from_rgb(220, 240, 255)
                        } else {
                            egui::Color32::WHITE
                        };
                        ui.label("Nom du joueur :");
                        egui::Frame::none()
                            .fill(player_bg)
                            .rounding(egui::Rounding::same(5.0))
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [300.0, 40.0],
                                    egui::TextEdit::singleline(&mut self.player_name).id(player_id)
                                );
                            });
                        ui.add_space(10.0);
                        if ui.button("Rejoindre l'équipe").clicked() {
                            on_register(RegistrationData::Join {
                                token: self.join_token.clone(),
                                player_name: self.player_name.clone(),
                            });
                        }
                    }
                }
            });
            
            // Colonne droite : Affichage de l'image et du texte animé.
            cols[1].vertical(|ui| {
                if let Some(ref tex) = self.title_texture {
                    let desired_height = 350.0;
                    let texture_size = tex.size_vec2();
                    let aspect_ratio = texture_size.x / texture_size.y;
                    let desired_width = desired_height * aspect_ratio;
                    ui.image(tex, egui::Vec2::new(desired_width, desired_height));
                } else {
                    ui.label("Image non chargée");
                }
                ui.add_space(20.0);
                let desired_width = if let Some(ref tex) = self.title_texture {
                    let desired_height = 350.0;
                    let texture_size = tex.size_vec2();
                    desired_height * (texture_size.x / texture_size.y)
                } else {
                    350.0
                };
                ui.allocate_ui(egui::Vec2::new(desired_width, 80.0), |ui| {
                    let t = ui.ctx().input(|i| i.time) as f32;
                    let base_font_size = 24.0;
                    let amplitude = 12.0;
                    let animated_font_size = base_font_size + amplitude * (t.sin() * 0.5 + 0.5);
                    let factor = t.sin() * 0.5 + 0.5;
                    let red = (factor * 255.0) as u8;
                    let animated_color = egui::Color32::from_rgb(red, 0, 0);
                    let animated_text = egui::RichText::new("Sauve qui peut... en sortirez-vous vivants ?")
                        .color(animated_color)
                        .size(animated_font_size);
                    ui.label(animated_text);
                });
            });
        });
    }
}

fn load_title_image(ctx: &egui::Context, path: &str) -> Option<egui::TextureHandle> {
    let img = ImageReader::open(Path::new(path)).ok()?.decode().ok()?;
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture("team_title", color_image, TextureOptions::default()))
}
