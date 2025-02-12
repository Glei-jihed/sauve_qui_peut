use eframe::{egui, App, Frame};
use egui::TextureOptions;
use std::sync::mpsc::Sender;
use std::path::Path;
use image::io::Reader as ImageReader;

/// Application GUI pour la création d'équipe.
pub struct TeamRegistrationApp {
    pub team_name: String,
    pub team_members: Vec<String>,
    // Texture pour l'image de titre.
    pub title_texture: Option<egui::TextureHandle>,
}

impl Default for TeamRegistrationApp {
    fn default() -> Self {
        Self {
            team_name: String::new(),
            // Par défaut, prévoyez 3 membres.
            team_members: vec![String::new(), String::new(), String::new()],
            title_texture: None,
        }
    }
}

/// Wrapper de l'application GUI qui inclut un canal pour transmettre les infos de création d'équipe.
pub struct TeamRegistrationAppWithChannel {
    pub inner: TeamRegistrationApp,
    pub tx: Sender<(String, Vec<String>)>,
}

impl App for TeamRegistrationAppWithChannel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Charger l'image de titre si elle n'est pas encore chargée.
        if self.inner.title_texture.is_none() {
            if let Some(texture) = load_title_image(ctx, "images/team.jpeg") {
                self.inner.title_texture = Some(texture);
            } else {
                eprintln!("Erreur de chargement de l'image de titre");
            }
        }
        
        // Diviser l'écran en deux colonnes égales.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |cols| {
                // --------------------
                // Colonne gauche : Formulaire de création d'équipe
                // --------------------
                cols[0].vertical(|ui| {
                    ui.heading("Création d'équipe");
                    ui.add_space(10.0);
                    
                    // Champ pour le nom de l'équipe.
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
                                egui::TextEdit::singleline(&mut self.inner.team_name)
                                    .id(team_name_id)
                            );
                        });
                    
                    ui.add_space(10.0);
                    ui.heading("Membres de l'équipe");
                    for i in 0..self.inner.team_members.len() {
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
                                        egui::TextEdit::singleline(&mut self.inner.team_members[i])
                                            .id(member_id)
                                    );
                                });
                        });
                        ui.add_space(5.0);
                    }
                    
                    ui.add_space(10.0);
                    let button_response = ui.add_sized([300.0, 40.0], egui::Button::new("Enregistrer l'équipe"));
                    if button_response.clicked() {
                        println!("Enregistrement de l'équipe : {}", self.inner.team_name);
                        println!("Membres : {:?}", self.inner.team_members);
                        if let Err(e) = self.tx.send((self.inner.team_name.clone(), self.inner.team_members.clone())) {
                            eprintln!("Erreur lors de l'envoi via le canal: {}", e);
                        }
                    }
                    ui.add_space(10.0);
                });
                
                // --------------------
                // Colonne droite : Image agrandie et texte animé
                // --------------------
                cols[1].vertical(|ui| {
                    // Affichage de l'image agrandie.
                    if let Some(ref tex) = self.inner.title_texture {
                        let desired_height = 350.0;
                        let texture_size = tex.size_vec2();
                        let aspect_ratio = texture_size.x / texture_size.y;
                        let desired_width = desired_height * aspect_ratio;
                        ui.image(tex, egui::Vec2::new(desired_width, desired_height));
                    } else {
                        ui.label("Image non chargée");
                    }
                    
                    ui.add_space(20.0);
                    
                    // Texte animé : le texte ne dépasse pas la largeur de l'image.
                    // On utilise ui.allocate_ui pour fixer une zone de largeur maximale égale à desired_width.
                    let desired_width = if let Some(ref tex) = self.inner.title_texture {
                        let desired_height = 350.0;
                        let texture_size = tex.size_vec2();
                        desired_height * (texture_size.x / texture_size.y)
                    } else {
                        350.0
                    };
                    ui.allocate_ui(egui::Vec2::new(desired_width, 80.0), |ui| {
                        let t = ctx.input(|i| i.time) as f32;
                        let base_font_size = 24.0;
                        let amplitude = 12.0;
                        let animated_font_size = base_font_size + amplitude * (t.sin() * 0.5 + 0.5);
                        // Animation de couleur du texte : de noir à rouge.
                        let factor = (t.sin() * 0.5 + 0.5);
                        let red = (factor * 255.0) as u8;
                        let animated_color = egui::Color32::from_rgb(red, 0, 0);
                        let animated_text = egui::RichText::new(" Sauve qui peut ... En sortirez-vous vivants ?")
                            .color(animated_color)
                            .size(animated_font_size);
                        ui.label(animated_text);
                    });
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
