use std::{cell::RefCell, rc::Rc};

use leetcode_easy::{ListNode, TreeNode};

struct Solution {}

fn main() {
    Solution::invert_tree(None);
    Solution::is_palindrome(None);
    let mut input: Vec<i32> = vec![0];
    Solution::move_zeroes(&mut input);
    Solution::diameter_of_binary_tree(None);
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn invert_tree_inner(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            match node {
                Some(node_rc) => {
                    let node_val = node_rc.borrow();
                    let new_node = TreeNode {
                        val: node_val.val,
                        left: invert_tree_inner(node_val.right.clone()),
                        right: invert_tree_inner(node_val.left.clone()),
                    };
                    Some(Rc::new(RefCell::new(new_node)))
                }
                None => None,
            }
        }
        invert_tree_inner(root)
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // Collect items into an array first
        let mut items: Vec<i32> = Vec::new();
        let mut current = &head;
        while let Some(node) = current {
            items.push(node.val);
            current = &node.next;
        }

        // Use two pointer technique to compare low and high cursors
        let mut low: usize = 0;
        let mut high: usize = items.len() - 1;

        // Don't need to reach both end, meeting at the middle is enough
        while low < high {
            if items[low] != items[high] {
                return false;
            }
            low += 1;
            high -= 1;
        }
        true
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut z: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(z, i);
                z += 1;
            }
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn get_height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let Some(node_val) = node else {
                return 0;
            };

            let node_rc = node_val.borrow();
            let hl = get_height(&node_rc.left);
            let hr = get_height(&node_rc.right);

            // Height of a tree is 1 + the height of left or right, whichever is higher
            1 + hl.max(hr)
        }

        fn get_diameter(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let Some(node_val) = node else {
                return 0;
            };

            let node_rc = node_val.borrow();

            let hl = get_height(&node_rc.left);
            let hr = get_height(&node_rc.right);

            let dl = get_diameter(&node_rc.left);
            let dr = get_diameter(&node_rc.right);

            // Diameter of a tree is any of the following, whichever is higher:
            // - diameter of left
            // - diameter of right
            // - height of left + height of right
            //   - we are only counting the steps to get to the other node
            //   - if we want to count each node, we need to add 1
            dl.max(dr).max(hl + hr)
        }

        get_diameter(&root)
    }
}

#[cfg(test)]
mod tests {
    use leetcode_easy::{create_list, create_tree, flatten_tree};

    use super::*;

    #[test]
    fn test_diameter_of_binary_tree_1() {
        let input: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
        let root = create_tree(input);
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_of_binary_tree_2() {
        let input: Vec<Option<i32>> = vec![Some(1), Some(2)];
        let root = create_tree(input);
        assert_eq!(Solution::diameter_of_binary_tree(root), 1);
    }

    #[test]
    fn test_move_zeroes_1() {
        let mut input: Vec<i32> = vec![0, 1, 0, 3, 12];
        let expected: Vec<i32> = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_move_zeroes_2() {
        let mut input: Vec<i32> = vec![0];
        let expected: Vec<i32> = vec![0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_move_zeroes_3() {
        let mut input: Vec<i32> = vec![0, 0, 1];
        let expected: Vec<i32> = vec![1, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_is_palindrome_1() {
        let head = create_list(vec![1, 2, 2, 1]);
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn test_is_palindrome_2() {
        let head = create_list(vec![1, 2]);
        assert_eq!(Solution::is_palindrome(head), false);
    }

    #[test]
    fn test_invert_tree_1() {
        let input: Vec<Option<i32>> = vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ];
        let root = create_tree(input);
        let inverted_root = Solution::invert_tree(root);
        let expected: Vec<Option<i32>> = vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ];
        assert_eq!(flatten_tree(inverted_root), expected);
    }
}
