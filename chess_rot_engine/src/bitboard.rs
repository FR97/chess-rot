use std::fmt;

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not, Shl, Shr};
use std::os::windows::raw;
use once_cell::sync::Lazy;
use crate::chess::Square;

const DE_BRUIJN: u64 = 0x07edd5e59a4e28c2_u64;
#[rustfmt::skip]
const BIT_POSITION_LOOKUP: [usize; 64] = [
    63,  0, 58,  1, 59, 47, 53,  2,
    60, 39, 48, 27, 54, 33, 42,  3,
    61, 51, 37, 40, 49, 18, 28, 20,
    55, 30, 34, 11, 43, 14, 22,  4,
    62, 57, 46, 52, 38, 26, 32, 41,
    50, 36, 17, 19, 29, 10, 13, 21,
    56, 45, 25, 31, 35, 16,  9, 12,
    44, 24, 15,  8, 23,  7,  6,  5,
    ];

const MSB: Lazy<[usize; 256]> = Lazy::new(calculate_msb_values);


#[derive(PartialEq, Eq, Copy, Clone)]
pub struct BitBoard {
    value: u64,
}

impl BitBoard {
    pub const START_BIT: u64 = 0;
    pub const END_BIT: u64 = 63;

    const EMPTY: BitBoard = BitBoard::from(0);
    const FULL: BitBoard = BitBoard::from(!0);
    pub const FILE_A: BitBoard = BitBoard::from(Square::A1.as_bb().raw() | Square::A2.as_bb().raw() | Square::A3.as_bb().raw() | Square::A4.as_bb().raw() | Square::A5.as_bb().raw() | Square::A6.as_bb().raw() | Square::A7.as_bb().raw() | Square::A8.as_bb().raw());
    pub const FILE_H: BitBoard = BitBoard::from(Square::H1.as_bb().raw() | Square::H2.as_bb().raw() | Square::H3.as_bb().raw() | Square::H4.as_bb().raw() | Square::H5.as_bb().raw() | Square::H6.as_bb().raw() | Square::H7.as_bb().raw() | Square::H8.as_bb().raw());
    pub const RANK_1: BitBoard = BitBoard::from(Square::A1.as_bb().raw() | Square::B1.as_bb().raw() | Square::C1.as_bb().raw() | Square::D1.as_bb().raw() | Square::E1.as_bb().raw() | Square::F1.as_bb().raw() | Square::G1.as_bb().raw() | Square::H1.as_bb().raw());
    pub const RANK_8: BitBoard = BitBoard::from(Square::A8.as_bb().raw() | Square::B8.as_bb().raw() | Square::C8.as_bb().raw() | Square::D8.as_bb().raw() | Square::E8.as_bb().raw() | Square::F8.as_bb().raw() | Square::G8.as_bb().raw() | Square::H8.as_bb().raw());
    pub const EDGES: BitBoard = BitBoard::from(BitBoard::RANK_1.raw() | BitBoard::RANK_8.raw() | BitBoard::FILE_A.raw() | BitBoard::FILE_H.raw());

    pub const DIAGONAL_A1H8: &'static [BitBoard] = &[
        BitBoard::from(Square::A8.as_bb().raw()),
        BitBoard::from(Square::A7.as_bb().raw() | Square::B8.as_bb().raw()),
        BitBoard::from(Square::A6.as_bb().raw() | Square::B7.as_bb().raw() | Square::C8.as_bb().raw()),
        BitBoard::from(Square::A5.as_bb().raw() | Square::B6.as_bb().raw() | Square::C7.as_bb().raw() | Square::D8.as_bb().raw()),
        BitBoard::from(Square::A4.as_bb().raw() | Square::B5.as_bb().raw() | Square::C6.as_bb().raw() | Square::D7.as_bb().raw() | Square::E8.as_bb().raw()),
        BitBoard::from(Square::A3.as_bb().raw() | Square::B4.as_bb().raw() | Square::C5.as_bb().raw() | Square::D6.as_bb().raw() | Square::E7.as_bb().raw() | Square::F8.as_bb().raw()),
        BitBoard::from(Square::A2.as_bb().raw() | Square::B3.as_bb().raw() | Square::C4.as_bb().raw() | Square::D5.as_bb().raw() | Square::E6.as_bb().raw() | Square::F7.as_bb().raw() | Square::G7.as_bb().raw()),
        BitBoard::from(Square::A1.as_bb().raw() | Square::B2.as_bb().raw() | Square::C3.as_bb().raw() | Square::D4.as_bb().raw() | Square::E5.as_bb().raw() | Square::F6.as_bb().raw() | Square::G7.as_bb().raw() | Square::H8.as_bb().raw()),
        BitBoard::from(Square::B1.as_bb().raw() | Square::C2.as_bb().raw() | Square::D3.as_bb().raw() | Square::A4.as_bb().raw() | Square::F5.as_bb().raw() | Square::G6.as_bb().raw() | Square::H7.as_bb().raw()),
        BitBoard::from(Square::C1.as_bb().raw() | Square::D2.as_bb().raw() | Square::E3.as_bb().raw() | Square::F4.as_bb().raw() | Square::G5.as_bb().raw() | Square::H6.as_bb().raw()),
        BitBoard::from(Square::D1.as_bb().raw() | Square::E2.as_bb().raw() | Square::F3.as_bb().raw() | Square::G4.as_bb().raw() | Square::H5.as_bb().raw()),
        BitBoard::from(Square::E1.as_bb().raw() | Square::F2.as_bb().raw() | Square::G3.as_bb().raw() | Square::H4.as_bb().raw()),
        BitBoard::from(Square::F1.as_bb().raw() | Square::G2.as_bb().raw() | Square::H3.as_bb().raw()),
        BitBoard::from(Square::G1.as_bb().raw() | Square::H2.as_bb().raw()),
        BitBoard::from(Square::H1.as_bb().raw()),
    ];


    pub const DIAGONAL_A8H1: &'static [BitBoard] = &[
        BitBoard::from(Square::A1.as_bb().raw()),
        BitBoard::from(Square::A2.as_bb().raw() | Square::B1.as_bb().raw()),
        BitBoard::from(Square::A3.as_bb().raw() | Square::B2.as_bb().raw() | Square::C1.as_bb().raw()),
        BitBoard::from(Square::A4.as_bb().raw() | Square::B3.as_bb().raw() | Square::C2.as_bb().raw() | Square::D1.as_bb().raw()),
        BitBoard::from(Square::A5.as_bb().raw() | Square::B4.as_bb().raw() | Square::C3.as_bb().raw() | Square::D2.as_bb().raw() | Square::E1.as_bb().raw()),
        BitBoard::from(Square::A6.as_bb().raw() | Square::B5.as_bb().raw() | Square::C4.as_bb().raw() | Square::D3.as_bb().raw() | Square::E2.as_bb().raw() | Square::F1.as_bb().raw()),
        BitBoard::from(Square::A7.as_bb().raw() | Square::B6.as_bb().raw() | Square::C5.as_bb().raw() | Square::D4.as_bb().raw() | Square::E3.as_bb().raw() | Square::F2.as_bb().raw() | Square::G1.as_bb().raw()),
        BitBoard::from(Square::A8.as_bb().raw() | Square::B7.as_bb().raw() | Square::C6.as_bb().raw() | Square::D5.as_bb().raw() | Square::E4.as_bb().raw() | Square::F3.as_bb().raw() | Square::G2.as_bb().raw() | Square::H1.as_bb().raw()),
        BitBoard::from(Square::B8.as_bb().raw() | Square::C7.as_bb().raw() | Square::D6.as_bb().raw() | Square::E5.as_bb().raw() | Square::F4.as_bb().raw() | Square::G3.as_bb().raw() | Square::H2.as_bb().raw()),
        BitBoard::from(Square::C8.as_bb().raw() | Square::D7.as_bb().raw() | Square::E6.as_bb().raw() | Square::F5.as_bb().raw() | Square::G4.as_bb().raw() | Square::H3.as_bb().raw()),
        BitBoard::from(Square::D8.as_bb().raw() | Square::E7.as_bb().raw() | Square::F6.as_bb().raw() | Square::G5.as_bb().raw() | Square::H4.as_bb().raw()),
        BitBoard::from(Square::E8.as_bb().raw() | Square::F7.as_bb().raw() | Square::G6.as_bb().raw() | Square::H5.as_bb().raw()),
        BitBoard::from(Square::F8.as_bb().raw() | Square::G7.as_bb().raw() | Square::H6.as_bb().raw()),
        BitBoard::from(Square::G8.as_bb().raw() | Square::H7.as_bb().raw()),
        BitBoard::from(Square::H8.as_bb().raw()),
    ];

    pub const SINGLE_BIT_BB: Lazy<[BitBoard; 64]> = Lazy::new(create_single_bit_bitboards);

    #[inline]
    pub const fn empty() -> Self {
        return Self::EMPTY;
    }

    #[inline]
    pub const fn full() -> Self {
        return Self::FULL;
    }

    pub const fn from(value: u64) -> Self {
        return BitBoard { value };
    }

    #[inline]
    pub const fn raw(self) -> u64 {
        return self.value;
    }


    #[inline]
    pub const fn is_empty(&self) -> bool {
        return self.value == 0;
    }

    #[inline]
    pub const fn is_empty_bit(&self, position: u64) -> bool {
        let mask = 1u64 << position;
        return self.value & mask == 0;
    }

    pub const fn is_bit_set(&self, position: u64) -> bool {
        return !self.is_empty_bit(position);
    }

    pub const fn remove_bit(&self, position: u64) -> BitBoard {
        let mask = 1u64 << position;
        let new_value = self.value & !mask;
        return BitBoard { value: new_value };
    }

    #[inline]
    pub const fn lsb(self) -> usize {
        // let i = ((self.value & (!self.value + 1)).wrapping_mul(DE_BRUIJN) >> 58) as usize;
        // return BIT_POSITION_LOOKUP[i];
        if self.value == 0 {
            return 0;
        }
        return self.value.trailing_zeros() as usize;
    }


    pub fn bit_count(&self) -> u32 {
        return self.value.count_ones();
    }

    pub fn msb(&self) -> usize {
        let (mut i, mut j) = (self.value, 0_usize);

        const HALVING_BITS: [u64; 3] = [
            0b0000000000000000000000000000000011111111111111111111111111111111,
            0b0000000000000000000000000000000000000000000000001111111111111111,
            0b0000000000000000000000000000000000000000000000000000000011111111
        ];

        for k in (0..HALVING_BITS.len()) {
            let offset = 32 / (k + 1);
            if (i > HALVING_BITS[k]) {
                i >>= offset;
                j += offset;
            }
        }

        return j + MSB[i as usize];
    }

    pub const fn mirrored_vertically(self) -> Self {
        let mut result = 0u64;
        let b = self.value;

        result |= (b >> 56) & BitBoard::RANK_1.value;
        result |= ((b >> 48) & BitBoard::RANK_1.value) << 8;
        result |= ((b >> 40) & BitBoard::RANK_1.value) << 16;
        result |= ((b >> 32) & BitBoard::RANK_1.value) << 24;
        result |= ((b >> 24) & BitBoard::RANK_1.value) << 32;
        result |= ((b >> 16) & BitBoard::RANK_1.value) << 40;
        result |= ((b >> 8) & BitBoard::RANK_1.value) << 48;
        result |= (b & BitBoard::RANK_1.value) << 56;

        return BitBoard::from(result);
    }

    pub const fn mirrored_horizontally(self) -> Self {
        const K1: u64 = 0x5555555555555555u64;
        const K2: u64 = 0x3333333333333333u64;
        const K4: u64 = 0x0f0f0f0f0f0f0f0fu64;

        let mut b = self.value;

        b = ((b >> 1) & K1) | ((b & K1) << 1);
        b = ((b >> 2) & K2) | ((b & K2) << 2);
        b = ((b >> 4) & K4) | ((b & K4) << 4);

        return BitBoard::from(b);
    }

    pub const fn mirrored_a1h8(self) -> Self {
        const K1: u64 = 0x5500550055005500u64;
        const K2: u64 = 0x3333000033330000u64;
        const K4: u64 = 0x0f0f0f0f00000000u64;

        let mut b = self.value;

        let mut t = K4 & (b ^ (b << 28));

        b ^= t ^ (t >> 28);
        t = K2 & (b ^ (b << 14));
        b ^= t ^ (t >> 14);
        t = K1 & (b ^ (b << 7));
        b ^= t ^ (t >> 7);

        return BitBoard::from(b);
    }

    pub const fn mirrored_a8h1(self) -> Self {
        const K1: u64 = 0xaa00aa00aa00aa00u64;
        const K2: u64 = 0xcccc0000cccc0000u64;
        const K4: u64 = 0xf0f0f0f00f0f0f0fu64;

        let mut b = self.value;
        let mut t = b ^ (b << 36);

        b ^= K4 & (t ^ (b >> 36));
        t = K2 & (b ^ (b << 18));
        b ^= t ^ (t >> 18);
        t = K1 & (b ^ (b << 9));
        b ^= t ^ (t >> 9);

        BitBoard::from(b)
    }

    pub const fn shifted_north(self) -> Self {
        return Self { value: self.value << 8 };
    }

    pub const fn shifted_northeast(self) -> Self {
        return Self { value: self.value << 9 & !BitBoard::FILE_A.value };
    }

    pub const fn shifted_east(self) -> Self {
        return Self { value: self.value << 1 & !BitBoard::FILE_A.value };
    }


    pub const fn shifted_southeast(self) -> Self {
        return Self { value: self.value >> 7 & !BitBoard::FILE_A.value };
    }

    pub const fn shifted_south(self) -> Self {
        return Self { value: self.value >> 8 };
    }

    pub const fn shifted_southwest(self) -> Self {
        return Self { value: self.value >> 9 & !BitBoard::FILE_H.value };
    }


    pub const fn shifted_west(self) -> Self {
        return Self { value: self.value >> 1 & !BitBoard::FILE_H.value };
    }

    pub const fn shifted_northwest(self) -> Self {
        return Self { value: self.value << 7 & !BitBoard::FILE_H.value };
    }

    pub const fn shifted(self, dx: isize, dy: isize) -> Self {
        let mut b = self.value;

        // dy = up/down
        if dy > 0 {
            b <<= dy * 8;
        }
        if dy < 0 {
            b >>= (-dy) * 8;
        }

        // dx = left / right
        if dx > 0 {
            let mut i = 0usize;
            while i < dx as usize {
                b = (b << 1) & !BitBoard::FILE_A.value;
                i += 1;
            }
        }
        if dx < 0 {
            let mut i = 0usize;
            while i < (-dx) as usize {
                b = (b >> 1) & !BitBoard::FILE_H.value;
                i += 1;
            }
        }

        return BitBoard::from(b);
    }


    fn format(&self) -> String {
        let mut str = String::new();
        for rank in 0..8 {
            for file in (0..8).rev() {
                let position = BitBoard::END_BIT - (rank * 8) - file;
                let mask = 1u64 << position;
                let char = if self.value & mask == 0 { '0' } else { '1' };
                str.push(char);
                str.push(' ');
            }
            str.push_str("\n");
        }
        return str;
    }
}


impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        return BitBoard::from(self.value & other.value);
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, other: Self) {
        self.value &= other.value;
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        return BitBoard::from(self.value | other.value);
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, other: Self) {
        self.value |= other.value;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        return BitBoard::from(self.value ^ other.value);
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        return BitBoard::from(!self.value);
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;
    fn shl(self, rhs: usize) -> BitBoard {
        return BitBoard::from(self.value << rhs);
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;
    fn shr(self, rhs: usize) -> BitBoard {
        return BitBoard::from(self.value >> rhs);
    }
}

impl Mul for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        return BitBoard::from(self.value.wrapping_mul(other.value));
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}


// Create list of all possible bitboards with single bit set
fn create_single_bit_bitboards() -> [BitBoard; 64] {
    let mut bits = [BitBoard::from(0); 64];
    for i in 0..64u64 {
        bits[i as usize] = BitBoard::from(1 << i);
    }
    return bits;
}

// Create list of most significant bits used for reverse bit scan
fn calculate_msb_values() -> [usize; 256] {
    let mut table = [0; 256];

    for i in 0..256 {
        if i < 2 {
            table[i] = 0
        } else if i < 4 {
            table[i] = 1
        } else if i < 8 {
            table[i] = 2
        } else if i < 16 {
            table[i] = 3
        } else if i < 32 {
            table[i] = 4
        } else if i < 64 {
            table[i] = 5
        } else if i < 128 {
            table[i] = 6
        } else {
            table[i] = 7
        }
    }

    return table;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsb() {
        let bb = BitBoard::from(0b1);
        debug_assert_eq!(0, bb.lsb());
        let bb = BitBoard::from(0b10);
        debug_assert_eq!(1, bb.lsb());
        let bb = BitBoard::from(0b11);
        debug_assert_eq!(0, bb.lsb());
        let bb = BitBoard::from(0b1000);
        debug_assert_eq!(3, bb.lsb());
        let bb = BitBoard::from(0b1000000000000000000000000000000000000000000000000000000000000000);
        debug_assert_eq!(63, bb.lsb());
    }


    #[test]
    fn test_msb() {
        let bb = BitBoard::from(0b1);
        debug_assert_eq!(0, bb.msb());
        let bb = BitBoard::from(0b10);
        debug_assert_eq!(1, bb.msb());

        let bb = BitBoard::from(0b11);
        debug_assert_eq!(1, bb.msb());

        let bb = BitBoard::from(0b1000);
        debug_assert_eq!(3, bb.msb());

        let bb = BitBoard::from(0b1000000000000000000000000000000000000000000000000000000000000000);
        debug_assert_eq!(63, bb.msb());

        let bb = BitBoard::from(0b0001000000000000000000000000000000000000000000000000000000000000);
        debug_assert_eq!(60, bb.msb());
    }
}
