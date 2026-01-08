use std::cmp::{max, min};
use itertools::Itertools;

mod s460;
use s460::LFUCache;



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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_happiness_sum() {
        assert_eq!(maximum_happiness_sum(vec![1,2,3], 2), 4);
        assert_eq!(maximum_happiness_sum(vec![1,1,1,1], 2), 1);
        assert_eq!(maximum_happiness_sum(vec![2,3,4,5], 1), 5);
    }

    #[test]
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
}
