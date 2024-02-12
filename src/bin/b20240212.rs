use core::panic;

struct Solution {}

fn main() {
    Solution::day_of_year("2019-01-09".to_string());
    Solution::semi_ordered_permutation(vec![2, 1, 4, 3]);
}

// Map months with number of days, index offset -1
const MONTHS: &'static[i32; 12] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        // Split the date parts
        let chunks: Vec<i32> = date.split("-").map(|chunk| {
            let num: i32 = chunk.parse().unwrap();
            num
        }).collect();

        if chunks.len() != 3 {
            panic!("Requires 3 date parts.");
        }

        let y = chunks[0];
        let m = chunks[1];
        let d = chunks[2];

        let is_leap_year = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);

        let mut doy: i32 = 0;

        for i in 0..m {
            if i == (m - 1) {
                doy += d;
            } else if i == 1 && is_leap_year {
                doy += 29;
            } else {
                doy += MONTHS[i as usize];
            }
        }

        doy
    }

    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let last: i32 = nums.len() as i32;
        let mut swaps: i32 = 0;

        // Find location of first and calculate swaps to get to first element
        let first_index: i32 = nums.iter().position(|&v| v == 1).unwrap() as i32;

        swaps += first_index;

        // Find location of last and calculate swaps to get to last element
        let last_index = nums.iter().position(|&v| v == last).unwrap() as i32;

        swaps += (nums.len() - 1) as i32 - last_index;

        // If first and last index crosses, subtract 1 to swaps as we saved swaps due to it
        if first_index > last_index {
            swaps -= 1;
        }

        // println!("first_index: {}, last_index: {}", first_index, last_index);
        swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semi_ordered_permutation() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
    }

    #[test]
    fn test_day_of_the_year() {
        assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
        assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
        assert_eq!(Solution::day_of_year("2004-03-01".to_string()), 61);
    }
}
