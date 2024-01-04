#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black
}

impl From<usize> for Color {
    fn from(value: usize) -> Self {
        return match value {
            0 => Color::White,
            1 => Color::Black,
            _ => panic!("Not a color!")
        }
    }
}