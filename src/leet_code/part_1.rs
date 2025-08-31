use std::cmp::max;
use std::collections::*;



/// 1957. Delete Characters to Make Fancy String <br>
/// https://leetcode.com/problems/delete-characters-to-make-fancy-string/
pub fn make_fancy_string(mut string: String) -> String {
    let mut cnt= 0;
    let mut prev = '.';

    string.retain(|ch| {
        if ch == prev { cnt += 1 }
        else { cnt = 1 }
        prev = ch;
        cnt <= 2
    });

    string
}


/// 1695. Maximum Erasure Value <br>
/// https://leetcode.com/problems/maximum-erasure-value/
pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    let mut left = 0;
    let mut res = 0;
    let mut sum = 0;

    for &num in nums.iter() {
        if seen.contains(&num) {
            loop {
                seen.remove(&nums[left]);
                sum -= nums[left];
                left += 1;

                if nums[left -1] == num { break }
            }
        }
        seen.insert(num);

        sum += num;
        res = max(res, sum);
    }

    res
}


/// 400. Nth Digit <br>
/// https://leetcode.com/problems/nth-digit/
pub fn find_nth_digit(n: i32) -> i32 {
    if !n.is_positive() {
        panic!("`n` must be a positive integer!");
    }

    let mut n: u64 = n as u64 - 1;
    let mut pos: u64 = 1;
    let mut len: u64 = 1;
    let dbg = |n, pos, len| // {};
        {println!("n={}, \tposition={}, \tlength={}", n, pos, len)};

    loop {
        dbg(n, pos, len);
        let step = 9 * pos * len;

        if n > step {
            n -= step;
        }
        else {
            pos += n / len;
            n %= len;

            dbg(n, pos, len);
            break;
        }

        pos *= 10;
        len += 1;
    }

    let result = pos.to_string().chars().nth(n as usize).unwrap().to_digit(10).unwrap() as i32;
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fancy_string() {
        assert_eq!(make_fancy_string("leeetcode".to_string()), "leetcode".to_string());
        assert_eq!(make_fancy_string("aaabaaaa".to_string()), "aabaa".to_string());
        assert_eq!(make_fancy_string("aab".to_string()), "aab".to_string());
    }

    #[test]
    fn test_maximum_unique_subarray() {
        // assert_eq!(maximum_unique_subarray(vec![1,2,3]), 6);
        assert_eq!(maximum_unique_subarray(vec![1,2,1]), 3);
        assert_eq!(maximum_unique_subarray(vec![4,2,4,5,6]), 17);
        assert_eq!(maximum_unique_subarray(vec![5,2,1,2,5,2,1,2,5]), 8);
    }

    #[test]
    fn test_find_nth_digit() {
        assert_eq!(find_nth_digit(3), 3);
        assert_eq!(find_nth_digit(11), 0);
        assert_eq!(find_nth_digit(10), 1);
        assert_eq!(find_nth_digit(12), 1);
        assert_eq!(find_nth_digit(189), 9);
        assert_eq!(find_nth_digit(1000000000), 1);
    }
}