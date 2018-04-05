use std::collections::HashSet;

#[allow(dead_code)]
pub struct DomainSet {
    dimension: usize,
    squares: Vec<HashSet<usize>>,
}

impl DomainSet {
    #[allow(dead_code)]
    pub fn reset_all(&mut self) {
        for domain in self.squares.iter_mut() {
            for value in 1..self.dimension {
                domain.insert(value);
            }
        }
    }
}

