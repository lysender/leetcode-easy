use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    Solution::search_insert(vec![1, 3, 5, 6], 5);
    Solution::climb_stairs(2);
    Solution::inorder_traversal_recursive(None);
    Solution::inorder_traversal(None);
}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if let Some(min) = nums.first() {
            if target < *min {
                return 0;
            }
        }
        if let Some(max) = nums.last() {
            if target > *max {
                return nums.len() as i32;
            }
        }
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;

        let mut target_index: Option<usize> = None;

        while l < r {
            let mid: usize = ((r - l) / 2) + l;
            if nums[mid] == target {
                // Found it
                return mid as i32;
            }

            if nums[mid] > target {
                // Move lower
                r = mid - 1;
                target_index = Some(r);
            } else {
                // Move higher
                l = mid + 1;
                target_index = Some(l);
            }
        }

        if let Some(ideal_index) = target_index {
            println!(
                "nums: {:?}, target: {}, ideal_index: {}",
                nums, target, ideal_index
            );
            if nums[ideal_index] < target {
                ideal_index as i32 + 1
            } else {
                ideal_index as i32
            }
        } else {
            0
        }
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev: i32 = 0;
        let mut current: i32 = 1;
        for _ in 0..n {
            let new_current = prev + current;
            prev = current;
            current = new_current;
        }

        current
    }

    pub fn inorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Use a recursive solution for now
        fn walk(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut items: Vec<i32> = Vec::new();
            if let Some(node_rc) = node {
                let node_val = node_rc.borrow();

                let mut left_items = walk(&node_val.left);
                items.append(&mut left_items);

                items.push(node_val.val);

                let mut right_items = walk(&node_val.right);
                items.append(&mut right_items);
            }
            items
        }

        walk(&root)
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Use an iterative solution, which uses a stack
        // A stack where the recursive uses btw, so is it really more efficient?
        let mut answer: Vec<i32> = Vec::new();
        // Tupple of node and a flag whether to push to answer or not
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, bool)> = vec![(root, false)];

        while let Some(item) = stack.pop() {
            let (node_opt, to_push) = item;
            if let Some(node) = node_opt {
                let node_rc = node.borrow();
                if to_push {
                    // Either we go up or we go right
                    answer.push(node_rc.val);

                    if let Some(right) = node_rc.right.clone() {
                        // We go right
                        stack.push((Some(right), false));
                    }
                } else {
                    // Push item back to stack but mark it as to push
                    stack.push((Some(node.clone()), true));
                    // Then we go left
                    stack.push((node_rc.left.clone(), false));
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_traversal_1() {
        let l2_left = TreeNode::new(3);
        let l1_right = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(l2_left))),
            right: None,
        };
        let root = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(l1_right))),
        };
        assert_eq!(
            Solution::inorder_traversal_recursive(Some(Rc::new(RefCell::new(root)))),
            vec![1, 3, 2]
        );
    }

    #[test]
    fn test_inorder_traversal_2() {
        assert_eq!(Solution::inorder_traversal_recursive(None), vec![]);
    }

    #[test]
    fn test_inorder_traversal_3() {
        let root = TreeNode::new(1);
        assert_eq!(
            Solution::inorder_traversal_recursive(Some(Rc::new(RefCell::new(root)))),
            vec![1]
        );
    }

    #[test]
    fn test_inorder_traversal_4() {
        let l2_left = TreeNode::new(3);
        let l1_right = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(l2_left))),
            right: None,
        };
        let root = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(l1_right))),
        };
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 3, 2]
        );
    }

    #[test]
    fn test_inorder_traversal_5() {
        assert_eq!(Solution::inorder_traversal(None), vec![]);
    }

    #[test]
    fn test_inorder_traversal_6() {
        let root = TreeNode::new(1);
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1]
        );
    }

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
        assert_eq!(Solution::climb_stairs(6), 13);
    }

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5,), 2,);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
    }
}
