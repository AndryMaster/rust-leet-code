use std::ops::Add;

#[derive(Copy, Clone)]
pub struct BitTime {
    pub hh: u8,
    pub mm: u8,
}

impl Add for BitTime {
    type Output = BitTime;
    fn add(self, rhs: BitTime) -> BitTime {
        BitTime {
            hh: self.hh + rhs.hh,
            mm: self.mm + rhs.mm
        }
    }
}

impl BitTime {
    pub fn new(hh: u8, mm: u8) -> Self {
        BitTime { hh, mm }
    }

    pub fn as_string(&self) -> Option<String> {
        if self.hh >= 12 || self.mm >= 60 {
            return None
        }
        Some(format!("{}:{:0>2}", self.hh, self.mm))
    }
}
