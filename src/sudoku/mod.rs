mod domain_set;
mod square;
mod square_set;


#[allow(dead_code)]
pub struct Puzzle {
    dimension: usize,
    squares: square_set::SquareSet,
    domains: domain_set::DomainSet,
    modified: bool,
}

impl Puzzle {
    #[allow(dead_code)]
    pub fn read_square(&self, x: usize, y: usize) -> Option<usize> {
        match self.squares.get(x, y) {
            Some(&square::Square{value: x}) => Some(x),
            None => None,
        }
    }
}

