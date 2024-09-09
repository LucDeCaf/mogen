pub mod board;
pub mod magic;
pub mod perft;
pub mod r#static;

use board::{bitboard::Bitboard, color::Color, piece::Piece, r#move::Move, square::Square, Board};
use magic::SlidingMoveGen;
use r#static::move_masks::{BLACK_PAWN_CAPTURE_MASKS, KING_MOVE_MASKS, WHITE_PAWN_CAPTURE_MASKS};

pub struct MoveGen {
    smg: SlidingMoveGen,
}

impl MoveGen {
    pub fn new() -> Self {
        Self {
            smg: SlidingMoveGen::new(),
        }
    }

    pub fn knight_moves(board: &Board, color: Color, square: Square, moves: &mut Vec<Move>) {
        let blocker_mask = board.color_bitboard(color);
        let mut move_mask =
            r#static::move_masks::KNIGHT_MOVE_MASKS[square as usize] & !blocker_mask;

        while !move_mask.is_empty() {
            let target = Square::ALL[move_mask.pop_lsb()];
            moves.push(Move::new(square, target, None));
        }
    }

    pub fn bishop_moves(&self, board: &Board, color: Color, square: Square, moves: &mut Vec<Move>) {
        let blockers = board.all_pieces();
        let friendly_pieces = board.color_bitboard(color);
        let mut move_mask = self.smg.bishop_moves(square, blockers) & !friendly_pieces;

        while !move_mask.is_empty() {
            let target = Square::ALL[move_mask.pop_lsb()];
            moves.push(Move::new(square, target, None));
        }
    }

    pub fn rook_moves(&self, board: &Board, color: Color, square: Square, moves: &mut Vec<Move>) {
        let blockers = board.all_pieces();
        let friendly_pieces = board.color_bitboard(color);
        let mut move_mask = self.smg.rook_moves(square, blockers) & !friendly_pieces;

        while !move_mask.is_empty() {
            let target = Square::ALL[move_mask.pop_lsb()];
            moves.push(Move::new(square, target, None));
        }
    }

    pub fn queen_moves(&self, board: &Board, color: Color, square: Square, moves: &mut Vec<Move>) {
        self.rook_moves(board, color, square, moves);
        self.bishop_moves(board, color, square, moves);
    }

    pub fn king_moves(board: &Board, color: Color, square: Square, moves: &mut Vec<Move>) {
        let friendly_pieces = board.color_bitboard(color);
        let mut move_mask = KING_MOVE_MASKS[square as usize] & !friendly_pieces;

        while !move_mask.is_empty() {
            let target = Square::ALL[move_mask.pop_lsb()];
            moves.push(Move::new(square, target, None));
        }
    }

    fn moves_with_possible_promotions(source: Square, target: Square, moves: &mut Vec<Move>) {
        // Rank 2 to 7
        if (8..56).contains(&(target as usize)) {
            moves.push(Move::new(source, target, None));
        }
        // Rank 1 and 8
        else {
            // Promotions
            moves.push(Move::new(source, target, Some(Piece::Knight)));
            moves.push(Move::new(source, target, Some(Piece::Bishop)));
            moves.push(Move::new(source, target, Some(Piece::Rook)));
            moves.push(Move::new(source, target, Some(Piece::Queen)));
        }
    }

    pub fn pawn_moves(board: &Board, color: Color, moves: &mut Vec<Move>) {
        let all_pieces = board.all_pieces();
        let pawns = board.bitboard(Piece::Pawn, color);
        let start_rank = match color {
            Color::White => Bitboard(0x000000000000ff00), // Rank 2
            Color::Black => Bitboard(0x00ff000000000000), // Rank 7
        };
        let unmoved_pawns = pawns & start_rank;

        let mut single_move_targets = match color {
            Color::White => (pawns << 8_u8) & !all_pieces,
            Color::Black => (pawns >> 8_u8) & !all_pieces,
        };

        let mut double_move_targets = match color {
            Color::White => (((unmoved_pawns << 8_u8) & !all_pieces) << 8_u8) & !all_pieces,
            Color::Black => (((unmoved_pawns >> 8_u8) & !all_pieces) >> 8_u8) & !all_pieces,
        };

        // * Single moves

        while !single_move_targets.is_empty() {
            let target_i = single_move_targets.pop_lsb();
            let source_i = (target_i as i8 - (8 * color.direction())) as usize;

            let target = Square::ALL[target_i];
            let source = Square::ALL[source_i];

            // Pawns may need to promote
            Self::moves_with_possible_promotions(source, target, moves);
        }

        // * Double moves

        while !double_move_targets.is_empty() {
            let target_i = double_move_targets.pop_lsb();
            let source_i = (target_i as i8 - (16 * color.direction())) as usize;

            let target = Square::ALL[target_i];
            let source = Square::ALL[source_i];

            // Double moves never lead to promotion
            moves.push(Move::new(source, target, None));
        }
    }

    // ? This may not be the fastest solution, benchmark others and compare against this
    fn pawn_captures(board: &Board, color: Color, moves: &mut Vec<Move>) {
        let enemy_pieces = board.color_bitboard(color.inverse());
        let capture_masks = match color {
            Color::White => &WHITE_PAWN_CAPTURE_MASKS,
            Color::Black => &BLACK_PAWN_CAPTURE_MASKS,
        };

        let en_passant = match board.en_passant_square() {
            Some(square) => square.bitboard(),
            None => Bitboard::EMPTY,
        };

        let mut pawns = board.bitboard(Piece::Pawn, color);
        while !pawns.is_empty() {
            let source_i = pawns.pop_lsb();
            let source = Square::ALL[source_i];

            let capture_mask = capture_masks[source_i];
            let mut targets = (capture_mask & enemy_pieces) | (capture_mask & en_passant);

            while !targets.is_empty() {
                let target_i = targets.pop_lsb();
                let target = Square::ALL[target_i];

                // Pawns may need to promote
                Self::moves_with_possible_promotions(source, target, moves);
            }
        }
    }

    pub fn pseudolegal_moves(&self, board: &Board, moves: &mut Vec<Move>) {
        let friendly_color = board.active_color;

        // Knight moves
        let mut knight_bitboard = board.bitboard(Piece::Knight, friendly_color);

        while !knight_bitboard.is_empty() {
            let i = knight_bitboard.pop_lsb();
            let from_square = Square::ALL[i];

            Self::knight_moves(board, friendly_color, from_square, moves);
        }

        // Bishop moves
        let mut bishop_bitboard = board.bitboard(Piece::Bishop, friendly_color);

        while !bishop_bitboard.is_empty() {
            let i = bishop_bitboard.pop_lsb();
            let from_square = Square::ALL[i];

            self.bishop_moves(board, friendly_color, from_square, moves);
        }

        // Rook moves
        let mut rook_bitboard = board.bitboard(Piece::Rook, friendly_color);

        while !rook_bitboard.is_empty() {
            let i = rook_bitboard.pop_lsb();
            let from_square = Square::ALL[i];

            self.rook_moves(board, friendly_color, from_square, moves);
        }

        // Queen moves
        let mut queen_bitboard = board.bitboard(Piece::Queen, friendly_color);

        while !queen_bitboard.is_empty() {
            let i = queen_bitboard.pop_lsb();
            let from_square = Square::ALL[i];

            self.queen_moves(board, friendly_color, from_square, moves);
        }

        // King moves
        let king_i = board.bitboard(Piece::King, friendly_color).trailing_zeros() as usize;
        let king_square = Square::ALL[king_i];

        Self::king_moves(board, friendly_color, king_square, moves);

        // Pawn moves
        Self::pawn_moves(board, friendly_color, moves);

        // Pawn captures
        Self::pawn_captures(board, friendly_color, moves);
    }
}

impl Default for MoveGen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use board::bitboard::Bitboard;

    use super::*;

    #[test]
    fn test_knight_moves() {
        let mut moves = Vec::new();
        MoveGen::knight_moves(&Board::new(), Color::White, Square::E4, &mut moves);

        assert_eq!(moves.len(), 8);

        let mask = moves[0].bitboard()
            | moves[1].bitboard()
            | moves[2].bitboard()
            | moves[3].bitboard()
            | moves[4].bitboard()
            | moves[5].bitboard()
            | moves[6].bitboard()
            | moves[7].bitboard();

        assert_eq!(
            mask,
            Square::E4.bitboard()
                | Square::D2.bitboard()
                | Square::C3.bitboard()
                | Square::C5.bitboard()
                | Square::D6.bitboard()
                | Square::F6.bitboard()
                | Square::G5.bitboard()
                | Square::G3.bitboard()
                | Square::F2.bitboard()
        );
    }

    #[test]
    fn test_knight_captures() {
        let mut board = Board::new();
        board.add_piece(Piece::Pawn, Color::White, Square::D2);
        board.add_piece(Piece::Pawn, Color::Black, Square::C3);
        board.add_piece(Piece::Pawn, Color::Black, Square::C5);
        board.add_piece(Piece::Pawn, Color::Black, Square::D6);
        board.add_piece(Piece::Pawn, Color::White, Square::F6);
        board.add_piece(Piece::Pawn, Color::White, Square::G5);
        board.add_piece(Piece::Pawn, Color::Black, Square::G3);
        board.add_piece(Piece::Pawn, Color::Black, Square::F2);

        let mut moves = vec![];
        MoveGen::knight_moves(&board, Color::White, Square::E4, &mut moves);

        assert_eq!(moves.len(), 5);

        let mask = moves[0].bitboard()
            | moves[1].bitboard()
            | moves[2].bitboard()
            | moves[3].bitboard()
            | moves[4].bitboard();

        assert_eq!(
            mask,
            Square::E4.bitboard()
                | Square::C3.bitboard()
                | Square::C5.bitboard()
                | Square::D6.bitboard()
                | Square::G3.bitboard()
                | Square::F2.bitboard()
        );
    }

    #[test]
    fn test_bishop_moves() {
        let mut board = Board::new();
        let move_gen = MoveGen::new();

        let mut moves = Vec::new();
        move_gen.bishop_moves(&board, Color::White, Square::E4, &mut moves);

        assert_eq!(moves.len(), 13);

        let mut mask = Bitboard::EMPTY;
        for mv in &moves {
            mask |= mv.to().bitboard();
        }

        assert_eq!(mask, Bitboard(0x182442800284482));

        board.add_piece(Piece::Pawn, Color::White, Square::C6);
        board.add_piece(Piece::Pawn, Color::White, Square::G6);
        board.add_piece(Piece::Pawn, Color::Black, Square::C2);
        board.add_piece(Piece::Pawn, Color::Black, Square::H1);

        moves.clear();
        move_gen.bishop_moves(&board, Color::White, Square::E4, &mut moves);

        assert_eq!(moves.len(), 7);

        let mut mask = Bitboard::EMPTY;
        for mv in moves {
            mask |= mv.to().bitboard();
        }

        assert_eq!(mask, Bitboard(0x2800284480));
    }

    #[test]
    fn test_rook_moves() {
        let mut board = Board::new();
        let move_gen = MoveGen::new();

        let mut moves = Vec::new();
        move_gen.rook_moves(&board, Color::White, Square::E1, &mut moves);

        assert_eq!(moves.len(), 14);

        let mut mask = Bitboard::EMPTY;
        for mv in &moves {
            mask |= mv.to().bitboard();
        }

        assert_eq!(mask, Bitboard(0x10101010101010ef));

        moves.clear();
        board.add_piece(Piece::Knight, Color::White, Square::B1);
        board.add_piece(Piece::Knight, Color::White, Square::E5);
        board.add_piece(Piece::Knight, Color::Black, Square::H1);

        move_gen.rook_moves(&board, Color::White, Square::E1, &mut moves);

        assert_eq!(moves.len(), 8);

        mask = Bitboard::EMPTY;
        for mv in &moves {
            mask |= mv.to().bitboard();
        }

        assert_eq!(mask, Bitboard(0x101010ec));
    }

    #[test]
    fn test_king_moves() {
        let mut board = Board::new();

        board.add_piece(Piece::Knight, Color::White, Square::D3);
        board.add_piece(Piece::Knight, Color::Black, Square::E5);

        let mut moves = Vec::new();
        MoveGen::king_moves(&board, Color::White, Square::E4, &mut moves);

        let mut mask = Bitboard::EMPTY;
        for mv in moves {
            mask |= mv.to().bitboard();
        }

        assert_eq!(mask, Bitboard(0x3828300000));
    }

    #[test]
    fn test_pawn_moves() {
        let mut board = Board::new();

        board.add_piece(Piece::Pawn, Color::White, Square::E2);
        board.add_piece(Piece::Pawn, Color::Black, Square::D7);

        let mut moves = Vec::new();

        MoveGen::pawn_moves(&board, board.active_color, &mut moves);
        assert_eq!(moves.len(), 2);

        for mv in &moves {
            assert!(
                *mv == Move::new(Square::E2, Square::E4, None)
                    || *mv == Move::new(Square::E2, Square::E3, None)
            );
        }

        board = board.make_move(Move::new(Square::E2, Square::E4, None));
        moves.clear();

        MoveGen::pawn_moves(&board, board.active_color, &mut moves);
        assert_eq!(moves.len(), 2);

        for mv in &moves {
            assert!(
                *mv == Move::new(Square::D7, Square::D5, None)
                    || *mv == Move::new(Square::D7, Square::D6, None)
            );
        }

        board = board.make_move(Move::new(Square::D7, Square::D6, None));
        moves.clear();

        MoveGen::pawn_moves(&board, board.active_color, &mut moves);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::new(Square::E4, Square::E5, None));

        board = board.make_move(moves[0]);
        moves.clear();

        MoveGen::pawn_moves(&board, board.active_color, &mut moves);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::new(Square::D6, Square::D5, None));
    }

    #[test]
    fn test_pawn_captures() {
        let mut board = Board::new();
        board.add_piece(Piece::Pawn, Color::White, Square::A2);
        board.add_piece(Piece::Pawn, Color::Black, Square::B3);
        board.add_piece(Piece::Pawn, Color::Black, Square::H3);

        let mut moves = Vec::new();

        MoveGen::pawn_captures(&board, board.active_color, &mut moves);
        assert_eq!(moves.len(), 1);

        assert_eq!(moves[0], Move::new(Square::A2, Square::B3, None));

        board.active_color = Color::Black;
        moves.clear();

        MoveGen::pawn_captures(&board, board.active_color, &mut moves);
        assert_eq!(moves.len(), 1);

        assert_eq!(moves[0], Move::new(Square::B3, Square::A2, None));
    }

    #[test]
    fn test_pawn_en_passant() {
        let mut board = Board::new();
        board.add_piece(Piece::Pawn, Color::White, Square::E2);
        board.add_piece(Piece::Pawn, Color::Black, Square::D4);

        board = board.make_move(Move::new(Square::E2, Square::E4, None));

        let mut moves = Vec::new();

        MoveGen::pawn_captures(&board, board.active_color, &mut moves);
        assert_eq!(moves.len(), 1);

        assert_eq!(moves[0], Move::new(Square::D4, Square::E3, None));
    }

    #[test]
    fn test_white_pawn_captures() {
        let mut board = Board::new();

        board.add_piece(Piece::Pawn, Color::White, Square::E6);
        board.add_piece(Piece::Knight, Color::Black, Square::E7);
        board.add_piece(Piece::Knight, Color::White, Square::D7);
        board.add_piece(Piece::Knight, Color::Black, Square::F7);

        let mut moves = Vec::new();
        MoveGen::pawn_captures(&board, Color::White, &mut moves);

        assert_eq!(moves.len(), 1);

        assert_eq!(moves[0].from(), Square::E6);
        assert_eq!(moves[0].to(), Square::F7);
        assert_eq!(moves[0].promotion(), None);

        moves.clear();
        board = Board::new();

        board.add_piece(Piece::Pawn, Color::White, Square::B7);
        board.add_piece(Piece::Knight, Color::Black, Square::C8);
        board.add_piece(Piece::Knight, Color::White, Square::A8);

        MoveGen::pawn_captures(&board, Color::White, &mut moves);

        assert_eq!(moves.len(), 4);

        let first = moves[0];

        assert_ne!(first.promotion(), None);

        for mv in &moves[1..] {
            assert_eq!(first.from(), mv.from());
            assert_eq!(first.to(), mv.to());
            assert_ne!(first.promotion(), mv.promotion());
            assert_ne!(mv.promotion(), None);
        }
    }

    #[test]
    fn test_black_pawn_captures() {
        let mut board = Board::new();

        board.add_piece(Piece::Pawn, Color::Black, Square::E3);
        board.add_piece(Piece::Knight, Color::White, Square::E2);
        board.add_piece(Piece::Knight, Color::Black, Square::D2);
        board.add_piece(Piece::Knight, Color::White, Square::F2);

        let mut moves = Vec::new();
        MoveGen::pawn_captures(&board, Color::Black, &mut moves);

        assert_eq!(moves.len(), 1);

        assert_eq!(moves[0].from(), Square::E3);
        assert_eq!(moves[0].to(), Square::F2);
        assert_eq!(moves[0].promotion(), None);

        moves.clear();
        board = Board::new();

        board.add_piece(Piece::Pawn, Color::Black, Square::B2);
        board.add_piece(Piece::Knight, Color::White, Square::C1);
        board.add_piece(Piece::Knight, Color::Black, Square::A1);

        MoveGen::pawn_captures(&board, Color::Black, &mut moves);

        assert_eq!(moves.len(), 4);

        let first = moves[0];

        assert_ne!(first.promotion(), None);

        for mv in &moves[1..] {
            assert_eq!(first.from(), mv.from());
            assert_eq!(first.to(), mv.to());
            assert_ne!(first.promotion(), mv.promotion());
            assert_ne!(mv.promotion(), None);
        }
    }
}
