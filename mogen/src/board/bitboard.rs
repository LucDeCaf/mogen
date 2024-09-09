use std::{
    fmt::Display,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Not, Shl,
        ShlAssign, Shr, ShrAssign,
    },
};

use super::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const UNIVERSE: Bitboard = Bitboard(u64::MAX);

    pub const A_FILE: Bitboard = Bitboard(0x101010101010101);
    pub const B_FILE: Bitboard = Bitboard(0x202020202020202);
    pub const C_FILE: Bitboard = Bitboard(0x404040404040404);
    pub const D_FILE: Bitboard = Bitboard(0x808080808080808);
    pub const E_FILE: Bitboard = Bitboard(0x1010101010101010);
    pub const F_FILE: Bitboard = Bitboard(0x2020202020202020);
    pub const G_FILE: Bitboard = Bitboard(0x4040404040404040);
    pub const H_FILE: Bitboard = Bitboard(0x8080808080808080);

    pub const RANK_1: Bitboard = Bitboard(0x00000000000000ff);
    pub const RANK_2: Bitboard = Bitboard(0x000000000000ff00);
    pub const RANK_3: Bitboard = Bitboard(0x0000000000ff0000);
    pub const RANK_4: Bitboard = Bitboard(0x00000000ff000000);
    pub const RANK_5: Bitboard = Bitboard(0x000000ff00000000);
    pub const RANK_6: Bitboard = Bitboard(0x0000ff0000000000);
    pub const RANK_7: Bitboard = Bitboard(0x00ff000000000000);
    pub const RANK_8: Bitboard = Bitboard(0xff00000000000000);

    pub const EDGES: Bitboard = Bitboard(0xff818181818181ff);

    pub fn subsets(&self) -> Subsets {
        Subsets {
            set: self.0,
            subset: self.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn pop_lsb(&mut self) -> usize {
        let i = self.trailing_zeros();
        self.0 &= self.0 - 1;
        i as usize
    }
}

pub struct Subsets {
    set: u64,
    subset: u64,
}

impl Iterator for Subsets {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.subset == 0 {
            return None;
        }

        let next = self.subset;
        self.subset = (self.subset - 1) & self.set;

        if self.subset == 0 {
            return Some(Bitboard::EMPTY);
        }

        Some(Bitboard(next))
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary = format!("{:064b}", self.0.reverse_bits());

        let mut output = String::new();

        for i in (0..8).rev() {
            for (i, ch) in binary[i * 8..=i * 8 + 7].chars().enumerate() {
                output.push(match ch {
                    '0' => '-',
                    '1' => '#',
                    _ => unreachable!(),
                });
                if i != 7 {
                    output.push(' ');
                }
            }

            if i != 0 {
                output.push('\n');
            }
        }

        write!(f, "{}", output)
    }
}

impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bitboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        value.bitboard()
    }
}

macro_rules! shift {
    ($num:ty) => {
        impl Shl<$num> for Bitboard {
            type Output = Self;

            fn shl(self, val: $num) -> Self::Output {
                Self(self.0 << val)
            }
        }

        impl ShlAssign<$num> for Bitboard {
            fn shl_assign(&mut self, val: $num) {
                self.0 <<= val
            }
        }

        impl Shr<$num> for Bitboard {
            type Output = Self;

            fn shr(self, val: $num) -> Self::Output {
                Self(self.0 >> val)
            }
        }

        impl ShrAssign<$num> for Bitboard {
            fn shr_assign(&mut self, val: $num) {
                self.0 >>= val
            }
        }
    };
}

shift!(u8);
shift!(u16);
shift!(u32);
shift!(u64);
shift!(u128);
shift!(usize);
shift!(i8);
shift!(i16);
shift!(i32);
shift!(i64);
shift!(i128);
shift!(isize);

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(u64, u64); 4] = [
        (79196, 80559),
        (56301, 9058),
        (16642, 43944),
        (31223, 55648),
    ];

    #[test]
    fn test_bitand() {
        for case in CASES {
            assert_eq!(
                Bitboard(case.0) & Bitboard(case.1),
                Bitboard(case.0 & case.1)
            );
        }
    }

    #[test]
    fn test_bitor() {
        for case in CASES {
            assert_eq!(
                Bitboard(case.0) | Bitboard(case.1),
                Bitboard(case.0 | case.1)
            );
        }
    }

    #[test]
    fn test_bitxor() {
        for case in CASES {
            assert_eq!(
                Bitboard(case.0) ^ Bitboard(case.1),
                Bitboard(case.0 ^ case.1)
            );
        }
    }

    #[test]
    fn test_bitand_assign() {
        for case in CASES {
            let mut result = Bitboard(case.0);
            result &= Bitboard(case.1);
            assert_eq!(result, Bitboard(case.0) & Bitboard(case.1))
        }
    }

    #[test]
    fn test_bitor_assign() {
        for case in CASES {
            let mut result = Bitboard(case.0);
            result |= Bitboard(case.1);
            assert_eq!(result, Bitboard(case.0) | Bitboard(case.1))
        }
    }

    #[test]
    fn test_bitxor_assign() {
        for case in CASES {
            let mut result = Bitboard(case.0);
            result ^= Bitboard(case.1);
            assert_eq!(result, Bitboard(case.0) ^ Bitboard(case.1))
        }
    }

    #[test]
    fn test_subsets() {
        let bb = Bitboard(0b1101);
        for subset in bb.subsets() {
            println!("{:04b}", subset.0);
        }
    }

    #[test]
    fn test_shift() {
        let bb = Bitboard(0b110101011011);

        // shl
        assert_eq!(bb << 4_u8, Bitboard(bb.0 << 4_u8));
        assert_eq!(bb << 4_u16, Bitboard(bb.0 << 4_u16));
        assert_eq!(bb << 4_u32, Bitboard(bb.0 << 4_u32));
        assert_eq!(bb << 4_u64, Bitboard(bb.0 << 4_u64));
        assert_eq!(bb << 4_u128, Bitboard(bb.0 << 4_u128));
        assert_eq!(bb << 4_usize, Bitboard(bb.0 << 4_usize));
        assert_eq!(bb << 4_i8, Bitboard(bb.0 << 4_i8));
        assert_eq!(bb << 4_i16, Bitboard(bb.0 << 4_i16));
        assert_eq!(bb << 4_i32, Bitboard(bb.0 << 4_i32));
        assert_eq!(bb << 4_i64, Bitboard(bb.0 << 4_i64));
        assert_eq!(bb << 4_i128, Bitboard(bb.0 << 4_i128));
        assert_eq!(bb << 4_isize, Bitboard(bb.0 << 4_isize));

        // shr
        assert_eq!(bb >> 4_u8, Bitboard(bb.0 >> 4_u8));
        assert_eq!(bb >> 4_u16, Bitboard(bb.0 >> 4_u16));
        assert_eq!(bb >> 4_u32, Bitboard(bb.0 >> 4_u32));
        assert_eq!(bb >> 4_u64, Bitboard(bb.0 >> 4_u64));
        assert_eq!(bb >> 4_u128, Bitboard(bb.0 >> 4_u128));
        assert_eq!(bb >> 4_usize, Bitboard(bb.0 >> 4_usize));
        assert_eq!(bb >> 4_i8, Bitboard(bb.0 >> 4_i8));
        assert_eq!(bb >> 4_i16, Bitboard(bb.0 >> 4_i16));
        assert_eq!(bb >> 4_i32, Bitboard(bb.0 >> 4_i32));
        assert_eq!(bb >> 4_i64, Bitboard(bb.0 >> 4_i64));
        assert_eq!(bb >> 4_i128, Bitboard(bb.0 >> 4_i128));
        assert_eq!(bb >> 4_isize, Bitboard(bb.0 >> 4_isize));
    }
}
