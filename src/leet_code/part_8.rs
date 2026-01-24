use std::mem;
use std::ops::Deref;
use itertools::Itertools;
use std::cmp::{max, min};

mod s143;
use s143::ListNode;



/// 51. N-Queens <br>
/// https://leetcode.com/problems/n-queens/
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n > 12 {
        panic!("too big n-queens");
    }
    if n == 1 {
        return vec![vec!['Q'.to_string()]];
    }
    if n <= 0 || n == 2 || n == 3 {
        return vec![];
    }

    fn solve(i: usize, side: usize, cols: &mut Vec<bool>, diag: &mut Vec<bool>, gaid: &mut Vec<bool>) -> Vec<Vec<String>> {
        let mut solutions = vec![];
        for j in 0..side {
            if cols[j] || diag[i + j] || gaid[(side - i - 1) + j] {
                continue;
            }

            let mut row = ".".repeat(side);
            row.replace_range(j..=j, "Q");

            if i+1 == side {
                let mut solution = Vec::with_capacity(side);
                solution.push(row);
                return vec![solution];
            }

            cols[j] = true;
            diag[i + j] = true;
            gaid[(side - i - 1) + j] = true;

            let recursive_solutions = solve(i+1, side, cols, diag, gaid);
            for mut solution in recursive_solutions.into_iter() {
                solution.push(row.clone());
                solutions.push(solution);
            }

            cols[j] = false;
            diag[i + j] = false;
            gaid[(side - i - 1) + j] = false;
        }

        solutions
    }

    let side = n as usize;
    let mut cols = vec![false; side];
    let mut diag = vec![false; side * 2 - 1];
    let mut gaid = vec![false; side * 2 - 1];

    solve(0, side, &mut cols, &mut diag, &mut gaid)
}

pub fn solve_n_queens_once(n: i32) -> Option<Vec<String>> {
    if n > 20 {
        panic!("too big n-queens");
    }
    if !n.is_positive() {
        return None::<Vec<String>>
    }

    fn solve(i: usize, side: usize, cols: &mut Vec<bool>, diag: &mut Vec<bool>, gaid: &mut Vec<bool>) -> Option<Vec<String>> {
        if i == side {
            return Some(Vec::with_capacity(side));
        }

        for j in 0..side {
            if cols[j] || diag[i + j] || gaid[(side - i - 1) + j] {
                continue;
            }

            cols[j] = true;
            diag[i + j] = true;
            gaid[(side - i - 1) + j] = true;

            if let Some(mut board) = solve(i+1, side, cols, diag, gaid) {
                let mut row = ".".repeat(side);
                row.replace_range(j..=j, "Q");

                board.push(row);
                return Some(board);
            }

            cols[j] = false;
            diag[i + j] = false;
            gaid[(side - i - 1) + j] = false;
        }

        None
    }

    let side = n as usize;
    let mut cols = vec![false; side];
    let mut diag = vec![false; side * 2 - 1];
    let mut gaid = vec![false; side * 2 - 1];

    solve(0, side, &mut cols, &mut diag, &mut gaid)
}

/// 52. N-Queens II <br>
/// https://leetcode.com/problems/n-queens-ii/
pub fn total_n_queens(n: i32) -> i32 {
    fn solve(i: usize, side: usize, cols: &mut Vec<bool>, diag: &mut Vec<bool>, gaid: &mut Vec<bool>) -> i32 {
        let mut solutions = 0;
        for j in 0..side {
            if cols[j] || diag[i + j] || gaid[(side - i - 1) + j] {
                continue;
            }

            if i+1 == side {
                return 1;
            }

            cols[j] = true;
            diag[i + j] = true;
            gaid[(side - i - 1) + j] = true;

            let recursive_solutions = solve(i+1, side, cols, diag, gaid);
            solutions += recursive_solutions;

            cols[j] = false;
            diag[i + j] = false;
            gaid[(side - i - 1) + j] = false;
        }

        solutions
    }

    const N: i32 = 10;
    match n {
        1 => 1,
        2..=3 => 0,
        4..N => {
            let side = n as usize;
            let mut cols = vec![false; side];
            let mut diag = vec![false; side * 2 - 1];
            let mut gaid = vec![false; side * 2 - 1];

            solve(0, side, &mut cols, &mut diag, &mut gaid)
        },
        N.. => panic!("too big n-queens"),
        _ => 0,
    }
}

pub const fn total_n_queens_fast(n: i32) -> i32 {
    match n {
        1 => 1,
        4 => 2,
        6 => 4,
        5 => 10,
        7 => 40,
        8 => 92,
        9 => 352,
        10.. => panic!("too big n-queens"),
        _ => 0,
    }
}

/// 79. Word Search <br>
/// https://leetcode.com/problems/word-search/
#[cold]
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    false
}

/// 143. Reorder List <br>
/// https://leetcode.com/problems/reorder-list/
#[cold]
pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
    if head.is_none() {
        return;
    }

    let arr = head.as_deref().unwrap().into_iter().collect::<Vec<_>>();
    let mut a = &mut Some(
        Box::new(
            ListNode::from_iter(
                head.as_deref_mut().unwrap().clone().into_iter().sorted(),
            )
        )
    );
}

/// 148. Sort List <br>
/// https://leetcode.com/problems/sort-list/
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    Some(
        Box::new(
            ListNode::from_iter(
                head.unwrap().deref().clone().into_iter().sorted(),
            )
        )
    )
}

/// 172. Factorial Trailing Zeroes <br>
/// https://leetcode.com/problems/factorial-trailing-zeroes/
pub fn trailing_zeroes(n: i32) -> i32 {
    let mut res = 0;
    let mut div = 5;

    while div <= n {
        res += n / div;
        div *= 5;
    }

    res
}

/// 75. Sort Colors <br>
/// https://leetcode.com/problems/sort-colors/
pub fn sort_colors(nums: &mut Vec<i32>) {
    let counts = nums.deref().iter().counts();
    let res = vec![
        std::iter::repeat_n(0, *counts.get(&0).unwrap_or(&0)),
        std::iter::repeat_n(1, *counts.get(&1).unwrap_or(&0)),
        std::iter::repeat_n(2, *counts.get(&2).unwrap_or(&0)),
    ].into_iter().kmerge().collect::<Vec<_>>();

    // mem::swap(nums, &mut res);
    let _ = mem::replace(nums, res);
}

/// 1005. Maximize Sum Of Array After K Negations <br>
/// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/description/
pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
    nums.sort_unstable();
    let mut last = i32::MAX;
    let mut sum = 0;

    for n in nums.into_iter() {
        if k > 0 && n.is_negative() {
            sum -= n;
            k -= 1;
        }
        else {
            sum += n;
        };
        last = last.min(n.abs());
    }

    if k > 0 && k % 2 == 1 {
        sum -= last * 2;
    }

    return sum;
}

/// 1013. Partition Array Into Three Parts With Equal Sum <br>
/// https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/
pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    let total_sum = arr.iter().sum::<i32>();
    let target_sum = total_sum / 3;
    if total_sum % 3 != 0 {
        return false;
    }

    let mut cnt = 0;
    let mut sum = 0;
    for n in arr.into_iter() {
        sum += n;
        if sum == target_sum {
            cnt += 1;
            sum = 0;
        }
    }

    cnt >= 3
}

/// 517. Super Washing Machines <br>
/// https://leetcode.com/problems/super-washing-machines/
pub fn find_min_moves(machines: Vec<i32>) -> i32 {
    let n = machines.len() as i32;
    let total_sum = machines.iter().sum::<i32>();
    let mean = total_sum / n;
    if total_sum % n != 0 {
        return -1;
    }

    let mut balance = 0;
    let mut ans = 0;
    for m in machines.into_iter() {
        let excess = m - mean;
        balance += excess;
        ans = max(ans, max(excess, balance.abs()));
    }
    ans
}

/// 416. Partition Equal Subset Sum <br>
/// https://leetcode.com/problems/partition-equal-subset-sum/
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum_total = nums.iter().sum::<i32>();
    let target = sum_total / 2;

    if sum_total % 2 == 1 {
        return false;
    }
    if nums.iter().max().unwrap() > &target {
        return false;
    }
    if nums.contains(&target) {
        return true;
    }

    let n = nums.len() + 1;
    let m = target as usize + 1;
    let mut dp = vec![vec![false; m]; n];
    // dp[i][j] represents whether we can achieve sum j using first i numbers
    // dp[i][j] = True if we can make sum j using nums[0...i-1]

    dp[0][0] = true;
    for i in 1..n {
        let num = nums[i-1] as usize;
        for j in 0..m {
            dp[i][j] = dp[i-1][j];
            if j >= num {
                dp[i][j] |= dp[i-1][j-num];
            }
        }
    }
    // dbg!(nums, target, &dp);
    dp[n-1][m-1]
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_n_queens() {
        // for i in -1..20 {
        //     dbg!(i, solve_n_queens_once(i));
        // }
        for i in -1..10 {
            let res = solve_n_queens(i);
            // dbg!(i, res.len(), res);
        }
    }

    #[test]
    fn test_total_n_queens() {
        for i in -1..=9 {
            let res1 = total_n_queens(i);
            let res2 = total_n_queens_fast(i);
            assert_eq!(res1, res2);
            // dbg!(res1, res2);
        }
    }

    #[test] #[ignore]
    fn test_exist() {
        let board = vec![
            "ABCE".chars().collect(),
            "SFCS".chars().collect(),
            "ADEE".chars().collect()
        ];
        assert_eq!(exist(board.clone(), "ABCCED".to_string()), true);
        assert_eq!(exist(board.clone(), "SEE".to_string()), true);
        assert_eq!(exist(board.clone(), "ABCB".to_string()), false);
    }

    #[test]
    fn test_list_node_143() {
        let arr1 = (1..6).collect::<Vec<i32>>();
        let list = ListNode::from_iter(arr1.clone());
        let arr2 = list.clone().into_iter().collect::<Vec<_>>();
        // dbg!(&arr1, &arr2, &list);
        assert_eq!(arr1, arr2);
    }

    #[test] #[ignore]
    fn test_reorder_list() {
        fn assert_test(nums: Vec<i32>, target: Vec<i32>) {
            let mut head = Some(
                Box::new(
                    ListNode::from_iter(
                        nums.clone(),
                    )
                )
            );

            reorder_list(&mut head);
            let res_nums = head.unwrap().into_iter().collect::<Vec<_>>();
            assert_eq!(res_nums, target);
        }

        assert_test(vec![1,2,3,4], vec![1,4,3,2]);
        assert_test(vec![1,2,3,4,5], vec![1,5,2,4,3]);
        assert_eq!(sort_list(None), None);
    }

    #[test]
    fn test_sort_list() {
        fn assert_test(nums: Vec<i32>) {
            let head = Some(
                Box::new(
                    ListNode::from_iter(
                        nums.clone(),
                    )
                )
            );

            let res = sort_list(head);
            let res_arr = res.unwrap().into_iter().collect::<Vec<_>>();
            let nums_sort = nums.iter().cloned().sorted().collect::<Vec<_>>();
            assert_eq!(res_arr, nums_sort);
        }

        assert_test(vec![4,2,1,3]);
        assert_test(vec![-1,5,3,4,0]);
        assert_eq!(sort_list(None), None);
    }

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(trailing_zeroes(0), 0);
        assert_eq!(trailing_zeroes(3), 0);
        assert_eq!(trailing_zeroes(5), 1);
        assert_eq!(trailing_zeroes(1000), 249);
    }

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2,0,2,1,1,0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);

        let mut nums = vec![2,0,1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }

    #[test]
    fn test_largest_sum_after_k_negations() {
        assert_eq!(largest_sum_after_k_negations(vec![4,2,3], 1), 5);
        assert_eq!(largest_sum_after_k_negations(vec![3,-1,0,2], 3), 6);
        assert_eq!(largest_sum_after_k_negations(vec![2,-3,-1,5,-4], 2), 13);
    }

    #[test]
    fn test_can_three_parts_equal_sum() {
        assert_eq!(can_three_parts_equal_sum(vec![0,2,1,-6,6,-7,9,1,2,0,1]), true);
        assert_eq!(can_three_parts_equal_sum(vec![0,2,1,-6,6,7,9,-1,2,0,1]), false);
        assert_eq!(can_three_parts_equal_sum(vec![3,3,6,5,-2,2,5,1,-9,4]), true);
        assert_eq!(can_three_parts_equal_sum(vec![0,0,0,0]), true);
    }

    #[test]
    fn test_find_min_moves() {
        assert_eq!(find_min_moves(vec![1,0,5]), 3);
        assert_eq!(find_min_moves(vec![0,3,0]), 2);
        assert_eq!(find_min_moves(vec![0,2,0]), -1);
        assert_eq!(find_min_moves(vec![4,0,0,4]), 2);
    }

    #[test]
    fn test_can_partition() {
        assert_eq!(can_partition(vec![1,5,11,5]), true);
        assert_eq!(can_partition(vec![1,2,3,5]), false);
        assert_eq!(can_partition(vec![2,2,3,5]), false);
        assert_eq!(can_partition(vec![3,3,3,4,5]), true);
    }
}
