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
    pub fn is_solved(&self) -> bool {
        self.squares.is_solved()
	}
}

