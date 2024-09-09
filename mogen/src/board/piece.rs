#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];

    pub fn promotion_mask(&self) -> u16 {
        match self {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 4,
            Piece::Queen => 8,
            Piece::King => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParsePieceCharError;

impl TryFrom<char> for Piece {
    type Error = ParsePieceCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'p' | 'P' => Ok(Piece::Pawn),
            'n' | 'N' => Ok(Piece::Knight),
            'b' | 'B' => Ok(Piece::Bishop),
            'r' | 'R' => Ok(Piece::Rook),
            'q' | 'Q' => Ok(Piece::Queen),
            'k' | 'K' => Ok(Piece::King),
            _ => Err(ParsePieceCharError),
        }
    }
}
