// client/src/app.rs
use eframe::{egui, App, Frame};
use std::sync::mpsc::{Receiver, Sender};
use crate::team_gui::{TeamRegistrationApp, RegistrationData};
use crate::game_gui::GameView;
use shared::messages::RelativeDirection; // Pour utiliser RelativeDirection
use std::time::Duration;
use std::thread;

#[derive(Debug, Clone)]
pub enum AppState {
    Registration,
    GameSetup {
        team_name: String,
        team_members: Vec<String>,
        token: String,
    },
    Game {
        team_name: String,
        team_members: Vec<String>,
        token: String,
    },
}

#[derive(Debug)]
pub enum NetworkMessage {
    RegistrationComplete {
        token: String,
        team_name: String,
        team_members: Vec<String>,
    },
    RegistrationFailed(String),
}

pub struct MainApp {
    pub state: AppState,
    pub team_registration: TeamRegistrationApp,
    pub rx_net: Receiver<NetworkMessage>,
    pub tx_gui_net: Sender<RegistrationData>,
    pub maze_texture: Option<egui::TextureHandle>,
    pub rotation_angle: f32,
}

impl MainApp {
    pub fn new(rx_net: Receiver<NetworkMessage>, tx_gui_net: Sender<RegistrationData>) -> Self {
        Self {
            state: AppState::Registration,
            team_registration: TeamRegistrationApp::default(),
            rx_net,
            tx_gui_net,
            maze_texture: None,
            rotation_angle: 0.0,
        }
    }
}

/// Charge la texture depuis "images/random_maze.png"
fn load_maze_texture(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    use image::io::Reader as ImageReader;
    use image::DynamicImage;
    use std::path::Path;

    let path = Path::new("images/random_maze.png");
    let reader = ImageReader::open(path).ok()?;
    let img: DynamicImage = reader.decode().ok()?;
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture("random_maze", color_image, egui::TextureOptions::default()))
}

impl App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Charger la texture si nécessaire
        if self.maze_texture.is_none() {
            self.maze_texture = load_maze_texture(ctx);
        }
        // Incrémenter l'angle de rotation pour le cube animé
        self.rotation_angle += 0.015;

        // Traitement des messages du réseau
        while let Ok(msg) = self.rx_net.try_recv() {
            match msg {
                NetworkMessage::RegistrationComplete { token, team_name, team_members } => {
                    self.state = AppState::GameSetup { team_name, team_members, token };
                }
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
            }
            AppState::GameSetup { team_name, team_members, token } => {
                let team_name_cl = team_name.clone();
                let team_members_cl = team_members.clone();
                let token_cl = token.clone();

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.columns(2, |cols| {
                        // Colonne gauche : Infos d'équipe et bouton pour lancer la partie
                        cols[0].vertical(|ui| {
                            ui.heading(egui::RichText::new("Détails de la Partie")
                                .size(28.0)
                                .color(egui::Color32::WHITE));
                            ui.separator();

                            ui.label(egui::RichText::new("Nom de l'équipe :")
                                .size(20.0)
                                .color(egui::Color32::LIGHT_GRAY));
                            ui.label(egui::RichText::new(team_name_cl.as_str())
                                .size(24.0)
                                .color(egui::Color32::WHITE));
                            ui.separator();

                            ui.label(egui::RichText::new("Membres :")
                                .size(20.0)
                                .color(egui::Color32::LIGHT_GRAY));
                            for member in &team_members_cl {
                                ui.label(egui::RichText::new(member)
                                    .size(22.0)
                                    .color(egui::Color32::WHITE));
                            }
                            ui.separator();

                            ui.label(egui::RichText::new("Token :")
                                .size(20.0)
                                .color(egui::Color32::LIGHT_GRAY));
                            ui.label(egui::RichText::new(token_cl.as_str())
                                .size(22.0)
                                .color(egui::Color32::WHITE));
                            ui.separator();

                            ui.add_space(20.0);
                            // Bouton pour lancer la partie
                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() * 0.1);
                                let big_button = ui.add_sized(
                                    [ui.available_width() * 0.8, 50.0],
                                    egui::Button::new(egui::RichText::new("Lancer la partie").size(24.0))
                                );
                                if big_button.clicked() {
                                    self.state = AppState::Game {
                                        team_name: team_name_cl.clone(),
                                        team_members: team_members_cl.clone(),
                                        token: token_cl.clone(),
                                    };
                                }
                            });
                        });

                        // Colonne droite : Cube texturé tournant avec image de fond
                        cols[1].vertical(|ui| {
                            ui.separator();
                            let available_size = ui.available_size();
                            let square_side = available_size.y.min(available_size.x);
                            let (rect, _resp) = ui.allocate_exact_size(
                                egui::Vec2::new(square_side, square_side),
                                egui::Sense::hover(),
                            );

                            // Dessiner l'image de fond dans la zone
                            if let Some(tex) = &self.maze_texture {
                                ui.painter().image(
                                    tex.id(),
                                    rect,
                                    egui::Rect::from_min_size(egui::Pos2::ZERO, tex.size_vec2()),
                                    egui::Color32::WHITE,
                                );
                            }

                            // Dessiner un carré tournant (simulateur de cube) par-dessus
                            let angle = self.rotation_angle;
                            let half = square_side / 4.0;
                            let corners = [
                                egui::Pos2::new(-half, -half),
                                egui::Pos2::new( half, -half),
                                egui::Pos2::new( half,  half),
                                egui::Pos2::new(-half,  half),
                            ];
                            let rotated_corners: Vec<egui::Pos2> = corners.iter().map(|p| {
                                let rx = p.x * angle.cos() - p.y * angle.sin();
                                let ry = p.x * angle.sin() + p.y * angle.cos();
                                egui::Pos2::new(rect.center().x + rx, rect.center().y + ry)
                            }).collect();

                            // Couleur du cube en noir/gris
                            let cube_color = egui::Color32::from_rgb(
                                (angle.cos() * 70.0 + 70.0) as u8,
                                (angle.sin() * 70.0 + 70.0) as u8,
                                (angle.sin() * 70.0 + 70.0) as u8,
                            );
                            let mut mesh = egui::Mesh::with_texture(self.maze_texture.as_ref().unwrap().id());
                            let color = egui::Color32::WHITE;
                            let v0 = egui::epaint::Vertex {
                                pos: rotated_corners[0],
                                uv: egui::pos2(0.0, 0.0),
                                color,
                            };
                            let v1 = egui::epaint::Vertex {
                                pos: rotated_corners[1],
                                uv: egui::pos2(1.0, 0.0),
                                color,
                            };
                            let v2 = egui::epaint::Vertex {
                                pos: rotated_corners[2],
                                uv: egui::pos2(1.0, 1.0),
                                color,
                            };
                            let v3 = egui::epaint::Vertex {
                                pos: rotated_corners[3],
                                uv: egui::pos2(0.0, 1.0),
                                color,
                            };
                            let i0 = mesh.vertices.len() as u32;
                            mesh.vertices.extend_from_slice(&[v0, v1, v2, v3]);
                            mesh.indices.extend_from_slice(&[i0, i0 + 1, i0 + 2, i0, i0 + 2, i0 + 3]);
                            ui.painter().add(egui::Shape::mesh(mesh));
                        });
                    });
                });
            }

            AppState::Game { team_name, team_members, token } => {
                egui::TopBottomPanel::top("game_info").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("Équipe : {}", team_name))
                            .size(20.0)
                            .color(egui::Color32::WHITE));
                        ui.separator();
                        ui.label(egui::RichText::new("Membres :")
                            .size(18.0)
                            .color(egui::Color32::LIGHT_GRAY));
                        for member in team_members.iter() {
                            ui.label(egui::RichText::new(member)
                                .color(egui::Color32::WHITE));
                        }
                        ui.separator();
                        ui.label(egui::RichText::new(format!("Token : {}", token))
                            .size(18.0)
                            .color(egui::Color32::WHITE));
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    GameView::default().update(ctx, frame);
                });
            }
        }
    }
}
