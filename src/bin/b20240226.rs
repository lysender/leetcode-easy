use core::panic;
use std::{borrow::Borrow, collections::BinaryHeap};

fn main() {
    Solution::remove_outer_parentheses("()()".to_string());
    Solution::roman_to_int("III".to_string());
    Solution::longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    Solution::is_valid_parenthesis("()".to_string());

    let list1 = Solution::to_list(vec![1, 2, 4]);
    let list2 = Solution::to_list(vec![1, 3, 4]);
    Solution::merge_two_lists(list1, list2);
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut answer: String = String::new();
        let mut level: i32 = 0;

        for c in s.chars() {
            if c == '(' {
                if level > 0 {
                    answer.push(c);
                }
                level += 1;
            } else {
                level -= 1;
                if level > 0 {
                    answer.push(c);
                }
            }
        }
        answer
    }

    pub fn roman_to_int(s: String) -> i32 {
        // Source: https://romannumerals.guide/how-roman-numerals-work
        fn multiplier(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Invalid Roman Numeral"),
            }
        }

        let mut prev: Option<char> = None;
        let mut prev_acc_val = 0;
        let mut running_man: i32 = 0;

        for c in s.chars() {
            let val = multiplier(c);
            if let Some(prev_c) = prev {
                let prev_val = multiplier(prev_c);
                if prev_val < val {
                    // Subtract the previous accumulated value of similar char from c value
                    let reduced_val = val - prev_acc_val;
                    // Update the running value but subtract first the prev accumulated value
                    running_man += reduced_val - prev_acc_val;
                    prev_acc_val = 0;
                    prev = Some(c);
                } else if prev_val == val {
                    // Add current value to the previous value
                    prev_acc_val += val;
                    // Also add to running value
                    // No need to update the previous char
                    running_man += val;
                } else {
                    // Just add value to the running value
                    running_man += val;
                    prev = Some(c);
                    prev_acc_val = val;
                }
            } else {
                // Just add to running value
                running_man += val;
                prev_acc_val = val;
                prev = Some(c);
            }
        }

        running_man
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut answer: Vec<u8> = Vec::new();

        let map: Vec<&[u8]> = strs.iter().map(|v| v.as_bytes()).collect();
        // Find the longest string
        let length = map.iter().map(|v| v.len()).max().unwrap();

        // Try checking each character one by one
        let mut x = 0;
        'outer: while x < length {
            let mut c: u8 = 0;
            let mut count: usize = 0;
            for (k, row) in map.iter().enumerate() {
                if row.len() > x {
                    if k == 0 {
                        c = row[x];
                        count += 1;
                    } else if c == row[x] {
                        count += 1;
                    } else {
                        // No need to proceed
                        break 'outer;
                    }
                }
            }

            if count == map.len() {
                answer.push(c);
            }
            x += 1;
        }
        String::from_utf8(answer).unwrap()
    }

    pub fn is_valid_parenthesis(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let opens: Vec<char> = vec!['(', '[', '{'];

        for c in s.chars() {
            if let Some(lc) = stack.last() {
                if opens.contains(&c) {
                    stack.push(c);
                    continue;
                } else {
                    // Check if the last char is a matching open parenthesis
                    let is_matching_close = match lc {
                        '(' => ')' == c,
                        '[' => ']' == c,
                        '{' => '}' == c,
                        _ => false,
                    };
                    if is_matching_close {
                        stack.pop();
                        continue;
                    }
                }
            } else {
                // Only push to stack when it is a valid open
                if opens.contains(&c) {
                    stack.push(c);
                    continue;
                }
            }

            // If it reaches here, it is invalid
            return false;
        }

        stack.len() == 0
    }

    pub fn to_list(items: Vec<i32>) -> Option<Box<ListNode>> {
        // We start at the end of the list in order to build it
        let mut current: Option<Box<ListNode>> = None;
        for i in items.iter().rev() {
            let parent = ListNode {
                val: *i,
                next: current.clone(),
            };
            current = Some(Box::new(parent));
        }
        current
    }

    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Use priority queue
        // Push all items into the queue, then recreate it by popping off last item one by one
        fn pusher(q: &mut BinaryHeap<i32>, head: Option<Box<ListNode>>) {
            let mut current = head.as_ref();
            while let Some(node) = current.borrow() {
                q.push(node.val);
                current = node.next.as_ref();
            }
        }

        let mut queue: BinaryHeap<i32> = BinaryHeap::new();
        pusher(&mut queue, list1);
        pusher(&mut queue, list2);

        // Recreate the list by starting at the last item to the first
        let mut current: Option<Box<ListNode>> = None;
        while let Some(i) = queue.pop() {
            let parent = ListNode {
                val: i,
                next: current,
            };
            current = Some(Box::new(parent));
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists_1() {
        let list1 = Solution::to_list(vec![1, 2, 4]);
        let list2 = Solution::to_list(vec![1, 3, 4]);
        let expected = Solution::to_list(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn test_merge_two_lists_2() {
        let list1 = Solution::to_list(vec![]);
        let list2 = Solution::to_list(vec![]);
        let expected = Solution::to_list(vec![]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn test_merge_two_lists_3() {
        let list1 = Solution::to_list(vec![]);
        let list2 = Solution::to_list(vec![0]);
        let expected = Solution::to_list(vec![0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn test_is_valid_parenthesis() {
        assert_eq!(Solution::is_valid_parenthesis("()".to_string()), true);
        assert_eq!(Solution::is_valid_parenthesis("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid_parenthesis("(]".to_string()), false);
        assert_eq!(Solution::is_valid_parenthesis("]".to_string()), false);
    }

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl".to_string(),
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string(),
            ]),
            "".to_string(),
        );
    }

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("CCXXVII".to_string()), 227);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn test_remove_outer_parentheses() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()"
        );
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())"
        );
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
    }
}
