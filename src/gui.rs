use std::path::PathBuf;

use anyhow::{Result, anyhow};
use eframe::{
    egui::{self, CentralPanel, Color32, ComboBox, DragValue, Panel, Rect, Sense, Stroke, Vec2},
    epaint::StrokeKind,
};

use crate::{
    config::AppConfig,
    game::{Color, Game, Player},
    piece::Piece,
    render::{BoardBounds, ImageRenderOptions, write_png},
};

const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 96.0;

pub fn run() -> Result<()> {
    let result = eframe::run_native(
        "Chromosaic",
        native_options(),
        Box::new(|_cc| Ok(Box::new(ChromosaicApp::new()))),
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(anyhow!(error.to_string())),
    }
}

fn native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 1000.0])
            .with_min_inner_size([1000.0, 700.0]),
        ..Default::default()
    }
}

struct ChromosaicApp {
    piece_types: Vec<String>,
    rows: Vec<Player>,
    board_size: usize,
    zoom: f32,
    pan: Vec2,
    game: Option<Game>,
    do_refresh: bool,
    status: Option<String>,
    preview_rect: Option<Rect>,
}

impl ChromosaicApp {
    fn new() -> ChromosaicApp {
        let board_size = 100_000;
        let rows = vec![
            Player {
                piece: Piece::from_name("knight").unwrap(),
                color: Color::black(),
            },
            Player {
                piece: Piece::from_name("knight").unwrap(),
                color: Color::red(),
            },
        ];
        let piece_types =
            Piece::get_all_piece_types().unwrap_or_else(|_| vec!["knight".to_string()]);

        let game = Game::new(board_size, rows.clone());

        ChromosaicApp {
            piece_types,
            rows,
            board_size,
            zoom: 3.0,
            pan: Vec2::ZERO,
            game: Some(game),
            do_refresh: true,
            status: None,
            preview_rect: None,
        }
    }

    fn config(&self) -> AppConfig {
        AppConfig::new(
            self.board_size,
            self.rows
                .iter()
                .map(|row| Player {
                    piece: row.piece.clone(),
                    color: row.color,
                })
                .collect(),
        )
    }

    fn recompute(&mut self) {
        match self.config().build_game() {
            Ok(game) => {
                self.game = Some(game);
                self.do_refresh = false;
                self.status = None;
            }
            Err(error) => {
                self.status = Some(error.to_string());
            }
        }
    }

    fn export_png(&mut self) {
        if self.do_refresh {
            self.recompute();
        }
        let Some(game) = self.game.as_ref() else {
            self.status = Some("No valid preview to export".to_string());
            return;
        };

        let mut path = PathBuf::from("images/".to_string() + game.game_name().as_str() + "_" + game.board().points().len().to_string().as_str());
        path.set_extension("png");
        match write_png(&path, game, ImageRenderOptions::default()) {
            Ok(()) => self.status = Some(format!("Exported {}", path.display())),
            Err(error) => self.status = Some(format!("Export failed: {error}")),
        }
    }

    fn fit_to_view(&mut self, rect: Rect) {
        let Some(game) = self.game.as_ref() else {
            return;
        };
        let Some(bounds) = BoardBounds::from_points(game.board().points()) else {
            return;
        };
        let available = rect.size();
        let zoom_x = available.x / bounds.width_cells() as f32;
        let zoom_y = available.y / bounds.height_cells() as f32;
        self.zoom = zoom_x.min(zoom_y).clamp(MIN_ZOOM, MAX_ZOOM);
        self.pan = Vec2::ZERO;
        self.status = Some(format!("Zoom level: {}", self.zoom));
    }

    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.heading("Chromosaic");

        ui.horizontal(|ui| {
            ui.label("Board cells");
            let response = ui.add(DragValue::new(&mut self.board_size).range(1..=1_000_000));
            if response.changed() {
                self.do_refresh = true;
            }
        });

        ui.separator();

        let mut remove_index = None;
        let mut move_up = None;
        let mut move_down = None;
        let row_count = self.rows.len();
        for index in 0..row_count {
            ui.horizontal(|ui| {
                let mut current_color = self.rows[index].color.into();
                let color_response = ui.color_edit_button_srgba(&mut current_color);
                if color_response.changed() {
                    self.do_refresh = true;
                    self.rows[index].color = Color::from(current_color);
                }

                ComboBox::from_id_salt(format!("piece-{index}"))
                    .selected_text(display_piece_name(&self.rows[index].piece.name))
                    .show_ui(ui, |ui| {
                        let piece_types = self.piece_types.clone();
                        for piece_name in piece_types {
                            let selected = self.rows[index].piece.name == piece_name;
                            if ui
                                .selectable_label(selected, display_piece_name(&piece_name))
                                .clicked()
                            {
                                match Piece::from_name(&piece_name) {
                                    Ok(piece) => {
                                        self.rows[index].piece = piece;
                                        self.do_refresh = true;
                                        self.status = None;
                                    }
                                    Err(error) => {
                                        self.status =
                                            Some(format!("Could not load {piece_name}: {error}"));
                                    }
                                }
                            }
                        }
                    });

                if ui.small_button("Up").clicked() && index > 0 {
                    move_up = Some(index);
                }
                if ui.small_button("Down").clicked() && index + 1 < row_count {
                    move_down = Some(index);
                }
                if ui.small_button("Remove").clicked() && row_count > 1 {
                    remove_index = Some(index);
                }
            });
        }

        if let Some(index) = move_up {
            self.rows.swap(index, index - 1);
            self.do_refresh = true;
        }
        if let Some(index) = move_down {
            self.rows.swap(index, index + 1);
            self.do_refresh = true;
        }
        if let Some(index) = remove_index {
            self.rows.remove(index);
            self.do_refresh = true;
        }

        if ui.button("Add color/piece").clicked() {
            let piece_name = self.piece_types.first().cloned().unwrap_or_default();
            self.rows.push(Player {
                color: Color::from_rgb(0, 0, 255),
                piece: Piece::from_name(&piece_name).unwrap(),
            });
            self.do_refresh = true;
        }
    }

    fn draw_actions(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Recompute").clicked() {
                self.recompute();
            }
            if ui.button("Zoom in").clicked() {
                self.zoom = (self.zoom * 1.2).clamp(MIN_ZOOM, MAX_ZOOM);
            }
            if ui.button("Zoom out").clicked() {
                self.zoom = (self.zoom / 1.2).clamp(MIN_ZOOM, MAX_ZOOM);
            }
            if ui.button("Fit").clicked()
                && let Some(rect) = self.preview_rect
            {
                self.fit_to_view(rect);
            }
            if ui.button("Export PNG").clicked() {
                self.export_png();
            }
            if self.do_refresh {
                ui.label("Preview needs recompute");
            }
            if let Some(status) = &self.status {
                ui.label(status);
            }
        });
    }

    fn draw_preview(&mut self, ui: &mut egui::Ui) -> Rect {
        if self.do_refresh {
            self.recompute();
        }

        let available = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(available, Sense::drag());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, Color32::from_rgb(248, 248, 245));

        if response.dragged() {
            self.pan += response.drag_delta();
        }

        if response.hovered() {
            let scroll_y = ui.input(|input| input.smooth_scroll_delta.y);
            if scroll_y != 0.0 {
                let factor = if scroll_y > 0.0 { 1.1 } else { 1.0 / 1.1 };
                self.zoom = (self.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
                self.status = Some(format!("Zoom level: {}", self.zoom));
            }
        }

        let Some(game) = self.game.as_ref() else {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "No preview available",
                egui::TextStyle::Body.resolve(ui.style()),
                Color32::DARK_GRAY,
            );
            return rect;
        };
        let Some(bounds) = BoardBounds::from_points(game.board().points()) else {
            return rect;
        };

        let board_size = Vec2::new(
            bounds.width_cells() as f32 * self.zoom,
            bounds.height_cells() as f32 * self.zoom,
        );
        let origin = rect.center() - board_size / 2.0 + self.pan;
        let stroke = if self.zoom >= 10.0 {
            Stroke::new(1.0, Color32::from_gray(120))
        } else {
            Stroke::NONE
        };

        for (index, coord) in game.board().points().iter().enumerate() {
            let x = (coord.x - bounds.min_x) as f32 * self.zoom;
            let y = (bounds.max_y - coord.y) as f32 * self.zoom;
            let cell_rect = Rect::from_min_size(origin + Vec2::new(x, y), Vec2::splat(self.zoom));
            if !cell_rect.intersects(rect) {
                continue;
            }

            let fill = game
                .coloring()
                .get(&index)
                .and_then(|color_index| game.players().get(*color_index))
                .map(|player| player.color.into())
                .unwrap_or_else(|| Color32::from_gray(220));

            painter.rect_filled(cell_rect, 0.0, fill);
            if stroke != Stroke::NONE {
                painter.rect_stroke(cell_rect, 0.0, stroke, StrokeKind::Inside);
            }
        }

        rect
    }
}

impl eframe::App for ChromosaicApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Panel::left("controls")
            .resizable(false)
            .default_size(330.0)
            .show_inside(ui, |ui| self.draw_controls(ui));

        Panel::bottom("actions").show_inside(ui, |ui| self.draw_actions(ui));

        CentralPanel::default().show_inside(ui, |ui| {
            self.preview_rect = Some(self.draw_preview(ui));
        });
    }
}

fn display_piece_name(name: &str) -> String {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return "Unknown".to_string();
    };
    first.to_uppercase().chain(chars).collect()
}
