use crate::bitboard::BitBoard;
use crate::chess::{BoardState, Color, Move, Piece, Square};
use crate::chess::move_provider::MoveProvider;
use crate::chess::movement::move_bitboard_constants::{*};

pub trait MoveGenerator {
    /// Generate attacks.
    fn generate_attacks(&self, board: &BoardState, color: Color) -> BitBoard;
    fn generate_moves(&self, board: &BoardState, f: &mut impl FnMut(Move));
}

/// Generator for diagonal moves (Bishop and Queen)
pub struct DiagonalMoveGenerator {
    pub a1h8_mask: [BitBoard; Square::ALL_FIELDS.len()],
    pub a1h8_magic: [BitBoard; Square::ALL_FIELDS.len()],
    pub a1h8_attacks: [[BitBoard; Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()],

    pub a8h1_mask: [BitBoard; Square::ALL_FIELDS.len()],
    pub a8h1_magic: [BitBoard; Square::ALL_FIELDS.len()],
    pub a8h1_attacks: [[BitBoard; Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()],
}

impl DiagonalMoveGenerator {
    /// Construct a new generator.
    pub const fn new() -> Self {
        let mut a1h8_index = [0usize; Square::ALL_FIELDS.len()];
        let mut a1h8_mask = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut a1h8_magic = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut a1h8_attacks =
            [[BitBoard::empty(); Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()];

        let mut a8h1_index = [0usize; Square::ALL_FIELDS.len()];
        let mut a8h1_mask = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut a8h1_magic = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut a8h1_attacks =
            [[BitBoard::empty(); Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()];

        let mut i = 0;
        while i < Square::ALL_FIELDS.len() {
            let rank = Square::from_usize(i).rank();
            let file = Square::from_usize(i).file();

            // compute index of diagonal for the field
            a1h8_index[i] = file + 7 - rank % 8;
            a8h1_index[i] = file + rank % 8;

            // compute 6-bit diagonal for the field
            a1h8_mask[i] =
                BitBoard::from(BitBoard::DIAGONAL_A1H8[a1h8_index[i]].raw() & !BitBoard::EDGES.raw());
            a8h1_mask[i] =
                BitBoard::from(BitBoard::DIAGONAL_A8H1[a8h1_index[i]].raw() & !BitBoard::EDGES.raw());

            // index magic multiplier for the field
            a1h8_magic[i] = MAGIC_A1H8[a1h8_index[i]];
            a8h1_magic[i] = MAGIC_A8H1[a8h1_index[i]];

            i += 1;
        }

        // precompute A1H8 moves
        // i is field index
        // n is 6 bit configuration
        // for all fields
        i = 0;
        while i < Square::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Square::ALL_FIELDS.len() {
                // get the diagonal
                let mut diagonal = BitBoard::DIAGONAL_A1H8[a1h8_index[i]];

                // reconstruct the state (number) into the diagonal
                // get the left/bottom bit - start of diagonal
                while diagonal.shifted_southwest().raw() != 0 {
                    diagonal = diagonal.shifted_southwest();
                }

                // traverse diagonal and set bits according to N
                let mut board = BitBoard::empty();

                let mut m = n as u64;
                while diagonal.raw() != 0 {
                    // shift down by one
                    diagonal = diagonal.shifted_northeast();
                    if (m & 1) != 0 {
                        board = BitBoard::from(board.raw() | diagonal.raw());
                    }
                    m >>= 1;
                }

                // make it 6-bit only
                board = BitBoard::from(board.raw() & !BitBoard::EDGES.raw());

                // pre-compute moves
                let mut moves = BitBoard::empty();

                // set piece in Ith position
                let mut piece = Square::new(i as u64).as_bb();

                // move in one direction
                while piece.raw() != 0 {
                    piece = piece.shifted_northeast();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // set piece back in Ith position
                piece = Square::new(i as u64).as_bb();

                // move in other direction
                while piece.raw() != 0 {
                    piece = piece.shifted_southwest();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // remember the moves
                a1h8_attacks[i][n] = moves;

                n += 1;
            }

            i += 1;
        }

        // precompute A8H1 moves
        // i is field index
        // n is 6 bit configuration
        // for all fields
        i = 0;
        while i < Square::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Square::ALL_FIELDS.len() {
                // get the diagonal
                let mut diagonal = BitBoard::DIAGONAL_A8H1[a8h1_index[i]];

                // get the left/top bit - start of the diagonal
                while diagonal.shifted_northwest().raw() != 0 {
                    diagonal = diagonal.shifted_northwest();
                }

                // traverse diagonal and set bits according to N
                let mut board = BitBoard::empty();

                let mut m = n as u64;
                while diagonal.raw() != 0 {
                    // shift down by one
                    diagonal = diagonal.shifted_southeast();
                    if (m & 1) != 0 {
                        board = BitBoard::from(board.raw() | diagonal.raw());
                    }
                    m >>= 1;
                }

                // make it 6-bit only
                board = BitBoard::from(board.raw() & !BitBoard::EDGES.raw());

                // pre-compute moves
                let mut moves = BitBoard::empty();

                // set piece in Ith position
                let mut piece = Square::new(i as u64).as_bb();

                // move in one direction
                while piece.raw() != 0 {
                    piece = piece.shifted_northwest();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // set piece back in Ith position
                piece = Square::new(i as u64).as_bb();

                // move in other direction
                while piece.raw() != 0 {
                    piece = piece.shifted_southeast();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // remember the moves
                a8h1_attacks[i][n] = moves;

                n += 1;
            }

            i += 1;
        }

        DiagonalMoveGenerator {
            a1h8_mask,
            a1h8_magic,
            a1h8_attacks,
            a8h1_mask,
            a8h1_magic,
            a8h1_attacks,
        }
    }

    /// Generate attacks for one piece.
    fn attacks(&self, i: Square, all_pieces: BitBoard) -> BitBoard {
        // use magic multipliers to get occupancy state index

        let index_a1h8 =
            ((all_pieces & self.a1h8_mask[i.as_usize()]).raw() * self.a1h8_magic[i.as_usize()].raw()) >> 57;
        let index_a8h1 =
            ((all_pieces & self.a8h1_mask[i.as_usize()]).raw() * self.a8h1_magic[i.as_usize()].raw()) >> 57;

        self.a1h8_attacks[i.as_usize()][index_a1h8 as usize]
            | self.a8h1_attacks[i.as_usize()][index_a8h1 as usize]
    }
}

impl MoveGenerator for DiagonalMoveGenerator {
    /// Generate attacks.
    fn generate_attacks(&self, board: &BoardState, color: Color) -> BitBoard {
        let mut b = board.pieces[color.index()][Piece::Bishop.index()] | board.pieces[color.index()][Piece::Queen.index()];
        let mut attacks = BitBoard::empty();

        let all_pieces = board.all_pieces();

        loop {
            if b.is_empty() {
                break;
            }

            let sq = b.lsb();
            b = BitBoard::from(b.raw() & (b.raw() - 1));
            attacks |= self.attacks(Square::from_usize(sq), all_pieces);
        }

        return attacks;
    }

    fn generate_moves(&self, board: &BoardState, f: &mut impl FnMut(Move)) {
        let all_pieces = board.all_pieces();
        let board_available = board.board_to_attack();

        for p in [Piece::Bishop, Piece::Queen] {
            let mut pieces = board.pieces[board.color_on_move.index()][p.index()];
            loop {
                if pieces.is_empty() {
                    break;
                }
                let from = Square::from_usize(pieces.lsb());
                pieces = BitBoard::from(pieces.raw() & (pieces.raw() - 1));
                let mut moves = self.attacks(from, all_pieces) & board_available;

                loop {
                    if moves.is_empty() { break; }

                    let to = moves.lsb();
                    moves = BitBoard::from(moves.raw() & (moves.raw() - 1));
                    f(Move::from_to_target(from.raw(), to as u64, Piece::None))
                }
            }
        }
    }
}


/// Generator for straight line moves (Rook and Queen)
pub struct LineMoveGenerator {
    pub rank_shift: [usize; Square::ALL_FIELDS.len()],
    pub rank_mask: [BitBoard; Square::ALL_FIELDS.len()],
    pub rank_attacks: [[BitBoard; Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()],

    pub file_magic: [BitBoard; Square::ALL_FIELDS.len()],
    pub file_mask: [BitBoard; Square::ALL_FIELDS.len()],
    pub file_attacks: [[BitBoard; Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()],
}

impl LineMoveGenerator {
    pub const fn new() -> Self {
        let file_a_mask = BitBoard::from(
            Square::A2.as_bb().raw()
                | Square::A3.as_bb().raw()
                | Square::A4.as_bb().raw()
                | Square::A5.as_bb().raw()
                | Square::A6.as_bb().raw()
                | Square::A7.as_bb().raw(),
        );

        let mut rank_shift = [0usize; Square::ALL_FIELDS.len()];
        let mut rank_mask = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut rank_attacks =
            [[BitBoard::empty(); Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()];

        let mut file_magic = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut file_mask = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut file_attacks =
            [[BitBoard::empty(); Square::ALL_FIELDS.len()]; Square::ALL_FIELDS.len()];

        let mut i = 0;
        while i < Square::ALL_FIELDS.len() {
            let rank = Square::from_usize(i).rank();
            let file = Square::from_usize(i).file();

            // get 6-bit mask for a rank
            rank_mask[i] = BitBoard::from(126 << (rank << 3));

            // compute needed rank shift
            rank_shift[i] = (rank << 3) + 1;

            // get 6-bit mask for a file
            file_mask[i] = BitBoard::from(file_a_mask.raw() << file);

            // index magic number directly fo field
            file_magic[i] = MAGIC_FILE[file];

            i += 1;
        }

        // precompute rank moves
        // for all pieces
        i = 0;
        while i < Square::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Square::ALL_FIELDS.len() {
                // reconstruct occupancy state
                let board = BitBoard::from(n as u64).shifted(1, Square::from_usize(i).rank() as isize);

                // generate available moves
                let mut moves = BitBoard::empty();

                // set piece in Ith position
                let mut piece = Square::from_usize(i).as_bb();

                // move in one direction
                while !piece.is_empty() {
                    piece = piece.shifted_west();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // set piece back in Ith position
                piece = Square::from_usize(i).as_bb();

                // move in other direction
                while !piece.is_empty() {
                    piece = piece.shifted_east();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // remember the moves
                rank_attacks[i][n] = moves;

                n += 1;
            }
            i += 1;
        }

        // precompute file moves
        // for all pieces
        i = 0;
        while i < Square::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Square::ALL_FIELDS.len() {
                // reconstruct occupancy state
                let board = BitBoard::from(n as u64)
                    .shifted(1, 0)
                    .mirrored_horizontally()
                    .mirrored_a1h8()
                    .shifted(Square::from_usize(i).file() as isize, 0);

                // generate available moves
                let mut moves = BitBoard::empty();

                // set piece in Ith position
                let mut piece = Square::from_usize(i).as_bb();

                // move in one direction
                while piece.raw() != 0 {
                    piece = piece.shifted_north();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // set piece back in Ith position
                piece = Square::from_usize(i).as_bb();

                // move in other direction
                while piece.raw() != 0 {
                    piece = piece.shifted_south();
                    moves = BitBoard::from(moves.raw() | piece.raw());

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.raw() & board.raw()) != 0 {
                        break;
                    }
                }

                // remember the moves
                file_attacks[i][n] = moves;
                n += 1;
            }
            i += 1;
        }

        LineMoveGenerator {
            rank_shift,
            rank_mask,
            rank_attacks,
            file_magic,
            file_mask,
            file_attacks,
        }
    }

    /// Generate attacks for one piece.
    fn attacks(&self, i: Square, all_pieces: BitBoard) -> BitBoard {
        // use magic multipliers to get occupancy state index
        let state_rank = (all_pieces & self.rank_mask[i.as_usize()]).raw() >> self.rank_shift[i.as_usize()];
        let state_file =
            ((all_pieces & self.file_mask[i.as_usize()]).raw() * self.file_magic[i.as_usize()].raw()) >> 57;

        // get possible attacks for field / occupancy state index
        self.rank_attacks[i.as_usize()][state_rank as usize]
            | self.file_attacks[i.as_usize()][state_file as usize]
    }
}

impl MoveGenerator for LineMoveGenerator {
    fn generate_attacks(&self, board: &BoardState, color: Color) -> BitBoard {
        let mut b = board.pieces[color.index()][Piece::Rook.index()] | board.pieces[color.index()][Piece::Queen.index()];
        let mut attacks = BitBoard::empty();

        let all_pieces = board.all_pieces();

        loop {
            if b.is_empty() {
                break;
            }

            let sq = b.lsb();
            b = BitBoard::from(b.raw() & (b.raw() - 1));
            attacks |= self.attacks(Square::from_usize(sq), all_pieces);
        }

        return attacks;
    }

    fn generate_moves(&self, board: &BoardState, f: &mut impl FnMut(Move)) {
        let all_pieces = board.all_pieces();
        let board_available = board.board_to_attack();

        for p in [Piece::Rook, Piece::Queen] {
            let mut pieces = board.pieces[board.color_on_move.index()][p.index()];
            loop {
                if pieces.is_empty() {
                    break;
                }
                let from = Square::from_usize(pieces.lsb());
                pieces = BitBoard::from(pieces.raw() & (pieces.raw() - 1));
                let mut moves = self.attacks(from, all_pieces) & board_available;

                loop {
                    if moves.is_empty() {
                        break;
                    }

                    let to = moves.lsb();
                    moves = BitBoard::from(moves.raw() & (moves.raw() - 1));
                    f(Move::from_to_target(from.raw(), to as u64, Piece::None))
                }
            }
        }
    }
}

pub struct KnightJumpMoveGenerator {
    pub cached_attacks: [BitBoard; Square::ALL_FIELDS.len()],
}

impl KnightJumpMoveGenerator {
    pub const fn new() -> Self {
        let mut cache = [BitBoard::empty(); Square::ALL_FIELDS.len()];
        let mut i = 0;
        while i < Square::ALL_FIELDS.len() {
            let b = Square::ALL_FIELDS[i].as_bb();
            cache[i] = BitBoard::from(
                b.shifted(2, 1).raw()
                    | b.shifted(2, -1).raw()
                    | b.shifted(1, 2).raw()
                    | b.shifted(-1, 2).raw()
                    | b.shifted(-2, 1).raw()
                    | b.shifted(-2, -1).raw()
                    | b.shifted(-1, -2).raw()
                    | b.shifted(1, -2).raw(),
            );
            i += 1;
        }
        return KnightJumpMoveGenerator {
            cached_attacks: cache,
        };
    }
}

impl MoveGenerator for KnightJumpMoveGenerator {
    fn generate_attacks(&self, board: &BoardState, color: Color) -> BitBoard {
        let mut b = board.pieces[color.index()][Piece::Knight.index()];
        let mut attacks = BitBoard::empty();
        loop {
            if b.is_empty() { break; }
            let sq = b.lsb();
            b = BitBoard::from(b.raw() & (b.raw() - 1));
            attacks |= self.cached_attacks[sq];
        }
        return attacks;
    }

    fn generate_moves(&self, board: &BoardState, f: &mut impl FnMut(Move)) {
        let mut pieces = board.pieces[board.color_on_move.index()][Piece::Knight.index()];
        loop {
            if pieces.is_empty() { break; }
            let from = pieces.lsb();
            pieces = BitBoard::from(pieces.raw() & (pieces.raw() - 1));
            let mut moves = self.cached_attacks[from] & board.board_to_attack();
            loop {
                if moves.is_empty() { break; }
                let to = moves.lsb();
                moves = BitBoard::from(moves.raw() & (moves.raw() - 1));
                f(Move::from_to_target(from as u64, to as u64, Piece::None))
            }
        }
    }
}

pub struct PawnMoveGenerator {
    pub cached_attacks: [[BitBoard; Square::ALL_FIELDS.len()]; 2],
}

impl PawnMoveGenerator {
    /// Construct a new generator.
    pub const fn new() -> Self {
        let mut cache = [[BitBoard::empty(); Square::ALL_FIELDS.len()]; 2];

        let mut i = 0;
        while i < Square::ALL_FIELDS.len() {
            let b = Square::ALL_FIELDS[i].as_bb();
            cache[Color::White as usize][i] = BitBoard::from(b.shifted_northeast().raw() | b.shifted_northwest().raw());
            cache[Color::Black as usize][i] = BitBoard::from(b.shifted_southeast().raw() | b.shifted_southwest().raw());
            i += 1;
        }

        PawnMoveGenerator {
            cached_attacks: cache,
        }
    }
}

impl MoveGenerator for PawnMoveGenerator {
    fn generate_attacks(&self, board: &BoardState, color: Color) -> BitBoard {
        let b = board.pieces[color.index()][Piece::Pawn.index()];

        match color {
            Color::White => b.shifted_northeast() | b.shifted_northwest(),
            Color::Black => b.shifted_southeast() | b.shifted_southwest(),
        }
    }

    fn generate_moves(&self, board: &BoardState, f: &mut impl FnMut(Move)) {
        let empty_squares = !board.all_pieces();
        let mut pieces = board.pieces[board.color_on_move.index()][Piece::Pawn.index()];
        loop {
            if pieces.is_empty() { break; }
            let from = pieces.lsb();
            let from_bb = Square::from_usize(from).as_bb();
            pieces = BitBoard::from(pieces.raw() & (pieces.raw() - 1));
            let (mut attacks, mut moves) = match board.color_on_move {
                Color::White => {
                    let mut moves = from_bb.shifted_north() & empty_squares;
                    if from < Square::A3.as_usize() {
                        moves |= moves.shifted_north() & empty_squares;
                    }
                    let attacks = from_bb.shifted_northwest() | from_bb.shifted_northeast();
                    moves |= attacks & board.opposite_pieces();
                    (attacks, moves)
                }
                Color::Black => {
                    let mut moves = from_bb.shifted_south() & empty_squares;
                    if from > Square::H6.as_usize() {
                        moves |= moves.shifted_south() & empty_squares;
                    }
                    let attacks = from_bb.shifted_southwest() | from_bb.shifted_southeast();
                    moves |= attacks & board.opposite_pieces();
                    (attacks, moves)
                }
            };

            loop {
                if moves.is_empty() { break; }
                let to = moves.lsb();
                moves = BitBoard::from(moves.raw() & (moves.raw() - 1));
                // If pawn can be promoted
                if (to > Square::H7.as_usize() || to < Square::A2.as_usize()) {
                    f(Move::from_to_target(from as u64, to as u64, Piece::Queen));
                    f(Move::from_to_target(from as u64, to as u64, Piece::Bishop));
                    f(Move::from_to_target(from as u64, to as u64, Piece::Knight));
                    f(Move::from_to_target(from as u64, to as u64, Piece::Rook));
                } else {
                    f(Move::from_to_target(from as u64, to as u64, Piece::None))
                }
            }

            if let Some(ep_square) = board.en_passant_position {
                moves = attacks & ep_square.as_bb();
                if (!moves.is_empty()) {
                    let to = moves.lsb();
                    f(Move::from_to_target(from as u64, to as u64, Piece::None))
                }
            }
        }
    }
}

pub struct KingMoveGenerator {
    pub cached_attacks: [BitBoard; Square::ALL_FIELDS.len()],
}

impl KingMoveGenerator {
    pub const fn new() -> Self {
        let mut cache = [BitBoard::empty(); Square::ALL_FIELDS.len()];

        let mut i = 0;
        while i < Square::ALL_FIELDS.len() {
            let b = Square::ALL_FIELDS[i].as_bb();

            cache[i] = BitBoard::from(
                b.shifted(1, -1).raw()
                    | b.shifted(1, 0).raw()
                    | b.shifted(1, 1).raw()
                    | b.shifted(0, -1).raw()
                    | b.shifted(0, 1).raw()
                    | b.shifted(-1, -1).raw()
                    | b.shifted(-1, 0).raw()
                    | b.shifted(-1, 1).raw(),
            );

            i += 1;
        }

        return KingMoveGenerator {
            cached_attacks: cache,
        };
    }
}

impl MoveGenerator for KingMoveGenerator {
    fn generate_attacks(&self, board: &BoardState, color: Color) -> BitBoard {
        let b = board.pieces[color.index()][Piece::King.index()];
        if b.is_empty() {
            return BitBoard::empty();
        }

        let square = b.lsb();
        return self.cached_attacks[square];
    }

    fn generate_moves(&self, board: &BoardState, f: &mut impl FnMut(Move)) {
        let b = board.pieces[board.color_on_move.index()][Piece::King.index()];
        if (b.is_empty()) {
            return;
        }
        let from = b.lsb();

        let mut moves = self.cached_attacks[from] & board.board_to_attack();

        loop {
            if moves.is_empty() { break; }
            let to = moves.lsb();
            moves = BitBoard::from(moves.raw() & (moves.raw() - 1));
            f(Move::from_to_target(from as u64, to as u64, Piece::None))
        }

        let all_pieces = board.all_pieces();

        match board.color_on_move {
            Color::White => {
                if board.castling.is_white_king_side_allowed()
                    && (all_pieces & WHITE_CASTLING_KING_SIDE_REQUIRED_EMPTY).is_empty()
                    && !MoveProvider::INSTANCE.is_under_attack(board, Color::Black, WHITE_CASTLING_KING_SIDE_ATTACK_MASK) {
                    f(Move::from_to_target(from as u64, Square::G1.raw(), Piece::None))
                }

                if board.castling.is_white_queen_side_allowed()
                    && (all_pieces & WHITE_CASTLING_QUEEN_SIDE_REQUIRED_EMPTY).is_empty()
                    && !MoveProvider::INSTANCE.is_under_attack(board, Color::Black, WHITE_CASTLING_QUEEN_SIDE_ATTACK_MASK) {
                    f(Move::from_to_target(from as u64, Square::C1.raw(), Piece::None))
                }
            }
            Color::Black => {
                if board.castling.is_black_king_side_allowed()
                    && (all_pieces & BLACK_CASTLING_KING_SIDE_REQUIRED_EMPTY).is_empty()
                    && !MoveProvider::INSTANCE.is_under_attack(board, Color::White, BLACK_CASTLING_KING_SIDE_ATTACK_MASK) {
                    f(Move::from_to_target(from as u64, Square::G8.raw(), Piece::None))
                }

                if board.castling.is_black_queen_side_allowed()
                    && (all_pieces & BLACK_CASTLING_QUEEN_SIDE_REQUIRED_EMPTY).is_empty()
                    && !MoveProvider::INSTANCE.is_under_attack(board, Color::White, BLACK_CASTLING_QUEEN_SIDE_ATTACK_MASK) {
                    f(Move::from_to_target(from as u64, Square::C8.raw(), Piece::None))
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::bitboard::BitBoard;
    use crate::chess::{BoardState, Color, Move, MoveType, Piece, Square, SquareLabel};
    use crate::chess::movement::move_generator::{DiagonalMoveGenerator, PawnMoveGenerator, KnightJumpMoveGenerator, LineMoveGenerator, MoveGenerator};

    #[test]
    fn test_move_generator() {


        // let bishop_generator = DiagonalMoveGenerator::new();
        // let rook_generator = LineMoveGenerator::new();
        // let knight_move_generator = KnightJumpMoveGenerator::new();
        // let pawn_move_generator = PawnMoveGenerator::new();
        //
        // // let board = BoardState::from_fen("4b3/7Q/2P2R2/1k5r/3q4/2R3b1/2B4B/2K5 w - - 0 1").unwrap();
        // let board = BoardState::default();
        //
        // println!("Board: {}", board.all_pieces());
        // println!("Piece: {}", board.get_piece_at(Square::C1.as_usize() as u8).unwrap().0);
        // let mut moves = Vec::new();
        // bishop_generator.generate_moves(&board, &mut |m| moves.push(m));
        // rook_generator.generate_moves(&board, &mut |m| moves.push(m));
        // knight_move_generator.generate_moves(&board, &mut |m| moves.push(m));
        // pawn_move_generator.generate_moves(&board, &mut |m| moves.push(m));
        //
        // for m in moves {
        //     println!("Move from {} to {} with piece {}", m.get_from(), m.get_to(), board.get_piece_at(m.get_from().raw() as u8).unwrap().0)
        // }
    }
}
