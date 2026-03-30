impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left_prefix = vec![0; nums.len()];
        let mut right_prefix = vec![0; nums.len()];
        for i in 0..nums.len() {
            if i == 0 {
                left_prefix[i] = 1;
            } else {
                left_prefix[i] = nums[i-1] * left_prefix[i-1];
            }
        }
        for i in (0..nums.len()).rev() {
            if i == nums.len() - 1 {
                right_prefix[i] = 1;
            } else {
                right_prefix[i] = nums[i+1] * right_prefix[i+1];
            }
        }
        let mut res = vec![0;nums.len()];
        for i in 0..nums.len() {
            res[i] = left_prefix[i] * right_prefix[i];
        }
        res
    }
}
