use std::cmp::{max, min};



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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimize_xor() {
        // assert_eq!(minimize_xor(3, 5), 3);
        // assert_eq!(minimize_xor(1, 12), 3);
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
}
