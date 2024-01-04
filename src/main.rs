mod bitboard;
mod move_gen;
mod board;
mod piece;
mod color;
mod moves;

fn main() {
    let board = board::Board::default();
    println!("{}", board);
    for i in board.generate_moves() {
        println!("{:?}", i);
    }
}
