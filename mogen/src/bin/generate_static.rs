use std::{
    fs::{self, File},
    io::{self, Write},
    process::Command,
};

use mogen::{board::color::Color, r#static::generation};

fn main() -> io::Result<()> {
    let path = "src/static";
    fs::create_dir_all(path).unwrap();

    let knight_masks = generation::generate_knight_masks();
    let bishop_masks = generation::generate_bishop_masks();
    let rook_masks = generation::generate_rook_masks();
    let king_masks = generation::generate_king_masks();
    let white_pawn_capture_masks = generation::generate_pawn_capture_masks(Color::White);
    let black_pawn_capture_masks = generation::generate_pawn_capture_masks(Color::Black);

    let mut move_masks = File::create(format!("{path}/move_masks.rs"))?;
    move_masks.write_all(
        format!(
            "use crate::board::bitboard::Bitboard;\n
pub const KNIGHT_MOVE_MASKS: [Bitboard; 64] = {knight_masks:#?};
pub const BISHOP_MOVE_MASKS: [Bitboard; 64] = {bishop_masks:#?};
pub const ROOK_MOVE_MASKS: [Bitboard; 64] = {rook_masks:#?};
pub const KING_MOVE_MASKS: [Bitboard; 64] = {king_masks:#?};
pub const WHITE_PAWN_CAPTURE_MASKS: [Bitboard; 64] = {white_pawn_capture_masks:#?};
pub const BLACK_PAWN_CAPTURE_MASKS: [Bitboard; 64] = {black_pawn_capture_masks:#?};
"
        )
        .as_bytes(),
    )?;

    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Ok(())
}
