impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums[0] < nums[nums.len() - 1] {
            return nums[0];
        }
        let (mut l, mut r) = (0i32, nums.len() as i32 - 1);
        while l <= r {
            let i = l + (r - l) / 2;
            if nums[i as usize] < nums[l as usize] {
                if i == l + 1 {
                    return nums[i as usize];
                }
                r = i;
            } else if nums[i as usize] > nums[r as usize] {
                if i == r - 1 {
                    return nums[r as usize];
                }
                l = i;
            } else {
                return nums[i as usize];
            }
        }
        -1
    }
}
