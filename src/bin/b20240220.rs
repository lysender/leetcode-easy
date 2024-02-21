struct Solution {}

fn main() {
    Solution::prefix_count(vec![], "at".to_string());
    Solution::distribute_candies(7, 3);
    Solution::most_visited(4, vec![1, 3, 1, 2]);
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

    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut visits: Vec<i32> = vec![0; n as usize];
        let mut prev: Option<usize> = None;

        for (k, v) in rounds.iter().enumerate().skip(1) {
            let start = rounds[k - 1] as usize;
            let end = rounds[k] as usize;
            let mut current = start;

            println!("k: {}, n: {}", k, v);


            // Round it up until we reach end
            loop {
                println!("current: {}", current);
                // Mark as visited but only if it was not previously visited
                let mut is_visited = false;
                if let Some(prev_val) = prev {
                    is_visited = prev_val == current;
                }

                if !is_visited {
                    visits[current - 1] += 1;
                }

                prev = Some(current);

                // Did we reach the end?
                if current == end {
                    break;
                }

                // Still need to run
                if current == n as usize {
                    // Reset to first sector
                    current = 1;
                } else {
                    current += 1;
                }
            }
        }

        println!("visits: {:?}", visits);
        // Find the most visited item
        let max = visits.iter().max();
        if let Some(max_val) = max {
            return visits
                .iter()
                .enumerate()
                .filter(|(_k, v)| *v == max_val)
                .map(|(k, _v)| (k as i32) + 1)
                .collect();
        }
        vec![] 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_visited() {
        assert_eq!(Solution::most_visited(4, vec![1, 3, 1, 2]), vec![1, 2]);
        assert_eq!(Solution::most_visited(2, vec![2,1,2,1,2,1,2,1,2]), vec![2]);
        assert_eq!(Solution::most_visited(7, vec![1,3,5,7]), vec![1,2,3,4,5,6,7]);
    }

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