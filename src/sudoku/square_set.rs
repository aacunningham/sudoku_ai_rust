use std::collections::HashSet;

use sudoku::square::Square;

#[derive(Clone)]
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

	pub fn all_filled(&self) -> bool {
        self.squares.iter().all(|square| square.value != 0)
    }

    pub fn is_valid(&self) -> bool {
        if !self.squares.iter().all(|square| square.is_valid()) {
            return false;
        }
        for counter in 0..self.dimension {
            let mut set = HashSet::new();
            for value in self.get_column(counter) {
                if !set.insert(value) {
                    return false;
                }
            }
            set.clear();
            for value in self.get_row(counter) {
                if !set.insert(value) {
                    return false;
                }
            }
            set.clear();

            let group_dimension = (self.dimension as f64).sqrt() as usize;
            let x = (counter % group_dimension) * group_dimension;
            let y = (counter / group_dimension) * group_dimension;
            for value in self.get_group(x, y) {
                if !set.insert(value) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_solved(&self) -> bool {
        self.is_valid() && self.all_filled()
            
    }

    pub fn reset_domains(&mut self) {
        for square in self.squares.iter_mut() {
            square.reset(self.dimension);
        }
    }

    pub fn update_domains(&mut self) {
        for counter in 0..self.squares.len() {
            if self.squares[counter].value != 0 {
                self.squares[counter].domain.clear();
                continue
            }

            let x = counter / self.dimension;
            let y = counter % self.dimension;

            let column = self.get_column(x);
            let row = self.get_row(y);
            let group = self.get_group(x, y);

            let square = &mut self.squares[counter];

            for value in column {
                square.domain.remove(&value);
            }
            for value in row {
                square.domain.remove(&value);
            }
            for value in group {
                square.domain.remove(&value);
            }
        }
    }

    pub fn get_column(&self, x: usize) -> Vec<usize> {
        let step = self.dimension;
        self.squares.iter().skip(x).step_by(step)
                    .filter(|square| square.value != 0).map(|square| square.value)
                    .collect()
    }

    pub fn get_row(&self, y: usize) -> Vec<usize> {
        let row_start = y * self.dimension;
        let row_end = row_start + self.dimension;
        self.squares[row_start..row_end].iter()
            .filter(|square| square.value != 0).map(|square| square.value)
            .collect()
    }

    pub fn get_group(&self, x: usize, y: usize) -> Vec<usize> {
        let group_dimension = (self.dimension as f64).sqrt() as usize;
        let group_initial_x = (x / group_dimension) * group_dimension;
        let group_initial_y = (y / group_dimension) * group_dimension;
        let initial = group_initial_x + group_initial_y * self.dimension;
        self.squares.iter()
                    .skip(initial).take(group_dimension)
                    .skip(group_dimension * 2).take(group_dimension)
                    .skip(group_dimension * 2).take(group_dimension)
                    .filter(|square| square.value != 0).map(|square| square.value)
                    .collect()
    }

    pub fn find_next_n_domain(&self, n: usize) -> Option<usize> {
        self.squares.iter().position(|square| square.domain.len() == n)
    }

    pub fn find_next_empty_square(&self) -> Option<usize> {
        self.squares.iter().position(|square| square.value == 0)
    }

    pub fn solve(&mut self) -> bool {
        let mut history: Vec<(Vec<Square>, usize)> = Vec::new();
        loop {
            self.update_domains();
            if !self.is_valid() {
                match history.pop() {
                    Some((previous_set, index)) => {
                        let wrong_value = self.squares[index].value;
                        self.squares = previous_set;
                        self.squares[index].domain.remove(&wrong_value);
                    },
                    None => break
                }
            }
            match self.find_next_n_domain(1) {
                Some(index) => {
                    let value = self.squares[index].domain.drain().last()
                                                   .expect("something awful happened");
                    self.squares[index].value = value;
                },
                None => {
                    match self.find_next_n_domain(2).or_else(|| self.find_next_empty_square()) {
                        Some(index) => {
                            history.push((self.squares.clone(), index));
                            let value = self.squares[index].domain.drain().last()
                                                           .expect("something awful happened");
                            self.squares[index].value = value;
                        },
                        None => break
                    }
                }
            }
        }
        true
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

