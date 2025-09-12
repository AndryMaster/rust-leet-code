use std::cmp::{max, min};
use std::collections::*;

mod s632;
use s632::SmRange;


/// 89. Gray Code <br>
/// https://leetcode.com/problems/gray-code/
pub fn gray_code(n: i32) -> Vec<i32> {
    fn bin_2_gray(n: i32) -> i32 {
        n ^ (n >> 1)
    }

    (0..(1 << n)).map(|num| bin_2_gray(num)).collect::<Vec<_>>()
}

/// 461. Hamming Distance <br>
/// https://leetcode.com/problems/hamming-distance/
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

/// 632. Smallest Range Covering Elements from K Lists <br>
/// https://leetcode.com/problems/max-area-of-island/
pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut mini;
    let mut maxi = i32::MIN;
    let mut res_min = 0;
    let mut res_max = 0;
    let mut result = i32::MAX;
    let mut min_const: Option<i32> = None;


    let mut queue = BinaryHeap::new();

    for vec in nums {
        queue.push(SmRange::new(vec));
    }

    for range in queue.iter() {
        maxi = max(maxi, range.get());
    }

    while let Some(mut range) = queue.pop() {
        // println!("{})\t {:?}", queue.len(), range);
        mini = min_const.unwrap_or(range.get());

        let new_res = maxi - mini;
        if new_res < result {
            result = new_res;
            res_min = mini;
            res_max = maxi;
        }

        if range.has_next() {
            range.next();
            maxi = max(maxi, range.get());
            queue.push(range);
        }
        else {
            min_const = Some(min(range.get(), min_const.unwrap_or(i32::MAX)));
        }
    }

    vec!(res_min, res_max)
}

/// 695. Max Area of Island <br>
/// https://leetcode.com/problems/max-area-of-island/
pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        let item = grid.get(y).and_then(|row| row.get(x)).unwrap_or(&0).clone();
        if item == 0 {
            return 0;
        }

        grid[y][x] = 0;

        return 1 +
            dfs(grid, x, y + 1) +
            dfs(grid, x + 1, y) +
            dfs(grid, x, y.checked_sub(1).unwrap_or(0)) +
            dfs(grid, x.checked_sub(1).unwrap_or(0), y);
    }

    let mut result = 0;
    for row_y in grid.clone().iter().enumerate() {
        for elem_x in row_y.1.iter().enumerate() {
            let island_count = dfs(grid.as_mut(), elem_x.0, row_y.0);
            result = max(result, island_count);
            // println!("({}, {}) - {} - {}", row_y.0, elem_x.0, island_count, result);
        }
    }

    result
}

/// 200. Number of Islands <br>
/// https://leetcode.com/problems/number-of-islands/
pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> i32 {
        let item = grid.get(y).and_then(|row| row.get(x)).unwrap_or(&'0').clone();
        if item == '0' {
            return 0;
        }

        grid[y][x] = '0';

        return 1 +
            dfs(grid, x, y + 1) +
            dfs(grid, x + 1, y) +
            dfs(grid, x, y.checked_sub(1).unwrap_or(0)) +
            dfs(grid, x.checked_sub(1).unwrap_or(0), y);
    }

    let mut count = 0;
    for row_y in grid.clone().iter().enumerate() {
        for elem_x in row_y.1.iter().enumerate() {
            let island = dfs(grid.as_mut(), elem_x.0, row_y.0);
            if island > 0 {
                count += 1;
            }   // println!("({}, {}) - {} - {}", row_y.0, elem_x.0, island_count, result);
        }
    }

    count
}

/// 405. Convert a Number to Hexadecimal <br>
/// https://leetcode.com/problems/convert-a-number-to-hexadecimal/
pub fn to_hex(num: i32) -> String {
    // format!("{:x}", num)

    if num == 0 {
        return String::from("0");
    }

    let mut num = num.cast_unsigned();
    let mut res = String::with_capacity(8);
    while num > 0 {
        let x = (num & 0xf) as u8;
        num >>= 4;
        println!("{x}");
        res.push(match x {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => panic!("x is too large"),
        });
    }

    res.chars().rev().collect()
}

/// 136. Single Number <br>
/// https://leetcode.com/problems/single-number/
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|acc, x| acc ^ x).unwrap_or(0)
}

/// 76. Minimum Window Substring <br>
/// https://leetcode.com/problems/minimum-window-substring/
pub fn min_window(s: String, t: String) -> String {
    fn check_all(counts: &HashMap<char, u16>, target: &HashMap<char, u16>) -> bool {
        for (k, v) in target.iter() {
            if counts.get(k).unwrap_or(&0) < v{
                return false;
            }
        }
        true
    }

    let text = s.chars().collect::<Vec<_>>();

    let mut result: Option<&str> = None;
    let mut counts: HashMap<char, u16> = HashMap::new();
    let     target: HashMap<char, u16> = t.chars()
        .fold(HashMap::new(), |mut target, c| {
            *target.entry(c).or_insert(0) += 1;
            target
        });

    let (mut left, mut right) = (0, 0);

    while right < text.len() {
        let c = text[right];
        *counts.entry(c).or_insert(0) += 1;

        while check_all(&counts, &target) {
            let new_result = &s[left..=right];
            if result.is_none() || new_result.len() < result.unwrap().len() {
                result = Some(new_result);
            }

            let c = text[left];
            counts.entry(c).and_modify(|e| { *e -= 1 });
            left += 1;
        }

        right += 1;
    }
    
    result.unwrap_or("").to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(1), vec![0,1]);
        assert_eq!(gray_code(2), vec![0,1,3,2]);
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(1, 4), 2);
        assert_eq!(hamming_distance(1, 3), 1);
    }

    #[test]
    fn test_smallest_range() {
        assert_eq!(smallest_range(vec![vec![4,10,15,24,26], vec![0,9,12,20], vec![5,18,22,30]]), vec![20,24]);
        assert_eq!(smallest_range(vec![vec![1,2,3], vec![1,2,3], vec![1,2,3]]), vec![1,1]);
    }

    #[test]
    fn test_max_area_of_island() {
        let grid_1 = vec![
            vec![0,0,1,0,0,0,0,1,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,1,1,0,1,0,0,0,0,0,0,0,0],
            vec![0,1,0,0,1,1,0,0,1,0,1,0,0],
            vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
            vec![0,0,0,0,0,0,0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,0,0,0,0]];

        let grid_2 = vec![
            vec![1,0,1],
            vec![0,1,1]];

        assert_eq!(max_area_of_island(grid_1), 6);
        assert_eq!(max_area_of_island(grid_2), 3);
    }

    #[test]
    fn test_num_islands() {
        let grid_1 = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ];
        let grid_2 = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1']
        ];
        assert_eq!(num_islands(grid_1), 1);
        assert_eq!(num_islands(grid_2), 3);
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex(26), "1a");
        assert_eq!(to_hex(-1), "ffffffff");
        assert_eq!(to_hex(0), "0");
    }

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(vec![2,2,1]), 1);
        assert_eq!(single_number(vec![4,1,2,1,2]), 4);
        assert_eq!(single_number(vec![1]), 1);
    }

    #[test]
    fn test_min_window() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a".to_string());
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "".to_string());
    }
}
