#[derive(Debug, Clone)]
pub struct Bitset {
    map: Vec<u8>,
    size: i32,
    last_byte_mask: u8,
}


impl Bitset {
    pub fn new(size: i32) -> Self {
        if !size.is_positive() {
            panic!("Size must be positive.");
        }

        let len = ((size + 7) / 8) as usize;
        let map = vec![0; len];

        let shift = (8 - size % 8) as u32;
        let last_byte_mask = u8::MAX.checked_shr(shift).unwrap_or(u8::MAX);

        Bitset {
            map,
            size,
            last_byte_mask,
        }
    }

    pub fn fix(&mut self, idx: i32) {
        let (pos, bit_mask) = self.get_pos(idx);
        self.map[pos] |= bit_mask;
    }

    pub fn unfix(&mut self, idx: i32) {
        let (pos, bit_mask) = self.get_pos(idx);
        self.map[pos] &= !bit_mask;
    }

    pub fn flip(&mut self) {
        for pos in 0..self.map.len() {
            self.map[pos] = !self.map[pos];
        }

        let last_byte_idx = self.map.len() - 1;
        self.map[last_byte_idx] &= self.last_byte_mask;
    }

    pub fn all(&self) -> bool {
        let all_not_last = self.map.iter().rev().skip(1).all(|&byte| byte == u8::MAX);
        let with_last = *self.map.last().unwrap() == self.last_byte_mask;
        return all_not_last && with_last;
    }

    pub fn one(&self) -> bool {
        self.map.iter().any(|&byte| byte != 0)
    }

    pub fn count(&self) -> i32 {
        self.map.iter().map(|&byte| byte.count_ones()).sum::<u32>() as i32
    }

    pub fn to_string(&self) -> String {
        let mut str = String::with_capacity(self.size as usize);
        str += &format!("{:08b}", self.map.last().unwrap())[(self.last_byte_mask.count_zeros() as usize)..];
        self.map.iter().rev().skip(1).for_each(|byte| str += &format!("{:08b}", byte));
        return str.chars().rev().collect::<String>();
    }

    fn get_pos(&self, idx: i32) -> (usize, u8) {
        if idx >= self.size || idx < 0 {
            panic!("Index out of range.");
        }

        ((idx / 8) as usize, 1u8 << (idx % 8))
    }
}
