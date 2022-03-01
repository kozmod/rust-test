#[cfg(test)]
mod two_sum_test {

    #[derive(Copy, Clone)]
    struct Solution;
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            for (i, x) in nums.iter().enumerate() {
                for (j, y) in nums.iter().enumerate() {
                    if x + y == target {
                        return vec![i as i32,j as i32]
                    }
                }
            }
            return vec![]
        }
    }
    #[test]
    fn simple() {
        assert_eq!(vec![0,1], Solution::two_sum(vec![2,7,11,15], 9));
        assert_eq!(vec![1,2], Solution::two_sum(vec![3,4,2], 6));
    }

}
