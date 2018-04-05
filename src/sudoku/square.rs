use std::collections::HashSet;

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
}

