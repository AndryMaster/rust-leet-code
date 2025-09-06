use itertools::Itertools;
use std::cmp::max;
use std::collections::*;

mod s328;
mod s401;

use s328::ListNode;
use s401::BitTime;

/// 1957. Delete Characters to Make Fancy String <br>
/// https://leetcode.com/problems/delete-characters-to-make-fancy-string/
pub fn make_fancy_string(mut string: String) -> String {
    let mut cnt = 0;
    let mut prev = '.';

    string.retain(|ch| {
        if ch == prev {
            cnt += 1
        } else {
            cnt = 1
        }
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

                if nums[left - 1] == num {
                    break;
                }
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
        } else {
            pos += n / len;
            n %= len;

            dbg(n, pos, len);
            break;
        }

        pos *= 10;
        len += 1;
    }

    let result = pos
        .to_string()
        .chars()
        .nth(n as usize)
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;
    result
}

/// 401. Binary Watch <br>
/// https://leetcode.com/problems/binary-watch/
pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let bit_times = vec![
        BitTime { hh: 0, mm: 1 },
        BitTime { hh: 0, mm: 2 },
        BitTime { hh: 0, mm: 4 },
        BitTime { hh: 0, mm: 8 },
        BitTime { hh: 0, mm: 16 },
        BitTime { hh: 0, mm: 32 },
        BitTime { hh: 1, mm: 0 },
        BitTime { hh: 2, mm: 0 },
        BitTime { hh: 4, mm: 0 },
        BitTime { hh: 8, mm: 0 },
    ];

    let mut result = vec![];

    for combination in bit_times.iter().combinations(turned_on as usize) {
        let mut time_sum = BitTime { hh: 0, mm: 0 };
        for time in combination {
            time_sum = time_sum + *time;
        }

        if time_sum.as_string().is_some() {
            result.push(time_sum.as_string().unwrap());
        }
    }

    result
}

/// 328. Odd Even Linked List <br>
/// https://leetcode.com/problems/odd-even-linked-list/
pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Length 0-1
    if head.as_ref()?.next.is_none() {
        return head;
    }

    let mut even_head = head.as_mut().unwrap().next.take();

    let mut odd_tail = head.as_mut().unwrap();
    let mut even_tail = even_head.as_mut().unwrap();

    // Iterate while there are next nodes to process
    loop {
        // Link next odd: even_tail.next becomes the next odd
        if let Some(next_odd) = even_tail.next.take() {
            odd_tail.next = Some(next_odd);
            odd_tail = odd_tail.next.as_mut().unwrap();
        } else {
            break;
        }

        // Link next even: odd_tail.next becomes the next even
        if let Some(next_even) = odd_tail.next.take() {
            even_tail.next = Some(next_even);
            even_tail = even_tail.next.as_mut().unwrap();
        } else {
            break;
        }
    }

    odd_tail.next = even_head;

    head
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fancy_string() {
        assert_eq!(
            make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(
            make_fancy_string("aaabaaaa".to_string()),
            "aabaa".to_string()
        );
        assert_eq!(make_fancy_string("aab".to_string()), "aab".to_string());
    }

    #[test]
    fn test_maximum_unique_subarray() {
        // assert_eq!(maximum_unique_subarray(vec![1,2,3]), 6);
        assert_eq!(maximum_unique_subarray(vec![1, 2, 1]), 3);
        assert_eq!(maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
        assert_eq!(maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
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

    #[test]
    fn test_read_binary_watch() {
        assert_eq!(
            read_binary_watch(1),
            vec![
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"
            ]
        );
        assert_eq!(read_binary_watch(9), Vec::<String>::new());
    }

    #[test]
    fn test_odd_even_list() {
        assert_eq!(odd_even_list(None), None);

        let node_1_box = Some(Box::new(ListNode::new(123).clone()));
        assert_eq!(odd_even_list(node_1_box.clone()), node_1_box);

        let mut node_1 = ListNode::new(1);
        let mut node_2 = ListNode::new(2);
        let mut node_3 = ListNode::new(3);
        let mut node_4 = ListNode::new(4);
        node_3.next = Some(Box::new(node_4));
        node_2.next = Some(Box::new(node_3));
        node_1.next = Some(Box::new(node_2));

        let mut test_1 = ListNode::new(1);
        let mut test_2 = ListNode::new(2);
        let mut test_3 = ListNode::new(3);
        let mut test_4 = ListNode::new(4);
        test_2.next = Some(Box::new(test_4));
        test_3.next = Some(Box::new(test_2));
        test_1.next = Some(Box::new(test_3));

        let node_1 = *odd_even_list(Some(Box::new(node_1))).unwrap();

        assert_eq!(node_1.collect(), test_1.collect());
    }
}
