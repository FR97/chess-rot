use std::fmt;

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

    pub fn is_empty(&self, position: u8) -> bool {
        let mask = 1u64 << position;
        return self.value & mask == 0;
    }

    pub fn is_bit_set(&self, position: u8) -> bool {
        return !self.is_empty(position);
    }

    pub fn remove_bit(&self, position: u8) -> BitBoard {
        let mask = 1u64 << position;
        let new_value = self.value & !mask;
        return BitBoard { value: new_value };
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
