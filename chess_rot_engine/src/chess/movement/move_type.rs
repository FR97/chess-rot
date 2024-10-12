#[repr(u64)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MoveType {
    Push = 0,
    PawnJump = 1,
    Capture = 2,
    Castling = 3,
    EnPassant = 4,
    Promotion = 5,
    Invalid = 6,
}

impl MoveType {
    pub fn to_u64(self) -> u64 {
        return self as u64;
    }

    pub fn index(self) -> usize {
        return usize::try_from(self.to_u64()).unwrap_or(6);
    }
}


impl TryFrom<u64> for MoveType {
    type Error = ();

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            x if x == MoveType::Push.to_u64() => Ok(MoveType::Push),
            x if x == MoveType::PawnJump.to_u64() => Ok(MoveType::PawnJump),
            x if x == MoveType::Capture.to_u64() => Ok(MoveType::Capture),
            x if x == MoveType::Castling.to_u64() => Ok(MoveType::Castling),
            x if x == MoveType::EnPassant.to_u64() => Ok(MoveType::EnPassant),
            x if x == MoveType::Promotion.to_u64() => Ok(MoveType::Promotion),
            x => panic!("trying to get move type for invalid u64 value {}", x),
        }
    }
}

impl TryFrom<usize> for MoveType {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == MoveType::Push.index() => Ok(MoveType::Push),
            x if x == MoveType::PawnJump.index() => Ok(MoveType::PawnJump),
            x if x == MoveType::Capture.index() => Ok(MoveType::Capture),
            x if x == MoveType::Castling.index() => Ok(MoveType::Castling),
            x if x == MoveType::EnPassant.index() => Ok(MoveType::EnPassant),
            x if x == MoveType::Promotion.index() => Ok(MoveType::Promotion),
            x => panic!("trying to get move type for invalid usize value {}", x),
        }
    }
}
