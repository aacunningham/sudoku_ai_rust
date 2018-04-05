use sudoku::square::Square;

#[allow(dead_code)]
pub struct SquareSet {
    dimension: usize,
    squares: Vec<Square>,
}

impl SquareSet {
    #[allow(dead_code)]
    pub fn new(size: usize) -> SquareSet {
        SquareSet {
            dimension: size,
            squares: Vec::with_capacity(size * size)
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, x: usize, y: usize) -> Option<&Square> {
        let location = x + y * self.dimension;
        self.squares.get(location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut square_set = SquareSet::new(4);
        let mut square = square_set.get(0, 0);
    }
}

