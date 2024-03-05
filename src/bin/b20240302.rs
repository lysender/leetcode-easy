use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode_easy::TreeNode;

fn main() {
    Solution::search_insert(vec![1, 3, 5, 6], 5);
    Solution::climb_stairs(2);
    Solution::inorder_traversal_recursive(None);
    Solution::inorder_traversal(None);
    Solution::is_symmetric(None);
    Solution::max_depth(None);
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
                    stack.push((node_rc.right.clone(), false));
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

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Just use top to bottom traversal where
        // top left is left leaning and
        // right is right leaning
        // then compare the result at the end

        // Use a top down traversal, not sure how it is called
        fn walk(root_node: Option<Rc<RefCell<TreeNode>>>, left_lean: bool) -> Vec<i32> {
            // This code is based on previous in order traversal solution
            let mut result: Vec<i32> = Vec::new();
            let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::from([root_node]);

            while let Some(item) = queue.pop_front() {
                if let Some(node) = item {
                    let node_rc = node.borrow();
                    result.push(node_rc.val);

                    if left_lean {
                        // Do left then right
                        queue.push_back(node_rc.left.clone());
                        queue.push_back(node_rc.right.clone());
                    } else {
                        // Do right then left
                        queue.push_back(node_rc.right.clone());
                        queue.push_back(node_rc.left.clone());
                    }
                } else {
                    // Add None value to ensure tree is represented correctly
                    result.push(-101);
                }
            }

            result
        }

        if let Some(root_node) = root {
            let root_rc = root_node.borrow();
            let left = walk(root_rc.left.clone(), true);
            let right = walk(root_rc.right.clone(), false);
            return left == right;
        }

        true
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn the_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node_val) => {
                    // We got 1 level plus all the levels down
                    let node_rc = node_val.borrow();
                    1 + the_depth(&node_rc.left).max(the_depth(&node_rc.right))
                }
                None => 0,
            }
        }
        the_depth(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth_1() {
        // [3,9,20,null,null,15,7]
        let l1_l = TreeNode {
            val: 9,
            left: None,
            right: None,
        };
        let l1_r = TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        };
        let root = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(l1_l))),
            right: Some(Rc::new(RefCell::new(l1_r))),
        };
        assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(root)))), 3,);
    }

    #[test]
    fn test_max_depth_2() {
        // [1,null,2]
        let root = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        };
        assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(root)))), 2,);
    }

    #[test]
    fn test_is_symmetric_1() {
        let l1_l = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        };
        let l1_r = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        };
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(l1_l))),
            right: Some(Rc::new(RefCell::new(l1_r))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))),
            true
        );
    }

    #[test]
    fn test_is_symmetric_2() {
        let l1_l = TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        };
        let l1_r = TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        };
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(l1_l))),
            right: Some(Rc::new(RefCell::new(l1_r))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))),
            false
        );
    }

    #[test]
    fn test_is_symmetric_3() {
        // [1,2,2,2,null,2]
        //       1
        //   2       2
        // 2   x   2
        let l1_l = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        };
        let l1_r = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        };
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(l1_l))),
            right: Some(Rc::new(RefCell::new(l1_r))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))),
            false
        );
    }

    #[test]
    fn test_is_symmetric_4() {
        // [5,4,1,null,1,null,4,2,null,2,null]
        //                   5
        //           4               1
        //        x     1         x     4
        //            2   x           2   x
        let l2_lr = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        };
        let l2_rr = TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        };
        let l1_l = TreeNode {
            val: 4,
            left: None,
            right: Some(Rc::new(RefCell::new(l2_lr))),
        };
        let l1_r = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(l2_rr))),
        };
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(l1_l))),
            right: Some(Rc::new(RefCell::new(l1_r))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))),
            false
        );
    }

    #[test]
    fn test_is_symmetric_5() {
        // [2,3,3,4,5,null,4]
        //                   2
        //           3               3
        //        4     5         x     4
        let l1_l = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        };
        let l1_r = TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        };
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(l1_l))),
            right: Some(Rc::new(RefCell::new(l1_r))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))),
            false
        );
    }

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
