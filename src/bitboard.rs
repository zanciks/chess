#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Not, Shl, Shr};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    // random bitboard, mainly for testing
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        BitBoard::new(rng.gen())
    }
    pub fn find_set_bits(&self) -> Vec<u8> {
        let mut binding = self.value();
        let mut indices = Vec::new();
        let mut index = 0;

        while binding > 0 {
            if binding & 1 == 1 {
                indices.push(index as u8);
            }

            binding >>= 1;
            index += 1;
        }
        return indices
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

pub const A_FILE: BitBoard = BitBoard(0x0101_0101_0101_0101);
pub const B_FILE: BitBoard = BitBoard(0x0202_0202_0202_0202);
pub const C_FILE: BitBoard = BitBoard(0x0404_0404_0404_0404);
pub const D_FILE: BitBoard = BitBoard(0x0808_0808_0808_0808);
pub const E_FILE: BitBoard = BitBoard(0x1010_1010_1010_1010);
pub const F_FILE: BitBoard = BitBoard(0x2020_2020_2020_2020);
pub const G_FILE: BitBoard = BitBoard(0x4040_4040_4040_4040);
pub const H_FILE: BitBoard = BitBoard(0x8080_8080_8080_8080);

pub const RANK_1: BitBoard = BitBoard(0x0000_0000_0000_00ff);
pub const RANK_2: BitBoard = BitBoard(0x0000_0000_0000_ff00);
pub const RANK_3: BitBoard = BitBoard(0x0000_0000_00ff_0000);
pub const RANK_4: BitBoard = BitBoard(0x0000_0000_ff00_0000);
pub const RANK_5: BitBoard = BitBoard(0x0000_00ff_0000_0000);
pub const RANK_6: BitBoard = BitBoard(0x0000_ff00_0000_0000);
pub const RANK_7: BitBoard = BitBoard(0x00ff_0000_0000_0000);
pub const RANK_8: BitBoard = BitBoard(0xff00_0000_0000_0000);

pub const FULL_BB: BitBoard = BitBoard(0xffff_ffff_ffff_ffff);
pub const EMPTY_BB: BitBoard = BitBoard(0);