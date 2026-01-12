use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

mod s460;
use s460::LFUCache;

mod s710;
use s710::Solution;



/// 3075. Maximize Happiness of Selected Children <br>
/// https://leetcode.com/problems/maximize-happiness-of-selected-children/
pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    happiness
        .iter()
        .sorted()
        .rev()
        .take(k as usize)
        .enumerate()
        .map(|(i, h)| max(0, h - i as i32) as i64)
        .sum::<i64>()
}

/// 1980. Find Unique Binary String <br>
/// https://leetcode.com/problems/find-unique-binary-string/
pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let nums = nums.iter().map(|x| {u32::from_str_radix(x, 2).unwrap()}).collect::<Vec<_>>();
    let res = nums.iter().enumerate().fold(0, |res, (i, num)| { res | ((!num) & (1 << i)) });

    // let nums = nums.iter().map(|x| {u32::from_str_radix(x, 2).unwrap()}).sorted().collect::<Vec<_>>();
    // let mut range = 0..(1<<n) as u32;
    // let res = range.find(|x| nums.binary_search(x).is_err()).unwrap();

    return format!("{:0n$b}", res);
}

/// 268. Missing Number <br>
/// https://leetcode.com/problems/missing-number/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    (0..=nums.len() as i32).merge(nums).fold(0, |acc, x| acc ^ x)
}

/// 448. Find All Numbers Disappeared in an Array <br>
/// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let range = (1..=n as i32);
    let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
    return range.filter(|i| !set.contains(i)).collect();

    // let mut set = vec![false; n+1];
    // for num in nums {
    //     set[num as usize] = true;
    // }
    // return range.filter(|&num| !set[num as usize]).collect();
}

/// 984. String Without AAA or BBB <br>
/// https://leetcode.com/problems/string-without-aaa-or-bbb/
pub fn str_without3a3b(a: i32, b: i32) -> String {
    // жадный алгоритм где больше там аа, где меньше б
    todo!()
}

/// 942. DI String Match <br>
/// https://leetcode.com/problems/di-string-match/
pub fn di_string_match(s: String) -> Vec<i32> {
    todo!()

}

/// 781. Rabbits in Forest <br>
/// https://leetcode.com/problems/rabbits-in-forest/
pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    answers
        .into_iter()
        .counts()
        .into_iter()
        .map(|(n, counts)| (counts as i32 + n) / (n + 1) * (n + 1))
        .sum::<i32>()
}

/// 670. Maximum Swap <br>
/// https://leetcode.com/problems/maximum-swap/
pub fn maximum_swap(num: i32) -> i32 {
    todo!()
}



#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_maximum_happiness_sum() {
        assert_eq!(maximum_happiness_sum(vec![1,2,3], 2), 4);
        assert_eq!(maximum_happiness_sum(vec![1,1,1,1], 2), 1);
        assert_eq!(maximum_happiness_sum(vec![2,3,4,5], 1), 5);
    }

    #[test]
    fn test_find_different_binary_string() {
        fn not_contains_test(nums: Vec<String>) -> bool {
            let res = find_different_binary_string(nums.clone());
            !nums.contains(&res)  // dbg!(&res, &nums);
        }

        assert!(not_contains_test(vec!["01".to_string(),"10".to_string()]));
        assert!(not_contains_test(vec!["01".to_string(),"00".to_string()]));
        assert!(not_contains_test(vec!["111".to_string(),"011".to_string(),"001".to_string()]));
        assert!(not_contains_test(vec!["0".to_string()]));
    }

    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(vec![3,0,1]), 2);
        assert_eq!(missing_number(vec![0,1]), 2);
        assert_eq!(missing_number(vec![9,6,4,2,3,5,7,0,1]), 8);
    }

    #[test]
    fn test_find_disappeared_numbers() {
        assert_eq!(find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
        assert_eq!(find_disappeared_numbers(vec![1,1]), vec![2]);
    }

    #[test] #[ignore]
    fn test_str_without3a3b() {
        fn assert_test(a: i32, b: i32) {
            let s = str_without3a3b(1, 2);
            let counts = s.chars().counts();
            assert_eq!(s.len(), (a + b) as usize);
            assert_eq!(*counts.get(&'a').unwrap_or(&0), a as usize);
            assert_eq!(*counts.get(&'b').unwrap_or(&0), b as usize);
            assert!(!s.contains("aaa"));
            assert!(!s.contains("bbb"));
        }
        assert_test(1,2);
        assert_test(4,1);
    }

    #[test] #[ignore]
    fn test_di_string_match() {
        fn assert_test(s: String) {
            let nums = di_string_match(s.clone());
        }
        assert_test("IDID".to_string());
        assert_test("III".to_string());
        assert_test("DDI".to_string());
    }

    #[test]
    fn test_num_rabbits() {
        assert_eq!(num_rabbits(vec![1,1,2]), 5);
        assert_eq!(num_rabbits(vec![10,10,10]), 11);
        assert_eq!(num_rabbits(vec![3,3,3,3,3]), 8);
    }

    #[test] #[ignore]
    fn test_maximum_swap() {
        assert_eq!(maximum_swap(2736), 7236);
        assert_eq!(maximum_swap(9973), 9973);
    }


    #[test]
    #[ignore]
    fn test_lfu_cache() {
        // cnt(x) = the use counter for key x
        // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
        lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
        lfu.get(1);      // return 1
        dbg!(&lfu);
        // cache=[1,2], cnt(2)=1, cnt(1)=2
        lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
        // cache=[3,1], cnt(3)=1, cnt(1)=2
        lfu.get(2);      // return -1 (not found)
        lfu.get(3);      // return 3
        dbg!(&lfu);
        // cache=[3,1], cnt(3)=2, cnt(1)=2
        lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
        // cache=[4,3], cnt(4)=1, cnt(3)=2
        lfu.get(1);      // return -1 (not found)
        dbg!(lfu.get(1));
        dbg!(&lfu);
        lfu.get(3);      // return 3
        dbg!(&lfu);
        // cache=[3,4], cnt(4)=1, cnt(3)=3
        lfu.get(4);      // return 4
        // cache=[4,3], cnt(4)=2, cnt(3)=3

        let mut cache = LFUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        println!("{:?}", cache.get(1)); // returns Some(1)
        cache.put(3, 3);              // evicts key 2
        println!("{:?}", cache.get(2)); // returns None
        println!("{:?}", cache.get(3)); // returns Some(3)
        cache.put(4, 4);              // evicts key 1
        println!("{:?}", cache.get(1)); // returns None
        println!("{:?}", cache.get(3)); // returns Some(3)
        println!("{:?}", cache.get(4)); // returns Some(4)
    }

    #[test]
    #[ignore]
    fn test_random_blacklist() {
        let blacklist = vec![1, 3, 5, 7];

        let mut random_black = Solution::new(10, blacklist.clone());
        let nums = (0..30).map(|_| random_black.pick()).collect::<Vec<_>>();
        dbg!(&random_black);
        dbg!(&nums);

        let any_black = nums.iter().unique().any(|x| blacklist.contains(x));
        assert!(!any_black);
    }
}
