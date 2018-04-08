use std::collections::HashSet;

#[derive(Clone)]
pub struct Square {
    pub value: usize,
    pub domain: HashSet<usize>,
}


impl Square {
    #[allow(dead_code)]
    pub fn reset(&mut self, range: usize) {
        for value in 1..range {
            self.domain.insert(value);
        }
    }

    pub fn is_valid(&self) -> bool {
        self.value != 0 || !self.domain.is_empty()
    }
}

