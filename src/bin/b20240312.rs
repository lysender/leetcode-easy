use std::{cell::RefCell, rc::Rc};

use leetcode_easy::{ListNode, TreeNode};

struct Solution {}

fn main() {
    Solution::invert_tree(None);
    Solution::is_palindrome(None);
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
}

#[cfg(test)]
mod tests {
    use leetcode_easy::{create_list, create_tree, flatten_tree};

    use super::*;

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
