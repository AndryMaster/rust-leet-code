// use rand;
use std::collections::{HashMap, HashSet};



#[derive(Debug)]
pub struct Solution {
    n: i32,
    k: i32,
    remapping: HashMap<i32, i32>,
    rng: LcgRng,
}


impl Solution {
    pub fn new(n: i32, blacklist: Vec<i32>) -> Self {
        if n <= 0 || n <= blacklist.len() as i32 {
            panic!("n is smaller than blacklist length");
        }

        // Calculate the size of valid numbers (total - blacklisted)
        let k = n - blacklist.len() as i32;

        // Create a set of blacklisted numbers for O(1) lookup
        let blacklist_set = HashSet::<_>::from_iter(blacklist);

        // Start searching for valid replacement numbers from end
        let mut replaced_last_num = n-1;

        // Remap blacklisted numbers in range [0, k) to valid numbers in range [k, n)
        let mut remapping = HashMap::new();
        for &black_num in blacklist_set.iter() {
            // Only need to remap if the blacklisted number is in our target range [0, k)
            if black_num < k {
                // Find the next non-blacklisted number in range [k, n)
                while blacklist_set.contains(&replaced_last_num) {
                    replaced_last_num -= 1;
                }
                // Map the blacklisted number to a valid replacement
                remapping.insert(black_num, replaced_last_num);
                replaced_last_num -= 1;
            }
        }

        Solution {
            n, k, remapping, rng: LcgRng::new(56)
        }
    }

    pub fn pick(&mut self) -> i32 {
        // let random = rand::random_range(0..self.k);
        let random = (self.rng.next() >> 1) as i32 % self.k;
        return *self.remapping.get(&random).unwrap_or(&random);
    }
}


#[derive(Debug)]
struct LcgRng {
    state: u32,
}

impl LcgRng {
    fn new(seed: u32) -> Self {
        let mut rng = LcgRng { state: seed };
        rng.next();
        rng
    }

    fn next(&mut self) -> u32 {
        const A: u32 = 1_664_525;
        const C: u32 = 1_013_904_223;

        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }
}
