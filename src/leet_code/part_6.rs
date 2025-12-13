use itertools::Itertools;



/// 22. Generate Parentheses <br>
/// https://leetcode.com/problems/generate-parentheses/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generate_parenthesis(s: String, opened: i32, need: i32) -> Vec<String> {
        if need == 0 {
            return vec![s];
        }

        let mut result = vec![];
        if opened < need {
            result.extend(generate_parenthesis(s.clone()+"(", opened+1, need-1));
        }
        if opened > 0 {
            result.extend(generate_parenthesis(s+")", opened-1, need-1));
        }
        result
    }

    generate_parenthesis(String::new(), 0, n*2)
}

/// 216. Combination Sum III <br>
/// https://leetcode.com/problems/combination-sum-iii/
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn combination_sum3(k: i32, n: i32, mut state: Vec<i32>) -> Vec<Vec<i32>> {
        if k == 0 {
            if n == 0 {
                return vec![state];
            }   return vec![];
        }

        let mut states = vec![];

        // for i in (*state.last().unwrap_or(&0)+1)..=min(n/k, 9)
        let last = *state.last().unwrap_or(&0);
        for i in 1..=9 {
            if last < i && i <= n {
                state.push(i);
                states.extend(combination_sum3(k-1, n-i, state.clone()));
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


    // let arr: Vec<u32> = arr
    //     .iter()
    //     .map(|s| str_as_set(s))
    //     .filter(|n| n.is_some())
    //     .map(|n| n.unwrap())
    //     .collect();
    //
    // let res = arr
    //     .iter().powerset()
    //     .map(|set| { union_sets(set)})
    //     .filter(|set| set.is_some())
    //     .map(|set| set.unwrap().count_ones())
    //     .max().unwrap_or(0);
    //
    // return res as i32

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
        .max().unwrap_or(0) as i32;
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
        assert_eq!(combination_sum3(3, 7), [[1,2,4]]);
        assert_eq!(combination_sum3(3, 9), [[1,2,6],[1,3,5],[2,3,4]]);
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
}
