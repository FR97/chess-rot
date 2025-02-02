// Add this dependency to your Cargo.toml file:
// eframe = "0.24" (adjust to the latest version)

mod player;

use crate::player::{Player, PlayerConfig};
use chess_rot_engine::chess;
use chess_rot_engine::chess::{BoardState, Color, Game, GameError, GameResult, Move, Piece, Square};
use eframe::egui::{Color32, Context, Painter, Rect, Response};
use eframe::{egui, App, Frame};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};
use eframe::egui::Key::S;
use chess_rot_engine::chess::ai::ai_strategy::{AiStrategy, Minimax, OpenAi};
use chess_rot_engine::chess::ai::evaluator::Evaluator;
use chess_rot_engine::chess::Color::White;
use chess_rot_engine::chess::move_provider::MoveProvider;

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
    MoveCompleted,
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
    // game: Arc<Mutex<chess::Game>>,
    game: Game,
    message: String,
    message_time: Instant,
    event: Event,
    pieces: [(Piece, Option<Color>); 64],
    on_move: Color,
    playing: bool,
    selected: Option<(u8, u8)>,
    possible_moves: Vec<Move>,
    possible_move_squares: Vec<(u8, u8)>,
    player_config: PlayerConfig,
    input_fen: String,
    current_fen: String,
    last_move: Option<Move>,
    last_ai_move_time: Instant,
    move_completed: bool,
}

impl ChessAppState {
    pub fn handle_event(&mut self, ctx: &Context) {
        let event = self.event.clone();
        match event {
            Event::NewGame => {
                // self.game = Arc::new(Mutex::new(Game::new()));
                self.event = Event::None;

                // if let Ok(g) = self.game.lock() {
                // }
                self.set_game(Game::new());
                self.set_timed_message("New Game Started!");
            }
            Event::LoadGame(fen) => {
                self.event = Event::None;
                match chess::Game::from_fen(&fen) {
                    Ok(game) => {
                        // self.game = Arc::new(Mutex::new(game));
                        self.input_fen = "".to_string();
                        // if let Ok(g) = self.game.lock() {
                        self.set_game(game);
                        self.set_timed_message("Game Loaded Successfully!");
                        // }
                    }
                    Err(err) => {
                        self.set_timed_message(&err.to_string());
                    }
                };
            }
            Event::Move(_) => {}
            Event::Undo => {}
            Event::Redo => {}
            Event::SelectPiece => {
                self.event = Event::None;
                println!("Selected piece");
                // if let Ok(game) = self.game.lock() {
                let select = self.selected.unwrap();
                let position = select.0 + select.1 * 8;

                for m in self.possible_moves.iter() {
                    println!("Move from {} to {}", m.get_from(), m.get_to());
                }

                self.possible_move_squares = self
                    .possible_moves
                    .iter()
                    .filter(|m| m.get_from().raw() as u8 == position)
                    .map(|m| ((m.get_to().raw() % 8) as u8, (m.get_to().raw() / 8) as u8))
                    .collect();
                println!("Possible moves: {:?}", self.possible_move_squares);
                // }
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
            Event::MoveCompleted => {
                ctx.request_repaint();
                self.event = Event::None;
            }
            Event::None => {}
        }
    }

    fn set_game(&mut self, game: Game) {
        self.possible_moves = Vec::new();
        self.possible_move_squares = Vec::new();
        self.game = game;
        self.current_fen = self.game.to_fen();
        self.on_move = self.game.current_state.on_move();
        self.last_move = None;
        self.last_ai_move_time = Instant::now();
        self.playing = true
    }

    fn make_move(&mut self, m: Move) {
        if let Some(err) = self.game.make_move(m) {
            self.set_timed_message(&err.to_string());
        } else {
            self.possible_moves = Vec::new();
            self.possible_move_squares = Vec::new();
            self.selected = None;
            self.current_fen = self.game.to_fen();
            self.on_move = self.game.current_state.color_on_move;
            self.last_move = Some(m);
            self.move_completed = true;
            self.event = Event::None;
        }
    }

    fn set_timed_message(&mut self, str: &str) {
        self.message_time = Instant::now();
        self.message = str.to_string();
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
            println!("Clicked on square {} {}", col, row);
            if let Some(selected) = self.selected {
                let clicked_square_index = (col + row * 8) as usize;
                let selected_square_index = (selected.0 + selected.1 * 8) as usize;
                if clicked_square_index == selected_square_index {
                    // Unselect
                    self.selected = None;
                    self.event = Event::UnselectPiece;
                } else if let Some(m) = self.find_move_from_to(selected_square_index, clicked_square_index) {
                    // Make move
                    // if let Ok(ref mut game) = self.game.try_lock() {
                    println!("Making move");
                    self.make_move(m);
                    // }
                }
            } else if piece.1.filter(|c| *c == self.on_move).is_some() {
                self.selected = Some((col, row));
                self.event = Event::SelectPiece;
            }
        }

        painter.rect_filled(rect, 0.0, color);
        let text_pos = rect.center();

        let piece_index = piece.0.index() + if piece.1.is_some() && piece.1.unwrap() == Color::White {
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

    pub fn find_move_from_to(&self, from: usize, to: usize) -> Option<Move> {
        return self.possible_moves.iter()
            .find(|m| m.get_from() == Square::from_usize(from) && m.get_to() == Square::from_usize(to))
            .map(|m| m.clone());
    }
}

impl Default for ChessAppState {
    fn default() -> Self {
        return ChessAppState {
            // game: Arc::new(Mutex::new(chess::Game::new())),
            game: Game::new(),
            message: "".to_string(),
            message_time: Instant::now(),
            event: Event::None,
            pieces: [(Piece::None, None); 64],
            selected: None,
            on_move: Color::White,
            possible_moves: Vec::new(),
            possible_move_squares: Vec::new(),
            playing: false,
            player_config: PlayerConfig::default(),
            input_fen: "".to_string(),
            current_fen: "".to_string(),
            last_move: None,
            last_ai_move_time: Instant::now(),
            move_completed: true,
        };
    }
}

impl eframe::App for ChessAppState {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.message.len() > 0 && Instant::now().duration_since(self.message_time).as_secs() > 4 {
            self.message = "".to_string();
        }

        if !matches!(self.event, Event::None) {
            println!("Handling event!");
            self.handle_event(ctx);
        }

        // if let Ok(ref mut game) = self.game.try_lock() {

        if self.playing && self.possible_moves.is_empty() && self.game.result == None {
            println!("Generating legal moves");
            let start = Instant::now();
            self.possible_moves = self.game.generate_legal_moves();
            println!("Generating {} moves took {}ns", self.possible_moves.len(), Instant::now().duration_since(start).as_nanos());
            if self.possible_moves.len() == 0 {
                match MoveProvider::INSTANCE.is_king_under_attack(&self.game.current_state) {
                    true => {
                        println!("Checkmate");
                        let winner = self.game.current_state.color_on_move.inverse();
                        self.game.result = Some(GameResult::Win(winner));
                        self.message = format!("{} won game!", winner);
                    }
                    false => {
                        println!("Stalemate");
                        self.game.result = Some(GameResult::Draw);
                        self.message = "Stalemate!".to_string();
                    }
                }
            }
        }

        if self.game.current_state.on_move() == White {
            match self.player_config.white_player {
                Player::Human => {}
                Player::Minimax => {
                    if self.player_config.white_ai_start
                        && Instant::now().duration_since(self.last_ai_move_time).as_secs() > 3 {
                        self.last_ai_move_time = Instant::now();
                        let optimal_move = Minimax::new(Evaluator::new(), self.player_config.white_max_depth, self.player_config.white_max_time)
                            .find_optimal_move(&self.game.current_state, &self.possible_moves);
                        match optimal_move {
                            Ok(m) => {
                                println!("Making a move {:?}", m);
                                self.make_move(m);
                            }
                            Err(err) => {
                                println!("Error: {}", err);
                                self.set_timed_message(&err.to_string());
                            }
                        }
                    }
                }
                Player::LLM => {
                    if (self.player_config.white_ai_start
                        && !self.player_config.white_api_key.is_empty()
                        && Instant::now().duration_since(self.last_ai_move_time).as_secs() > 3) {
                        self.last_ai_move_time = Instant::now();
                        let optimal_move = OpenAi::new(&self.player_config.white_api_key)
                            .find_optimal_move(&self.game.current_state, &self.possible_moves);
                        match optimal_move {
                            Ok(m) => {
                                println!("Making a move {:?}", m);
                                self.make_move(m);
                            }
                            Err(err) => {
                                println!("Error: {}", err);
                                self.set_timed_message(&err.to_string());
                            }
                        }
                    } else if self.message.is_empty() {
                        self.set_timed_message("waiting for ai to make move!");
                    }
                }
            }
        } else {
            match self.player_config.black_player {
                Player::Human => {}
                Player::Minimax => {
                    if self.player_config.black_ai_start
                        && Instant::now().duration_since(self.last_ai_move_time).as_secs() > 3 {
                        self.last_ai_move_time = Instant::now();
                        let optimal_move = Minimax::new(Evaluator::new(), self.player_config.black_max_depth, self.player_config.black_max_time)
                            .find_optimal_move(&self.game.current_state, &self.possible_moves);
                        match optimal_move {
                            Ok(m) => {
                                println!("Making a move {:?}", m);
                                self.make_move(m);
                            }
                            Err(err) => {
                                println!("Error: {}", err);
                                self.set_timed_message(&err.to_string());
                            }
                        }
                    }
                }
                Player::LLM => {
                    if (self.player_config.black_ai_start
                        && !self.player_config.black_api_key.is_empty()
                        && Instant::now().duration_since(self.last_ai_move_time).as_secs() > 3) {
                        self.last_ai_move_time = Instant::now();
                        let optimal_move = OpenAi::new(&self.player_config.black_api_key)
                            .find_optimal_move(&self.game.current_state, &self.possible_moves);
                        match optimal_move {
                            Ok(m) => {
                                println!("Making a move {:?}", m);
                                self.make_move(m);
                            }
                            Err(err) => {
                                println!("Error: {}", err);
                                self.set_timed_message(&err.to_string());
                            }
                        }
                    } else if self.message.is_empty() {
                        self.set_timed_message("waiting for ai to make move!");
                    }
                }
            }
        }

        for (sqr, piece, color) in self.game.board_iter() {
            self.pieces[sqr] = (piece, color);
        }

        if (self.move_completed) {
            self.move_completed = false;
            ctx.request_repaint();
        }
        // }

        egui::SidePanel::right("control_panel")
            .min_width(360f32)
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(18.0, eframe::epaint::FontFamily::Proportional),
                );

                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Heading,
                    egui::FontId::new(18.0, eframe::epaint::FontFamily::Proportional),
                );


                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(16.0, eframe::epaint::FontFamily::Proportional),
                );

                if ui.button("New Game").clicked() {
                    self.event = Event::NewGame;
                }

                ui.label("Load From Fen:");
                ui.text_edit_singleline(&mut self.input_fen);
                if ui.button("Load").clicked() {
                    if (self.input_fen.len() < 2) {
                        self.message = "Invalid FEN".to_string();
                    } else {
                        self.event = Event::LoadGame(self.input_fen.clone());
                    }
                };

                ui.separator();
                ui.heading("White Player Settings");

                egui::ComboBox::from_id_salt(ui.next_auto_id())
                    .selected_text(format!("{:?}", self.player_config.white_player))
                    .show_ui(ui, |ui| {
                        if ui.selectable_value(
                            &mut self.player_config.white_player,
                            Player::Human,
                            "Human",
                        ).changed() {
                            self.player_config.white_ai_start = false;
                        };
                        if ui.selectable_value(
                            &mut self.player_config.white_player,
                            Player::Minimax,
                            "Minimax",
                        ).changed() {
                            self.player_config.white_ai_start = false;
                        };
                        if ui.selectable_value(
                            &mut self.player_config.white_player,
                            Player::LLM,
                            "LLM",
                        ).changed() {
                            self.player_config.white_ai_start = false;
                        };
                    });

                if self.player_config.white_player == Player::Minimax {
                    ui.label("Max Depth:");
                    ui.add(egui::Slider::new(&mut self.player_config.white_max_depth, 1..=8));
                    ui.label("Max Search Time:");
                    ui.add(egui::Slider::new(&mut self.player_config.white_max_time, 1.0..=20.0));

                    ui.text_edit_singleline(&mut self.player_config.white_api_key);
                    let start_pause_text = if self.player_config.white_ai_start {
                        "Pause"
                    } else { "Start" };
                    if ui.button(start_pause_text).clicked() {
                        self.player_config.white_ai_start = !self.player_config.white_ai_start
                    }
                } else if self.player_config.white_player == Player::LLM {
                    ui.label("OpenAI API Key:");
                    ui.text_edit_singleline(&mut self.player_config.white_api_key);
                    let start_pause_text = if self.player_config.white_ai_start {
                        "Pause"
                    } else { "Start" };
                    if ui.button(start_pause_text).clicked() {
                        self.player_config.white_ai_start = !self.player_config.white_ai_start
                    }
                }

                ui.separator();
                ui.heading("Black Player Settings");

                egui::ComboBox::from_id_salt(ui.next_auto_id())
                    .selected_text(format!("{:?}", self.player_config.black_player))
                    .show_ui(ui, |ui| {
                        if ui.selectable_value(
                            &mut self.player_config.black_player,
                            Player::Human,
                            "Human",
                        ).changed() {
                            self.player_config.black_ai_start = false;
                        };
                        if ui.selectable_value(
                            &mut self.player_config.black_player,
                            Player::Minimax,
                            "Minimax",
                        ).changed() {
                            self.player_config.black_ai_start = false;
                        };
                        if ui.selectable_value(
                            &mut self.player_config.black_player,
                            Player::LLM,
                            "LLM",
                        ).changed() {
                            self.player_config.black_ai_start = false;
                        };
                    });

                if self.player_config.black_player == Player::Minimax {
                    ui.label("Max Depth:");
                    ui.add(egui::Slider::new(&mut self.player_config.black_max_depth, 1..=8));
                    ui.label("Max Search Time:");
                    ui.add(egui::Slider::new(&mut self.player_config.black_max_time, 1.0..=20.0));
                    let start_pause_text = if self.player_config.black_ai_start {
                        "Pause"
                    } else { "Start" };
                    if ui.button(start_pause_text).clicked() {
                        self.player_config.black_ai_start = !self.player_config.black_ai_start
                    }
                } else if self.player_config.black_player == Player::LLM {
                    ui.label("OpenAI API Key:");
                    ui.text_edit_singleline(&mut self.player_config.black_api_key);
                    let start_pause_text = if self.player_config.black_ai_start {
                        "Pause"
                    } else { "Start" };
                    if ui.button(start_pause_text).clicked() {
                        self.player_config.black_ai_start = !self.player_config.black_ai_start
                    }
                }

                if (self.playing == true) {
                    ui.separator();
                    ui.heading("Game Information:");

                    let on_move = match self.on_move {
                        Color::White => "White",
                        Color::Black => "Black",
                    };

                    ui.label(format!("On Move: {}", on_move));

                    // if let Ok(game) = self.game.lock() {
                    ui.label(format!("PLY: {}", self.game.current_state.ply()));
                    // }
                    ui.label(format!("Current FEN: {}", self.current_fen));
                }

                ui.separator();

                if ui.button("Quit").clicked() {
                    self.event = Event::Quit(QuitState::Initiated);
                }
            });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading(self.message.clone())
            });
        });

        let mut x: i8 = -1;
        let mut y: i8 = -1;
        egui::CentralPanel::default().show(ctx, |ui| {
            if (self.playing) {
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
                        let p = col + (7 - row) * 8;
                        let color = if (self.selected.is_some()
                            && self.selected.unwrap().0 == col
                            && self.selected.unwrap().1 == 7 - row)
                        {
                            egui::Color32::from_rgb(205, 205, 55)
                        } else if (self.possible_move_squares.contains(&(col, 7 - row))) {
                            egui::Color32::from_rgb(100, 255, 100)
                        } else if self.last_move.is_some()
                            && (self.last_move.unwrap().get_from().raw() == p as u64 || self.last_move.unwrap().get_to().raw() == p as u64) {
                            egui::Color32::from_rgb(245, 132, 66)
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

                        responses.push((response, rect, color, col, 7 - row));
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
            }
        });
    }
}
