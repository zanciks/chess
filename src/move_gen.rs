#![allow(dead_code)]

use crate::bitboard::{A_FILE, B_FILE, BitBoard, EMPTY_BB, G_FILE, H_FILE, RANK_1, RANK_2, RANK_4, RANK_5, RANK_7, RANK_8};

pub fn generate_king_mask(square: u8) -> BitBoard {
    let mut attacks = EMPTY_BB;
    let mut piece_board = EMPTY_BB;
    piece_board.set(square);

    attacks |= (piece_board << 7) & !H_FILE & !RANK_1; // up left
    attacks |= (piece_board << 8) & !RANK_1; // up
    attacks |= (piece_board << 9) & !A_FILE & !RANK_1; // up right
    attacks |= (piece_board << 1) & !A_FILE; // right
    attacks |= (piece_board >> 7) & !A_FILE & !RANK_8; // down right
    attacks |= (piece_board >> 8) & !RANK_8; // down
    attacks |= (piece_board >> 9) & !H_FILE & !RANK_8; // down left
    attacks |= (piece_board >> 1) & !H_FILE; // left

    return attacks;
}

pub fn generate_knight_mask(square: u8) -> BitBoard {
    let mut attacks = EMPTY_BB;
    let mut piece_board = EMPTY_BB;
    piece_board.set(square);

    attacks |= (piece_board << 6 ) & !RANK_1 & !G_FILE & !H_FILE; // up1 left2
    attacks |= (piece_board << 15) & !RANK_1 & !RANK_2 & !H_FILE; // up2 left1
    attacks |= (piece_board << 17) & !RANK_1 & !RANK_2 & !A_FILE; // up2 right1
    attacks |= (piece_board << 10) & !RANK_1 & !A_FILE & !B_FILE; // up1 right2
    attacks |= (piece_board >> 6 ) & !RANK_8 & !A_FILE & !B_FILE; // down1 right2
    attacks |= (piece_board >> 15) & !RANK_7 & !RANK_8 & !A_FILE; // down2 right1
    attacks |= (piece_board >> 17) & !RANK_7 & !RANK_8 & !H_FILE; // down2 left1
    attacks |= (piece_board >> 10) & !RANK_8 & !G_FILE & !H_FILE; // down1 left2

    return attacks;
}

pub fn generate_white_pawn_mask(square: u8, opposites: BitBoard) -> BitBoard {
    let mut attacks = EMPTY_BB;
    let mut piece_board = EMPTY_BB;
    piece_board.set(square);

    attacks |= piece_board << 8; // up1
    attacks |= (piece_board << 16) & RANK_4; // up2 (land on RANK_4)
    attacks |= (piece_board << 7) & !H_FILE & opposites ;
    attacks |= (piece_board << 9) & !A_FILE & opposites;

    return attacks;
}

pub fn generate_black_pawn_mask(square: u8, opposites: BitBoard) -> BitBoard {
    let mut attacks = EMPTY_BB;
    let mut piece_board = EMPTY_BB;
    piece_board.set(square);

    attacks |= piece_board >> 8; // down1
    attacks |= (piece_board >> 16) & RANK_5; // down2 (land on RANK_4)
    attacks |= (piece_board >> 7) & !A_FILE & opposites;
    attacks |= (piece_board >> 9) & !H_FILE & opposites;

    return attacks;
}

pub fn generate_rook_mask(square: u8, combined: BitBoard) -> BitBoard {
    let mut attacks = EMPTY_BB;
    let mut piece_board = EMPTY_BB;
    piece_board.set(square);

    let rank = square / 8;
    let file= square % 8;

    // north movement
    for i in rank + 1..8 {
        let offset = 8 * (i - rank) as u64;
        attacks |= piece_board << offset;
        if combined & (piece_board << offset) != EMPTY_BB {break}
    }

    // south movement
    for i in 0..rank {
        let offset = 8 * (i + 1) as u64;
        attacks |= piece_board >> offset;
        if combined & (piece_board >> offset) != EMPTY_BB {break}
    }

    // west movement
    for i in 0..file {
        let offset = (i + 1) as u64;
        attacks |= piece_board >> offset;
        if combined & (piece_board >> offset) != EMPTY_BB {break}
    }

    // east movement
    for i in file + 1..8 {
        let offset = (i - file) as u64;
        attacks |= piece_board << offset;
        if combined & (piece_board >> offset) != EMPTY_BB {break}
    }

    return attacks;
}

pub fn generate_bishop_mask(square: u8, combined: BitBoard) -> BitBoard {
    let mut attacks = EMPTY_BB;
    let mut piece_board = EMPTY_BB;
    piece_board.set(square);

    let rank = square / 8;
    let file= square % 8;

    // North-East movement
    for i in 1..8 {
        if (rank + i) >= 8 || (file + i) >= 8 {break}
        let offset = (9 * i) as u64;
        attacks |= piece_board << offset;
        if combined & (piece_board << offset) != EMPTY_BB {break}
    }

    // South-East movement
    for i in 1..8 {
        if i > rank || (file + i) >= 8 {break}
        let offset = 7 * i as u64;
        attacks |= piece_board >> offset;
        if combined & (piece_board >> offset) != EMPTY_BB {break}
    }

    // South-West movement
    for i in 1..8 {
        if i > rank || i > file {break}
        let offset = 9 * i as u64;
        attacks |= piece_board >> offset;
        if combined & (piece_board >> offset) != EMPTY_BB {break}
    }

    // North-West movement
    for i in 1..8 {
        if (rank + i) >= 8 || i > file {break}
        let offset = 7 * i as u64;
        attacks |= piece_board << offset;
        if combined & (piece_board << offset) != EMPTY_BB {break}
    }

    return attacks;
}

pub fn generate_queen_mask(square: u8, combined: BitBoard) -> BitBoard {
    let mut attacks = EMPTY_BB;
    let mut piece_board = EMPTY_BB;
    piece_board.set(square);

    attacks |= generate_rook_mask(square, combined);
    attacks |= generate_bishop_mask(square, combined);

    return attacks;
}