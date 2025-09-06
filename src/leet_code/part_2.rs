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
pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    0
}



///  <br>
///
pub fn solution(n: i32) -> Vec<i32> {
    vec!()
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
        // smallest_range(vec![vec![1, 6, 2], vec![3, 1, 3], vec![4, 1, 4]]);

        assert_eq!(smallest_range(vec![vec![4,10,15,24,26], vec![0,9,12,20], vec![5,18,22,30]]), vec![20,24]);
        assert_eq!(smallest_range(vec![vec![1,2,3], vec![1,2,3], vec![1,2,3]]), vec![1,1]);
    }
}
