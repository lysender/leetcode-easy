use core::panic;

struct Solution {}

fn main() {
    Solution::remove_outer_parentheses("()()".to_string());
    Solution::roman_to_int("III".to_string());
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

    pub fn roman_to_int(s: String) -> i32 {
        // Source: https://romannumerals.guide/how-roman-numerals-work
        fn multiplier(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Invalid Roman Numeral")
            }
        }

        let mut prev: Option<char> = None;
        let mut prev_acc_val = 0;
        let mut running_man: i32 = 0;

        for c in s.chars() {
            let val = multiplier(c);
            if let Some(prev_c) = prev {
                let prev_val = multiplier(prev_c);
                if prev_val < val {
                    // Subtract the previous accumulated value of similar char from c value
                    let reduced_val = val - prev_acc_val;
                    // Update the running value but subtract first the prev accumulated value
                    running_man += reduced_val - prev_acc_val;
                    prev_acc_val = 0;
                    prev = Some(c);
                } else if prev_val == val {
                    // Add current value to the previous value
                    prev_acc_val += val;
                    // Also add to running value
                    // No need to update the previous char
                    running_man += val;
                } else {
                    // Just add value to the running value
                    running_man += val;
                    prev = Some(c);
                    prev_acc_val = val;
                }
            } else {
                // Just add to running value
                running_man += val;
                prev_acc_val = val;
                prev = Some(c);
            }
        }

        running_man
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("CCXXVII".to_string()), 227);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn test_remove_outer_parentheses() {
        assert_eq!(Solution::remove_outer_parentheses("(()())(())".to_string()), "()()()");
        assert_eq!(Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()), "()()()()(())");
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
    }
}
