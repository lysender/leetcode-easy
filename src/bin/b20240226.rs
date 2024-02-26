struct Solution {}

fn main() {
    Solution::remove_outer_parentheses("()()".to_string());
}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut answer: String = String::new();
        let mut level: i32 = 0;

        for c in s.chars() {
            if c == '(' {
                if level > 0 {
                    answer.push(c);
                }
                level += 1;
            } else {
                level -= 1;
                if level > 0 {
                    answer.push(c);
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_outer_parentheses() {
        assert_eq!(Solution::remove_outer_parentheses("(()())(())".to_string()), "()()()");
        assert_eq!(Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()), "()()()()(())");
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
    }
}
