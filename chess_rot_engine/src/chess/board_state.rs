use std::error::Error;
use std::fmt;
use std::ops::{BitAndAssign, BitOrAssign};
use crate::bitboard::BitBoard;
use crate::chess::{CastlingRight, Color, GameError, Move, MoveType, Piece, Square, SquareLabel};
use crate::chess::game::Game;

#[derive(Debug, Copy, Clone)]
pub struct BoardState {
    pieces_for_color: [BitBoard; 2],
    pieces: [[BitBoard; 6]; 2],
    half_move_clock: u16,
    ply: u16,
    score: i16,
    color_on_move: Color,
    castling: CastlingRight,
    en_passant_position: Option<Square>,
}


impl BoardState {
    pub fn from_fen(fen: &str) -> Result<BoardState, GameError> {
        let has_error = Self::validate_fen(fen);
        if (has_error.is_some()) {
            return Err(GameError::FenFormatError(has_error.unwrap()));
        }


        let split: Vec<String> = fen.split(" ").map(|s| s.to_string()).collect();
        let fen_pieces = split.get(0).unwrap();

        let mut pieces_for_color = [BitBoard::empty(), BitBoard::empty()];
        let mut pieces = [
            [
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
            ],
            [
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
                BitBoard::empty(),
            ],
        ];


        let mut sq_index = 63;
        for p in fen_pieces.chars().into_iter() {
            match p {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    sq_index -= p.to_digit(10).unwrap();
                }
                'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => {
                    let piece = Piece::try_from(p.to_ascii_lowercase())
                        .expect("char must be valid piece representation");
                    pieces_for_color[Color::White.index()] |= BitBoard::SINGLE_BIT_BB[sq_index as usize];
                    pieces[Color::White.index()][piece.index()] |= BitBoard::SINGLE_BIT_BB[sq_index as usize];
                    if (sq_index > 0) {
                        sq_index -= 1
                    }
                }
                'k' | 'q' | 'r' | 'b' | 'n' | 'p' => {
                    let piece = Piece::try_from(p)
                        .expect("char must be valid piece representation");
                    pieces_for_color[Color::Black.index()] |= BitBoard::SINGLE_BIT_BB[sq_index as usize];
                    pieces[Color::Black.index()][piece.index()] |= BitBoard::SINGLE_BIT_BB[sq_index as usize];
                    if (sq_index > 0) {
                        sq_index -= 1
                    }
                }
                _ => {}
            }
        }

        let color_on_move: Color = split.get(1).unwrap().chars()
            .nth(0)
            .map(|c| Color::try_from(c).unwrap())
            .unwrap();

        let fen_castling = split.get(2).unwrap();
        let mut castling = CastlingRight::default();
        if !fen_castling.contains("K") {
            castling = castling.remove_king_side_castle(Color::White)
        } else if !fen_castling.contains("Q") {
            castling = castling.remove_queen_side_castle(Color::White)
        } else if !fen_castling.contains("k") {
            castling = castling.remove_king_side_castle(Color::Black)
        } else if !fen_castling.contains("q") {
            castling = castling.remove_queen_side_castle(Color::Black)
        }

        // TODO split part 3 (en passant)

        let half_move_clock: u16 = split.get(4)
            .map(|s| s.parse::<u16>().expect("half move clock must be valid number"))
            .unwrap();


        let full_move_number: u16 = split.get(5)
            .map(|s| s.parse::<u16>().expect("full move number must be valid number"))
            .unwrap();

        let ply = full_move_number * 2 + if color_on_move == Color::White { 0 } else { 1 };

        return Ok(BoardState {
            pieces_for_color,
            pieces,
            half_move_clock,
            ply,
            score: 0,
            color_on_move,
            castling,
            en_passant_position: None,
        });
    }

    pub fn get_piece_at(&self, sqr: u8) -> Option<(Piece, Color)> {
        return if self.pieces_for_color[Color::White.index()].is_bit_set(sqr) {
            self.find_piece_at_square_for_color(Color::White, sqr)
                .map(|p| (p, Color::White))
        } else if self.pieces_for_color[Color::Black.index()].is_bit_set(sqr) {
            self.find_piece_at_square_for_color(Color::Black, sqr)
                .map(|p| (p, Color::Black))
        } else {
            None
        };
    }

    pub fn make_move(&self, m: Move) -> BoardState {
        debug_assert_ne!(m.get_type(), MoveType::Invalid, "cannot make invalid move");
        debug_assert_eq!(m.get_color(), self.color_on_move, "move must match color currently on move");

        let on_move = m.get_color();
        let next_on_move = on_move.inverse();
        let from_bb = m.get_from().as_bb();
        let to_bb = m.get_to().as_bb();
        let from_to_bb = from_bb | to_bb;

        let mut pieces = self.pieces.clone();
        let mut pieces_for_color = self.pieces_for_color.clone();
        let mut half_move_clock = self.half_move_clock + 1;
        let ply = self.ply + 1;

        let mut castling = self.castling;
        let mut en_passant_position = None;

        pieces[on_move.index()][m.get_piece().index()] ^= from_to_bb;
        pieces_for_color[on_move.index()] ^= from_to_bb;

        match m.get_type() {
            MoveType::PawnJump => {
                en_passant_position = if on_move == Color::White {
                    Some(Square::new(m.get_to().raw() - 8))
                } else {
                    Some(Square::new(m.get_to().raw() + 8))
                }
            }
            MoveType::Capture => {
                half_move_clock = 0;
                pieces[next_on_move.index()][m.get_target_piece().index()] ^= to_bb;
                pieces_for_color[next_on_move.index()] ^= to_bb;
            }
            MoveType::Castling => {
                let rook_move = match self.color_on_move {
                    Color::White => {
                        // checks whether its king or queen side castle
                        if m.get_to().raw() == SquareLabel::C1.as_u64() {
                            SquareLabel::A1.to_bb() | SquareLabel::D1.to_bb()
                        } else {
                            SquareLabel::H1.to_bb() | SquareLabel::F1.to_bb()
                        }
                    }

                    Color::Black => {
                        if m.get_to().raw() == SquareLabel::C8.as_u64() {
                            SquareLabel::A8.to_bb() | SquareLabel::D8.to_bb()
                        } else {
                            SquareLabel::H8.to_bb() | SquareLabel::F8.to_bb()
                        }
                    }
                };

                pieces[on_move.index()][Piece::Rook.index()] ^= rook_move;
                pieces_for_color[on_move.index()] ^= rook_move;
            }
            MoveType::EnPassant => {
                debug_assert!(self.en_passant_position.is_some(), "en passant square must be set");
                half_move_clock = 0;
                let captured_pawn_pos = if on_move == Color::White {
                    self.en_passant_position.unwrap().as_bb() >> 8
                } else {
                    self.en_passant_position.unwrap().as_bb() << 8
                };

                pieces[next_on_move.index()][Piece::Pawn.index()] ^= captured_pawn_pos;
                pieces_for_color[next_on_move.index()] ^= captured_pawn_pos;
            }
            MoveType::Promotion => {
                pieces[on_move.index()][Piece::Pawn.index()] &= !to_bb;
                let new_piece = m.get_target_piece();
                pieces[on_move.index()][new_piece.index()] |= to_bb;
            }
            _ => {}
        }

        match m.get_piece() {
            Piece::King => {
                castling = self.castling.remove_both_side_castle(on_move)
            }
            Piece::Rook => {
                match on_move {
                    Color::White => {
                        if m.get_from().raw() == SquareLabel::A1.as_u64() {
                            castling = self.castling.remove_king_side_castle(Color::White);
                        } else if m.get_from().raw() == SquareLabel::A8.as_u64() {
                            castling = self.castling.remove_queen_side_castle(Color::White);
                        }
                    }
                    Color::Black => {
                        if m.get_from().raw() == SquareLabel::H1.as_u64() {
                            castling = self.castling.remove_king_side_castle(Color::Black);
                        } else if (m.get_from().raw() == SquareLabel::H8.as_u64()) {
                            castling = self.castling.remove_queen_side_castle(Color::Black);
                        }
                    }
                }
            }
            Piece::Pawn => {
                half_move_clock = 0
            }
            _ => {}
        }

        return BoardState {
            pieces,
            pieces_for_color,
            color_on_move: next_on_move,
            castling,
            en_passant_position,
            half_move_clock,
            ply,
            score: self.score,
        };
    }

    pub fn remove_piece(&mut self, sqr: u8) {
        let removed = self.get_piece_at(sqr);
        if removed.is_some() {
            let p = removed.unwrap();
            let bb = self.pieces[p.1.index()][p.0.index()];
            let new_bb = bb.remove_bit(sqr);
            self.pieces[p.1.index()][p.0.index()] = new_bb;
        }
    }

    pub fn full_moves(&self) -> u16 {
        return self.ply / 2;
    }

    fn find_piece_at_square_for_color(&self, color: Color, sqr: u8) -> Option<Piece> {
        let pieces = self.pieces[color.index()];
        for i in 0..6 {
            if pieces[i].is_bit_set(sqr) {
                return Piece::try_from(i).ok();
            }
        }
        return None;
    }

    fn find_piece_bb_at_sqr_for_color(&self, color: Color, sqr: u8) -> Option<BitBoard> {
        let pieces = self.pieces[color.index()];
        for i in 0..6 {
            if pieces[i].is_bit_set(sqr) {
                return Some(pieces[i]);
            }
        }
        return None;
    }

    pub fn validate_fen(fen: &str) -> Option<String> {
        let split: Vec<String> = fen.split(" ").map(|s| s.to_string()).collect();

        if split.len() != 6 {
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
                if !allowed_values.contains(&c) {
                    return Some("invalid FEN format: invalid casting rights value".to_string());
                } else if i < castling.len() - 1 {
                    let current = allowed_values.iter().position(|v| *v == c).unwrap();
                    let next = allowed_values.iter().position(|v| *v == castling.chars().nth(i + 1).unwrap()).unwrap();
                    if next < current {
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
                match self.get_piece_at(position) {
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
        str.push_str(&self.full_moves().to_string());

        return str;
    }
}

impl Default for BoardState {
    fn default() -> BoardState {
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
            score: 0,
            color_on_move: Color::White,
            castling: CastlingRight::default(),
            en_passant_position: None,
        };
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.to_fen());
    }
}

#[cfg(test)]
mod test {
    use crate::chess::{BoardState, Color, Move, MoveType, Piece, SquareLabel};

    #[test]
    fn fen() {
        let board_state = BoardState::default();

        let m = Move::new(MoveType::PawnJump, SquareLabel::B2.as_u64(), SquareLabel::B4.as_u64(), Piece::Pawn, Color::White, Piece::None);
        let next_state = board_state.make_move(m);
        println!("Fen: {}", next_state.to_fen());

        let m = Move::new(MoveType::PawnJump, SquareLabel::B7.as_u64(), SquareLabel::B5.as_u64(), Piece::Pawn, Color::Black, Piece::None);
        let next_state = next_state.make_move(m);
        println!("Fen: {}", next_state.to_fen());

        let m = Move::new(MoveType::Push, SquareLabel::C1.as_u64(), SquareLabel::A3.as_u64(), Piece::Knight, Color::White, Piece::None);
        let next_state = next_state.make_move(m);
        println!("Fen: {}", next_state.to_fen());

        let m = Move::new(MoveType::Push, SquareLabel::C8.as_u64(), SquareLabel::A6.as_u64(), Piece::Knight, Color::Black, Piece::None);
        let next_state = next_state.make_move(m);
        println!("Fen: {}", next_state.to_fen());

        // small hack with queen jump over pawn
        let m = Move::new(MoveType::Push, SquareLabel::D1.as_u64(), SquareLabel::D3.as_u64(), Piece::Queen, Color::White, Piece::None);
        let next_state = next_state.make_move(m);
        println!("Fen: {}", next_state.to_fen());

        let m = Move::new(MoveType::Push, SquareLabel::D8.as_u64(), SquareLabel::D5.as_u64(), Piece::Queen, Color::Black, Piece::None);
        let next_state = next_state.make_move(m);
        println!("Fen: {}", next_state.to_fen());

        let castling = Move::new(MoveType::Castling, SquareLabel::E1.as_u64(), SquareLabel::C1.as_u64(), Piece::King, Color::White, Piece::None);
        let next_state = next_state.make_move(castling);
        println!("Fen: {}", next_state.to_fen());


        let fen = BoardState::default().to_fen();
        println!("FEN: {}", fen);
        let from_fen = BoardState::from_fen(fen.as_str()).unwrap();
        print!("FEN2: {}", from_fen);
    }
}