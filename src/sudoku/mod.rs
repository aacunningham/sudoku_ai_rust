mod square;
mod square_set;

use self::square_set::SquareSet;


#[allow(dead_code)]
pub struct Puzzle {
    dimension: usize,
    squares: SquareSet,
    modified: bool,
}

impl Puzzle {
    #[allow(dead_code)]
    pub fn read_square(&self, x: usize, y: usize) -> Option<usize> {
        match self.squares.get(x, y) {
            Some(&square::Square{value, ..}) => Some(value),
            None => None,
        }
    }
}

