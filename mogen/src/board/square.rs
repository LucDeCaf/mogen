use std::fmt::Display;

use crate::r#static::generation::coords;

use super::bitboard::Bitboard;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    pub const ALL: [Square; 64] = [
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8,
    ];

    pub fn bitboard(&self) -> Bitboard {
        Bitboard(1 << *self as u8)
    }

    pub fn from_coords(rank: u8, file: u8) -> Self {
        Square::ALL[(rank * 8 + file) as usize]
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rank, file) = coords(*self as u8);

        let file_char = (file + b'a') as char;
        let rank_char = (rank + b'1') as char;

        write!(f, "{file_char}{rank_char}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseSquareStringError {
    WrongLength,
    BadSquare,
}

impl TryFrom<&str> for Square {
    type Error = ParseSquareStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a1" => Ok(Square::A1),
            "b1" => Ok(Square::B1),
            "c1" => Ok(Square::C1),
            "d1" => Ok(Square::D1),
            "e1" => Ok(Square::E1),
            "f1" => Ok(Square::F1),
            "g1" => Ok(Square::G1),
            "h1" => Ok(Square::H1),
            "a2" => Ok(Square::A2),
            "b2" => Ok(Square::B2),
            "c2" => Ok(Square::C2),
            "d2" => Ok(Square::D2),
            "e2" => Ok(Square::E2),
            "f2" => Ok(Square::F2),
            "g2" => Ok(Square::G2),
            "h2" => Ok(Square::H2),
            "a3" => Ok(Square::A3),
            "b3" => Ok(Square::B3),
            "c3" => Ok(Square::C3),
            "d3" => Ok(Square::D3),
            "e3" => Ok(Square::E3),
            "f3" => Ok(Square::F3),
            "g3" => Ok(Square::G3),
            "h3" => Ok(Square::H3),
            "a4" => Ok(Square::A4),
            "b4" => Ok(Square::B4),
            "c4" => Ok(Square::C4),
            "d4" => Ok(Square::D4),
            "e4" => Ok(Square::E4),
            "f4" => Ok(Square::F4),
            "g4" => Ok(Square::G4),
            "h4" => Ok(Square::H4),
            "a5" => Ok(Square::A5),
            "b5" => Ok(Square::B5),
            "c5" => Ok(Square::C5),
            "d5" => Ok(Square::D5),
            "e5" => Ok(Square::E5),
            "f5" => Ok(Square::F5),
            "g5" => Ok(Square::G5),
            "h5" => Ok(Square::H5),
            "a6" => Ok(Square::A6),
            "b6" => Ok(Square::B6),
            "c6" => Ok(Square::C6),
            "d6" => Ok(Square::D6),
            "e6" => Ok(Square::E6),
            "f6" => Ok(Square::F6),
            "g6" => Ok(Square::G6),
            "h6" => Ok(Square::H6),
            "a7" => Ok(Square::A7),
            "b7" => Ok(Square::B7),
            "c7" => Ok(Square::C7),
            "d7" => Ok(Square::D7),
            "e7" => Ok(Square::E7),
            "f7" => Ok(Square::F7),
            "g7" => Ok(Square::G7),
            "h7" => Ok(Square::H7),
            "a8" => Ok(Square::A8),
            "b8" => Ok(Square::B8),
            "c8" => Ok(Square::C8),
            "d8" => Ok(Square::D8),
            "e8" => Ok(Square::E8),
            "f8" => Ok(Square::F8),
            "g8" => Ok(Square::G8),
            "h8" => Ok(Square::H8),
            _ => Err(ParseSquareStringError::BadSquare),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_display() {
        assert_eq!(format!("{}", Square::E4), "e4");
        assert_eq!(format!("{}", Square::A7), "a7");
        assert_eq!(format!("{}", Square::B3), "b3");
        assert_eq!(format!("{}", Square::H8), "h8");
    }

    #[test]
    fn test_square_from_coords() {
        let squares = [Square::E7, Square::A2, Square::C8, Square::H1, Square::F6];

        for square in squares {
            let (rank, file) = coords(square as u8);
            assert_eq!(square, Square::from_coords(rank, file));
        }
    }
}
