struct Solution {}

fn main() {
    Solution::count_asterisks("".to_string());
    Solution::has_alternating_bits(5);
}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut count: i32 = 0;
        let mut stack_level: u32 = 0;

        for ch in s.chars() {
            if ch == '*' {
                if stack_level == 0 {
                    count += 1;
                }
            } else if ch == '|' {
                if stack_level == 0 {
                    stack_level = 1;
                } else {
                    stack_level = 0;
                }
            }
        }
        count
    }

    pub fn has_alternating_bits(n: i32) -> bool {
        // Convert to binary then string?
        // Or use some bit shifting logic (I don't know bit shifting!)
        // Will just use string and be done with it
        let bin_s = format!("{:b}", n);
        let mut prev: Option<char> = None;
        for ch in bin_s.chars() {
            if let Some(prev_val) = prev {
                if prev_val == ch {
                    return false;
                }
            }
            prev = Some(ch);
        }
        // println!("{:?}", bin_s);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_asterisks() {
        assert_eq!(Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()), 2);
        assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
        assert_eq!(Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()), 5);
    }

    #[test]
    fn test_alternating_bits() {
        assert_eq!(Solution::has_alternating_bits(5), true);
        assert_eq!(Solution::has_alternating_bits(7), false);
        assert_eq!(Solution::has_alternating_bits(11), false);
    }
}
