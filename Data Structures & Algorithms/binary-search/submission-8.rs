impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0i32, nums.len() as i32 - 1);
        while l <= r {
            let i = (r + l) / 2;
            if nums[i as usize] < target { l = i + 1; }
            else if nums[i as usize] > target { r = i - 1; }
            else {
                return i as i32;
            }
        }
        -1
    }
}
