mod bitboard;
mod square;
mod move_gen;

fn main() {
    for i in 0..64 {
        move_gen::generate_knight_mask(i);
    }
}
