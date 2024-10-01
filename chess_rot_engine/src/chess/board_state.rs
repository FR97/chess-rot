use std::error::Error;
use std::fmt;
use crate::bitboard::BitBoard;
use crate::chess::{CastlingRight, Color, Move, Piece};

#[derive(Debug, Copy, Clone)]
pub struct BoardState {
    pieces_for_color: [BitBoard; 2],
    pieces: [[BitBoard; 6]; 2],
    half_move_clock: u16,
    ply: u16,
    full_move: u16,
    score: i16,
    color_on_move: Color,
    castling: CastlingRight,
    en_passant_position: Option<u8>,
}

#[derive(Debug)]
pub struct FenFormatError {
    msg: String,
}

impl fmt::Display for FenFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FenFormatError[{}]", self.msg)
    }
}

impl Error for FenFormatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }
}

impl BoardState {
    pub fn new() -> BoardState {
        return BoardState {
            pieces_for_color: [Piece::default_bitboard_for_color(Color::White), Piece::default_bitboard_for_color(Color::Black)],
            pieces: [
                [
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::King),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Queen),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Rook),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Bishop),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Knight),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Pawn),
                ],
                [
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::King),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Queen),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Rook),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Bishop),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Knight),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Pawn),
                ],
            ],
            half_move_clock: 0,
            ply: 0,
            full_move: 1,
            score: 0,
            color_on_move: Color::White,
            castling: CastlingRight::default(),
            en_passant_position: None,
        };
    }

    pub fn from_fen(fen: String) -> Result<BoardState, FenFormatError> {
        let has_error = Self::validate_fen(fen);
        if (has_error.is_some()) {
            return Err(FenFormatError { msg: has_error.unwrap() });
        }


        return Ok(BoardState {
            pieces_for_color: [Piece::default_bitboard_for_color(Color::White), Piece::default_bitboard_for_color(Color::Black)],
            pieces: [
                [
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::King),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Queen),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Rook),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Bishop),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Knight),
                    Piece::default_bitboard_for_color_and_type(Color::White, Piece::Pawn),
                ],
                [
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::King),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Queen),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Rook),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Bishop),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Knight),
                    Piece::default_bitboard_for_color_and_type(Color::Black, Piece::Pawn),
                ],
            ],
            half_move_clock: 0,
            ply: 0,
            full_move: 1,
            score: 0,
            color_on_move: Color::White,
            castling: CastlingRight::default(),
            en_passant_position: None,
        });
    }

    pub fn piece_at(&self, position: u8) -> Option<(Piece, Color)> {
        return if self.pieces_for_color[Color::White.index()].is_occupied(position) {
            self.find_piece_at_position_for_color(Color::White, position)
                .map(|p| (p, Color::White))
        } else if self.pieces_for_color[Color::Black.index()].is_occupied(position) {
            self.find_piece_at_position_for_color(Color::Black, position)
                .map(|p| (p, Color::Black))
        } else {
            None
        };
    }

    pub fn make_move(&self, m: Move) -> BoardState {
        return BoardState::new();
    }

    pub fn remove_piece(&mut self, position: u8) {
        let removed = self.piece_at(position);

        if (removed.is_some()) {
            let p = removed.unwrap();
            let bb = self.pieces[p.1.index()][p.0.index()];
            let new_bb = bb.remove_bit(position);
            self.pieces[p.1.index()][p.0.index()] = new_bb;
        }
    }


    fn find_piece_at_position_for_color(&self, color: Color, position: u8) -> Option<Piece> {
        let pieces = self.pieces[color.index()];
        for i in 0..6 {
            if pieces[i].is_occupied(position) {
                return Piece::try_from(i).ok();
            }
        }
        return None;
    }

    pub fn validate_fen(fen: String) -> Option<String> {
        let split: Vec<String> = fen.split(" ").map(|s| s.to_string()).collect();

        if (split.len() != 6) {
            return Some(format!("invalid FEN format: fen must have 6 parts but found {}", split.len()));
        }

        let piece_placement = split.get(0).unwrap();

        let color_on_move = split.get(1).unwrap();
        if color_on_move != "w" && color_on_move != "b" {
            return Some("invalid FEN format: active color can either be 'w' or 'b'".to_string());
        }

        let castling = split.get(2).unwrap().to_string();
        if castling.is_empty() || castling.len() > 4 {
            return Some("invalid FEN format: invalid casting rights value".to_string());
        }

        if (castling != "-") {
            let allowed_values = ['K', 'Q', 'k', 'q'];
            for (i, c) in castling.chars().enumerate() {
                if (!allowed_values.contains(&c)) {
                    return Some("invalid FEN format: invalid casting rights value".to_string());
                } else if i < castling.len() - 1 {
                    let current = allowed_values.iter().position(|v| *v == c).unwrap();
                    let next = allowed_values.iter().position(|v| *v == castling.chars().nth(i + 1).unwrap()).unwrap();
                    if (next < current) {
                        return Some("invalid FEN format: invalid casting rights order".to_string());
                    }
                }
            }
        }

        let en_passant = split.get(3).unwrap();

        let half_move_clock = split.get(4).and_then(|hmc| hmc.parse::<u8>().ok());
        if half_move_clock.is_none() {
            return Some("invalid FEN format: invalid half move clock".to_string());
        }


        let full_moves = split.get(5).and_then(|hmc| hmc.parse::<u16>().ok());
        if full_moves.is_none() {
            return Some("invalid FEN format: invalid half move clock".to_string());
        }

        return None;
    }

    // Start from A8 and goes to H1
    // Reference: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn to_fen(self) -> String {
        let mut str = String::new();

        for rank in 0..8 {
            let mut empty_count = 0;
            for file in (0..8).rev() {
                let position = (BitBoard::END_BIT - (rank * 8) - file) as u8;
                match self.piece_at(position) {
                    None => {
                        empty_count = empty_count + 1;
                    }
                    Some(p) => {
                        if empty_count > 0 {
                            str.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        str.push(p.0.to_char_representation(p.1))
                    }
                }
            }
            if empty_count > 0 {
                str.push_str(&empty_count.to_string());
                empty_count = 0;
            }

            if rank < 7 {
                str.push_str("/")
            }
        }

        str.push(' ');
        str.push(self.color_on_move.to_char());

        str.push(' ');

        str.push_str(&self.castling.to_string());

        str.push(' ');
        match self.en_passant_position {
            None => str.push('-'),
            Some(pos) => str.push_str("a1"),
        }


        str.push(' ');
        str.push_str(&self.half_move_clock.to_string());

        str.push(' ');
        str.push_str(&self.full_move.to_string());

        return str;
    }
}


impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.to_fen());
    }
}
