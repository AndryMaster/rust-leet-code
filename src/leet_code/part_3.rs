use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;



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


/// 42. Trapping Rain Water <br>
/// https://leetcode.com/problems/trapping-rain-water-ii/
pub fn trap(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut result = 0;
    let mut level = 0;
    // let mut dbg = height.clone();

    while left < right {
        // LEFT
        if height[left] <= height[right] {  // dbg[left] = max(height[left], level);
            result += max(height[left], level) - height[left];
            level = max(level, min(height[left], height[right]));
            left += 1;
        }
        // RIGHT
        else {  // dbg[right] = max(height[right], level);
            result += max(height[right], level) - height[right];
            level = max(level, min(height[left], height[right]));
            right -= 1;
        }
        // println!("left: {} right: -{} res: {} level: {}\n{:?}\n{:?}",
        //          left, height.len() - right, result, level, height, dbg);
    }   // println!("{:?}\n{:?}", height, dbg);

    result
}


/// 407. Trapping Rain Water II <br>
/// https://leetcode.com/problems/trapping-rain-water-ii/
pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (height_map.len(), height_map[0].len());
    let mut visited = vec![vec![false; cols]; rows];
    let mut min_heap = BinaryHeap::new();

    for i in 0..rows {
        for j in 0..cols {
            if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                min_heap.push(Reverse((height_map[i][j], i, j)));
                visited[i][j] = true;
            }
        }
    }

    let mut total_water = 0;

    while !min_heap.is_empty() {
        let popped = min_heap.pop().unwrap();
        let (height_cur, row, col) = popped.0;

        for (drow, dcol) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_row = row as i32 + drow;
            let next_col = col as i32 + dcol;

            if (0..rows as i32).contains(&next_row)
                && (0..cols as i32).contains(&next_col) {
                let next_row = next_row as usize;
                let next_col = next_col as usize;

                if !visited[next_row][next_col] {
                    let height_next = height_map[next_row][next_col];

                    total_water += max(0, height_cur - height_next);

                    visited[next_row][next_col] = true;

                    min_heap.push(Reverse((
                        max(height_cur, height_next),
                        next_row,
                        next_col
                    )));
                }
            }
        }
    }

    total_water
}
// pub fn trap_rain_water_incorrect(height_map: Vec<Vec<i32>>) -> i32 {
//     fn trap_line(height: &Vec<i32>) -> Vec<i32> {
//         let (mut left, mut right) = (0, height.len() - 1);
//         let mut level = 0;
//         let mut result = height.clone();
//
//         while left < right {
//             if height[left] <= height[right] {  // LEFT
//                 result[left] = max(result[left], level);
//                 level = max(level, min(height[left], height[right]));
//                 left += 1;
//             }
//             else {  // RIGHT
//                 result[right] = max(result[right], level);
//                 level = max(level, min(height[left], height[right]));
//                 right -= 1;
//             }
//         }
//
//         result
//     }
//
//     let print_map = |map: &Vec<Vec<i32>>| {
//         for row in map {
//             for elem in row {print!("{:>2} ", elem);} println!();
//         }
//     };
//     let (rows, cols) = (height_map.len(), height_map[0].len());
//     if rows < 3 || cols < 3 {
//         return 0;
//     }
//
//     let mut result_map = Vec::new();
//
//     // ROWS
//     result_map.push(height_map.first().unwrap().clone());
//     for row in &height_map[1..(rows-1)] {
//         result_map.push(trap_line(row));
//     }
//     result_map.push(height_map.last().unwrap().clone());
//
//     println!("Result map (rows):");
//     print_map(&result_map);
//
//     // COLUMNS
//     for col_idx in 1..(cols-1) {
//         let column = trap_line(
//             &height_map
//                 .iter()
//                 .map(|row| row[col_idx])
//                 .collect::<Vec<i32>>()
//         );
//
//         for (row_idx, elem) in column.into_iter().enumerate() {
//             result_map[row_idx][col_idx] = max(elem, result_map[row_idx][col_idx]);
//         }
//     }
//
//     println!("Height map:");
//     print_map(&height_map);
//     println!("Result map (all):");
//     print_map(&result_map);
//
//     let result =
//         result_map.into_iter().map(|row| row.into_iter().sum::<i32>()).sum::<i32>() -
//         height_map.into_iter().map(|row| row.into_iter().sum::<i32>()).sum::<i32>();
//     result
// }

/// 78. Subsets <br>
/// https://leetcode.com/problems/subsets/
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let count = 1 << nums.len();
    let mut result = Vec::with_capacity(count);

    for key in 0..count {
        let mut subset = Vec::with_capacity(key.count_ones() as usize);
        for i in 0..nums.len() {
            if (key & (1 << i)) > 0 {
                subset.push(nums[i]);
            }
        }
        result.push(subset);
    }

    result
}

/// 393. UTF-8 Validation <br>
/// https://leetcode.com/problems/utf-8-validation/
pub fn valid_utf8(data: Vec<i32>) -> bool {
    // for d in &data { print!("{:0>8b} ", *d as u8) } println!();

    let mut count = 0;
    for char in data.into_iter().map(|n| n as u8) {
        let ones = char.leading_ones() as i32;
        if count == 0 {
            count = ones;
            if !(ones <= 4 && ones != 1) {
                return false;
            }
        } else if ones != 1 {
            return false;
        }

        count = max(count - 1, 0);
    }

    count == 0
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

    #[test]
    fn test_trap() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(trap(vec![4,2,0,3,2,5]), 9);
        assert_eq!(trap(vec![1,2,3]), 0);
        assert_eq!(trap(vec![0]), 0);
    }

    #[test]
    fn test_trap_rain_water() {
        assert_eq!(trap_rain_water(vec![
            vec![1,4,3,1,3,2],
            vec![3,2,1,3,2,4],
            vec![2,3,3,2,3,1]]
        ), 4);
        assert_eq!(trap_rain_water(vec![
            vec![3,3,3,3,3],
            vec![3,2,2,2,3],
            vec![3,2,1,2,3],
            vec![3,2,2,2,3],
            vec![3,3,3,3,3]]
        ), 10);
        assert_eq!(trap_rain_water(vec![
            vec![12,13, 1,12],
            vec![13, 4,13,12],
            vec![13, 8,10,12],
            vec![12,13,12,12],
            vec![13,13,13,13]]
        ), 14);
    }

    #[test]
    fn test_subsets() {
        assert_eq!(subsets(vec![1,2,3]), vec![vec![], vec![1], vec![2], vec![1,2], vec![3], vec![1,3], vec![2,3], vec![1,2,3]]);
    }

    #[test]
    fn test_valid_utf8() {
        assert_eq!(valid_utf8(vec![197, 130, 1]), true);
        assert_eq!(valid_utf8(vec![235, 140, 4]), false);
        assert_eq!(valid_utf8(vec![250,145,145,145,145]), false);
    }
}