struct Solution {}

fn main() {
    Solution::count_asterisks("".to_string());
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
}
