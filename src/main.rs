mod bitboard;
mod move_gen;
mod board;
mod piece;
mod color;
mod r#move;

fn main() {
    let bitboard = bitboard::EMPTY_BB;
    move_gen::generate_queen_mask(27, bitboard);
}
