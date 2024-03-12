use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn create_list(items: Vec<i32>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;

    for i in items.iter().rev() {
        let node = ListNode {
            val: *i,
            next: prev,
        };
        prev = Some(Box::new(node));
    }
    prev
}

pub fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut items: Vec<i32> = Vec::new();
    let mut current = &head;

    while let Some(node) = current {
        items.push(node.val);
        current = &node.next;
    }
    items
}

pub fn create_tree(items: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if items.len() == 0 {
        return None;
    }

    if let Some(root_val) = items[0] {
        let root_node = TreeNode::new(root_val);
        let root = Some(Rc::new(RefCell::new(root_node)));
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::from([root.clone()]);
        let mut i: usize = 1;

        while let Some(item) = queue.pop_front() {
            if let Some(node) = item {
                let mut node_rc = node.borrow_mut();
                if i < items.len() {
                    if let Some(left) = items[i] {
                        node_rc.left = Some(Rc::new(RefCell::new(TreeNode::new(left))));
                        queue.push_back(node_rc.left.clone());
                    } else {
                        node_rc.left = None;
                    }
                    i += 1;
                }
                if i < items.len() {
                    if let Some(right) = items[i] {
                        node_rc.right = Some(Rc::new(RefCell::new(TreeNode::new(right))));
                        queue.push_back(node_rc.right.clone());
                    } else {
                        node_rc.right = None;
                    }
                    i += 1;
                }
            }
        }
        return root;
    }

    None
}

pub fn flatten_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut items: Vec<Option<i32>> = Vec::new();
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::from([root]);
    while let Some(item) = queue.pop_front() {
        if let Some(node) = item {
            let node_rc = node.borrow();
            items.push(Some(node_rc.val));

            queue.push_back(node_rc.left.clone());
            queue.push_back(node_rc.right.clone());
        }
    }
    items
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

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_list, create_tree, flatten_tree, list_to_vec};

    #[test]
    fn test_tree() {
        let input: Vec<Option<i32>> = vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ];
        let root = create_tree(input.clone());
        assert_eq!(flatten_tree(root), input);
    }

    #[test]
    fn test_list() {
        let input = vec![1, 2, 3, 4, 5];
        let head = create_list(input.clone());
        assert_eq!(list_to_vec(head), input);
    }
}
