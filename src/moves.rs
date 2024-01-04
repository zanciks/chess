#![allow(dead_code)]

use crate::bitboard::BitBoard;

#[derive(Debug)]
pub struct Move {
    initial_square: u8,
    target_square: u8,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            initial_square: 0,
            target_square: 0,
        }
    }
}

impl Move {
    pub fn new(initial_square: u8, target_square: u8) -> Self {
        Move {initial_square, target_square}
    }
    pub fn from_square_bitboard(square: u8, bitboard: BitBoard) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        for index in bitboard.find_set_bits() {
            moves.push(Move::new(square, index))
        }
        return moves;
    }
}