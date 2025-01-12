// Add this dependency to your Cargo.toml file:
// eframe = "0.24" (adjust to the latest version)

mod player;

use crate::player::{Player, PlayerConfig};
use chess_rot_engine::chess;
use chess_rot_engine::chess::{BoardState, Color, Game, GameError, Piece};
use eframe::egui::Key::N;
use eframe::egui::{Color32, Context, Painter, Rect, Response};
use eframe::{egui, Frame};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const SYMBOLS: [&str; 13] = [
    "♚", "♛", "♜", "♝", "♞", "♟", "", "♔", "♕", "♖", "♗", "♘", "♙",
];

fn main() -> Result<(), eframe::Error> {
    println!("Starting UI");
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1600.0, 1200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "RotChess",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<ChessAppState>::default())
        }),
    )
}

#[derive(Debug, PartialEq, Clone)]
enum Event {
    NewGame,
    LoadGame(String),
    SelectPiece,
    Move(String),
    Undo,
    Redo,
    Quit(QuitState),
    None,
    UnselectPiece,
}

#[derive(Debug, PartialEq, Clone)]
enum QuitState {
    Initiated,
    Confirmed,
    Cancelled,
}

struct ChessAppState {
    game: Arc<Mutex<chess::Game>>,
    message: String,
    event: Event,
    pieces: [(Piece, Option<Color>); 64],
    on_move: Color,
    playing: bool,
    selected: Option<(u8, u8)>,
    possible_moves: Vec<chess::Move>,
    possible_move_squares: Vec<(u8, u8)>,
    player_config: PlayerConfig,
    input_fen: String,
    max_depth: usize,
    max_time: f32,
}

impl ChessAppState {
    pub fn handle_event(&mut self, ctx: &Context) {
        let event = self.event.clone();
        match event {
            Event::NewGame => {
                self.game = Arc::new(Mutex::new(chess::Game::new()));
                self.playing = true;
                self.event = Event::None;
            }
            Event::LoadGame(fen) => {
                match chess::Game::from_fen(&fen) {
                    Ok(game) => {
                        self.game = Arc::new(Mutex::new(game));
                        self.playing = true;
                    }
                    Err(err) => {
                        self.message = format!("{}", err);
                    }
                };
                self.event = Event::None;
            }
            Event::Move(_) => {}
            Event::Undo => {}
            Event::Redo => {}
            Event::SelectPiece => {
                self.event = Event::None;
                println!("Selected piece");
                if let Ok(game) = self.game.lock() {
                    let select = self.selected.unwrap();
                    let position = select.0 + select.1 * 8;
                    let g = game.clone();
                    self.possible_moves = g.possible_moves_for_position(position);
                    self.possible_move_squares = self
                        .possible_moves
                        .iter()
                        .map(|m| ((m.get_to().raw() % 8) as u8, (m.get_to().raw() / 8) as u8))
                        .collect();
                    println!("Possible moves: {:?}", self.possible_move_squares);
                }
            }
            Event::UnselectPiece => {
                self.event = Event::None;
                self.selected = None;
                self.possible_move_squares = Vec::new();
            }
            Event::Quit(state) => match state {
                QuitState::Initiated => {
                    egui::Window::new("Do you want to quit?")
                        .collapsible(false)
                        .resizable(false)
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Yes").clicked() {
                                    self.event = Event::Quit(QuitState::Confirmed);
                                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                                }

                                if ui.button("No").clicked() {
                                    self.event = Event::Quit(QuitState::Cancelled);
                                }
                            });
                        });
                }
                QuitState::Cancelled => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                }
                QuitState::Confirmed => {}
            },
            Event::None => {}
        }
    }

    fn setup_chessboard_square(
        &mut self,
        mut x: i8,
        mut y: i8,
        square_size: f32,
        painter: &Painter,
        response: Response,
        rect: Rect,
        color: Color32,
        col: u8,
        row: u8,
    ) {
        let piece = self.pieces[(col + row * 8) as usize];
        if response.clicked() {
            x = col as i8;
            y = row as i8;
            println!("Clicked on square {}{}", col, row);
            if let Some(selected) = self.selected {
                if selected.0 == col && selected.1 == row {
                    self.selected = None;
                    self.event = Event::UnselectPiece;
                }
            } else if piece.1.filter(|c| *c == self.on_move).is_some() {
                self.selected = Some((col, row));
                self.event = Event::SelectPiece;
            }
        }

        painter.rect_filled(rect, 0.0, color);
        let text_pos = rect.center();

        let piece_index = piece.0.index()
            + if piece.1.is_some() && piece.1.unwrap() == Color::White {
                7
            } else {
                0
            };
        painter.text(
            text_pos,
            egui::Align2::CENTER_CENTER,
            SYMBOLS[piece_index],
            egui::FontId::proportional(square_size * 0.8),
            egui::Color32::BLACK,
        );
    }
}

impl Default for ChessAppState {
    fn default() -> Self {
        return ChessAppState {
            game: Arc::new(Mutex::new(chess::Game::new())),
            message: "".to_string(),
            event: Event::None,
            pieces: [(Piece::None, None); 64],
            selected: None,
            on_move: Color::White,
            possible_moves: Vec::new(),
            possible_move_squares: Vec::new(),
            playing: false,
            player_config: PlayerConfig::default(),
            input_fen: String::default(),
            max_depth: 5,
            max_time: 5.0,
        };
    }
}

impl eframe::App for ChessAppState {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if !matches!(self.event, Event::None) {
            self.handle_event(ctx);
        }

        if let Ok(ref mut game) = self.game.try_lock() {
            for (sqr, piece, color) in game.board_iter() {
                self.pieces[sqr] = (piece, color);
            }
        }

        egui::SidePanel::right("control_panel")
            .min_width(360f32)
            .show(ctx, |ui| {
                if ui.button("New Game").clicked() {
                    self.event = Event::NewGame;
                }

                ui.label("Load From Fen:");
                ui.text_edit_singleline(&mut self.input_fen);
                if ui.button("Load").clicked() {
                    if (self.input_fen.len() < 2) {
                        self.message = "Invalid FEN".to_string();
                    } else {
                        self.event == Event::LoadGame(self.input_fen.clone());
                    }
                };

                ui.separator();

                egui::ComboBox::from_id_salt(ui.next_auto_id())
                    .selected_text(format!("White Player: {:?}", self.player_config.white_player))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.player_config.white_player,
                            Player::Human,
                            "Human",
                        );
                        ui.selectable_value(
                            &mut self.player_config.white_player,
                            Player::Minimax,
                            "Minimax",
                        );
                        ui.selectable_value(
                            &mut self.player_config.white_player,
                            Player::LLM,
                            "LLM",
                        );
                    });

                if self.player_config.white_player == Player::Minimax {
                    ui.add(egui::Slider::new(&mut self.max_depth, 1..=8).text("Max Depth:"));
                    ui.add(egui::Slider::new(&mut self.max_time, 1.0..=20.0).text("Max Search Time:"));
                }

                egui::ComboBox::from_id_salt(ui.next_auto_id())
                    .selected_text(format!("Black Player: {:?}", self.player_config.black_player))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.player_config.black_player,
                            Player::Human,
                            "Human",
                        );
                        ui.selectable_value(
                            &mut self.player_config.black_player,
                            Player::Minimax,
                            "Minimax",
                        );
                        ui.selectable_value(
                            &mut self.player_config.black_player,
                            Player::LLM,
                            "LLM",
                        );
                    });

                ui.separator();

                if ui.button("Quit").clicked() {
                    self.event = Event::Quit(QuitState::Initiated);
                }
            });

        let mut x: i8 = -1;
        let mut y: i8 = -1;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.message.clone());
            let available_size = ui.available_size();
            let central_panel_rect = ui.min_rect();
            let center_x = central_panel_rect.center().x;
            let center_y = central_panel_rect.center().y;
            let mut responses = Vec::new();
            let board_size = available_size.min_elem();
            let square_size = board_size / 8.0;
            let board_top_left = egui::Pos2 {
                x: center_x - (4.0 * square_size),
                y: center_y - (4.0 * square_size),
            };
            for row in 0..8 {
                for col in 0..8 {
                    let p = col + row * 8;
                    let color = if (self.selected.is_some()
                        && self.selected.unwrap().0 == 7 - col
                        && self.selected.unwrap().1 == 7 - row)
                    {
                        egui::Color32::from_rgb(205, 205, 55)
                    } else if (self.possible_move_squares.contains(&(7 - col, 7 - row))) {
                        egui::Color32::from_rgb(100, 255, 100)
                    } else if (row + col) % 2 == 0 {
                        egui::Color32::from_rgb(255, 255, 255)
                    } else {
                        egui::Color32::from_rgb(180, 180, 180)
                    };

                    let top_left = egui::Pos2 {
                        x: board_top_left.x + (col as f32 * square_size),
                        y: board_top_left.y + (row as f32 * square_size),
                    };
                    let bottom_right = egui::Pos2 {
                        x: top_left.x + square_size,
                        y: top_left.y + square_size,
                    };
                    let rect = egui::Rect::from_two_pos(top_left, bottom_right);
                    let response = ui.allocate_rect(rect, egui::Sense::click());

                    responses.push((response, rect, color, 7 - col, 7 - row));
                }
            }
            let painter = ui.painter();
            for (response, rect, color, col, row) in responses {
                self.setup_chessboard_square(
                    x,
                    y,
                    square_size,
                    painter,
                    response,
                    rect,
                    color,
                    col,
                    row,
                );
            }
        });
    }
}
