use std::collections::HashSet;

#[derive(Clone)]
pub struct Square {
    pub value: usize,
    domain: HashSet<usize>,
}


impl Square {
    pub fn new(value: usize) -> Square {
        let domain = HashSet::new();
        Square{value, domain}
    }

    pub fn reset(&mut self, range: usize) {
        for value in 1..range+1 {
            self.domain.insert(value);
        }
    }

    pub fn is_valid(&self, max: usize) -> bool {
        self.value > 0 && self.value <= max || !self.domain.is_empty()
    }

    pub fn clear_domain(&mut self) {
        self.domain.clear()
    }

    pub fn remove_from_domain(&mut self, value: &usize) -> bool {
        self.domain.remove(value)
    }

    pub fn get_domain_size(&self) -> usize {
        self.domain.len()
    }

    pub fn set_value_from_domain(&mut self) {
        match self.domain.len() {
            1 => self.value = self.domain.drain().last().unwrap(),
            _ => (),
        }
    }
}

