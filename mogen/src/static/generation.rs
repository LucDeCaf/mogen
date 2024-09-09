use crate::board::{bitboard::Bitboard, color::Color, square::Square};

pub fn coords(val: u8) -> (u8, u8) {
    (val / 8, val % 8)
}

pub fn in_bounds(val: i8) -> bool {
    (0..64).contains(&val)
}

pub fn knight_move_mask(square: Square) -> Bitboard {
    const OFFSETS: [i8; 8] = [15, 17, 6, 10, -15, -17, -6, -10];

    let i = square as i8;

    let (start_rank, start_file) = coords(square as u8);

    let mut mask = Bitboard::EMPTY;

    for off in OFFSETS {
        let target = i + off;

        if !in_bounds(target) {
            continue;
        }

        // Rank / file wrapping check
        let (target_rank, target_file) = coords(target as u8);
        if start_rank.abs_diff(target_rank) > 2 {
            continue;
        }
        if start_file.abs_diff(target_file) > 2 {
            continue;
        }

        mask |= Bitboard(1 << target);
    }

    mask
}

pub fn sliding_move_mask(square: Square, offsets: &[i8]) -> Bitboard {
    let i = square as i8;

    let mut mask = Bitboard::EMPTY;

    for off in offsets {
        let mut origin = i;

        while in_bounds(origin) {
            let (origin_rank, origin_file) = coords(origin as u8);

            let target = origin + off;
            if !in_bounds(target) {
                break;
            }

            let (rank, file) = coords(target as u8);
            if origin_rank.abs_diff(rank) > 1 || origin_file.abs_diff(file) > 1 {
                break;
            }

            mask |= Bitboard(1 << target);

            origin = target;
        }
    }

    mask
}

pub fn bishop_move_mask(square: Square) -> Bitboard {
    sliding_move_mask(square, &[-7, -9, 7, 9])
}

pub fn rook_move_mask(square: Square) -> Bitboard {
    sliding_move_mask(square, &[1, 8, -1, -8])
}

pub fn king_move_mask(square: Square) -> Bitboard {
    const OFFSETS: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];

    let mut mask = Bitboard::EMPTY;

    let (source_rank, source_file) = coords(square as u8);

    for off in OFFSETS {
        let target = square as i8 + off;

        if !(0..64).contains(&target) {
            continue;
        }

        let (target_rank, target_file) = coords(target as u8);

        if source_rank.abs_diff(target_rank) > 1 || source_file.abs_diff(target_file) > 1 {
            continue;
        }

        mask |= Bitboard(1 << target);
    }

    mask
}

pub fn pawn_capture_mask(square: Square, color: Color) -> Bitboard {
    if (square as u8) < 8 || (square as u8) > 55 {
        return Bitboard::EMPTY;
    }

    let offsets = match color {
        Color::White => [7, 9],
        Color::Black => [-7, -9],
    };

    let (source_rank, source_file) = coords(square as u8);

    let mut mask = Bitboard::EMPTY;

    for off in offsets {
        let target = square as i8 + off;

        if !in_bounds(target) {
            continue;
        }

        let (rank, file) = coords(target as u8);

        if source_rank.abs_diff(rank) != 1 || source_file.abs_diff(file) != 1 {
            continue;
        }

        mask |= Bitboard(1 << target);
    }

    mask
}

pub fn generate_knight_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard::EMPTY; 64];
    for square in Square::ALL {
        masks[square as usize] = knight_move_mask(square);
    }
    masks
}

pub fn generate_bishop_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard::EMPTY; 64];
    for square in Square::ALL {
        masks[square as usize] = bishop_move_mask(square);
    }
    masks
}

pub fn generate_rook_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard::EMPTY; 64];
    for square in Square::ALL {
        masks[square as usize] = rook_move_mask(square);
    }
    masks
}

pub fn generate_king_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard::EMPTY; 64];
    for square in Square::ALL {
        masks[square as usize] = king_move_mask(square);
    }
    masks
}

pub fn generate_pawn_capture_masks(color: Color) -> [Bitboard; 64] {
    let mut masks = [Bitboard::EMPTY; 64];
    for square in Square::ALL {
        masks[square as usize] = pawn_capture_mask(square, color);
    }
    masks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords() {
        assert_eq!(coords(Square::B7 as u8), (6, 1));
        assert_eq!(coords(Square::A8 as u8), (7, 0));
    }

    #[test]
    fn test_knight_move_mask() {
        let e4_moves = Bitboard(
            1 << Square::D6 as u8
                | 1 << Square::F6 as u8
                | 1 << Square::C5 as u8
                | 1 << Square::G5 as u8
                | 1 << Square::C3 as u8
                | 1 << Square::G3 as u8
                | 1 << Square::D2 as u8
                | 1 << Square::F2 as u8,
        );
        assert_eq!(e4_moves, knight_move_mask(Square::E4));

        let a1_moves = Bitboard(1 << Square::B3 as u8 | 1 << Square::C2 as u8);
        assert_eq!(a1_moves, knight_move_mask(Square::A1));

        let h4_moves = Bitboard(
            1 << Square::G2 as u8
                | 1 << Square::F3 as u8
                | 1 << Square::F5 as u8
                | 1 << Square::G6 as u8,
        );
        assert_eq!(h4_moves, knight_move_mask(Square::H4));
    }

    #[test]
    fn test_bishop_move_mask() {
        let e4_moves = Bitboard(
            1 << Square::A8 as u8
                | 1 << Square::B7 as u8
                | 1 << Square::C6 as u8
                | 1 << Square::D5 as u8
                | 1 << Square::F3 as u8
                | 1 << Square::G2 as u8
                | 1 << Square::H1 as u8
                | 1 << Square::B1 as u8
                | 1 << Square::C2 as u8
                | 1 << Square::D3 as u8
                | 1 << Square::F5 as u8
                | 1 << Square::G6 as u8
                | 1 << Square::H7 as u8,
        );
        assert_eq!(e4_moves, bishop_move_mask(Square::E4));

        let a1_moves = Bitboard(
            1 << Square::B2 as u8
                | 1 << Square::C3 as u8
                | 1 << Square::D4 as u8
                | 1 << Square::E5 as u8
                | 1 << Square::F6 as u8
                | 1 << Square::G7 as u8
                | 1 << Square::H8 as u8,
        );
        assert_eq!(a1_moves, bishop_move_mask(Square::A1));

        let h4_moves = Bitboard(
            1 << Square::E1 as u8
                | 1 << Square::F2 as u8
                | 1 << Square::G3 as u8
                | 1 << Square::D8 as u8
                | 1 << Square::E7 as u8
                | 1 << Square::F6 as u8
                | 1 << Square::G5 as u8,
        );
        assert_eq!(h4_moves, bishop_move_mask(Square::H4));
    }

    #[test]
    fn test_rook_move_mask() {
        let e4_moves = Bitboard(
            1 << Square::A4 as u8
                | 1 << Square::B4 as u8
                | 1 << Square::C4 as u8
                | 1 << Square::D4 as u8
                | 1 << Square::F4 as u8
                | 1 << Square::G4 as u8
                | 1 << Square::H4 as u8
                | 1 << Square::E1 as u8
                | 1 << Square::E2 as u8
                | 1 << Square::E3 as u8
                | 1 << Square::E5 as u8
                | 1 << Square::E6 as u8
                | 1 << Square::E7 as u8
                | 1 << Square::E8 as u8,
        );
        assert_eq!(e4_moves, rook_move_mask(Square::E4));

        let a1_moves = Bitboard(
            1 << Square::A2 as u8
                | 1 << Square::A3 as u8
                | 1 << Square::A4 as u8
                | 1 << Square::A5 as u8
                | 1 << Square::A6 as u8
                | 1 << Square::A7 as u8
                | 1 << Square::A8 as u8
                | 1 << Square::B1 as u8
                | 1 << Square::C1 as u8
                | 1 << Square::D1 as u8
                | 1 << Square::E1 as u8
                | 1 << Square::F1 as u8
                | 1 << Square::G1 as u8
                | 1 << Square::H1 as u8,
        );
        assert_eq!(a1_moves, rook_move_mask(Square::A1));

        let h4_moves = Bitboard(
            1 << Square::H1 as u8
                | 1 << Square::H2 as u8
                | 1 << Square::H3 as u8
                | 1 << Square::H5 as u8
                | 1 << Square::H6 as u8
                | 1 << Square::H7 as u8
                | 1 << Square::H8 as u8
                | 1 << Square::A4 as u8
                | 1 << Square::B4 as u8
                | 1 << Square::C4 as u8
                | 1 << Square::D4 as u8
                | 1 << Square::E4 as u8
                | 1 << Square::F4 as u8
                | 1 << Square::G4 as u8,
        );
        assert_eq!(h4_moves, rook_move_mask(Square::H4));
    }

    #[test]
    fn test_king_move_mask() {
        let a1_moves = Bitboard(0x302);
        let e4_moves = Bitboard(0x3828380000);
        let h4_moves = Bitboard(0xc040c00000);

        assert_eq!(a1_moves, king_move_mask(Square::A1));
        assert_eq!(e4_moves, king_move_mask(Square::E4));
        assert_eq!(h4_moves, king_move_mask(Square::H4));
    }

    #[test]
    fn test_pawn_capture_mask() {
        assert_eq!(
            Square::D3.bitboard() | Square::F3.bitboard(),
            pawn_capture_mask(Square::E2, Color::White)
        );
        assert_eq!(
            Square::D1.bitboard() | Square::F1.bitboard(),
            pawn_capture_mask(Square::E2, Color::Black)
        );

        assert_eq!(
            Square::B8.bitboard(),
            pawn_capture_mask(Square::A7, Color::White)
        );
        assert_eq!(
            Square::B6.bitboard(),
            pawn_capture_mask(Square::A7, Color::Black)
        );

        assert_eq!(
            Square::D5.bitboard() | Square::F5.bitboard(),
            pawn_capture_mask(Square::E4, Color::White)
        );
        assert_eq!(
            Square::D3.bitboard() | Square::F3.bitboard(),
            pawn_capture_mask(Square::E4, Color::Black)
        );

        let first_rank = &Square::ALL[..8];
        for square in first_rank {
            assert_eq!(Bitboard::EMPTY, pawn_capture_mask(*square, Color::White));
            assert_eq!(Bitboard::EMPTY, pawn_capture_mask(*square, Color::Black));
        }

        let last_rank = &Square::ALL[56..];
        for square in last_rank {
            assert_eq!(Bitboard::EMPTY, pawn_capture_mask(*square, Color::White));
            assert_eq!(Bitboard::EMPTY, pawn_capture_mask(*square, Color::Black));
        }
    }
}
