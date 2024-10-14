use std::fmt;

use std::ops::{BitAnd, BitOr, Mul};
use once_cell::sync::Lazy;


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

pub static SINGLE_BIT_BB: Lazy<[BitBoard; 64]> = Lazy::new(create_single_bit_bitboards);


#[derive(PartialEq, Eq, Copy, Clone)]
pub struct BitBoard {
    value: u64,
}

impl BitBoard {
    pub const START_BIT: u64 = 0;
    pub const END_BIT: u64 = 63;

    pub fn empty() -> BitBoard {
        return BitBoard { value: 0 };
    }

    pub fn from(value: u64) -> BitBoard {
        return BitBoard { value };
    }

    pub fn raw_value(&self) -> u64 {
        return self.value;
    }

    pub fn is_empty(&self) -> bool {
        return self.value == 0;
    }

    pub fn is_empty_bit(&self, position: u8) -> bool {
        let mask = 1u64 << position;
        return self.value & mask == 0;
    }

    pub fn is_bit_set(&self, position: u8) -> bool {
        return !self.is_empty_bit(position);
    }

    pub fn remove_bit(&self, position: u8) -> BitBoard {
        let mask = 1u64 << position;
        let new_value = self.value & !mask;
        return BitBoard { value: new_value };
    }

    pub fn lsb(&self) -> usize {
        let i = ((self.value & (!self.value + 1)).wrapping_mul(DE_BRUIJN) >> 58) as usize;
        return BIT_POSITION_LOOKUP[i];
    }

    pub fn msb(&self) -> usize {
        let  (mut i, mut j) = (self.value, 0_usize);

        const HALVING_BITS: [u64; 3] = [
            0b0000000000000000000000000000000011111111111111111111111111111111,
            0b0000000000000000000000000000000000000000000000001111111111111111,
            0b0000000000000000000000000000000000000000000000000000000011111111
        ];

        for k in (0..HALVING_BITS.len()) {
            let offset = 32 / (k+1);
            if (i > HALVING_BITS[k]) {
                i >>= offset;
                j += offset;
            }
        }

        return j + MSB[i as usize];
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

impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        return BitBoard::from(self.value | other.value);
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
        assert_eq!(0, bb.lsb());
        let bb = BitBoard::from(0b10);
        assert_eq!(1, bb.lsb());

        let bb = BitBoard::from(0b11);
        assert_eq!(0, bb.lsb());

        let bb = BitBoard::from(0b1000);
        assert_eq!(3, bb.lsb());

        let bb = BitBoard::from(0b1000000000000000000000000000000000000000000000000000000000000000);
        assert_eq!(63, bb.lsb());
    }


    #[test]
    fn test_msb() {
        let bb = BitBoard::from(0b1);
        assert_eq!(0, bb.msb());
        let bb = BitBoard::from(0b10);
        assert_eq!(1, bb.msb());

        let bb = BitBoard::from(0b11);
        assert_eq!(1, bb.msb());

        let bb = BitBoard::from(0b1000);
        assert_eq!(3, bb.msb());

        let bb = BitBoard::from(0b1000000000000000000000000000000000000000000000000000000000000000);
        assert_eq!(63, bb.msb());

        let bb = BitBoard::from(0b0001000000000000000000000000000000000000000000000000000000000000);
        assert_eq!(60, bb.msb());
    }
}
