use std::cmp::{max, min};
use std::collections::*;
use std::io::Empty;

/// 190. Reverse Bits <br>
/// https://leetcode.com/problems/reverse-bits/
pub fn reverse_bits(n: i32) -> i32 {
    n.reverse_bits()
}

/// 231. Power of Two <br>
/// https://leetcode.com/problems/power-of-two/
pub fn is_power_of_two(n: i32) -> bool {
    (n.count_ones() == 1) & (n > 0)
}

/// 1518. Water Bottles <br>
/// https://leetcode.com/problems/water-bottles/
pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut result = num_bottles;
    let mut empty = 0;
    while num_bottles > 0 {
        empty += num_bottles;
        num_bottles = empty / num_exchange;
        result += num_bottles;
        empty %= num_exchange;
    }

    result
}

/// 3100. Water Bottles II <br>
/// https://leetcode.com/problems/water-bottles-ii/
pub fn max_bottles_drunk(mut bottles_full: i32, mut num_exchange: i32) -> i32 {
    struct CalcResult {
        bottles_empty_exchange: i32,
        bottles_full_get: i32,
    }
    fn calc_bottles(bottles_empty: i32, n: i32) -> CalcResult {
        let a = 1;
        let b = 2 * n - 1;
        let c = -2 * bottles_empty;

        let disc_sqrt = (b.pow(2) - 4*a*c).isqrt();
        let (_, k2) = ((-b - disc_sqrt) / 2*a, (-b + disc_sqrt) / 2*a);
        let bottles_empty_need = (n + (n+k2-1)) * k2 / 2;

        CalcResult {
            bottles_empty_exchange: bottles_empty_need,
            bottles_full_get: k2,
        }
    }

    let mut result = 0;
    let mut bottles_empty = 0;
    while bottles_full > 0 {
        result += bottles_full;
        bottles_empty += bottles_full;

        let calc = calc_bottles(bottles_empty, num_exchange);
        bottles_full = calc.bottles_full_get;
        num_exchange += calc.bottles_full_get;
        bottles_empty -= calc.bottles_empty_exchange;
    }

    result
}

/// 1558. Minimum Numbers of Function Calls to Make Target Array <br>
/// https://leetcode.com/problems/minimum-numbers-of-function-calls-to-make-target-array/
pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut result: i32 = nums.iter().map(|n| { n.count_ones() as i32 }).sum();
    let max_pow_2 = 32 - nums.iter().max().unwrap().leading_zeros() as i32;
    if max_pow_2 > 0 {
        result += max_pow_2 - 1;
    }

    result
}

/// 407. Trapping Rain Water II <br>
/// https://leetcode.com/problems/trapping-rain-water-ii/
pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(0b01111111111111111111111111110000), 0b00001111111111111111111111111110);
    }

    #[test]
    fn test_is_power_of_two() {
        assert_eq!(is_power_of_two(1), true);
        assert_eq!(is_power_of_two(1 << 15), true);
        assert_eq!(is_power_of_two(1 << 20 | 1 << 10), false);
        assert_eq!(is_power_of_two(-4), false);
        assert_eq!(is_power_of_two(i32::MIN), false);
        assert_eq!(is_power_of_two(-2147483646), false);
    }

    #[test]
    fn test_num_water_bottles() {
        assert_eq!(num_water_bottles(9, 3), 13);
        assert_eq!(num_water_bottles(15, 4), 19);
    }

    #[test]
    fn test_max_bottles_drunk() {
        assert_eq!(max_bottles_drunk(13, 6), 15);
        assert_eq!(max_bottles_drunk(10, 3), 13);
    }

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(vec![1,5]), 5);
        assert_eq!(min_operations(vec![2,2]), 3);
        assert_eq!(min_operations(vec![0]), 0);
    }
}