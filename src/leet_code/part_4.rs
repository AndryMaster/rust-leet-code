use std::cmp::{Reverse};
use std::collections::{BinaryHeap, HashMap};
use itertools::{Itertools};


/// 2429. Minimize XOR <br>
/// https://leetcode.com/problems/minimize-xor/
pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let mut delta = num1.count_ones() as i32 - num2.count_ones() as i32;
    let mut num = num1;

    if delta == 0 {
        num
    } else {
        for bit in (0..32).map(|i| 1<<i) {
            if (delta > 0) && ((num & bit) == bit) {
                num ^= bit;
                delta -= 1;
            }
            else if (delta < 0) && ((num & bit) == 0) {
                num ^= bit;
                delta += 1;
            }
            if delta == 0 {
                break;
            }
        }
        num
    }
}

/// 29. Divide Two Integers <br>
/// https://leetcode.com/problems/divide-two-integers/
pub fn divide(a: i32, b: i32) -> i32 {
    let is_positive = (a > 0) == (b > 0);
    let mut a = a.wrapping_abs() as u32;
    let mut b = b.wrapping_abs() as u32;
    let mut ans: u32 = 0;

    if a < b {
        return 0;
    }

    let full_shift = b.leading_zeros() - a.leading_zeros();
    for shift in (0..=full_shift).rev() {
        if a >= b<<shift {
            a -= b<<shift;
            ans += 1<<shift;
        }
    }

    let very_much = i32::MIN.wrapping_abs() as u32;
    match (ans, is_positive) {
        (n, true)  if n == very_much => { i32::MAX }
        (n, false) if n == very_much => { i32::MIN }
        (_, true) => { ans as i32 }
        (_, false) => { -(ans as i32) }
    }
}

/// 409. Longest Palindrome <br>
/// https://leetcode.com/problems/longest-palindrome/
pub fn longest_palindrome(s: String) -> i32 {
    let counts = s.chars().counts();
    counts.iter().map(|(_, count)| count & !1).sum::<usize>() as i32 +
        counts.iter().any(|(_, count)| count & 1 == 1) as i32
}

/// 413. Arithmetic Slices <br>
/// https://leetcode.com/problems/arithmetic-slices/
pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    let mut res = 0;
    let mut count = 2;
    let mut diff = nums[0] - nums[1];
    let mut last = nums[1];

    for n in nums.into_iter().skip(2) {
        let diff_new = last - n;
        if diff_new == diff {
            count += 1;
        }
        else {
            diff = diff_new;
            if count >= 3 {
                res += (count-1)*(count-2)/2;
            }
            count = 2;
        }

        last = n;
    }
    if count >= 3 {
        res += (count-1)*(count-2)/2;
    }

    res
}

/// 295. Find Median from Data Stream <br>
/// https://leetcode.com/problems/find-median-from-data-stream/
struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}
impl MedianFinder {
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));

        if self.min_heap.len() > self.max_heap.len() + 1 {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    pub fn find_median(&self) -> f64 {
        let min_peek = self.min_heap.peek().unwrap().0;

        if self.min_heap.len() == self.max_heap.len() {
            let max_peek = *self.max_heap.peek().unwrap();
            return (min_peek + max_peek) as f64 / 2.;
        }

        min_peek as f64
    }
}

/// 1488. Avoid Flood in The City <br>
/// https://leetcode.com/problems/avoid-flood-in-the-city/
pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut rainy = HashMap::new();
    let mut sunny = vec![];
    let mut result = vec![-1; rains.len()];

    for (pos, &val) in rains.iter().enumerate() {
        if val > 0 {
            if rainy.contains_key(&val) {
                let rainy_pos = *rainy.get(&val).unwrap();

                let sunny_idx;
                let sunny_idx_res = sunny.binary_search(&rainy_pos);
                match sunny_idx_res {
                    Ok(idx) => sunny_idx = idx,
                    Err(idx) => sunny_idx = idx,
                }
                let sunny_pos = sunny.get(sunny_idx);

                if sunny_pos.is_some() && sunny_pos.unwrap() < &pos {
                    result[*sunny_pos.unwrap()] = val;
                    sunny.remove(sunny_idx);
                }
                else {
                    return vec![];
                }
            }
            rainy.insert(val, pos);
        }
        else {
            result[pos] = 1;
            sunny.push(pos);
        }
    }

    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimize_xor() {
        assert_eq!(minimize_xor(3, 5), 3);
        assert_eq!(minimize_xor(1, 12), 3);
        assert_eq!(minimize_xor(79, 74), 76);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 3), 3);
        assert_eq!(divide(7, -3), -2);
        assert_eq!(divide(1, 1), 1);
        assert_eq!(divide(-2147483648, -1), 2147483647);
        assert_eq!(divide(-2147483648, 1), -2147483648);
        assert_eq!(divide(1, 2), 0);
    }

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome(String::from("abccccdd")), 7);
        assert_eq!(longest_palindrome(String::from("ad")), 1);
    }

    #[test]
    fn test_number_of_arithmetic_slices() {
        assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(number_of_arithmetic_slices(vec![1,2,3,8,9,10]), 2);
        assert_eq!(number_of_arithmetic_slices(vec![1]), 0);
    }

    #[test]
    fn test_median() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        assert_eq!(median_finder.find_median(), 1.5); // return 1.5 (i.e., (1 + 2) / 2)
        median_finder.add_num(3);
        assert_eq!(median_finder.find_median(), 2.0); // return 2.0
    }

    #[test]
    fn test_avoid_flood() {
        assert_eq!(avoid_flood(vec![1,2,3,4]), vec![-1,-1,-1,-1]);
        assert_eq!(avoid_flood(vec![1,2,0,0,2,1]), vec![-1,-1,2,1,-1,-1]);
        assert_eq!(avoid_flood(vec![1,2,0,1,2]), vec![]);
        assert_eq!(avoid_flood(vec![69,0,0,0,69]), vec![-1,69,1,1,-1]);
        assert_eq!(avoid_flood(vec![0,1,1]), vec![]);
        assert_eq!(avoid_flood(vec![1,0,2,0,2,1]), vec![-1,1,-1,2,-1,-1]);
        assert_eq!(avoid_flood(vec![1,0,2,0,3,0,2,0,0,0,1,2,3]), vec![-1,1,-1,2,-1,3,-1,2,1,1,-1,-1,-1]);
    }
}
