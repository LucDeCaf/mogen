use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use super::{bitboard::Bitboard, piece::Piece, square::Square};

// F - From
// T - To
// D - Data
// Move: FFFFFFTTTTTTDDDD (16-bit word)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move(u16);

impl Move {
    pub fn new(source: Square, target: Square, promotion: Option<Piece>) -> Move {
        let source = (source as u16) << 10;
        let target = (target as u16) << 4;
        let promotion = match promotion {
            Some(piece) => piece.promotion_mask(),
            None => 0,
        };
        Move(source | target | promotion)
    }

    pub fn source(&self) -> Square {
        Square::ALL[(self.0 >> 10) as usize]
    }

    pub fn target(&self) -> Square {
        Square::ALL[0b111111 & (self.0 >> 4) as usize]
    }

    pub fn promotion(&self) -> Option<Piece> {
        match self.0 & 0b1111 {
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            4 => Some(Piece::Rook),
            8 => Some(Piece::Queen),
            _ => None,
        }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.source().bitboard() | self.target().bitboard()
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(piece) = self.promotion() {
            let promotion_char = match piece {
                Piece::Knight => 'k',
                Piece::Bishop => 'b',
                Piece::Rook => 'r',
                Piece::Queen => 'q',
                _ => unreachable!(),
            };
            write!(f, "{}{}{}", self.source(), self.target(), promotion_char)
        } else {
            write!(f, "{}{}", self.source(), self.target())
        }
    }
}

#[derive(Debug)]
pub struct ParseMoveError;

impl TryFrom<&str> for Move {
    type Error = ParseMoveError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let length = value.len();
        if length != 4 && length != 5 {
            return Err(ParseMoveError);
        }

        let Ok(source) = Square::try_from(&value[0..2]) else {
            return Err(ParseMoveError);
        };
        let Ok(target) = Square::try_from(&value[2..4]) else {
            return Err(ParseMoveError);
        };

        let promotion = if length == 5 {
            let promotion_ch = value.as_bytes()[4];

            match promotion_ch {
                b'n' => Some(Piece::Knight),
                b'b' => Some(Piece::Bishop),
                b'r' => Some(Piece::Rook),
                b'q' => Some(Piece::Queen),
                _ => return Err(ParseMoveError),
            }
        } else {
            None
        };

        Ok(Move::new(source, target, promotion))
    }
}

impl Deref for Move {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Move {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_move() {
        let mv = Move::new(Square::E2, Square::E4, Some(Piece::Rook));

        assert_eq!(mv.source(), Square::E2);
        assert_eq!(mv.target(), Square::E4);
        assert_eq!(mv.promotion(), Some(Piece::Rook));
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Move::new(Square::E4, Square::B6, None)),
            "e4b6"
        );
        assert_eq!(
            format!("{}", Move::new(Square::A1, Square::H8, None)),
            "a1h8"
        );
        assert_eq!(
            format!("{}", Move::new(Square::F2, Square::F1, Some(Piece::Rook))),
            "f2f1r"
        );
        assert_eq!(
            format!("{}", Move::new(Square::B7, Square::B8, Some(Piece::Knight))),
            "b7b8k"
        );
    }
}
