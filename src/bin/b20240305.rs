fn main() {
    Solution::generate_pascal_triangle(5);
}

struct Solution {}

impl Solution {
    pub fn generate_pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for x in 0..num_rows {
            if x == 0 {
                result.push(vec![1]);
            } else if x == 1 {
                result.push(vec![1, 1]);
            } else {
                let mut row: Vec<i32> = vec![1];
                let prev_index = x as usize - 1;
                for y in 0..(result[prev_index].len() - 1) {
                    row.push(result[prev_index][y] + result[prev_index][y + 1]);
                }
                row.push(1);
                result.push(row);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pascal_triangle_1() {
        let expected: Vec<Vec<i32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate_pascal_triangle(5), expected);
    }

    #[test]
    fn test_generate_pascal_triangle_2() {
        let expected: Vec<Vec<i32>> = vec![vec![1]];
        assert_eq!(Solution::generate_pascal_triangle(1), expected);
    }
}
