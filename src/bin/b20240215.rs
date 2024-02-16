struct Solution {}

fn main() {
    Solution::find_relative_ranks(vec![]);
    Solution::is_happy(2);
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

            println!("digits: {:?}, sum: {}", digits, sum);

            if sum < 10 {
                if sum == single_digit {
                    // Infinite loop, let's get out of here
                    break;
                }
                single_digit = sum;
            }

            digits = to_digits(sum);
        }

        println!("{:?}", digits);

        return digits[0] == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
