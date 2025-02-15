use eframe::{egui, App, Frame};
use rand::Rng; // Ajoutez la dépendance `rand` dans votre Cargo.toml pour ce module

/// Vue du jeu qui simule dynamiquement le RadarView du labyrinthe.
/// Affiche une grille 7x7 avec la cellule centrale en vert.
/// La grille est mise à jour toutes les 2 secondes pour simuler des changements.
pub struct GameView {
    pub grid: Vec<Vec<bool>>,
    pub player_pos: (usize, usize),
    pub last_update: f64,
}

impl Default for GameView {
    fn default() -> Self {
        // Initialisation d'une grille statique
        let grid = vec![
            vec![true, true, true, true, true, true, true],
            vec![true, false, false, false, false, false, true],
            vec![true, false, true, false, true, false, true],
            vec![true, false, false, false, false, false, true],
            vec![true, false, true, false, true, false, true],
            vec![true, false, false, false, false, false, true],
            vec![true, true, true, true, true, true, true],
        ];
        Self {
            grid,
            player_pos: (3, 3),
            last_update: 0.0,
        }
    }
}

impl App for GameView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Simulation d'une mise à jour dynamique toutes les 2 secondes
        let time = ctx.input(|i| i.time);
        if time - self.last_update > 2.0 {
            self.last_update = time;
            // Mise à jour aléatoire de la grille pour simuler une nouvelle vue Radar
            let mut rng = rand::thread_rng();
            for row in self.grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = rng.gen_bool(0.5);
                }
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Vue Radar du Labyrinthe (Dynamique)");
            let cell_size = 50.0;
            ui.horizontal_centered(|ui| {
                for (i, row) in self.grid.iter().enumerate() {
                    ui.vertical(|ui| {
                        for (j, &cell) in row.iter().enumerate() {
                            let (rect, _response) = ui.allocate_exact_size(egui::Vec2::new(cell_size, cell_size), egui::Sense::hover());
                            let color = if cell {
                                egui::Color32::DARK_GRAY
                            } else if (i, j) == self.player_pos {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::LIGHT_GRAY
                            };
                            ui.painter().rect_filled(rect, 2.0, color);
                        }
                    });
                }
            });
            ui.add_space(20.0);
        });
    }
}
