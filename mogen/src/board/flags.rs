use std::ops::{Deref, DerefMut};

use super::color::Color;

// R - Castling Rights
// E - Can en passant
// F - En passant file
// Flags - FFFECCCC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(pub u8);

impl Flags {
    pub const WHITE_KINGSIDE: u8 = 0b000000001;
    pub const WHITE_QUEENSIDE: u8 = 0b000000010;
    pub const BLACK_KINGSIDE: u8 = 0b000000100;
    pub const BLACK_QUEENSIDE: u8 = 0b000001000;
    pub const CASTLING_MASK: u8 = (Self::WHITE_KINGSIDE
        | Self::WHITE_QUEENSIDE
        | Self::BLACK_KINGSIDE
        | Self::BLACK_QUEENSIDE);
    pub const EN_PASSANT_MASK: u8 = 0b00010000;
    pub const FILE_MASK: u8 = 0b11100000;

    pub fn new(value: u8) -> Flags {
        Flags(value)
    }

    pub fn kingside(&self, color: Color) -> bool {
        let mask = match color {
            Color::White => Self::WHITE_KINGSIDE,
            Color::Black => Self::BLACK_KINGSIDE,
        };
        (self.0 & mask) > 0
    }

    pub fn queenside(&self, color: Color) -> bool {
        let mask = match color {
            Color::White => Self::WHITE_QUEENSIDE,
            Color::Black => Self::BLACK_QUEENSIDE,
        };
        (self.0 & mask) > 0
    }

    pub fn can_en_passant(&self) -> bool {
        (self.0 & Self::EN_PASSANT_MASK) > 0
    }

    pub fn en_passant_file(&self) -> u8 {
        self.0 >> 5
    }

    pub fn set_en_passant(&mut self, value: bool) {
        let mask = (value as u8) << 4;
        // Clear en passant bit
        self.0 &= !Self::EN_PASSANT_MASK;
        // Set en passant bit
        self.0 |= mask;
    }

    pub fn set_en_passant_file(&mut self, value: u8) {
        // Clear file
        self.0 &= !Self::FILE_MASK;
        // Set file
        self.0 |= value << 5;
    }
}

impl Deref for Flags {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Flags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
