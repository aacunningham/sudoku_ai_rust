mod square;
mod square_set;

use std::collections::HashSet;

#[allow(dead_code)]
pub struct Puzzle {
    dimension: u32,
    squares: square_set::SquareSet,
    domains: Vec<Vec<HashSet<u32>>>,
    modified: bool,
}

impl Puzzle {
    #[allow(dead_code)]
    pub fn reset_domains(&mut self) {
        for domain_row in self.domains.iter_mut() {
            for domain in domain_row.iter_mut() {
                for value in 1..self.dimension {
                    domain.insert(value);
                }
            }
        }
    }
}

