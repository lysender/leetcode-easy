struct Solution {}

fn main() {
    Solution::count_asterisks("".to_string());
    Solution::has_alternating_bits(5);
    Solution::is_prefix_string("".to_string(), vec![]);
    Solution::title_to_number("AA".to_string());
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

    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        for (k, _v) in words.iter().enumerate() {
            let partial = words[0..(k + 1)].join("");
            if partial == s {
                return true;
            }
        }
        return false;
    }

    pub fn title_to_number(column_title: String) -> i32 {
        let mut sum: i32 = 0;
        column_title
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .for_each(|(exp, d)| {
                let d26: i32 = *d as i32 - 64;
                let d_val = d26 * 26_i32.pow(exp as u32);
                sum += d_val;
            });

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excel_title_to_number() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }

    #[test]
    fn test_is_prefix_string() {
        assert_eq!(
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "apples".to_string(),
                ]
            ),
            true,
        );

        assert_eq!(
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "apples".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                ]
            ),
            false,
        );

        assert_eq!(
            Solution::is_prefix_string(
                "a".to_string(),
                vec![
                    "aa".to_string(),
                    "aaaa".to_string(),
                    "banana".to_string(),
                ]
            ),
            false,
        );

        assert_eq!(
            Solution::is_prefix_string(
                "z".to_string(),
                vec![
                    "z".to_string(),
                ]
            ),
            true,
        );

        assert_eq!(
            Solution::is_prefix_string(
                "aaaaaaa".to_string(),
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                ]
            ),
            true,
        );
    }

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
