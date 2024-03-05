use leetcode_easy::ListNode;

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

    Solution::two_out_of_three(vec![], vec![], vec![]);
    Solution::remove_to_equalize_freq("foo".to_string());
    Solution::middle_node(Some(Box::new(ListNode::new(1))));
}

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut text = s.clone();
        let smol_k = k as usize;

        while text.len() > smol_k {
            let chars: Vec<char> = text.chars().collect();
            let digits: Vec<String> = chars
                .chunks(smol_k)
                .map(|chunk| {
                    let num: i32 = chunk
                        .iter()
                        .map(|n| {
                            let n_str = n.to_string();
                            let n_v: i32 = n_str.parse().unwrap();
                            n_v
                        })
                        .sum();
                    num.to_string()
                })
                .collect();
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
            let tmp_mid = (l + r) / 2;
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

    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let node: [i32; 3] = [0; 3];
        let mut map: [[i32; 3]; 101] = [node; 101];

        // Find the max len of the 3 arrays
        let mut max: usize = nums1.len();
        if nums2.len() > max {
            max = nums2.len();
        }
        if nums3.len() > max {
            max = nums3.len();
        }

        for i in 0..max {
            if let Some(num1) = nums1.get(i) {
                map[*num1 as usize][0] = 1;
            }
            if let Some(num2) = nums2.get(i) {
                map[*num2 as usize][1] = 1;
            }
            if let Some(num3) = nums3.get(i) {
                map[*num3 as usize][2] = 1;
            }
        }

        let result: Vec<i32> = map
            .iter()
            .enumerate()
            .filter(|(_k, v)| {
                let sum: i32 = v.iter().sum();
                sum > 1
            })
            .map(|(k, _v)| k as i32)
            .collect();
        result
    }

    pub fn remove_to_equalize_freq(word: String) -> bool {
        let offset: usize = 97;

        // Track each 26 character and count each
        let mut map: [i32; 26] = [0; 26];

        for i in word.as_bytes().iter() {
            let index = *i as usize - offset;
            map[index] += 1;
        }

        // Collect various number of counts
        // Key is the count, value is the frequency
        let mut counts: [i32; 101] = [0; 101];
        for i in map.iter() {
            if *i > 0 {
                counts[*i as usize] += 1;
            }
        }

        // Find the count keys
        let count_keys: Vec<usize> = counts
            .iter()
            .enumerate()
            .filter(|(k, v)| *k > 0 && **v > 0)
            .map(|(k, _v)| k)
            .collect();

        // println!("{:?}", counts);
        // println!("{:?}", count_keys);
        // println!("{:?}", word);

        // To allow equalizing, any the following must be true:
        // one count and count == 1
        // one count and count frequency == 1
        if count_keys.len() == 1 {
            if count_keys[0] == 1 {
                return true;
            }
            // count frequency
            if counts[count_keys[0]] == 1 {
                return true;
            }
        }

        if count_keys.len() == 2 {
            // To allow equalizing, all of the following must be true:
            // difference between count must be 1
            // one of the count frequency must be == 1
            // count with frequency == 1 must have higher count than the other
            let diff: i32 = count_keys[0] as i32 - count_keys[1] as i32;
            if diff.abs() == 1 {
                if counts[count_keys[0]] == 1 && count_keys[0] > count_keys[1] {
                    return true;
                }
                if counts[count_keys[1]] == 1 && count_keys[1] > count_keys[0] {
                    return true;
                }
            }

            // Or if one of the count == 1 and frequency == 1
            if count_keys[0] == 1 && counts[count_keys[0]] == 1 {
                return true;
            }
            if count_keys[1] == 1 && counts[count_keys[1]] == 1 {
                return true;
            }
        }

        // Will not equalize
        return false;
    }

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Find the length of the list to get the middle
        let mut length: usize = 0;
        let mut current = head.clone();
        // Track each element into an array
        let mut items: Vec<Option<Box<ListNode>>> = Vec::new();
        while let Some(item) = current {
            items.push(Some(item.clone()));

            length += 1;
            current = item.next;
        }

        // println!("Len: {}", length);
        if length == 0 {
            return None;
        }

        let mid: usize = (length / 2) + 1;
        match items.get(mid - 1) {
            Some(val) => val.clone(),
            None => None,
        }
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

        Self {
            k: k as usize,
            nums: heap,
        }
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
            let items: Vec<i32> = self
                .items
                .iter()
                .filter(|x| x != &&key)
                .map(|x| *x)
                .collect();
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

    #[test]
    fn test_two_out_of_three() {
        let nums1: Vec<i32> = vec![1, 1, 3, 2];
        let nums2: Vec<i32> = vec![2, 3];
        let nums3: Vec<i32> = vec![3];
        let exp: Vec<i32> = vec![2, 3];
        let mut result = Solution::two_out_of_three(nums1, nums2, nums3);
        result.sort();
        assert_eq!(result, exp);
    }

    #[test]
    fn test_two_out_of_three_data2() {
        let nums1: Vec<i32> = vec![2, 15, 10, 11, 14, 12, 14, 11, 9, 1];
        let nums2: Vec<i32> = vec![8, 9, 13, 2, 11, 8];
        let nums3: Vec<i32> = vec![13, 5, 15, 7, 12, 7, 8, 3, 13, 15];
        let exp: Vec<i32> = vec![2, 8, 9, 11, 12, 13, 15];
        let mut result = Solution::two_out_of_three(nums1, nums2, nums3);
        result.sort();
        assert_eq!(result, exp);
    }

    #[test]
    fn test_remove_letter_to_equalize() {
        assert_eq!(Solution::remove_to_equalize_freq("abcc".to_string()), true);
        assert_eq!(Solution::remove_to_equalize_freq("aazz".to_string()), false);
        assert_eq!(Solution::remove_to_equalize_freq("bac".to_string()), true);
        assert_eq!(
            Solution::remove_to_equalize_freq("ddaccb".to_string()),
            false
        );
        assert_eq!(
            Solution::remove_to_equalize_freq("cbccca".to_string()),
            false
        );
        assert_eq!(Solution::remove_to_equalize_freq("zz".to_string()), true);
        assert_eq!(Solution::remove_to_equalize_freq("cccd".to_string()), true);
        assert_eq!(
            Solution::remove_to_equalize_freq("aaaabbbbccc".to_string()),
            false
        );
        assert_eq!(Solution::remove_to_equalize_freq("abbcc".to_string()), true);
    }

    #[test]
    fn test_middle_node1() {
        let n5 = ListNode { next: None, val: 5 };
        let n4 = ListNode {
            next: Some(Box::new(n5)),
            val: 4,
        };
        let n3 = ListNode {
            next: Some(Box::new(n4)),
            val: 3,
        };
        let n2 = ListNode {
            next: Some(Box::new(n3)),
            val: 2,
        };
        let n1 = ListNode {
            next: Some(Box::new(n2)),
            val: 1,
        };

        let mid = Solution::middle_node(Some(Box::new(n1)));

        // Collect mid to end
        let mut items: Vec<i32> = Vec::new();
        let mut current = mid.clone();
        while let Some(ref current_val) = current {
            items.push(current_val.val);

            current = current_val.next.clone();
        }
        assert_eq!(items, vec![3, 4, 5]);
    }
}
