use crate::bitboard::{A_FILE, B_FILE, BitBoard, G_FILE, H_FILE, RANK_1, RANK_2, RANK_7, RANK_8};

pub fn generate_king_mask(square: u8) {
    let mut attacks = BitBoard::default();
    let mut piece_board = BitBoard::default();
    piece_board.set(square);

    attacks |= (piece_board << 7) & !H_FILE & !RANK_1; // up left
    attacks |= (piece_board << 8) & !RANK_1; // up
    attacks |= (piece_board << 9) & !A_FILE & !RANK_1; // up right
    attacks |= (piece_board << 1) & !A_FILE; // right
    attacks |= (piece_board >> 7) & !A_FILE & !RANK_8; // down right
    attacks |= (piece_board >> 8) & !RANK_8; // down
    attacks |= (piece_board >> 9) & !H_FILE & !RANK_8; // down left
    attacks |= (piece_board >> 1) & !H_FILE; // left

    println!("{}", attacks.value());
}

pub fn generate_knight_mask(square: u8) {
    let mut attacks = BitBoard::default();
    let mut piece_board = BitBoard::default();
    piece_board.set(square);

    attacks |= (piece_board << 6 ) & !RANK_1 & !G_FILE & !H_FILE; // up1 left2
    attacks |= (piece_board << 15) & !RANK_1 & !RANK_2 & !H_FILE; // up2 left1
    attacks |= (piece_board << 17) & !RANK_1 & !RANK_2 & !A_FILE; // up2 right1
    attacks |= (piece_board << 10) & !RANK_1 & !A_FILE & !B_FILE; // up1 right2
    attacks |= (piece_board >> 6 ) & !RANK_8 & !A_FILE & !B_FILE; // down1 right2
    attacks |= (piece_board >> 15) & !RANK_7 & !RANK_8 & !A_FILE; // down2 right1
    attacks |= (piece_board >> 17) & !RANK_7 & !RANK_8 & !H_FILE; // down2 left1
    attacks |= (piece_board >> 10) & !RANK_8 & !G_FILE & !H_FILE; // down1 left2

    println!("{}", attacks);
}