#[cfg(test)]
mod two_sum_test {
    #[test]
    fn simple() {
        #[derive(Copy, Clone)]
        struct Solution;
        impl Solution {
            pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
                for (i, x) in nums.iter().enumerate() {
                    for (j, y) in nums.iter().enumerate() {
                        if i == j {
                            continue;
                        }
                        if x + y == target {
                            return vec![i as i32, j as i32];
                        }
                    }
                }
                return vec![];
            }
        }
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 4, 2], 6));
    }

    use std::collections::HashMap;

    #[test]
    fn map() {
        #[derive(Copy, Clone)]
        struct Solution;
        impl Solution {
            pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
                let mut map: HashMap<i32, i32> = HashMap::new();
                for i in 0..nums.len() {
                    let idx = i as i32;
                    let diff = target - nums[i];
                    if let Some(index) = map.get(&diff) {
                        return vec![*index, idx];
                    }
                    map.insert(nums[i], idx);
                }
                return vec![];
            }
        }
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 4, 2], 6));
    }
}
