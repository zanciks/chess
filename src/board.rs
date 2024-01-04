#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use crate::bitboard::BitBoard;
use crate::color::Color;
use crate::move_gen::{generate_bishop_mask, generate_black_pawn_mask, generate_king_mask, generate_knight_mask, generate_queen_mask, generate_rook_mask, generate_white_pawn_mask};
use crate::moves::Move;
use crate::piece::Piece;

pub struct Board {
    pieces: [BitBoard; 6], // pawns, knight, bishops, rooks, queens, kings
    colors: [BitBoard; 2], // white, black
    combined: BitBoard, // all pieces of all colors,
    turn: Color,
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
            turn: Color::White,
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
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut output_moves: Vec<Move> = vec![];
        let friendlies = match self.turn {
            Color::White => self.colors[0],
            Color::Black => self.colors[1]
        };

        for square in 0..64 {
            if let Some(piece) = self.piece_at(square) {
                if self.color_at(square) == Some(self.turn) {
                    let movement_mask = match piece {
                        Piece::Knight => generate_knight_mask(square),
                        Piece::Bishop => generate_bishop_mask(square, self.combined),
                        Piece::Rook => generate_rook_mask(square, self.combined),
                        Piece::Queen => generate_queen_mask(square, self.combined),
                        Piece::King => generate_king_mask(square),
                        Piece::Pawn => match self.turn {
                            Color::White => generate_white_pawn_mask(square, self.colors[1]),
                            Color::Black => generate_black_pawn_mask(square, self.colors[0]),
                        },
                    };
                    let movement_bitboard = movement_mask & !friendlies;
                    output_moves.append(&mut Move::from_square_bitboard(square, movement_bitboard))
                }
            }
        }
        return output_moves;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        for rank in (0..8).rev() {
            output.push('\n');
            for file in 0..8 {
                let index = rank * 8 + file;
                output.push(match (self.piece_at(index), self.color_at(index)) {
                    (Some(Piece::Pawn),   Some(Color::White)) => 'P',
                    (Some(Piece::Knight), Some(Color::White)) => 'N',
                    (Some(Piece::Bishop), Some(Color::White)) => 'B',
                    (Some(Piece::Rook),   Some(Color::White)) => 'R',
                    (Some(Piece::Queen),  Some(Color::White)) => 'Q',
                    (Some(Piece::King),   Some(Color::White)) => 'K',

                    (Some(Piece::Pawn),   Some(Color::Black)) => 'p',
                    (Some(Piece::Knight), Some(Color::Black)) => 'n',
                    (Some(Piece::Bishop), Some(Color::Black)) => 'b',
                    (Some(Piece::Rook),   Some(Color::Black)) => 'r',
                    (Some(Piece::Queen),  Some(Color::Black)) => 'q',
                    (Some(Piece::King),   Some(Color::Black)) => 'k',
                    _ => '.',
                });
                output.push(' ');
            }
        }
        write!(f, "{}", output)
    }
}