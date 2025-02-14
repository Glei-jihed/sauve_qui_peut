use eframe::{egui, App, Frame};

/// Vue du jeu qui simule le RadarView du labyrinthe.
/// Affiche une grille 7x7 avec la cellule centrale en vert.
pub struct GameView {
    pub grid: Vec<Vec<bool>>,
    pub player_pos: (usize, usize),
}

impl Default for GameView {
    fn default() -> Self {
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
        }
    }
}

impl App for GameView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Vue Radar du Labyrinthe");
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
