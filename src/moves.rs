#![allow(dead_code)]

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
}