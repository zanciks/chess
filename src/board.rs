#![allow(dead_code)]

use crate::bitboard::BitBoard;
use crate::color::Color;
use crate::piece::Piece;

pub struct Board {
    pieces: [BitBoard; 6], // pawns, knight, bishops, rooks, queens, kings
    colors: [BitBoard; 2], // white, black
    combined: BitBoard, // all pieces of all colors
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [
                BitBoard::new(0xff00000000ff00), // pawns
                BitBoard::new(0x4200000000000042), // knights
                BitBoard::new(0x2400000000000024), // bishops
                BitBoard::new(0x8100000000000081), // rooks
                BitBoard::new(0x800000000000008), // queens
                BitBoard::new(0x1000000000000010), // kings
            ],
            colors: [
                BitBoard::new(0xffff), // white
                BitBoard::new(0xffff000000000000), // black
            ],
            combined: BitBoard::new(0xffff00000000ffff),
        }
    }
}

impl Board {
    pub fn white(&self) -> BitBoard {
        self.colors[0]
    }
    pub fn black(&self) -> BitBoard {
        self.colors[1]
    }
    pub fn pawns(&self) -> BitBoard {
        self.pieces[0]
    }
    pub fn knights(&self) -> BitBoard {
        self.pieces[1]
    }
    pub fn bishops(&self) -> BitBoard {
        self.pieces[2]
    }
    pub fn rooks(&self) -> BitBoard {
        self.pieces[3]
    }
    pub fn queens(&self) -> BitBoard {
        self.pieces[4]
    }
    pub fn kings(&self) -> BitBoard {
        self.pieces[5]
    }
    pub fn piece_at(&self, square: u8) -> Option<Piece> {
        for (i, bitboard) in self.pieces.iter().enumerate() {
            if bitboard.get(square) == 1 {
                return Some(Piece::from(i));
            }
        }
        return None;
    }
    pub fn color_at(&self, square: u8) -> Option<Color> {
        for (i, bitboard) in self.colors.iter().enumerate() {
            if bitboard.get(square) == 1 {
                return Some(Color::from(i));
            }
        }
        return None;
    }
}