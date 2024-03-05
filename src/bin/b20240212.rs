use core::panic;
use std::{cell::RefCell, rc::Rc};

use leetcode_easy::TreeNode;

struct Solution {}

fn main() {
    Solution::day_of_year("2019-01-09".to_string());
    Solution::semi_ordered_permutation(vec![2, 1, 4, 3]);
    Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());
    Solution::prefixes_div_by5(vec![0, 1, 1]);
    Solution::sort_array_by_parity_ii(vec![0, 1, 1]);
    Solution::evaluate_tree(None);
}

// Map months with number of days, index offset -1
const MONTHS: &'static [i32; 12] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        // Split the date parts
        let chunks: Vec<i32> = date
            .split("-")
            .map(|chunk| {
                let num: i32 = chunk.parse().unwrap();
                num
            })
            .collect();

        if chunks.len() != 3 {
            panic!("Requires 3 date parts.");
        }

        let y = chunks[0];
        let m = chunks[1];
        let d = chunks[2];

        let is_leap_year = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);

        let mut doy: i32 = 0;

        for i in 0..m {
            if i == (m - 1) {
                doy += d;
            } else if i == 1 && is_leap_year {
                doy += 29;
            } else {
                doy += MONTHS[i as usize];
            }
        }

        doy
    }

    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let last: i32 = nums.len() as i32;
        let mut swaps: i32 = 0;

        // Find location of first and calculate swaps to get to first element
        let first_index: i32 = nums.iter().position(|&v| v == 1).unwrap() as i32;

        swaps += first_index;

        // Find location of last and calculate swaps to get to last element
        let last_index = nums.iter().position(|&v| v == last).unwrap() as i32;

        swaps += (nums.len() - 1) as i32 - last_index;

        // If first and last index crosses, subtract 1 to swaps as we saved swaps due to it
        if first_index > last_index {
            swaps -= 1;
        }

        // println!("first_index: {}, last_index: {}", first_index, last_index);
        swaps
    }

    pub fn is_subsequence(s: String, t: String) -> bool {
        // Loop s and t at the same time
        // Only move s index when it matched t element
        let mut s_index: usize = 0;

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        for i in t_bytes.iter() {
            if s_index < s_bytes.len() && s_bytes[s_index] == *i {
                s_index += 1;

                if s_index >= s.len() {
                    return true;
                }
            }
        }

        s_index >= s.len()
    }

    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        // Very inefficient
        // TODO: Use a more efficient approach
        // URL: https://leetcode.com/problems/binary-prefix-divisible-by-5/description/
        // fn to_base10(partial: &[i32]) -> i32 {
        //     let mut running_man: i32 = 0;
        //     for (k, v) in partial.iter().rev().enumerate() {
        //         running_man += *v as i32 * 2_i32.pow(k as u32);
        //     }
        //     running_man
        // }
        // let mut answer: Vec<bool> = Vec::new();
        //
        // for (k, v) in nums.iter().enumerate() {
        //     let val = to_base10(&nums[0..(k + 1)]);
        //     answer.push(val % 5 == 0);
        // }
        // answer

        // Gave up on this, will just copy/steal (same thing) code instead
        // Source: https://leetcode.com/problems/binary-prefix-divisible-by-5/solutions/296249/java-solution-with-best-explanation-don-t-know-why-some-explanation-not-concise/
        let mut answer: Vec<bool> = Vec::new();
        let mut running_man: i32 = 0;

        for i in nums.iter() {
            running_man = ((running_man * 2) + i) % 5;
            if running_man == 0 {
                answer.push(true);
            } else {
                answer.push(false);
            }
        }
        answer
    }

    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = nums.clone();
        let mut odd: usize = 1;
        let mut even: usize = 0;

        for i in nums.iter() {
            if i % 2 == 0 {
                result[even] = *i;
                even += 2;
            } else {
                result[odd] = *i;
                odd += 2;
            }
        }
        result
    }

    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn eval_node(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(node_rc) = node {
                // Extract value from rc
                let node_val = node_rc.borrow();
                return match node_val.val {
                    0 => false,
                    1 => true,
                    2 => eval_node(node_val.left.clone()) || eval_node(node_val.right.clone()),
                    _ => eval_node(node_val.left.clone()) && eval_node(node_val.right.clone()),
                };
            }
            false
        }

        eval_node(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_tree() {
        let l2 = TreeNode::new(0);
        let r2 = TreeNode::new(1);
        let l1 = TreeNode::new(1);
        let r1 = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(l2))),
            right: Some(Rc::new(RefCell::new(r2))),
        };
        let root = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(l1))),
            right: Some(Rc::new(RefCell::new(r1))),
        };
        assert_eq!(
            Solution::evaluate_tree(Some(Rc::new(RefCell::new(root)))),
            true
        );
    }

    #[test]
    fn test_sort_array_by_parity_ii() {
        assert_eq!(
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            [4, 5, 2, 7]
        );
    }

    #[test]
    fn test_prefixes_div_by5() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1]),
            vec![true, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1]),
            vec![false, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]),
            vec![true, false, false, false, true, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![
                1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1
            ]),
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, true, true,
                true, true, false
            ],
        );
    }

    #[test]
    fn test_is_subsequence() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_semi_ordered_permutation() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
    }

    #[test]
    fn test_day_of_the_year() {
        assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
        assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
        assert_eq!(Solution::day_of_year("2004-03-01".to_string()), 61);
    }
}
