pub mod bitboard;
pub mod color;
pub mod flags;
pub mod r#move;
pub mod piece;
pub mod square;

use bitboard::Bitboard;
use color::Color;
use flags::Flags;
use piece::Piece;
use r#move::Move;
use square::Square;

use crate::r#static::generation::coords;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseFenError {
    WrongSectionCount,
    BadPosition,
    BadActiveColor,
    BadCastlingRights,
    BadEnPassant,
    BadHalfmoves,
    BadFullmoves,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub bitboards: [Bitboard; 8],
    pub active_color: Color,
    pub flags: Flags,
    pub halfmoves: u8, // Max halfmoves is 100 (50 move rule) or 150 (75 move rule) < u8::MAX
    pub fullmoves: u16, // u8::MAX < Max fullmoves in one game < u16::MAX
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboards: [Bitboard::EMPTY; 8],
            active_color: Color::White,
            flags: Flags(0),
            halfmoves: 0,
            fullmoves: 0,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, ParseFenError> {
        let mut board = Board::new();

        let mut parts = fen.split_ascii_whitespace();

        let Some(position_string) = parts.next() else {
            return Err(ParseFenError::WrongSectionCount);
        };

        let mut rank: i8 = 7;
        let mut file: i8 = 0;

        for char in position_string.chars() {
            match char {
                '0'..='8' => {
                    let digit = char.to_digit(9).unwrap() as i8;
                    file += digit;
                }
                'p' | 'n' | 'b' | 'r' | 'q' | 'k' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                    let color = if char.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    let piece = Piece::try_from(char).unwrap();

                    let square = Square::ALL[rank as usize * 8 + file as usize];
                    board.add_piece(piece, color, square);

                    file += 1;
                }
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                _ => return Err(ParseFenError::BadPosition),
            }
        }

        let Some(active_color) = parts.next() else {
            return Err(ParseFenError::WrongSectionCount);
        };

        if active_color.len() != 1 {
            return Err(ParseFenError::BadActiveColor);
        }

        board.active_color = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(ParseFenError::BadActiveColor),
        };

        let Some(castling_rights) = parts.next() else {
            return Err(ParseFenError::WrongSectionCount);
        };

        for ch in castling_rights.chars() {
            board.flags.0 |= match ch {
                'K' => Flags::WHITE_KINGSIDE,
                'Q' => Flags::WHITE_QUEENSIDE,
                'k' => Flags::BLACK_KINGSIDE,
                'q' => Flags::BLACK_QUEENSIDE,
                _ => return Err(ParseFenError::BadCastlingRights),
            };
        }

        let Some(en_passant) = parts.next() else {
            return Err(ParseFenError::WrongSectionCount);
        };

        if en_passant != "-" {
            if en_passant.len() != 2 {
                return Err(ParseFenError::BadEnPassant);
            }

            let mut chars = en_passant.chars();

            let Some(file) = chars.next() else {
                return Err(ParseFenError::BadEnPassant);
            };

            board.flags.0 |= match file {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => return Err(ParseFenError::BadEnPassant),
            };

            match chars.next() {
                Some('3') | Some('6') => (),
                _ => return Err(ParseFenError::BadEnPassant),
            }
        }

        let Some(halfmoves) = parts.next() else {
            return Err(ParseFenError::WrongSectionCount);
        };

        if let Ok(value) = halfmoves.parse::<u8>() {
            board.halfmoves = value;
        } else {
            return Err(ParseFenError::BadHalfmoves);
        }

        let Some(fullmoves) = parts.next() else {
            return Err(ParseFenError::WrongSectionCount);
        };

        if let Ok(value) = fullmoves.parse::<u16>() {
            board.fullmoves = value;
        } else {
            return Err(ParseFenError::BadFullmoves);
        }

        Ok(board)
    }

    pub fn all_pieces(&self) -> Bitboard {
        self.color_bitboard(Color::White) | self.color_bitboard(Color::Black)
    }

    pub fn add_piece(&mut self, piece: Piece, color: Color, square: Square) {
        let position = square.bitboard();
        *self.piece_bitboard_mut(piece) |= position;
        *self.color_bitboard_mut(color) |= position;
    }

    pub fn piece_bitboard(&self, piece: Piece) -> Bitboard {
        self.bitboards[piece as usize]
    }

    pub fn piece_bitboard_mut(&mut self, piece: Piece) -> &mut Bitboard {
        &mut self.bitboards[piece as usize]
    }

    pub fn color_bitboard(&self, color: Color) -> Bitboard {
        self.bitboards[color as usize + 6]
    }

    pub fn color_bitboard_mut(&mut self, color: Color) -> &mut Bitboard {
        &mut self.bitboards[color as usize + 6]
    }

    pub fn bitboard(&self, piece: Piece, color: Color) -> Bitboard {
        self.piece_bitboard(piece) & self.color_bitboard(color)
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let mask = square.bitboard();

        for (i, bitboard) in self.bitboards.into_iter().enumerate() {
            if !(bitboard & mask).is_empty() {
                return Some(Piece::ALL[i]);
            }
        }

        None
    }

    pub fn make_move(&self, mv: Move) -> Self {
        let mut board = self.clone();
        board.flags.set_en_passant(false);

        let from = mv.from();
        let to = mv.to();
        let promotion = mv.promotion();

        let from_color = if (board.color_bitboard(Color::White) & from.bitboard()).is_empty() {
            Color::Black
        } else {
            Color::White
        };

        let Some(from_piece) = board.piece_at(from) else {
            return board;
        };
        let to_piece = board.piece_at(to);

        // En passant
        if from_piece == Piece::Pawn {
            let (from_rank, from_file) = coords(from as u8);
            let (to_rank, to_file) = coords(to as u8);

            let rank_diff = from_rank.abs_diff(to_rank);

            let ep_rank: u8 = match from_color {
                Color::White => 5,
                Color::Black => 2,
            };
            let ep_file = self.flags.en_passant_file();

            // Double move
            if rank_diff == 2 {
                board.flags.set_en_passant(true);
                board.flags.set_en_passant_file(from_file);
            }
            // En passant
            else if self.flags.can_en_passant() && to_rank == ep_rank && to_file == ep_file {
                let captured_pawn_rank = from_rank;
                let captured_pawn_file = to_file;
                let captured_pawn_i = (captured_pawn_rank * 8) + captured_pawn_file;
                let mask = Bitboard(1 << captured_pawn_i);

                // Remove pawn
                *board.piece_bitboard_mut(Piece::Pawn) ^= mask;
                *board.color_bitboard_mut(from_color.inverse()) ^= mask;
            }
        }

        // From
        *board.piece_bitboard_mut(from_piece) ^= from.bitboard();
        *board.color_bitboard_mut(from_color) ^= from.bitboard() | to.bitboard();

        // To
        if let Some(piece) = to_piece {
            *board.piece_bitboard_mut(piece) ^= to.bitboard();
            *board.color_bitboard_mut(from_color.inverse()) ^= to.bitboard();
        }

        // Replace pieces
        if let Some(piece) = promotion {
            *board.piece_bitboard_mut(piece) ^= to.bitboard();
        } else {
            *board.piece_bitboard_mut(from_piece) ^= to.bitboard();
        }

        board
    }
}

impl Default for Board {
    // Returns a board with the standard starting position loaded
    fn default() -> Self {
        Self {
            bitboards: [
                Bitboard(0xff00000000ff00),
                Bitboard(0x4200000000000042),
                Bitboard(0x2400000000000024),
                Bitboard(0x8100000000000081),
                Bitboard(0x800000000000008),
                Bitboard(0x1000000000000010),
                Bitboard(0xffff),
                Bitboard(0xffff000000000000),
            ],
            active_color: Color::White,
            flags: Flags(0b00001111),
            halfmoves: 0,
            fullmoves: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_startpos() {
        let fen_board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let startpos = Board::default();

        assert_eq!(fen_board, startpos);
    }

    #[test]
    fn test_piece_at() {
        let board = Board::default();

        assert_eq!(board.piece_at(Square::A1), Some(Piece::Rook));
        assert_eq!(board.piece_at(Square::B1), Some(Piece::Knight));
        assert_eq!(board.piece_at(Square::C1), Some(Piece::Bishop));
        assert_eq!(board.piece_at(Square::D1), Some(Piece::Queen));
        assert_eq!(board.piece_at(Square::E1), Some(Piece::King));
        assert_eq!(board.piece_at(Square::A2), Some(Piece::Pawn));
        assert_eq!(board.piece_at(Square::E4), None);
    }

    #[test]
    fn test_make_move_quiet() {
        let mut initial = Board::new();
        initial.add_piece(Piece::Rook, Color::White, Square::E4);
        let after = initial.make_move(Move::new(Square::E4, Square::E7, None));

        assert_eq!(
            after.bitboard(Piece::Rook, Color::White),
            Square::E7.bitboard()
        );
    }

    #[test]
    fn test_make_move_capture() {
        let mut initial = Board::new();
        initial.add_piece(Piece::Rook, Color::White, Square::E4);
        initial.add_piece(Piece::Bishop, Color::Black, Square::E7);
        let after = initial.make_move(Move::new(Square::E4, Square::E7, None));

        assert_eq!(
            after.bitboard(Piece::Rook, Color::White),
            Square::E7.bitboard()
        );
        assert_eq!(after.bitboard(Piece::Bishop, Color::Black), Bitboard::EMPTY);
    }

    #[test]
    fn test_white_en_passant() {
        let mut board = Board::new();

        board.add_piece(Piece::Pawn, Color::White, Square::E2);
        board.add_piece(Piece::Pawn, Color::Black, Square::D4);

        let board = board.make_move(Move::new(Square::E2, Square::E4, None));

        assert!(board.flags.can_en_passant());
        assert_eq!(board.flags.en_passant_file(), 4);

        let board = board.make_move(Move::new(Square::D4, Square::E3, None));

        assert!(!board.flags.can_en_passant());

        assert_eq!(board.piece_bitboard(Piece::Pawn), Bitboard(0x100000));
        assert_eq!(board.color_bitboard(Color::White), Bitboard::EMPTY);
        assert_eq!(board.color_bitboard(Color::Black), Bitboard(0x100000));
    }

    #[test]
    fn test_black_en_passant() {
        let mut board = Board::new();

        board.add_piece(Piece::Pawn, Color::Black, Square::E7);
        board.add_piece(Piece::Pawn, Color::White, Square::D5);

        let board = board.make_move(Move::new(Square::E7, Square::E5, None));

        assert!(board.flags.can_en_passant());
        assert_eq!(board.flags.en_passant_file(), 4);

        let board = board.make_move(Move::new(Square::D5, Square::E6, None));

        assert!(!board.flags.can_en_passant());

        assert_eq!(board.piece_bitboard(Piece::Pawn), Bitboard(0x100000000000));
        assert_eq!(board.color_bitboard(Color::White), Bitboard(0x100000000000));
        assert_eq!(board.color_bitboard(Color::Black), Bitboard::EMPTY);
    }
}
