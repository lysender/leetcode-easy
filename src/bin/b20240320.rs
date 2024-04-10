use std::collections::HashSet;

fn main() {
    Solution::contains_duplicate(vec![1]);
    Solution::count_students(vec![], vec![]);
}

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashSet<i32> = HashSet::new();
        for i in nums.iter() {
            if map.contains(i) {
                return true;
            }
            map.insert(*i);
        }
        false
    }

    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut queue = std::collections::VecDeque::from(students);
        let mut i: usize = 0;
        let mut pushes: usize = 0;

        while let Some(student) = queue.pop_front() {
            if student == sandwiches[i] {
                i += 1;
                pushes = 0;
            } else {
                queue.push_back(student);
                pushes += 1;
            }

            if pushes == queue.len() {
                // We just do an infinite loop
                break;
            }
        }

        queue.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_students() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
