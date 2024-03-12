use leetcode_easy::ListNode;

fn main() {
    Solution::generate_pascal_triangle(5);
    Solution::max_profit(vec![7, 6, 4, 3, 1]);
    Solution::single_number(vec![1]);
    Solution::single_number_xor(vec![1]);
    Solution::majority_element(vec![3, 2, 3]);
    Solution::reverse_list(None);
}

struct Solution {}

impl Solution {
    pub fn generate_pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for x in 0..num_rows {
            if x == 0 {
                result.push(vec![1]);
            } else if x == 1 {
                result.push(vec![1, 1]);
            } else {
                let mut row: Vec<i32> = vec![1];
                let prev_index = x as usize - 1;
                for y in 0..(result[prev_index].len() - 1) {
                    row.push(result[prev_index][y] + result[prev_index][y + 1]);
                }
                row.push(1);
                result.push(row);
            }
        }
        result
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low: i32 = prices[0];
        let mut profit: i32 = 0;

        for v in prices.iter() {
            if *v < low {
                low = *v;
            }

            let new_profit = *v - low;
            if new_profit > profit {
                profit = new_profit;
            }
        }

        profit
    }

    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map: Vec<i32> = vec![0; 60000];
        let mut indexes: Vec<usize> = Vec::with_capacity(nums.len());
        for i in nums.iter() {
            let new_index: usize = *i as usize + 30000;
            map[new_index] += 1;

            if map[new_index] == 1 {
                indexes.push(new_index);
            }
        }
        for i in indexes.iter() {
            if map[*i] == 1 {
                return (*i - 30000) as i32;
            }
        }
        0
    }

    pub fn single_number_xor(nums: Vec<i32>) -> i32 {
        // Copied/stolen (same thing) solution from others
        // The idea is that if you XOR a value with itself, it will cancel each other, ie: 0
        // The good thing is that the XOR effect basically strips the value out of the other value
        let mut x: i32 = 0;
        for i in nums.iter() {
            x = x ^ i;
        }
        x
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // Track each candidate and increment or decrement count
        // or switch candidate if it repeats again
        let mut candidate: (i32, i32) = (nums[0], 0);
        for i in nums.iter() {
            if *i == candidate.0 {
                // Increate candidate count
                candidate.1 += 1;
            } else {
                if candidate.1 == 0 {
                    // Switch candiate
                    candidate = (*i, 1);
                } else {
                    // Decrease candiate count
                    candidate.1 -= 1;
                }
            }
        }

        // Verify if candidate actually occurs n/2 times
        let count = nums.iter().filter(|x| **x == candidate.0).count();
        if count > (nums.len() / 2) {
            return candidate.0;
        }
        -1
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &head;
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(node) = current {
            if prev.is_some() {
                let next = ListNode {
                    val: node.val,
                    next: prev,
                };
                prev = Some(Box::new(next));
                current = &node.next;
            } else {
                // Tail
                let mut next = ListNode::new(node.val);
                next.next = prev;
                prev = Some(Box::new(next));
                current = &node.next;
            }
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use leetcode_easy::create_list;

    use super::*;

    #[test]
    fn test_reverse_list_1() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let new_head = Solution::reverse_list(head);
        assert_eq!(new_head.is_some(), true);
        if let Some(new_head_rc) = new_head {
            assert_eq!(new_head_rc.val, 5);
        }
    }

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
        assert_eq!(Solution::majority_element(vec![1, 2, 3]), -1);
    }

    #[test]
    fn test_single_number_xor() {
        assert_eq!(Solution::single_number_xor(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number_xor(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number_xor(vec![1]), 1);
    }

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_generate_pascal_triangle_1() {
        let expected: Vec<Vec<i32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate_pascal_triangle(5), expected);
    }

    #[test]
    fn test_generate_pascal_triangle_2() {
        let expected: Vec<Vec<i32>> = vec![vec![1]];
        assert_eq!(Solution::generate_pascal_triangle(1), expected);
    }
}
