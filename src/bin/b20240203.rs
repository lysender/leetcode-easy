use std::{time::Instant, collections::HashSet};
use std::rc::Rc;
use std::cell::RefCell;

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

fn main() {
    solution_categorize_box();
    solution_sum_of_square_numbers();
    solution_sum_of_square_numbers_hash_map();
    solution_sum_of_square_numbers_bs();
    solution_sum_of_left_leaves();
    solution_largest_number_after_digits_swap();
    solution_number_of_even_digits();
}

fn solution_number_of_even_digits() {
    assert_eq!(even_number_of_digits(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(even_number_of_digits(vec![555, 901, 482, 1771]), 1);
}

fn even_number_of_digits(nums: Vec<i32>) -> i32 {
    let evens: Vec<&i32> = nums.iter().filter(|n| {
        n.to_string().len() % 2 == 0
    }).collect();

    evens.len() as i32
}

fn solution_largest_number_after_digits_swap() {
    let timer = Instant::now();
    assert_eq!(largest_number_after_digits_swap(1234), 3412);
    assert_eq!(largest_number_after_digits_swap(266), 662);
    assert_eq!(largest_number_after_digits_swap(568), 586);
    assert_eq!(largest_number_after_digits_swap(2399), 2993);
    assert_eq!(largest_number_after_digits_swap(3159), 9531);
    assert_eq!(largest_number_after_digits_swap(65875), 87655);
    let duration = timer.elapsed().as_millis();
    println!("largest_number_of_digits_swap: {} ms", duration);
}

fn largest_number_after_digits_swap(num: i32) -> i32 {
    if num >= 10 {
        let mut num_vec2: Vec<i32> = Vec::new();

        // Collect digits in reversed order
        let mut digits = num.clone();
        while digits > 0 {
            num_vec2.push(digits % 10);
            digits /= 10;
        }

        // Starting from the left, swap digits with the smallest value
        let mut start: usize = 0;

        for _x in 0..num_vec2.len() {
            let start_val = num_vec2[start];
            let is_even: bool = start_val % 2 == 0;

            let mut smallest: i32 = start_val;
            let mut smallest_index: usize = 0;

            for y in (start + 1)..num_vec2.len() {
                let current_val = num_vec2[y];
                let current_is_even: bool = current_val % 2 == 0;
                if is_even == current_is_even && current_val < smallest {
                    smallest = current_val;
                    smallest_index = y;
                }
            }

            // Do the swapping
            if smallest_index > start {
                num_vec2.swap(start, smallest_index);
            }

            start += 1;
        }

        // Convert back to integer
        let mut result: u32 = 0;
        for (index, val) in num_vec2.iter().enumerate() {
            let power: u32 = index as u32;
            let fat_val: u32 = *val as u32;
            result += fat_val * 10_u32.pow(power);
        }

        // Convert to int
        let final_result: i32 = result as i32;
        return final_result;
    }

    // Single digit, already the largest
    num
}

fn solution_sum_of_left_leaves() {
    let node_15 = Rc::new(RefCell::new(TreeNode::new(15)));
    let node_7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let node_20 = Rc::new(RefCell::new(TreeNode {
        val: 20,
        left: Some(node_15),
        right: Some(node_7),
    }));
    let node_9 = Rc::new(RefCell::new(TreeNode::new(9)));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(node_9),
        right: Some(node_20),
    }));

    let sum = sum_of_left_leaves(Some(root));
    println!("{}", sum);
}

fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let timer = Instant::now();

    fn inner_sum(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        let mut sum: i32 = 0;
            if let Some(node_val) = node {
            let inner_node = node_val.borrow();
            if inner_node.left.is_none() && inner_node.right.is_none() {
                if is_left {
                    sum += inner_node.val;
                }
            }
            if let Some(left) = &inner_node.left {
                sum += inner_sum(Some(left.clone()), true);
            }
            if let Some(right) = &inner_node.right {
                sum += inner_sum(Some(right.clone()), false);
            }
        }

        sum
    }

    let sum = inner_sum(root, false);
    let duration = timer.elapsed().as_millis();
    println!("sum_of_left_leaves: {} ms", duration);
    sum
}

fn solution_sum_of_square_numbers_bs() {
    assert!(sum_of_square_numbers_bs(5));
    assert!(sum_of_square_numbers_bs(4));
    assert!(!sum_of_square_numbers_bs(3));
    assert!(sum_of_square_numbers_bs(100000));
    // assert!(!sum_of_square_numbers_bs(999999999));
}

fn sum_of_square_numbers_bs(c: i32) -> bool {
    let timer = Instant::now();
    let fat_c: i64 = c as i64;
    let max_len: i64 = (c as f64).sqrt() as i64;
    let mut result: bool = false;

    let bs_mid = |start: i64| -> bool {
        let mut l: i64 = 0;
        let mut r: i64 = max_len;

        while l <= r {
            // Find mid
            let mid: i64 = (l + r) / 2;
            let sum = (start * start) + (mid * mid);
            if sum == fat_c {
                // Found it
                return true;
            } else if sum > fat_c {
                // Overshoot, find the lower half
                r = mid - 1;
            } else {
                // Too short, find the higher half
                l = mid + 1;
            }
        }

        false
    };

    for x in 0..max_len + 1 {
        let fat_x = x as i64;
        if bs_mid(fat_x) {
            result = true;
        }
    }

    let duration = timer.elapsed().as_millis();
    println!("binary search: c={} in {} ms", c, duration);
    result
}

fn solution_sum_of_square_numbers() {
    assert!(sum_of_square_numbers(5));
    assert!(sum_of_square_numbers(4));
    assert!(!sum_of_square_numbers(3));
    assert!(sum_of_square_numbers(100000));
    // assert!(!sum_of_square_numbers(999999999));
}

fn sum_of_square_numbers(c: i32) -> bool {
    let timer = Instant::now();
    let fat_c: i64 = c as i64;
    let max_len: i64 = (c as f64).sqrt() as i64;
    let mut result: bool = false;
    for x in 0..=max_len + 1 {
        for y in 0..=max_len + 1 {
            let fat_x: i64 = x as i64;
            let fat_y: i64 = y as i64;
            let sum: i64 = (fat_x * fat_x) + (fat_y * fat_y);
            if sum == fat_c {
                result = true;
                break;
            }
        }
    }
    let duration = timer.elapsed().as_millis();
    println!("sqrt double loop: c={} in {} ms", c, duration);
    result
}

fn solution_sum_of_square_numbers_hash_map() {
    assert!(sum_of_square_numbers_hash_map(5));
    assert!(sum_of_square_numbers_hash_map(4));
    assert!(!sum_of_square_numbers_hash_map(3));
    assert!(sum_of_square_numbers_hash_map(100000));
    // assert!(!sum_of_square_numbers_hash_map(999999999));
}

fn sum_of_square_numbers_hash_map(c: i32) -> bool {
    // Uses too much memoru
    let timer = Instant::now();
    let fat_c: i128 = c as i128;
    let mut result: bool = false;

    let mut map: HashSet<i128> = HashSet::new();

    for x in 0..c {
        let fat_x: i128 = x as i128;
        map.insert(fat_x * fat_x);
        let index: i128 = fat_c - ((fat_x * fat_x) as i128);
        if map.contains(&index) {
            result = true;
            break;
        }
    }

    let duration = timer.elapsed().as_millis();
    println!("hash map: c={} in {} ms", c, duration);
    result
}

fn solution_categorize_box() {
    let result = categorize_box(2909, 3968, 3272, 727);
    println!("{}", result);
}

fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let l: i64 = length as i64;
    let w: i64 = width as i64;
    let h: i64 = height as i64;
    let m: i64 = mass as i64;
    let pow_104: i64 = 10_i64.pow(4);
    let pow_109: i64 = 10_i64.pow(9);
    let volume: i64 = (l * w * h) as i64;

    let mut bulky = false;
    let mut heavy = false;

    if l>= pow_104 || w>= pow_104 || h>= pow_104 || volume >= pow_109{
        bulky = true;
    }

    if m>= 100 {
        heavy = true;
    }

    let mut category: String = String::from("Neither");

    if bulky && heavy {
        category = String::from("Both");
    } else if bulky {
        category = String::from("Bulky");
    } else if heavy {
        category = String::from("Heavy");
    }

    category
}
