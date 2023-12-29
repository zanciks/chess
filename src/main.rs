mod bitboard;
mod move_gen;
mod board;
mod piece;
mod color;
mod moves;

fn main() {
    for i in (0..64).rev() {
        println!("{}", move_gen::generate_bishop_mask(i, bitboard::BitBoard::default()));
    }
}
