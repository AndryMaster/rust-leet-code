use std::cmp::max;
use itertools::Itertools;

mod s421;
use s421::Trie;

mod s83;
use s83::ListNode;



/// 421. Maximum XOR of Two Numbers in an Array <br>
/// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut trie = Trie::new();
    let mut max_xor = 0;

    for n in nums {
        trie.insert(n);
        max_xor = max(max_xor, trie.search(n));
    }

    max_xor
}

/// 83. Remove Duplicates from Sorted List <br>
/// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut items = head?.to_vec();
    items.dedup();

    Some(Box::new(ListNode::from_vec(items)?))
}

/// 93. Restore IP Addresses <br>
/// https://leetcode.com/problems/restore-ip-addresses/
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    todo!()
}

/// 77. Combinations <br>
/// https://leetcode.com/problems/combinations/
pub fn combine_old(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn comb_rec(n: i32, k: i32, tmp: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if k == 0 {
            return tmp.to_owned();
        }

        let mut result: Vec<Vec<i32>> = vec![];

        for comb in tmp.iter() {
            let next = *comb.last().unwrap() + 1;
            for i in next..=n {
                let mut new = comb.clone();
                new.push(i);
                result.push(new);
            }
        }

        comb_rec(n, k - 1, &result)
    }

    let nums: Vec<Vec<i32>> = (1..=n).map(|x| {vec![x]}).collect();
    let res = comb_rec(n, k-1, &nums);
    res
}
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    // println!("n: {}, k: {}", n, k);
    if k == 0 {
        return vec![vec![]];
    }
    let mut res = vec![];
    for i in k..=n {
        for mut pre in combine(i-1, k-1) {  // Self::combine
            pre.push(i);
            res.push(pre);
        }
    }
    res.sort();
    res
}

/// 40. Combination Sum II <br>
/// https://leetcode.com/problems/combination-sum-ii/
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        if target <= 0 {
            return result;
        }

        for (pos, &n) in candidates.iter().enumerate().dedup_by(|&x, &y| x.1 == y.1) {
            if n > target {
                break;
            }
            if n == target {
                result.push(vec![n]);
            }
            else {
                for mut res in backtrack(candidates[(pos + 1)..].to_vec(), target - n) {
                    res.push(n);
                    result.push(res);
                }
            }
        }

        result
    }

    candidates.sort();
    backtrack(candidates, target)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_xor() {
        assert_eq!(find_maximum_xor(vec![3,10,5,25,2,8]), 28);
        assert_eq!(find_maximum_xor(vec![14,70,53,83,49,91,36,80,92,51,66,70]), 127);
    }

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(ListNode::from_vec(vec![]), None);
        assert_eq!(ListNode::from_vec(vec![1, 2, 3, 4, 5]).unwrap().to_vec(), vec![1, 2, 3, 4, 5]);
        assert_eq!(delete_duplicates(None), None);
        assert_eq!(delete_duplicates(Some(Box::new(ListNode::from_vec(vec![1, 1, 2, 2, 3]).unwrap()))).unwrap().to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_restore_ip_addresses() {

    }

    #[test]
    fn test_combine() {
        assert_eq!(combine(4, 2), [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]);
        assert_eq!(combine(1, 1), [[1]]);
    }

    #[test]
    fn test_combination_sum2() {
        assert_eq!(combination_sum2(vec![10,1,2,7,6,1,5], 8), vec![vec![6,1,1],vec![5,2,1],vec![7,1],vec![6,2]]);
        assert_eq!(combination_sum2(vec![2,5,2,1,2], 5), vec![vec![2,2,1],vec![5]]);
    }
}
