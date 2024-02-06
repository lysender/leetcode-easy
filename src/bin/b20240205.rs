struct Solution {}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];
    let slice: &[i32] = &nums[0..0];
    println!("{:?}", slice);

    println!("{}", Solution::digit_sum("11111222223".to_string(), 3));
    println!("{}", Solution::my_sqrt(4));

    let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("{}", k.add(3));

    println!("{}", Solution::remove_duplicates("abbaca".to_string()));

    let mut h = MyHashSet::new();
    h.add(1);
    h.add(2);
    assert_eq!(h.contains(1), true);
    assert_eq!(h.contains(3), false);
    h.add(2);
    assert_eq!(h.contains(2), true);
    h.remove(2);
    assert_eq!(h.contains(2), false);
}

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut text = s.clone();
        let smol_k = k as usize;

        while text.len() > smol_k {
            let chars: Vec<char> = text.chars().collect();
            let digits: Vec<String> = chars.chunks(smol_k).map(|chunk| {
                let num: i32 = chunk.iter().map(|n| {
                    let n_str = n.to_string();
                    let n_v: i32 = n_str.parse().unwrap();
                    n_v
                }).sum();
                num.to_string()
            }).collect();
            text = digits.join("");
        }

        text
    }

    pub fn my_sqrt(n: i32) -> i32 {
        // Use the binary search approach
        if n < 2 {
            return n;
        }

        let big_n = n as i64;
        let mut l: i64 = 1;
        let mut r: i64 = n as i64;
        let mut mid: i64 = 0;
        let mut prev_mid: i64 = 0;

        while l <= r {
            // Find mid
            let tmp_mid = (l  + r) / 2;
            if tmp_mid == prev_mid {
                // We are in a loop, get out
                let res: i32 = prev_mid as i32;
                return res;
            }

            prev_mid = mid;
            mid = tmp_mid;

            let current = mid * mid;
            //println!("l: {}, r: {}, mid: {}, prev_mid: {}, cur: {}, n: {}", l, r, mid, prev_mid, current, big_n);
            if current == big_n {
                // Found it
                return mid as i32;
            }

            // Not found? Decide whether its low or high
            if current > big_n {
                // Find lower half
                r = mid;
            } else {
                // Find higher half
                l = mid;
            }
        }

        // If we reached this far, we just need to choose the lower value
        return l as i32;
    }

    pub fn remove_duplicates(s: String) -> String {
        let mut result: Vec<char> = vec![];

        for ch in s.chars() {
            if result.len() > 0 && *result.last().unwrap() == ch {
                result.pop();
            } else {
                result.push(ch);
            }
        }

        result.iter().collect()
    }
}

struct KthLargest {
    k: usize,
    nums: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = std::collections::BinaryHeap::new();
        // Insert all items
        for num in nums.iter() {
            heap.push(std::cmp::Reverse(*num));
        }

        // Remove excess items, if there are any
        while heap.len() > k as usize {
            heap.pop();
        }

        Self { k: k as usize, nums: heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        // Get the current kth largest element
        self.nums.push(std::cmp::Reverse(val));

        while self.nums.len() > self.k {
            self.nums.pop();
        }

        let num = self.nums.peek().unwrap();
        num.0
    }
}

struct MyHashSet {
    items: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        MyHashSet { items: vec![] }
    }

    fn add(&mut self, key: i32) {
        if !self.contains(key) {
            self.items.push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        if self.contains(key) {
            let items: Vec<i32> = self.items.iter().filter(|x| x != &&key).map(|x| *x).collect();
            self.items = items;
        }
    }

    fn contains(&self, key: i32) -> bool {
        self.items.contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squirt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(3), 1);
        assert_eq!(Solution::my_sqrt(5), 2);
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }

    #[test]
    fn test_kth_largest_data1() {
        // 2 4 5 8
        // 2 4 5 6 8
        let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(k.add(3), 4);
        assert_eq!(k.add(5), 5);
        assert_eq!(k.add(10), 5);
        assert_eq!(k.add(9), 8);
        assert_eq!(k.add(4), 8);
    }

    #[test]
    fn test_kth_largest_data2() {
        let mut k = KthLargest::new(2, vec![0]);
        assert_eq!(k.add(-1), -1);
        assert_eq!(k.add(1), 0);
        assert_eq!(k.add(-2), 0);
        assert_eq!(k.add(-4), 0);
        assert_eq!(k.add(3), 1);
    }

    #[test]
    fn test_remove_adjacent_duplicates() {
        assert_eq!(
            Solution::remove_duplicates("abbaca".to_string()),
            "ca".to_string()
        );

        assert_eq!(
            Solution::remove_duplicates("azxxzy".to_string()),
            "ay".to_string()
        );
    }

    #[test]
    fn test_manual_hash_map() {
        let mut h = MyHashSet::new();
        h.add(1);
        h.add(2);
        assert_eq!(h.contains(1), true);
        assert_eq!(h.contains(3), false);
        h.add(2);
        assert_eq!(h.contains(2), true);
        h.remove(2);
        assert_eq!(h.contains(2), false);
    }
}
