use eframe::{egui, App, Frame};
use std::sync::mpsc::{Receiver, Sender};
use crate::team_gui::{TeamRegistrationApp, RegistrationData};
use crate::game_gui::GameView;
use shared::messages::RelativeDirection;
use std::env;
use std::path::Path;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use egui::TextureOptions;

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
    RadarViewReceived(String),
    HintReceived(serde_json::Value),
    ChallengeReceived(serde_json::Value),
    ActionErrorReceived(String),
}

pub struct MainApp {
    pub state: AppState,
    pub team_registration: TeamRegistrationApp,
    pub rx_net: Receiver<NetworkMessage>,
    pub tx_gui_net: Sender<RegistrationData>,
    /// Texture pour le fond par défaut ("random_maze.png")
    pub maze_texture: Option<egui::TextureHandle>,
    /// Texture pour le fond lors d'une action ("random_maze_in_action.png")
    pub in_game_texture: Option<egui::TextureHandle>,
    /// Angle de rotation cumulée pour le cube animé
    pub rotation_angle: f32,
    /// Indique qu'une action de déplacement est en cours (direction et timestamp)
    pub active_move: Option<(RelativeDirection, f64)>,
}

impl MainApp {
    pub fn new(rx_net: Receiver<NetworkMessage>, tx_gui_net: Sender<RegistrationData>) -> Self {
        Self {
            state: AppState::Registration,
            team_registration: TeamRegistrationApp::default(),
            rx_net,
            tx_gui_net,
            maze_texture: None,
            in_game_texture: None,
            rotation_angle: 0.0,
            active_move: None,
        }
    }
}

/// Charge une texture depuis un chemin relatif, en construisant un chemin absolu basé sur le répertoire courant.
fn load_texture(ctx: &egui::Context, relative_path: &str) -> Option<egui::TextureHandle> {
    let current_dir = env::current_dir().ok()?;
    let full_path = current_dir.join(relative_path);
    println!("Loading texture from: {}", full_path.display());
    let reader = ImageReader::open(&full_path).ok()?;
    let img: DynamicImage = reader.decode().ok()?;
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(full_path.to_string_lossy(), color_image, TextureOptions::default()))
}

/// Crée un bouton de flèche avec rétroaction visuelle.
fn arrow_button(ui: &mut egui::Ui, label: &str, active: bool) -> egui::Response {
    ui.add_sized(
        [70.0, 70.0],
        egui::Button::new(egui::RichText::new(label).size(36.0))
            .fill(if active { egui::Color32::from_rgb(255, 0, 0) } else { egui::Color32::from_gray(80) }),
    )
}

impl App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Charger les textures si nécessaire.
        if self.maze_texture.is_none() {
            self.maze_texture = load_texture(ctx, "images/random_maze.png");
        }
        if self.in_game_texture.is_none() {
            self.in_game_texture = load_texture(ctx, "images/random_maze_in_action.png");
        }

        // Incrémenter l'angle de rotation pour l'animation du cube.
        self.rotation_angle += 0.015;

        // Gestion du délai d'affichage du fond "in_game" lors d'une action.
        let now = ctx.input(|i| i.time);
        if let Some((_, timestamp)) = self.active_move {
            if now - timestamp > 1.0 {
                self.active_move = None;
            }
        }

        // Traiter les messages réseau.
        while let Ok(msg) = self.rx_net.try_recv() {
            match msg {
                NetworkMessage::RegistrationComplete { token, team_name, team_members } => {
                    self.state = AppState::GameSetup { team_name, team_members, token };
                }
                NetworkMessage::RegistrationFailed(err) => {
                    eprintln!("Registration failed: {}", err);
                }
                NetworkMessage::RadarViewReceived(rv) => {
                    println!("RadarView reçue: {}", rv);
                }
                NetworkMessage::HintReceived(hint) => {
                    println!("Hint reçu: {:?}", hint);
                }
                NetworkMessage::ChallengeReceived(challenge) => {
                    println!("Challenge reçu: {:?}", challenge);
                }
                NetworkMessage::ActionErrorReceived(err) => {
                    eprintln!("ActionError reçu: {}", err);
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
                        // Colonne gauche : Infos d'équipe et bouton de lancement.
                        cols[0].vertical(|ui| {
                            ui.heading(egui::RichText::new("Détails de la Partie").size(28.0).color(egui::Color32::WHITE));
                            ui.separator();
                            ui.label(egui::RichText::new("Nom de l'équipe :").size(20.0).color(egui::Color32::LIGHT_GRAY));
                            ui.label(egui::RichText::new(&team_name_cl).size(24.0).color(egui::Color32::WHITE));
                            ui.separator();
                            ui.label(egui::RichText::new("Membres :").size(20.0).color(egui::Color32::LIGHT_GRAY));
                            for member in &team_members_cl {
                                ui.label(egui::RichText::new(member).size(22.0).color(egui::Color32::WHITE));
                            }
                            ui.separator();
                            ui.label(egui::RichText::new("Token :").size(20.0).color(egui::Color32::LIGHT_GRAY));
                            ui.label(egui::RichText::new(&token_cl).size(22.0).color(egui::Color32::WHITE));
                            ui.separator();
                            ui.add_space(20.0);
                            // Bouton pour lancer la partie.
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
                        // Colonne droite : Affichage du cube animé.
                        cols[1].vertical(|ui| {
                            ui.separator();
                            let available_size = ui.available_size();
                            let square_side = available_size.y.min(available_size.x);
                            let (rect, _resp) = ui.allocate_exact_size(egui::Vec2::new(square_side, square_side), egui::Sense::hover());
                            let background_tex = if self.active_move.is_some() {
                                self.in_game_texture.as_ref().unwrap()
                            } else {
                                self.maze_texture.as_ref().unwrap()
                            };
                            ui.painter().image(
                                background_tex.id(),
                                rect,
                                egui::Rect::from_min_size(egui::Pos2::ZERO, background_tex.size_vec2()),
                                egui::Color32::WHITE,
                            );
                            // Dessiner un carré tournant (simulation d'un cube).
                            let angle = self.rotation_angle;
                            let half = square_side / 4.0;
                            let corners = [
                                egui::Pos2::new(-half, -half),
                                egui::Pos2::new(half, -half),
                                egui::Pos2::new(half, half),
                                egui::Pos2::new(-half, half),
                            ];
                            let rotated_corners: Vec<egui::Pos2> = corners.iter().map(|p| {
                                let rx = p.x * angle.cos() - p.y * angle.sin();
                                let ry = p.x * angle.sin() + p.y * angle.cos();
                                egui::Pos2::new(rect.center().x + rx, rect.center().y + ry)
                            }).collect();
                            let mut mesh = egui::Mesh::with_texture(self.maze_texture.as_ref().unwrap().id());
                            let color = egui::Color32::WHITE;
                            let i0 = mesh.vertices.len() as u32;
                            mesh.vertices.extend_from_slice(&[
                                egui::epaint::Vertex { pos: rotated_corners[0], uv: egui::pos2(0.0, 0.0), color },
                                egui::epaint::Vertex { pos: rotated_corners[1], uv: egui::pos2(1.0, 0.0), color },
                                egui::epaint::Vertex { pos: rotated_corners[2], uv: egui::pos2(1.0, 1.0), color },
                                egui::epaint::Vertex { pos: rotated_corners[3], uv: egui::pos2(0.0, 1.0), color },
                            ]);
                            mesh.indices.extend_from_slice(&[i0, i0 + 1, i0 + 2, i0, i0 + 2, i0 + 3]);
                            ui.painter().add(egui::Shape::mesh(mesh));
                        });
                    });
                });
            },
            // ÉTAT : Partie lancée – affichage de la vue radar et des contrôles.
            AppState::Game { team_name, team_members, token } => {
                // Panneau d'infos en haut.
                egui::TopBottomPanel::top("game_info").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("Équipe : {}", team_name)).size(20.0).color(egui::Color32::WHITE));
                        ui.separator();
                        ui.label(egui::RichText::new("Membres :").size(18.0).color(egui::Color32::LIGHT_GRAY));
                        for member in team_members.iter() {
                            ui.label(egui::RichText::new(member).color(egui::Color32::WHITE));
                        }
                        ui.separator();
                        ui.label(egui::RichText::new(format!("Token : {}", token)).size(18.0).color(egui::Color32::WHITE));
                    });
                });
                // Panneau central : affichage du fond et de la vue radar.
                egui::CentralPanel::default().show(ctx, |ui| {
                    let background_tex = if self.active_move.is_some() {
                        self.in_game_texture.as_ref().unwrap()
                    } else {
                        self.maze_texture.as_ref().unwrap()
                    };
                    let available = ui.available_size();
                    let rect = egui::Rect::from_min_size(ui.min_rect().min, available);
                    ui.painter().image(
                        background_tex.id(),
                        rect,
                        egui::Rect::from_min_size(egui::Pos2::ZERO, background_tex.size_vec2()),
                        egui::Color32::WHITE,
                    );
                    GameView::default().update(ctx, frame);
                });
                // Panneau de contrôles en bas.
                egui::TopBottomPanel::bottom("move_controls").show(ctx, |ui| {
                    let current_time = ctx.input(|i| i.time);
                    let active_dir = self.active_move.as_ref().map(|(d, _)| d.clone());
                    ui.horizontal_centered(|ui| {
                        if arrow_button(ui, "⬅️", active_dir == Some(RelativeDirection::Left)).clicked() {
                            self.active_move = Some((RelativeDirection::Left, current_time));
                            crate::game::GameClient::send_move_action_static("127.0.0.1:8778", RelativeDirection::Left);
                        }
                        if arrow_button(ui, "⬆️", active_dir == Some(RelativeDirection::Front)).clicked() {
                            self.active_move = Some((RelativeDirection::Front, current_time));
                            crate::game::GameClient::send_move_action_static("127.0.0.1:8778", RelativeDirection::Front);
                        }
                        if arrow_button(ui, "➡️", active_dir == Some(RelativeDirection::Right)).clicked() {
                            self.active_move = Some((RelativeDirection::Right, current_time));
                            crate::game::GameClient::send_move_action_static("127.0.0.1:8778", RelativeDirection::Right);
                        }
                        if arrow_button(ui, "⬇️", active_dir == Some(RelativeDirection::Back)).clicked() {
                            self.active_move = Some((RelativeDirection::Back, current_time));
                            crate::game::GameClient::send_move_action_static("127.0.0.1:8778", RelativeDirection::Back);
                        }
                    });
                });
            }
        }
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
