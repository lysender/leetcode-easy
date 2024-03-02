fn main() {
    Solution::search_insert(vec![1, 3, 5, 6], 5);
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
            } else {
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
        }
        
        if let Some(ideal_index) = target_index {
            println!("nums: {:?}, target: {}, ideal_index: {}", nums, target, ideal_index);
            if nums[ideal_index] < target {
                return ideal_index as i32 + 1;
            } else {
                return ideal_index as i32;
            }
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
    }
}
