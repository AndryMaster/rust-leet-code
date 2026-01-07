use std::cmp::{max, min};
use itertools::Itertools;

mod s2166;
use s2166::Bitset;



/// 22. Generate Parentheses <br>
/// https://leetcode.com/problems/generate-parentheses/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generate_parenthesis(s: String, opened: i32, need: i32) -> Vec<String> {
        if need == 0 {
            return vec![s];
        }

        let mut result = vec![];
        if opened < need {
            result.extend(generate_parenthesis(s.clone() + "(", opened + 1, need - 1));
        }
        if opened > 0 {
            result.extend(generate_parenthesis(s + ")", opened - 1, need - 1));
        }
        result
    }

    generate_parenthesis(String::new(), 0, n * 2)
}

/// 216. Combination Sum III <br>
/// https://leetcode.com/problems/combination-sum-iii/
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn combination_sum3(k: i32, n: i32, mut state: Vec<i32>) -> Vec<Vec<i32>> {
        if k == 0 {
            if n == 0 {
                return vec![state];
            }
            return vec![];
        }

        let mut states = vec![];

        // for i in (*state.last().unwrap_or(&0)+1)..=min(n/k, 9)
        let last = *state.last().unwrap_or(&0);
        for i in 1..=9 {
            if last < i && i <= n {
                state.push(i);
                states.extend(combination_sum3(k - 1, n - i, state.clone()));
                state.pop();
            }
        }

        states
    }

    combination_sum3(k, n, vec![])
}

/// 1239. Maximum Length of a Concatenated String with Unique Characters <br>
/// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
pub fn max_length(arr: Vec<String>) -> i32 {
    fn str_as_set(s: &str) -> Option<u32> {
        let mut set = 0u32;
        for c in s.chars().filter(|c| c.is_ascii_lowercase()) {
            let shift = c as u32 - 'a' as u32;
            let pos = 1 << shift;
            if set & pos != 0 {
                return None;
            }
            set |= pos;
        }
        Some(set)
    }
    fn union_sets(sets: Vec<u32>) -> Option<u32> {
        let mut res = 0u32;
        for set in sets {
            if res & set != 0 {
                return None;
            }
            res |= set;
        }
        Some(res)
    }

    return arr
        .iter()
        .map(|str| str_as_set(str))
        .filter(|set| set.is_some())
        .map(|set| set.unwrap())
        .powerset()
        .map(|set| union_sets(set))
        .filter(|set| set.is_some())
        .map(|set| set.unwrap())
        .map(|set| set.count_ones())
        .max()
        .unwrap_or(0) as i32;
}

/// 797. All Paths From Source to Target <br>
/// https://leetcode.com/problems/all-paths-from-source-to-target/
pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn get_all_paths(graph: &Vec<Vec<i32>>, path: Vec<i32>) -> Vec<Vec<i32>> {
        let target = (graph.len() - 1) as i32;
        let pos = *path.last().unwrap();

        if pos == target {
            return vec![path];
        }

        let mut paths = vec![];
        for &a in graph[pos as usize].iter() {
            let mut new_path = path.clone();
            new_path.push(a);
            paths.extend(get_all_paths(graph, new_path));
        }
        return paths;
    }

    return get_all_paths(&graph, vec![0]);
}

/// 6. Zigzag Conversion <br>
/// https://leetcode.com/problems/zigzag-conversion/
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        return s;
    }

    let mut res = String::with_capacity(s.len());
    let mut rows = vec![vec![]; num_rows as usize];

    let delta = (num_rows - 1) * 2;
    for i in 0..delta {
        rows[min(i, (delta - i)) as usize].push(i)
    }

    for row in rows.iter() {
        let mut step = 0;
        'row: loop {
            for idx in row {
                let pos = (step * delta + idx) as usize;
                if pos >= s.len() {
                    break 'row;
                }
                res.push(s.chars().nth(pos).unwrap());
            }
            step += 1;
        }
    }

    res
}

/// 397. Integer Replacement <br>
/// https://leetcode.com/problems/integer-replacement/
pub fn integer_replacement(mut n: i32) -> i32 {
    if n == i32::MAX {
        return 32;
    }
    let mut cnt = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        }
        else if n != 3 && n.trailing_ones() >= 2 {
            n += 1;
        }
        else {
            n -= 1;
        }

        cnt += 1;
        // dbg!(format!("{n:0b} - {cnt} - {n}"));
    }
    cnt
}

/// 91. Decode Ways <br>
/// https://leetcode.com/problems/decode-ways/
pub fn num_decodings(s: String) -> i32 {
    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1;

    let mut last_num = ' ';

    for (i, num) in s.chars().enumerate().map(|(i, c)| (i + 1, c)) {
        if ('1'..='9').contains(&num) {
            dp[i] += dp[i-1];
        }
        if i > 1 && (last_num == '1' || last_num == '2' && ('0'..='6').contains(&num)) {
            dp[i] += dp[i-2];
        }
        last_num = num;
    }

    // match num {
    //     '1'..='9' => dp[i] += dp[i-1],
    //     _ => (),
    // }
    // match last {
    //     '1' => dp[i] += dp[i-2],
    //     '2' if ('0'..='6').contains(&num) => dp[i] += dp[i-2],
    //     _ => (),
    // }

    // dbg!(s, &dp);
    return *dp.last().unwrap();
}

/// 396. Rotate Function <br>
/// https://leetcode.com/problems/rotate-function/
pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len() as i32;
    let sum_nums = nums.iter().sum::<i32>();

    let mut sum_cur = nums.iter().enumerate().map(|(i, &num)| num * (i as i32)).sum();
    let mut sum_max = sum_cur;

    for i in 0..(n-1) {
        sum_cur += n * nums[i as usize] - sum_nums;
        sum_max = max(sum_max, sum_cur);
    }

    sum_max
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(generate_parenthesis(1), ["()"]);
        assert_eq!(generate_parenthesis(3), ["((()))","(()())","(())()","()(())","()()()"]);
    }

    #[test]
    fn test_combination_sum3() {
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(combination_sum3(3, 7), [[1, 2, 4]]);
        assert_eq!(combination_sum3(3, 9), [[1, 2, 6], [1, 3, 5], [2, 3, 4]]);
        assert_eq!(combination_sum3(2, 18), empty);
    }

    #[test]
    fn test_max_length() {
        assert_eq!(max_length(Vec::from(["un".to_string(), "iq".to_string(), "ue".to_string()])), 4);
        assert_eq!(max_length(Vec::from(["cha".to_string(),"r".to_string(),"act".to_string(),"ers".to_string()])), 6);
        assert_eq!(max_length(Vec::from(["abcdefghijklmnopqrstuvwxyz".to_string()])), 26);
        assert_eq!(max_length(Vec::from(["aa".to_string(),"bb".to_string()])), 0);
        assert_eq!(max_length(Vec::from(["a".to_string(),"b".to_string(),"c".to_string()])), 3);
    }

    #[test]
    fn test_all_paths_source_target() {
        assert_eq!(all_paths_source_target(vec![vec![1,2],vec![3],vec![3],vec![]]),
                   vec![vec![0,1,3],vec![0,2,3]]);
        assert_eq!(all_paths_source_target(vec![vec![4,3,1],vec![3,2,4],vec![3],vec![4],vec![]]),
                   vec![vec![0,4],vec![0,3,4],vec![0,1,3,4],vec![0,1,2,3,4],vec![0,1,4]]);
    }

    #[test]
    fn test_bitset() {
        let mut bs = Bitset::new(5);              // bitset = "00000".
        bs.fix(3);                                   // the value at idx = 3 is updated to 1, so bitset = "00010".
        // println!("{}", bs.to_string());
        bs.fix(1);                                   // the value at idx = 1 is updated to 1, so bitset = "01010".
        // println!("{}", bs.to_string());
        bs.flip();                                       // the value of each bit is flipped, so bitset = "10101".
        // println!("{}", bs.to_string());
        assert_eq!(bs.all(), false);                     // return False, as not all values of the bitset are 1.
        bs.unfix(0);                                 // the value at idx = 0 is updated to 0, so bitset = "00101".
        // println!("{}", bs.to_string());
        bs.flip();                                       // the value of each bit is flipped, so bitset = "11010".
        // println!("{}", bs.to_string());
        assert_eq!(bs.one(), true);                      // return True, as there is at least 1 index with value 1.
        bs.unfix(0);                                 // the value at idx = 0 is updated to 0, so bitset = "01010".
        // println!("{}", bs.to_string());
        assert_eq!(bs.count(), 2);                       // return 2, as there are 2 bits with value 1.
        assert_eq!(bs.to_string(), "01010".to_string()); // return "01010", which is the composition of bitset.
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert(String::from("PAYPALISHIRING"), 3), String::from("PAHNAPLSIIGYIR"));
        assert_eq!(convert(String::from("PAYPALISHIRING"), 4), String::from("PINALSIGYAHRPI"));
        assert_eq!(convert(String::from("A"), 1), String::from("A"));
    }

    #[test]
    fn test_integer_replacement() {
        assert_eq!(integer_replacement(8), 3);
        assert_eq!(integer_replacement(7), 4);
        assert_eq!(integer_replacement(4), 2);
        assert_eq!(integer_replacement(100000000), 31);
        assert_eq!(integer_replacement(2147483647), 32);
    }

    #[test]
    fn test_num_decodings() {
        assert_eq!(num_decodings("12".to_string()), 2);
        assert_eq!(num_decodings("226".to_string()), 3);
        assert_eq!(num_decodings("06".to_string()), 0);
        assert_eq!(num_decodings("11106".to_string()), 2);
    }

    #[test]
    fn test_max_rotate_function() {
        assert_eq!(max_rotate_function(vec![4,3,2,6,5,9,12,14,13,1,0,7,4,3,7,8,5,3,12,10,9]), 1667);
        assert_eq!(max_rotate_function(vec![4,3,2,6]), 26);
        assert_eq!(max_rotate_function(vec![100]), 0);
    }
}
