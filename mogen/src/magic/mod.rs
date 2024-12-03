use rand::{self, thread_rng, Rng};

use crate::{
    board::{bitboard::Bitboard, square::Square},
    r#static::generation::{bishop_move_mask, coords, in_bounds, rook_move_mask},
};

#[derive(Debug)]
pub struct SlidingMoveGen {
    rook_tables: Vec<Vec<Bitboard>>,
    bishop_tables: Vec<Vec<Bitboard>>,
    rook_magics: Vec<MagicEntry>,
    bishop_magics: Vec<MagicEntry>,
}

impl SlidingMoveGen {
    pub fn new() -> Self {
        let mut rook_tables: Vec<Vec<Bitboard>> = Vec::with_capacity(64);
        let mut bishop_tables: Vec<Vec<Bitboard>> = Vec::with_capacity(64);
        let mut rook_magics: Vec<MagicEntry> = Vec::with_capacity(64);
        let mut bishop_magics: Vec<MagicEntry> = Vec::with_capacity(64);

        for square in Square::ALL.into_iter() {
            let (ortho_magic, ortho_table) = generate_magic(square, Direction::Orthogonal, 12);
            let (diag_magic, diag_table) = generate_magic(square, Direction::Diagonal, 10);

            rook_tables.push(ortho_table);
            bishop_tables.push(diag_table);
            rook_magics.push(ortho_magic);
            bishop_magics.push(diag_magic);
        }

        Self {
            rook_tables,
            bishop_tables,
            rook_magics,
            bishop_magics,
        }
    }

    pub fn rook_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let i = square as usize;
        self.rook_tables[i][magic_index(&self.rook_magics[i], blockers)]
    }

    pub fn bishop_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let i = square as usize;
        self.bishop_tables[i][magic_index(&self.bishop_magics[i], blockers)]
    }
}

impl Default for SlidingMoveGen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Orthogonal,
    Diagonal,
}

impl Direction {
    fn blockers(&self, square: Square) -> Bitboard {
        match self {
            Self::Orthogonal => Self::rook_blockers(square),
            Self::Diagonal => Self::bishop_blockers(square),
        }
    }

    fn rook_blockers(square: Square) -> Bitboard {
        let mut mask = rook_move_mask(square);
        let (rank, file) = coords(square as u8);

        if rank != 0 {
            mask &= !Bitboard::RANK_1;
        }
        if rank != 7 {
            mask &= !Bitboard::RANK_8;
        }
        if file != 0 {
            mask &= !Bitboard::A_FILE;
        }
        if file != 7 {
            mask &= !Bitboard::H_FILE;
        }

        mask
    }

    fn bishop_blockers(square: Square) -> Bitboard {
        bishop_move_mask(square) & !Bitboard::EDGES
    }

    fn moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        match self {
            Direction::Orthogonal => Self::sliding_moves(square, blockers, [1, 8, -1, -8]),
            Direction::Diagonal => Self::sliding_moves(square, blockers, [7, 9, -7, -9]),
        }
    }

    fn sliding_moves(square: Square, blockers: Bitboard, offsets: [i8; 4]) -> Bitboard {
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

                if (1 << target) & blockers.0 > 0 {
                    break;
                }

                origin = target;
            }
        }

        mask
    }
}

#[derive(Debug)]
struct MagicEntry {
    mask: Bitboard,
    magic: u64,
    index_bits: u8,
}

impl Default for MagicEntry {
    fn default() -> Self {
        Self {
            mask: Bitboard::EMPTY,
            magic: 0,
            index_bits: 0,
        }
    }
}

fn random_u64() -> u64 {
    thread_rng().gen::<u64>()
}

fn random_magic() -> u64 {
    random_u64() & random_u64() & random_u64()
}

fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let blockers = blockers & entry.mask;
    let hash = blockers.0.wrapping_mul(entry.magic);
    (hash >> (64 - entry.index_bits)) as usize
}

#[derive(Debug)]
struct FillTableError;

fn try_fill_table(
    square: Square,
    direction: Direction,
    entry: &MagicEntry,
) -> Result<Vec<Bitboard>, FillTableError> {
    let mut table = vec![Bitboard::EMPTY; 1 << entry.index_bits];

    for blockers in entry.mask.subsets() {
        let moves = direction.moves(square, blockers);
        let table_entry = &mut table[magic_index(entry, blockers)];
        if *table_entry == Bitboard::EMPTY {
            *table_entry = moves;
        } else if *table_entry != moves {
            return Err(FillTableError);
        }
    }

    Ok(table)
}

fn generate_magic(
    square: Square,
    direction: Direction,
    index_bits: u8,
) -> (MagicEntry, Vec<Bitboard>) {
    let blockers = direction.blockers(square);
    loop {
        let magic = random_magic();
        let entry = MagicEntry {
            mask: blockers,
            magic,
            index_bits,
        };
        if let Ok(table) = try_fill_table(square, direction, &entry) {
            return (entry, table);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_sliding_moves() {
        let square = Square::D4;
        let mask = Bitboard(0x38000a04012082c0);
        let expected_moves = Bitboard(0x808f7080808);

        assert_eq!(Direction::Orthogonal.moves(square, mask), expected_moves)
    }

    #[test]
    fn test_bishop_sliding_moves() {
        let square = Square::D4;
        let mask = Bitboard(0x38000a04012082c0);
        let expected_moves = Bitboard(0x8040201400142240);

        assert_eq!(Direction::Diagonal.moves(square, mask), expected_moves);

        let square = Square::F1;
        let mask = Bitboard(0x1020);
        let expected_moves = Bitboard(0x805000);

        assert_eq!(Direction::Diagonal.moves(square, mask), expected_moves);
    }
}
