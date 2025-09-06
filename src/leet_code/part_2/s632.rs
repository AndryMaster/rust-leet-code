use std::cmp::Ordering;

#[derive(Debug, Ord, Eq)]
pub struct SmRange {
    pos: usize,
    vec: Vec<i32>,
}

impl SmRange {
    pub fn new(vec: Vec<i32>) -> Self {
        if vec.len() == 0 {
            panic!("vec length is 0");
        }
        Self {
            pos: 0,
            vec: vec,
        }
    }
    pub fn get(&self) -> i32 {
        self.vec[self.pos]
    }
    pub fn has_next(&self) -> bool {
        self.pos+1 < self.vec.len()
    }
    pub fn next(&mut self) {
        self.pos += 1;
    }
}

impl PartialEq<Self> for SmRange {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl PartialOrd for SmRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.get().partial_cmp(&self.get())
    }
}
