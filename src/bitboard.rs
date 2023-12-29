#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Not, Shl, Shr};

#[derive(Copy, Clone)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
    // get bit at index
    pub fn get(&self, index: u8) -> u8 {
        ((self.value() >> index) & 1) as u8
    }
    // set bit at index to 1
    pub fn set(&mut self, index: u8) {
        self.0 |= 1u64 << index;
    }
    // flip the value at index
    pub fn flip(&mut self, index: u8) {
        self.0 ^= 1u64 << index;
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard::new(0)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output = "".to_string();
        for rank in (0..8).rev() {
            output.push('\n');
            for file in 0..8 {
                let index = rank * 8 + file;
                output.push(if self.get(index) == 1 {'1'} else {'.'});
                output.push(' ');
            }
        }
        write!(f, "{}", output)
    }
}

impl Deref for BitBoard {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: Self) -> Self::Output {BitBoard::new(self.value() & rhs.value())}
}

impl BitOr for BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: Self) -> Self::Output {BitBoard::new(self.value() | rhs.value())}
}

impl Not for BitBoard {
    type Output = BitBoard;
    fn not(self) -> Self::Output {BitBoard(!self.0)}
}

impl BitXor for BitBoard {
    type Output = BitBoard;
    fn bitxor(self, rhs: Self) -> Self::Output {BitBoard::new(self.value() ^ rhs.value())}
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {self.0 &= rhs.value()}
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {self.0 |= rhs.value()}
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {self.0 ^= rhs.value()}
}

impl Shr<u64> for BitBoard {
    type Output = BitBoard;
    fn shr(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl Shl<u64> for BitBoard {
    type Output = BitBoard;
    fn shl(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

pub const A_FILE: BitBoard = BitBoard(0x0101010101010101);
pub const B_FILE: BitBoard = BitBoard(0x0202020202020202);
pub const C_FILE: BitBoard = BitBoard(0x0404040404040404);
pub const D_FILE: BitBoard = BitBoard(0x0808080808080808);
pub const E_FILE: BitBoard = BitBoard(0x1010101010101010);
pub const F_FILE: BitBoard = BitBoard(0x2020202020202020);
pub const G_FILE: BitBoard = BitBoard(0x4040404040404040);
pub const H_FILE: BitBoard = BitBoard(0x8080808080808080);

pub const RANK_1: BitBoard = BitBoard(0x00000000000000FF);
pub const RANK_2: BitBoard = BitBoard(0x000000000000FF00);
pub const RANK_3: BitBoard = BitBoard(0x0000000000FF0000);
pub const RANK_4: BitBoard = BitBoard(0x00000000FF000000);
pub const RANK_5: BitBoard = BitBoard(0x000000FF00000000);
pub const RANK_6: BitBoard = BitBoard(0x0000FF0000000000);
pub const RANK_7: BitBoard = BitBoard(0x00FF000000000000);
pub const RANK_8: BitBoard = BitBoard(0xFF00000000000000);