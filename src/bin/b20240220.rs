struct Solution {}

fn main() {
    Solution::prefix_count(vec![], "at".to_string());
    Solution::distribute_candies(7, 3);
}

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|x| x.starts_with(&pref)).count() as i32 
    }

    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![0; num_people as usize];
        let mut x: i32 = candies;
        let mut row: i32 = 0;

        while x > 0 {
            for k in 0..num_people {
                let share: i32 = (num_people * row) + (k + 1);
                let remaining = x - share;
                println!("row: {}, k: {}, share: {}, remaining: {}", row, k, share, remaining);
                if remaining < 1 {
                    // We run out of candies
                    // Give all remaining candies to the user
                    answer[k as usize] += x;
                    x = 0;
                    break;
                } else {
                    x = remaining;
                    answer[k as usize] += share;
                }
            }
            row += 1;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(7, 4), vec![1, 2, 3, 1]);
        assert_eq!(Solution::distribute_candies(10, 3), vec![5, 2, 3]);
    }

    #[test]
    fn test_prefix_count() {
        assert_eq!(
            Solution::prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string(),
                ],
                "at".to_string()),
            2,
        );

        assert_eq!(
            Solution::prefix_count(
                vec![
                    "leetcode".to_string(),
                    "win".to_string(),
                    "loops".to_string(),
                    "success".to_string(),
                ],
                "code".to_string()),
            0,
        );
    }
}
