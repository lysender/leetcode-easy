use std::{cell::RefCell, rc::Rc};

use leetcode_easy::TreeNode;

struct Solution {}

fn main() {
    Solution::invert_tree(None);
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
}

#[cfg(test)]
mod tests {
    use leetcode_easy::{create_tree, flatten_tree};

    use super::*;

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
