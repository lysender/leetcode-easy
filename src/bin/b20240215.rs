struct Solution {}

fn main() {
    Solution::find_relative_ranks(vec![]);
    Solution::is_happy(2);
    Solution::sorted_squares(vec![1, 2, 3]);
    Solution::min_max_game(vec![1,3,5,2,4,8,2,2]);
    Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]);
    Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
}

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        // Re-map the scrores so that it can be sorted where keys are still maintained
        // Tupple, first item is the key, second item is the value
        let mut map: Vec<(usize, &i32)> = score.iter().enumerate().map(|(k, v)| {
            (k, v)
        }).collect();

        // Sort descending
        map.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        // Allocate memory with same size as score to allow direct memory write access
        let mut answer: Vec<String> = vec!["".to_string(); score.len()];

        for (k, v) in map.iter().enumerate() {
            // Find the string rank
            let rank: String = match k {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (k + 1).to_string(),
            };
            // Write back to its original position
            answer[v.0] = rank;
        }

        answer
    }

    pub fn is_happy(n: i32) -> bool {
        fn to_digits(x: i32) -> Vec<i32> {
            x.to_string()
            .chars()
            .map(|ch| {
                let digit: i32 = ch.to_string().parse().unwrap();
                digit
            })
            .collect()
        }

        let mut digits = to_digits(n);

        fn is_done(x_digits: &Vec<i32>) -> bool {
            if x_digits.len() == 0 {
                // No more items, for some reason
                return true;
            }
            if x_digits.len() == 1 && x_digits[0] == 1 {
                return true;
            }
            false
        }

        let mut single_digit: i32 = 0;

        while !is_done(&digits) {
            let sum: i32 = digits
                .iter()
                .map(|x| x * x)
                .sum();

            // println!("digits: {:?}, sum: {}", digits, sum);

            if sum < 10 {
                if sum == single_digit {
                    // Infinite loop, let's get out of here
                    break;
                }
                single_digit = sum;
            }

            digits = to_digits(sum);
        }

        // println!("{:?}", digits);

        return digits[0] == 1;
    }

    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = nums
            .iter()
            .map(|x| x * x)
            .collect();

        answer.sort();
        answer
    }

    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut half_life: usize = 0;
        if nums.len() > 1 {
            half_life = nums.len() / 2;
        }

        while half_life > 0 {
            // Compute values in-place, replacing items in nums
            // but decrease max length on each process
            for i in 0..half_life {
                let v1 = nums[2_usize * i];
                let v2 = nums[(2_usize * i) + 1];

                if i % 2 == 0 {
                    // Use min
                    if v1 < v2 {
                        nums[i] = v1;
                    } else {
                        nums[i] = v2;
                    }
                } else {
                    // Use max
                    if v1 > v2 {
                        nums[i] = v1;
                    } else {
                        nums[i] = v2;
                    }
                }
            }

            // Reduce in half
            half_life /= 2;
        }

        return nums[0];
    }

    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let markers_len = grid.len() * grid.len();
        let mut markers: Vec<bool> = vec![false; markers_len];
        let mut answer: Vec<i32> = vec![0, 0];

        for x in 0..(grid.len()) {
            for y in 0..(grid.len()) {
                let val = (grid[x][y] as usize) - 1_usize;

                let existing = markers[val];
                if existing {
                    // This is the repeated value
                    answer[0] = grid[x][y];
                }
                markers[val] = true;
            }
        }
        // Find the missing value
        for (k, v) in markers.iter().enumerate() {
            if !v {
                answer[1] = k as i32 + 1_i32;
            }
        }
        answer
    }

    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![0; nums.len()];
        let mut even: usize = 0;
        let mut odd: usize = nums.len() - 1;

        for v in nums.iter() {
            if v % 2 == 0 {
                answer[even] = *v;
                even += 1;
            } else {
                answer[odd] = *v;
                odd -= 1;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(Solution::sort_array_by_parity(vec![3, 1, 2, 4]), vec![2, 4, 1, 3]);
    }

    #[test]
    fn test_missing_and_repeated_values() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
            vec![2, 4],
        );
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]]),
            vec![9, 5],
        );
    }

    #[test]
    fn test_min_max_game() {
        assert_eq!(Solution::min_max_game(vec![1,3,5,2,4,8,2,2]), 1);
        assert_eq!(Solution::min_max_game(vec![3]), 3);
    }

    #[test]
    fn test_sorted_squares() {
        assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
        assert_eq!(Solution::sorted_squares(vec![-7,-3,2,3,11]), vec![4,9,9,49,121]);
    }

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(7), true);
        assert_eq!(Solution::is_happy(2), false);
    }

    #[test]
    fn test_find_relative_rank() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5,4,3,2,1]),
            vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"]
        );

        assert_eq!(
            Solution::find_relative_ranks(vec![10,3,8,9,4]),
            vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"]
        );
    }
}
