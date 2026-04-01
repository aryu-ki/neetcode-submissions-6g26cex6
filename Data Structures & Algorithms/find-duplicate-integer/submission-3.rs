impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return nums[i];
            }
        }
        -1
    }
}