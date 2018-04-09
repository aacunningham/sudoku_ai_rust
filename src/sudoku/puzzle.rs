use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use super::square::Square;


pub struct Puzzle {
    dimension: usize,
    squares: Vec<Square>,
}

impl Puzzle {
    pub fn read_from_file(source: &mut File) -> Puzzle {
        let mut contents = String::new();
        source.read_to_string(&mut contents).unwrap();
        let squares = contents.split(" ")
                              .filter_map(|x| x.parse::<usize>().ok())
                              .map(|x| Square::new(x))
                              .collect::<Vec<_>>();
        let size = squares.len();
        let dimension = (size as f64).sqrt() as usize;
        Puzzle {
            dimension,
            squares,
        }
    }

    pub fn read_from_string(source: &str) -> Puzzle {
        let squares = source.split(" ")
                            .filter_map(|x| x.parse::<usize>().ok())
                            .map(|x| Square::new(x))
                            .collect::<Vec<_>>();
        let size = squares.len();
        let dimension = (size as f64).sqrt() as usize;
        Puzzle {
            dimension,
            squares,
        }
    }

    pub fn is_solved(&self) -> bool {
        self.is_valid() && self.all_filled()
            
    }

	fn all_filled(&self) -> bool {
        self.squares.iter().all(|square| square.value != 0)
    }

    fn is_valid(&self) -> bool {
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

    fn reset_domains(&mut self) {
        for square in self.squares.iter_mut() {
            square.reset(self.dimension);
        }
    }

    fn update_domains(&mut self) {
        for counter in 0..self.squares.len() {
            if self.squares[counter].value != 0 {
                self.squares[counter].clear_domain();
                continue
            }

            let x = counter % self.dimension;
            let y = counter / self.dimension;

            let mut to_be_removed = Vec::with_capacity(self.dimension * 3);
            to_be_removed.append(&mut self.get_column(x));
            to_be_removed.append(&mut self.get_row(y));
            to_be_removed.append(&mut self.get_group(x, y));
            to_be_removed.sort();
            to_be_removed.dedup();

            let square = &mut self.squares[counter];

            for value in to_be_removed {
                square.remove_from_domain(&value);
            }
        }
    }

    fn get_column(&self, x: usize) -> Vec<usize> {
        let step = self.dimension;
        self.squares.iter().skip(x).step_by(step)
                    .filter(|square| square.value != 0).map(|square| square.value)
                    .collect()
    }

    fn get_row(&self, y: usize) -> Vec<usize> {
        let row_start = y * self.dimension;
        let row_end = row_start + self.dimension;
        self.squares[row_start..row_end].iter()
            .filter(|square| square.value != 0).map(|square| square.value)
            .collect()
    }

    fn get_group(&self, x: usize, y: usize) -> Vec<usize> {
        let group_dimension = (self.dimension as f64).sqrt() as usize;
        let group_initial_x = (x / group_dimension) * group_dimension;
        let group_initial_y = (y / group_dimension) * group_dimension;
        let initial = group_initial_x + group_initial_y * self.dimension;
        let mut result = Vec::new();
        for counter in 0..group_dimension {
            let initial_skip = initial + (counter * group_dimension * group_dimension);
            result.extend(self.squares.iter().skip(initial_skip).take(group_dimension));
        }
        result.iter().filter(|square| square.value != 0)
              .map(|square| square.value).collect()
    }

    fn find_next_n_domain(&self, n: usize) -> Option<usize> {
        self.squares.iter().position(|square| square.get_domain_size() == n)
    }

    fn find_next_empty_square(&self) -> Option<usize> {
        self.squares.iter().position(|square| square.value == 0)
    }

    pub fn solve(&mut self) -> bool {
        self.reset_domains();
        let mut history: Vec<Snapshot> = Vec::new();
        loop {
            self.update_domains();
            if !self.is_valid() {
                match history.pop() {
                    Some(Snapshot{squares, index}) => {
                        let wrong_value = self.squares[index].value;
                        self.squares = squares;
                        self.squares[index].remove_from_domain(&wrong_value);
                    },
                    None => break
                }
            }
            match self.find_next_n_domain(1) {
                Some(index) => {
                    self.squares[index].set_value_from_domain();
                },
                None => {
                    match self.find_next_n_domain(2).or_else(|| self.find_next_empty_square()) {
                        Some(index) => {
                            let squares = self.squares.clone();
                            history.push(Snapshot{squares, index});
                            self.squares[index].set_value_from_domain();
                        },
                        None => break
                    }
                }
            }
        }
        true
    }
}


impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.squares.chunks(self.dimension) {
            for square in row {
                write!(f, "{} ", square.value)?;
            }
            write!(f, "\n")?;
        }
        writeln!(f, "")
    }
}


struct Snapshot {
    squares: Vec<Square>,
    index: usize,
}

